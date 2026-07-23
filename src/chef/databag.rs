use crate::{chef::crypt, client, config::KnifeConfig, parse::traverse_json};
use colored::Colorize;
use core::error::Error;
use serde_json::Value;
use std::fs;

/// list - Lists all data bags from Chef Server
pub async fn list(config: &KnifeConfig) -> Result<(), Box<dyn Error>> {
    let request_path = format!("/organizations/{}/data", config.organization);

    match client::request::get(config, &request_path, "").await {
        Ok(db) => match db.status {
            200 => {
                let databags: Value = serde_json::from_str(&db.body)?;
                if let Some(obj) = databags.as_object() {
                    for k in obj.keys() {
                        println!("{k}");
                    }
                }
                Ok(())
            }
            _ => Err(format!("Failed to list data bags. Status: {}", db.status).into()),
        },
        Err(e) => Err(e),
    }
}

/// show - Shows data bag contents or a specific item
pub async fn show(
    config: &KnifeConfig,
    databag: &str,
    item: Option<String>,
) -> Result<(), Box<dyn Error>> {
    match item {
        Some(databag_item) => show_databag_item(config, databag, &databag_item).await,
        None => list_databag_items(config, databag).await,
    }
}

/// show_databag_item - Retrieves and displays a specific data bag item
async fn show_databag_item(
    config: &KnifeConfig,
    databag: &str,
    item: &str,
) -> Result<(), Box<dyn Error>> {
    let request_path = format!(
        "/organizations/{}/data/{}/{}",
        config.organization, databag, item
    );

    match client::request::get(config, &request_path, "").await {
        Ok(d) => match d.status {
            200 => {
                let databag_item: Value = serde_json::from_str(&d.body)?;
                traverse_json(&databag_item, "");
                Ok(())
            }
            404 => {
                println!(
                    "Data bag item '{}' not found in data bag '{}'.",
                    item, databag
                );
                Ok(())
            }
            _ => Err(format!(
                "Failed to get data bag item '{}'. Status: {}",
                item, d.status
            )
            .into()),
        },
        Err(e) => Err(e),
    }
}

/// list_databag_items - Lists all items in a specific data bag
async fn list_databag_items(config: &KnifeConfig, databag: &str) -> Result<(), Box<dyn Error>> {
    let request_path = format!("/organizations/{}/data/{}", config.organization, databag);

    match client::request::get(config, &request_path, "").await {
        Ok(d) => match d.status {
            200 => {
                let databag_items: Value = serde_json::from_str(&d.body)?;
                if let Some(obj) = databag_items.as_object() {
                    for k in obj.keys() {
                        println!("{k}");
                    }
                }
                Ok(())
            }
            404 => {
                println!("Data bag '{}' not found.", databag);
                Ok(())
            }
            _ => Err(format!("Failed to list data bag items. Status: {}", d.status).into()),
        },
        Err(e) => Err(e),
    }
}

/// upload_encrypted_item - Decrypts an encrypted file and uploads it as a data bag item
///
/// The encrypted file should contain valid JSON with an "id" field.
/// If the item exists, it will be updated; otherwise, it will be created.
pub async fn upload_encrypted_item(
    config: &KnifeConfig,
    databag: &str,
    encrypted_file: &str,
    secret: Option<&str>,
    secret_file: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    // Get passphrase and decrypt the file
    let passphrase = crypt::get_passphrase(config, secret, secret_file)?;
    let encoded_data = fs::read_to_string(encrypted_file)
        .map_err(|e| format!("Failed to read encrypted file '{}': {}", encrypted_file, e))?;

    let decrypted = crypt::decrypt_data(&passphrase, &encoded_data)?;

    // Parse the JSON to extract the item ID and validate structure
    let item: Value = serde_json::from_slice(&decrypted)
        .map_err(|e| format!("Decrypted content is not valid JSON: {}", e))?;

    let item_id = item
        .get("id")
        .and_then(|v| v.as_str())
        .ok_or("Data bag item must have an 'id' field")?;

    let body = String::from_utf8(decrypted)
        .map_err(|e| format!("Decrypted content is not valid UTF-8: {}", e))?;

    // First check if the data bag exists
    let databag_path = format!("/organizations/{}/data/{}", config.organization, databag);

    match client::request::get(config, &databag_path, "").await {
        Ok(response) if response.status == 404 => {
            return Err(format!(
                "Data bag '{}' does not exist. Create it first with 'knife data bag create {}'",
                databag, databag
            )
            .into());
        }
        Err(e) => {
            return Err(format!("Failed to check data bag existence: {}", e).into());
        }
        _ => {}
    }

    // Try to update existing item first (PUT)
    let item_path = format!(
        "/organizations/{}/data/{}/{}",
        config.organization, databag, item_id
    );

    match client::request::put(config, &item_path, &body).await {
        Ok(response) => match response.status {
            200 => {
                println!(
                    "{} Updated item '{}' in data bag '{}'",
                    "✓".green().bold(),
                    item_id,
                    databag
                );
                Ok(())
            }
            404 => {
                // Item doesn't exist, create it (POST to the data bag)
                match client::request::post_body(config, &databag_path, &body).await {
                    Ok(r) => match r.status {
                        201 => {
                            println!(
                                "{} Created item '{}' in data bag '{}'",
                                "✓".green().bold(),
                                item_id,
                                databag
                            );
                            Ok(())
                        }
                        409 => Err(format!(
                            "Item '{}' already exists in data bag '{}' (conflict)",
                            item_id, databag
                        )
                        .into()),
                        _ => Err(format!(
                            "Failed to create item. Status: {}, Body: {}",
                            r.status, r.body
                        )
                        .into()),
                    },
                    Err(e) => Err(format!("Failed to create item: {}", e).into()),
                }
            }
            401 => Err("Authentication failed. Check your client key and node name.".into()),
            403 => Err(format!(
                "Permission denied. You may not have access to data bag '{}'.",
                databag
            )
            .into()),
            _ => Err(format!(
                "Failed to update item. Status: {}, Body: {}",
                response.status, response.body
            )
            .into()),
        },
        Err(e) => Err(format!("Failed to upload item: {}", e).into()),
    }
}

/// upload_item - Uploads a plain JSON file as a data bag item
pub async fn upload_item(
    config: &KnifeConfig,
    databag: &str,
    json_file: &str,
) -> Result<(), Box<dyn Error>> {
    let body = fs::read_to_string(json_file)
        .map_err(|e| format!("Failed to read JSON file '{}': {}", json_file, e))?;

    // Validate JSON and extract item ID
    let item: Value = serde_json::from_str(&body)
        .map_err(|e| format!("File content is not valid JSON: {}", e))?;

    let item_id = item
        .get("id")
        .and_then(|v| v.as_str())
        .ok_or("Data bag item must have an 'id' field")?;

    // Check if data bag exists
    let databag_path = format!("/organizations/{}/data/{}", config.organization, databag);

    match client::request::get(config, &databag_path, "").await {
        Ok(response) if response.status == 404 => {
            return Err(format!("Data bag '{}' does not exist. Create it first.", databag).into());
        }
        Err(e) => {
            return Err(format!("Failed to check data bag existence: {}", e).into());
        }
        _ => {}
    }

    // Try PUT first (update), then POST (create)
    let item_path = format!(
        "/organizations/{}/data/{}/{}",
        config.organization, databag, item_id
    );

    match client::request::put(config, &item_path, &body).await {
        Ok(response) => match response.status {
            200 => {
                println!(
                    "{} Updated item '{}' in data bag '{}'",
                    "✓".green().bold(),
                    item_id,
                    databag
                );
                Ok(())
            }
            404 => {
                // Item doesn't exist, create it (POST to the data bag)
                match client::request::post_body(config, &databag_path, &body).await {
                    Ok(r) => match r.status {
                        201 => {
                            println!(
                                "{} Created item '{}' in data bag '{}'",
                                "✓".green().bold(),
                                item_id,
                                databag
                            );
                            Ok(())
                        }
                        409 => Err(format!(
                            "Item '{}' already exists in data bag '{}' (conflict)",
                            item_id, databag
                        )
                        .into()),
                        _ => Err(format!(
                            "Failed to create item. Status: {}, Body: {}",
                            r.status, r.body
                        )
                        .into()),
                    },
                    Err(e) => Err(format!("Failed to create item: {}", e).into()),
                }
            }
            401 => Err("Authentication failed. Check your client key and node name.".into()),
            403 => Err(format!(
                "Permission denied. You may not have access to data bag '{}'.",
                databag
            )
            .into()),
            _ => Err(format!(
                "Failed to update item. Status: {}, Body: {}",
                response.status, response.body
            )
            .into()),
        },
        Err(e) => Err(format!("Failed to upload item: {}", e).into()),
    }
}

/// create_databag - Creates a new data bag
pub async fn create_databag(config: &KnifeConfig, databag: &str) -> Result<(), Box<dyn Error>> {
    let request_path = format!("/organizations/{}/data", config.organization);
    let body = serde_json::json!({ "name": databag }).to_string();

    match client::request::post_body(config, &request_path, &body).await {
        Ok(response) => match response.status {
            201 => {
                println!("{} Created data bag '{}'", "✓".green().bold(), databag);
                Ok(())
            }
            409 => {
                println!("Data bag '{}' already exists.", databag);
                Ok(())
            }
            _ => Err(format!(
                "Failed to create data bag. Status: {}, Body: {}",
                response.status, response.body
            )
            .into()),
        },
        Err(e) => Err(format!("Failed to create data bag: {}", e).into()),
    }
}

/// delete_databag - Deletes a data bag
pub async fn delete_databag(config: &KnifeConfig, databag: &str) -> Result<(), Box<dyn Error>> {
    let request_path = format!("/organizations/{}/data/{}", config.organization, databag);

    match client::request::delete(config, &request_path).await {
        Ok(response) => match response.status {
            200 => {
                println!("{} Deleted data bag '{}'", "✓".green().bold(), databag);
                Ok(())
            }
            404 => {
                println!("Data bag '{}' not found.", databag);
                Ok(())
            }
            _ => Err(format!("Failed to delete data bag. Status: {}", response.status).into()),
        },
        Err(e) => Err(format!("Failed to delete data bag: {}", e).into()),
    }
}

/// delete_item - Deletes a data bag item
pub async fn delete_item(
    config: &KnifeConfig,
    databag: &str,
    item: &str,
) -> Result<(), Box<dyn Error>> {
    let request_path = format!(
        "/organizations/{}/data/{}/{}",
        config.organization, databag, item
    );

    match client::request::delete(config, &request_path).await {
        Ok(response) => match response.status {
            200 => {
                println!(
                    "{} Deleted item '{}' from data bag '{}'",
                    "✓".green().bold(),
                    item,
                    databag
                );
                Ok(())
            }
            404 => {
                println!("Item '{}' not found in data bag '{}'.", item, databag);
                Ok(())
            }
            _ => Err(format!("Failed to delete item. Status: {}", response.status).into()),
        },
        Err(e) => Err(format!("Failed to delete item: {}", e).into()),
    }
}
