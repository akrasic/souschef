use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use openssl::hash::MessageDigest;
use openssl::pkcs5::pbkdf2_hmac;
use openssl::symm::{Cipher, Crypter, Mode};
use rand::Rng;
use std::error::Error;
use std::fs;
use std::io::Write;

use crate::config::KnifeConfig;

/// Decrypt base64-encoded data and return the plaintext bytes
pub fn decrypt_data(passphrase: &str, encoded_data: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let encoded_data: String = encoded_data
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    let encrypted_data = BASE64.decode(&encoded_data)?;

    if encrypted_data.len() < 16 || &encrypted_data[0..8] != b"Salted__" {
        return Err("Invalid OpenSSL format".into());
    }

    let salt = &encrypted_data[8..16];
    let ciphertext = &encrypted_data[16..];

    let (key, iv) = derive_key_iv(passphrase, salt)?;

    let cipher = Cipher::aes_256_cbc();
    let mut crypter = Crypter::new(cipher, Mode::Decrypt, &key, Some(&iv))?;
    crypter.pad(true);

    let mut output = vec![0u8; ciphertext.len() + cipher.block_size()];
    let count = crypter.update(ciphertext, &mut output)?;
    let final_count = crypter.finalize(&mut output[count..])?;
    output.truncate(count + final_count);

    Ok(output)
}

/// Encrypt data and return base64-encoded ciphertext in OpenSSL format
pub fn encrypt_data(passphrase: &str, plaintext: &[u8]) -> Result<String, Box<dyn Error>> {
    let mut salt = [0u8; 8];
    rand::rng().fill(&mut salt);

    let (key, iv) = derive_key_iv(passphrase, &salt)?;

    let cipher = Cipher::aes_256_cbc();
    let mut crypter = Crypter::new(cipher, Mode::Encrypt, &key, Some(&iv))?;
    crypter.pad(true);

    let mut output = vec![0u8; plaintext.len() + cipher.block_size()];
    let count = crypter.update(plaintext, &mut output)?;
    let final_count = crypter.finalize(&mut output[count..])?;
    output.truncate(count + final_count);

    // OpenSSL format: Salted__<salt><ciphertext>
    let mut result = Vec::with_capacity(16 + output.len());
    result.extend_from_slice(b"Salted__");
    result.extend_from_slice(&salt);
    result.extend_from_slice(&output);

    Ok(BASE64.encode(&result))
}

/// Derive key and IV from passphrase and salt using PBKDF2
fn derive_key_iv(passphrase: &str, salt: &[u8]) -> Result<(Vec<u8>, Vec<u8>), Box<dyn Error>> {
    let mut key_iv = vec![0u8; 32 + 16];
    pbkdf2_hmac(
        passphrase.as_bytes(),
        salt,
        10000,
        MessageDigest::sha256(),
        &mut key_iv,
    )?;

    let key = key_iv[0..32].to_vec();
    let iv = key_iv[32..48].to_vec();
    Ok((key, iv))
}

/// Get passphrase from config or CLI overrides
///
/// Priority: direct secret > CLI secret_file > config secret_file
pub fn get_passphrase(
    config: &KnifeConfig,
    secret: Option<&str>,
    secret_file: Option<&str>,
) -> Result<String, Box<dyn Error>> {
    // Direct secret takes highest priority
    if let Some(s) = secret {
        return Ok(s.to_string());
    }

    // Then CLI secret_file, then config secret_file
    let file_path = secret_file
        .or(config.secret_file.as_deref())
        .ok_or("No secret or secret_file provided. Set knife[:secret_file] in knife.rb or pass --secret/--secret-file")?;

    let passphrase = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read secret file '{}': {}", file_path, e))?
        .trim()
        .to_string();

    Ok(passphrase)
}

/// Decrypt a file and write plaintext to stdout (CLI command)
pub fn decrypt_file(
    config: &KnifeConfig,
    enc_file: &str,
    secret: Option<&str>,
    secret_file: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    let passphrase = get_passphrase(config, secret, secret_file)?;
    let encoded_data = fs::read_to_string(enc_file)
        .map_err(|e| format!("Failed to read encrypted file '{}': {}", enc_file, e))?;

    let plaintext = decrypt_data(&passphrase, &encoded_data)?;
    std::io::stdout().write_all(&plaintext)?;

    Ok(())
}

/// Encrypt a file and write base64 ciphertext to stdout (CLI command)
pub fn encrypt_file(
    config: &KnifeConfig,
    json_file: &str,
    secret: Option<&str>,
    secret_file: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    let passphrase = get_passphrase(config, secret, secret_file)?;
    let plaintext = fs::read_to_string(json_file)
        .map_err(|e| format!("Failed to read JSON file '{}': {}", json_file, e))?;

    let encrypted = encrypt_data(&passphrase, plaintext.as_bytes())?;
    println!("{}", encrypted);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let passphrase = "test-passphrase";
        let plaintext = b"Hello, World!";

        let encrypted = encrypt_data(passphrase, plaintext).unwrap();
        let decrypted = decrypt_data(passphrase, &encrypted).unwrap();

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_decrypt_invalid_format() {
        let passphrase = "test";
        let invalid = BASE64.encode(b"NotSalted");

        let result = decrypt_data(passphrase, &invalid);
        assert!(result.is_err());
    }
}
