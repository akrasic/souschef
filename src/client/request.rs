use crate::config::KnifeConfig;
use reqwest;
use serde_json::json;
use std::collections::HashMap;
use std::error::Error;

use super::headers::request_headers;

/// Chef Server API resonse that returns the HTTP reponse status and
/// parsed body as `String`
pub struct ChefServerResponse {
    pub status: u16,
    pub body: String,
}

/// get - issues a GET request to Chef Server API returning the `ChefServerResponse` struct
pub async fn get(
    config: &KnifeConfig,
    request_path: &str,
    query: &str,
) -> Result<ChefServerResponse, Box<dyn Error + Send + Sync>> {
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
    let headers = request_headers(config, request_path, "GET", None)?;

    // let start_timer = std::time::Instant::now();
    let response = client
        .get(full_url)
        .query(&query_params)
        .headers(headers)
        .send()
        .await?;

    // println!("{}", response.status());
    let status = response.status().as_u16();
    let body = response.text().await?;

    // Get verbose
    // let duration = start_timer.elapsed();
    // println!("Request took: {}ms", duration.as_millis());

    let resp = ChefServerResponse { status, body };

    Ok(resp)
}

/// get - issues a GET request to Chef Server API returning the `ChefServerResponse` struct
pub async fn post(
    config: &KnifeConfig,
    request_path: &str,
    query: &str,
) -> Result<ChefServerResponse, Box<dyn Error + Send + Sync>> {
    let client = reqwest::ClientBuilder::new()
        .http1_title_case_headers()
        .danger_accept_invalid_certs(true)
        .build()?;

    let request_body = json!({

        "node_name": ["name"],
        "chef_environment": ["chef_environment"],
        "hostname": ["hostname"],
        "ipaddress": ["ipaddress"],
        "platform_family": ["platform_family"],
        "platform_version": ["platform_version"],
        "roles": ["roles"],
        "run_list": ["run_list"],
        "macaddress": ["macaddress"],
        "os": ["os"],
        "os_version": ["os_version"],
        "fqdn": ["fqdn"],
        "platform": ["platform_version"],
        "recipes": ["recipes"]
    });

    let body = serde_json::to_string(&request_body)?;

    let mut query_params = HashMap::new();
    if !query.is_empty() {
        query_params.insert("q", query);
        query_params.insert("start", "0");
        query_params.insert("rows", "1000");
    }

    println!("Query_Params: {:?}", query_params);
    let base_url = url::Url::parse(&config.chef_server_url)?;
    let full_url = base_url.join(request_path)?;
    let headers = request_headers(config, request_path, "POST", Some(body.clone()))?;

    println!("{}", full_url);
    println!("{}", body);

    // let start_timer = std::time::Instant::now();
    let response = client
        .post(full_url)
        .query(&query_params)
        .headers(headers)
        .body(body)
        .send()
        .await?;

    // println!("{}", response.status());
    let status = response.status().as_u16();
    let body = response.text().await?;

    // Get verbose
    // let duration = start_timer.elapsed();
    // println!("Request took: {}ms", duration.as_millis());

    let resp = ChefServerResponse { status, body };

    Ok(resp)
}
