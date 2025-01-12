use crate::config::KnifeConfig;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use chrono::Utc;
use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE};
use std::error::Error;
use std::fs;
use std::str::FromStr;

/// sign_request - Create signed headers encrypted with the users `client_key`
pub fn sign_request(
    key_path: &str,
    node_name: &str,
    http_method: &str,
    path: &str,
    body: &str,
    timestamp: &str,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    // Read the client key
    let client_key_content = match fs::read_to_string(key_path) {
        Ok(s) => s,
        Err(e) => return Err(format!("opening {}: {}", key_path, e).into()),
    };

    let key = match PKey::private_key_from_pem(client_key_content.as_bytes()) {
        Ok(k) => k,
        Err(e) => return Err(format!("reading private key from PEM {}: {}", key_path, e).into()),
    };

    let rsa = match key.rsa() {
        Ok(r) => r,
        Err(e) => return Err(format!("reading RSA key {}: {}", key_path, e).into()),
    };

    let hashed_path = BASE64.encode(openssl::hash::hash(MessageDigest::sha1(), path.as_bytes())?);
    let ops_content_hash =
        BASE64.encode(openssl::hash::hash(MessageDigest::sha1(), body.as_bytes())?);

    let user_id = BASE64.encode(openssl::hash::hash(
        MessageDigest::sha1(),
        node_name.as_bytes(),
    )?);

    let canonical_header = format!(
        "Method:{}\nHashed Path:{}\nX-Ops-Content-Hash:{}\nX-Ops-Timestamp:{}\nX-Ops-UserId:{}",
        http_method, hashed_path, ops_content_hash, timestamp, user_id
    );

    // Create a buffer for the encrypted output
    let mut buf = vec![0; rsa.size() as usize];

    // Use RSA private_encrypt with PKCS1 padding
    let encrypted_len = rsa.private_encrypt(
        canonical_header.as_bytes(),
        &mut buf,
        openssl::rsa::Padding::PKCS1,
    )?;

    // Only take the actual encrypted bytes
    buf.truncate(encrypted_len);

    Ok(BASE64.encode(buf))
}

/// request_headers - creates Chef specific request headers to authenticate with the Chef Server
/// API for your request.
pub fn request_headers(
    config: &KnifeConfig,
    request_path: &str,
    http_method: &str,
    request_body: Option<String>,
) -> Result<HeaderMap, Box<dyn Error + Send + Sync>> {
    let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();

    let body: String = match request_body {
        Some(b) => b,
        None => "".to_string(),
    };

    let signature = sign_request(
        &config.client_key,
        &config.node_name,
        http_method,
        request_path,
        &body,
        &timestamp,
    )?;

    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert("X-Chef-Version", HeaderValue::from_static("12.22.5"));
    headers.insert(
        "X-Ops-Sign",
        HeaderValue::from_static("algorithm=sha1;version=1.1;"),
    );
    headers.insert("X-Ops-Timestamp", HeaderValue::from_str(&timestamp)?);
    headers.insert("X-Ops-Userid", HeaderValue::from_str(&config.node_name)?);

    for (i, chunk) in signature.as_bytes().chunks(60).enumerate() {
        let header_name = format!("X-Ops-Authorization-{}", i + 1);
        let header_value = String::from_utf8_lossy(chunk).into_owned();
        headers.insert(
            HeaderName::from_str(&header_name)?,
            HeaderValue::from_str(&header_value)?,
        );
    }

    // X-Ops-Content-Hash
    // Hash the body.
    let x_ops_content_hash =
        BASE64.encode(openssl::hash::hash(MessageDigest::sha1(), body.as_bytes())?);
    headers.insert(
        "X-Ops-Content-Hash",
        HeaderValue::from_str(&x_ops_content_hash)?,
    );

    Ok(headers)
}
