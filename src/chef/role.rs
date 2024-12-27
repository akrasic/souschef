use crate::{client, config::KnifeConfig, parse::traverse_json};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct ChefRole {
    name: String,
    description: String,
    run_list: Vec<String>,
    default_attributes: Value,
    override_attributes: Value,
    env_run_lists: Value,
}

/// lists roles
pub async fn list(config: &KnifeConfig) -> Result<(), Box<dyn Error>> {
    let request_path = format!("/organizations/{}/roles", config.organization);

    match client::request::get(config, &request_path, "").await {
        Ok(r) => {
            let roles: Value = serde_json::from_str(&r.body)?;

            for (k, _) in roles.as_object().unwrap() {
                println!("{k}")
            }
            Ok(())
        }

        Err(e) => Err(format!("role list: {e}").into()),
    }
}

/// show - Shows role settings
pub async fn show(config: &KnifeConfig, role: &str) -> Result<(), Box<dyn Error>> {
    let request_path = format!("/organizations/{}/roles/{}", config.organization, role);

    match client::request::get(config, &request_path, "").await {
        Ok(r) => {
            let roles: Value = serde_json::from_str(&r.body)?;
            traverse_json(&roles, "");

            Ok(())
        }
        Err(e) => Err(format!("role show: {e}").into()),
    }
}
