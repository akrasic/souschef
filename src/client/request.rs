use crate::config::KnifeConfig;
use reqwest;
use std::collections::HashMap;
use std::error::Error;

use super::headers::request_headers;

pub struct ChefServerResponse {
    pub status: u16,
    pub body: String,
}

/// get - issues a GET request to Chef Server API
pub async fn get(
    config: &KnifeConfig,
    request_path: &str,
    query: &str,
) -> Result<ChefServerResponse, Box<dyn Error>> {
    let client = reqwest::ClientBuilder::new()
        .http1_title_case_headers()
        .danger_accept_invalid_certs(true)
        .build()?;

    let mut query_params = HashMap::new();
    if !query.is_empty() {
        query_params.insert("q", query);
        query_params.insert("start", "0");
        query_params.insert("rows", "1000");
    }

    let base_url = url::Url::parse(&config.chef_server_url)?;
    let full_url = base_url.join(request_path)?;
    let headers = request_headers(config, request_path, "GET")?;

    let start_timer = std::time::Instant::now();
    let response = client
        .get(full_url)
        .query(&query_params)
        .headers(headers)
        .send()
        .await?;

    println!("{}", response.status());
    let status = response.status().as_u16();
    let body = response.text().await?;

    let duration = start_timer.elapsed();
    println!("Request took: {}ms", duration.as_millis());

    let resp = ChefServerResponse { status, body };

    Ok(resp)
}
