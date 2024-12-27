use crate::{client, config::KnifeConfig, parse::traverse_json};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct ChefEnvironment {
    name: String,
    description: String,
    cookbook_versions: Value,
    default_attributes: Value,
    override_attributes: Value,
}

/// list - Lists environments
pub async fn list(config: &KnifeConfig) -> Result<(), Box<dyn Error>> {
    println!("List errors");
    let request_path = format!("/organizations/{}/environments", config.organization);

    match client::request::get(config, &request_path, "").await {
        Ok(env) => match env.status {
            200 => {
                let environments_list: Value = match serde_json::from_str(&env.body) {
                    Ok(el) => el,
                    Err(e) => return Err(format!("Environment list body parse: {}", e).into()),
                };

                for (k, _) in environments_list.as_object().unwrap() {
                    println!("{k}");
                }

                Ok(())
            }

            _ => Err(format!("Issue getting request: {}", env.status).into()),
        },

        Err(e) => Err(format!("Environment list: {}", e).into()),
    }
}

/// show - Shows the selected environment or shows an error
pub async fn show(config: &KnifeConfig, environment: &str) -> Result<(), Box<dyn Error>> {
    let request_path = format!(
        "/organizations/{}/environments/{}",
        config.organization, environment
    );

    match client::request::get(config, &request_path, "").await {
        Ok(env) => {
            // let env: Value = serde_json::from_str(&env.body)?;
            match env.status {
                200 => {
                    let chef_environment: Value = serde_json::from_str(&env.body)?;
                    //let pretty = serde_json::to_string_pretty(&chef_environment)?;

                    //println!("{}", pretty);
                    traverse_json(&chef_environment, "");
                }

                404 => {
                    println!("Environment {} not found.", environment);
                }

                _ => {
                    println!("Server said: {}", env.status);
                }
            }
            Ok(())
        }

        Err(e) => Err(e),
    }
}
