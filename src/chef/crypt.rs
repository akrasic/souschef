use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};

// use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use openssl::hash::MessageDigest;
use openssl::pkcs5::pbkdf2_hmac;
use openssl::symm::{Cipher, Crypter, Mode};
use rand::Rng;
use std::error::Error;
use std::fs;
use std::io::Write;

use crate::config::KnifeConfig;

pub fn decrypt_file(config: &KnifeConfig, enc_file: String) -> Result<(), Box<dyn Error>> {
    let passphrase = secret_file(config)?;

    // Read the encrypted file and decode the BASE64 encoded string
    let encoded_data = fs::read_to_string(enc_file)?;
    let encoded_data = encoded_data
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();

    let encrypted_data = BASE64.decode(encoded_data).expect("Failed to decode");

    if encrypted_data.len() < 16 || &encrypted_data[0..8] != b"Salted__" {
        return Err("Invalid SSL format".into());
    }

    let salt = &encrypted_data[8..16];
    let ciphertext = &encrypted_data[16..];

    //  Derive the key and IV using PBKDF2
    let mut key_iv = vec![0u8; 32 + 16];

    pbkdf2_hmac(
        passphrase.as_bytes(),
        salt,
        10000,
        MessageDigest::sha256(),
        &mut key_iv,
    )
    .expect("PBKDF2 derivation failed!");

    let key = &key_iv[0..32];
    let iv = &key_iv[32..48];

    let cipher = Cipher::aes_256_cbc();
    let mut crypter =
        Crypter::new(cipher, Mode::Decrypt, key, Some(iv)).expect("Failed to create crypter");
    crypter.pad(true);

    let block_size = cipher.block_size();
    let mut output = vec![0u8; ciphertext.len() + block_size];

    let count = crypter
        .update(ciphertext, &mut output)
        .expect("Decryption failed");

    // Handle finalization errors gracefully
    let final_count = match crypter.finalize(&mut output[count..]) {
        Ok(count) => count,
        Err(e) => {
            eprintln!(
                "Warning: Finalization error (possibly incorrect passphrase): {}",
                e
            );
            0 // No additional bytes written during finalization
        }
    };

    output.truncate(count + final_count);

    std::io::stdout().write_all(&output)?;

    Ok(())
}

pub fn encrypt_file(config: &KnifeConfig, json_file: String) -> Result<(), Box<dyn Error>> {
    // Read the secret file
    let passphrase = secret_file(config)?;
    let json_data = fs::read_to_string(&json_file)?;

    let mut salt = [0u8; 8];
    rand::rng().fill(&mut salt);

    let mut key_iv = vec![0u8; 32 + 16];

    pbkdf2_hmac(
        passphrase.as_bytes(),
        &salt,
        10000,
        MessageDigest::sha256(),
        &mut key_iv,
    )
    .expect("PBKDF2 derivation failed");

    let key = &key_iv[0..32];
    let iv = &key_iv[32..48];

    let cipher = Cipher::aes_256_cbc();
    let mut crypter =
        Crypter::new(cipher, Mode::Encrypt, key, Some(iv)).expect("Failed to create crypter");

    let data = json_data.as_bytes();
    let block_size = cipher.block_size();
    let mut output = vec![0u8; data.len() + block_size];

    let count = crypter
        .update(data, &mut output)
        .expect("Encryption failed");

    // Handle finalization errors gracefully
    let final_count = match crypter.finalize(&mut output[count..]) {
        Ok(count) => count,
        Err(e) => {
            eprintln!(
                "Warning: Finalization error (possibly incorrect passphrase): {}",
                e
            );
            0 // No additional bytes written during finalization
        }
    };

    output.truncate(count + final_count);

    // Prepare the OpenSSL format: Salted__<salt><encrypted data>
    let mut result = Vec::with_capacity(16 + output.len());
    result.extend_from_slice(b"Salted__");
    result.extend_from_slice(&salt);
    result.extend_from_slice(&output);

    // Base64 encode the result
    let encoded = BASE64.encode(&result);

    // Write the result to stdout
    std::io::stdout().write_all(encoded.as_bytes())?;
    std::io::stdout().write_all(b"\n")?;

    Ok(())
}

fn secret_file(config: &KnifeConfig) -> Result<String, Box<dyn Error>> {
    let secret_file = match config.secret_file.as_ref() {
        Some(s) => s,
        None => {
            return Err("knife.rb doesn't have secret_file set, please pass it".into());
        }
    };

    // Read the secret file
    let passphrase = fs::read_to_string(secret_file)?;
    let passphrase = passphrase.trim();
    Ok(passphrase.to_string())
}
