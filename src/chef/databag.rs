use crate::{client, config::KnifeConfig, parse::traverse_json};
use core::error::Error;
use serde_json::Value;

pub async fn list(config: &KnifeConfig) -> Result<(), Box<dyn Error>> {
    let request_path = format!("/organizations/{}/data", config.organization);

    match client::request::get(config, &request_path, "").await {
        Ok(db) => match db.status {
            200 => {
                let databags: Value = serde_json::from_str(&db.body)?;
                for (k, _) in databags.as_object().unwrap() {
                    println!("{k}");
                }
                Ok(())
            }
            _ => Err(format!("Status code: {}", db.status).into()),
        },

        Err(e) => Err(e),
    }
}

pub async fn show(
    config: &KnifeConfig,
    databag: String,
    item: Option<String>,
) -> Result<(), Box<dyn Error>> {
    match item {
        Some(databag_item) => show_databag_item(config, databag, databag_item).await?,

        None => list_databag_items(config, databag).await?,
    };

    Ok(())
}

/// show_databag_item - Calls Chef Server API and pulls the databag items. So far we have access to
/// unencrypoted data.
async fn show_databag_item(
    config: &KnifeConfig,
    databag: String,
    item: String,
) -> Result<(), Box<dyn Error>> {
    let request_path = format!(
        "/organizations/{}/data/{}/{}",
        config.organization, databag, item
    );

    match client::request::get(config, &request_path, "").await {
        Ok(d) => match d.status {
            200 => {
                println!("OK");
                let databag_item: Value = serde_json::from_str(&d.body)?;

                traverse_json(&databag_item, "");
                Ok(())
            }

            _ => Err(format!("Wrong status code returned: {}", d.status).into()),
        },

        Err(e) => Err(e),
    }
}

/// list_databag_items - Calls Chef Server API to get all data bag items and displays them.
async fn list_databag_items(config: &KnifeConfig, databag: String) -> Result<(), Box<dyn Error>> {
    let request_path = format!("/organizations/{}/data/{}", config.organization, databag);

    match client::request::get(config, &request_path, "").await {
        Ok(d) => match d.status {
            200 => {
                let databag_items: Value = serde_json::from_str(&d.body)?;

                for (k, _) in databag_items.as_object().unwrap() {
                    println!("{k}");
                }
                Ok(())
            }

            _ => Err(format!("Wrong status code returned: {}", d.status).into()),
        },

        Err(e) => Err(e),
    }
}
