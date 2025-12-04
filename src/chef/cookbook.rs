use crate::{client, config::KnifeConfig};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;

/// CookbookVersions represents the list of versions for a cookbook
#[derive(Deserialize, Serialize, Debug)]
pub struct CookbookVersions {
    pub url: String,
    pub versions: Vec<CookbookVersion>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CookbookVersion {
    pub url: String,
    pub version: String,
}

/// CookbookDetail represents detailed information about a specific cookbook version
#[derive(Deserialize, Serialize, Debug)]
pub struct CookbookDetail {
    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub version: String,

    #[serde(default)]
    pub description: String,

    #[serde(default)]
    pub maintainer: String,

    #[serde(default)]
    pub maintainer_email: String,

    #[serde(default)]
    pub license: String,

    #[serde(default)]
    pub chef_type: String,

    #[serde(default)]
    pub frozen: Option<bool>,

    #[serde(default)]
    pub recipes: Vec<CookbookRecipe>,

    #[serde(default)]
    pub attributes: Vec<Value>,

    #[serde(default)]
    pub dependencies: Value,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CookbookRecipe {
    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub path: String,

    #[serde(default)]
    pub url: String,
}

impl CookbookDetail {
    pub fn display(&self) {
        println!("{}:     {}", "Cookbook".green().bold(), self.name);
        println!("{}:      {}", "Version".green().bold(), self.version);

        if !self.description.is_empty() {
            println!("{}:  {}", "Description".green().bold(), self.description);
        }

        if !self.maintainer.is_empty() {
            println!("{}:   {}", "Maintainer".green().bold(), self.maintainer);
        }

        if !self.license.is_empty() {
            println!("{}:      {}", "License".green().bold(), self.license);
        }

        if let Some(frozen) = self.frozen {
            println!("{}:       {}", "Frozen".green().bold(), frozen);
        }

        if !self.recipes.is_empty() {
            let recipe_names: Vec<&str> = self.recipes.iter().map(|r| r.name.as_str()).collect();
            println!("{}:      {}", "Recipes".green().bold(), recipe_names.join(", "));
        }

        if self.dependencies.is_object() {
            if let Some(deps) = self.dependencies.as_object() {
                if !deps.is_empty() {
                    let dep_list: Vec<String> = deps
                        .iter()
                        .map(|(k, v)| format!("{} ({})", k, v.as_str().unwrap_or("any")))
                        .collect();
                    println!("{}: {}", "Dependencies".green().bold(), dep_list.join(", "));
                }
            }
        }

        println!();
    }
}

/// list - Lists all cookbooks from Chef Server
pub async fn list(config: &KnifeConfig) -> Result<(), Box<dyn Error>> {
    let request_path = format!("/organizations/{}/cookbooks", config.organization);

    match client::request::get(config, &request_path, "").await {
        Ok(response) => match response.status {
            200 => {
                let cookbooks: Value = serde_json::from_str(&response.body)?;

                if let Some(obj) = cookbooks.as_object() {
                    for (cookbook_name, versions_info) in obj {
                        // Extract the latest version if available
                        if let Some(versions) = versions_info.get("versions") {
                            if let Some(versions_arr) = versions.as_array() {
                                if let Some(latest) = versions_arr.first() {
                                    if let Some(version) = latest.get("version") {
                                        println!(
                                            "{} ({})",
                                            cookbook_name,
                                            version.as_str().unwrap_or("unknown")
                                        );
                                        continue;
                                    }
                                }
                            }
                        }
                        println!("{}", cookbook_name);
                    }
                }
                Ok(())
            }
            404 => {
                println!("No cookbooks found.");
                Ok(())
            }
            _ => Err(format!("Failed to list cookbooks. Status: {}", response.status).into()),
        },
        Err(e) => Err(format!("cookbook list: {}", e).into()),
    }
}

/// show - Shows cookbook details, optionally for a specific version
pub async fn show(
    config: &KnifeConfig,
    cookbook: &str,
    version: Option<String>,
) -> Result<(), Box<dyn Error>> {
    match version {
        Some(v) => show_cookbook_version(config, cookbook, &v).await,
        None => show_cookbook_versions(config, cookbook).await,
    }
}

/// show_cookbook_versions - Lists all versions of a specific cookbook
async fn show_cookbook_versions(config: &KnifeConfig, cookbook: &str) -> Result<(), Box<dyn Error>> {
    let request_path = format!(
        "/organizations/{}/cookbooks/{}",
        config.organization, cookbook
    );

    match client::request::get(config, &request_path, "").await {
        Ok(response) => match response.status {
            200 => {
                let cookbook_info: Value = serde_json::from_str(&response.body)?;

                println!("{}: {}", "Cookbook".green().bold(), cookbook);
                println!("{}:", "Versions".green().bold());

                if let Some(obj) = cookbook_info.as_object() {
                    if let Some(cookbook_data) = obj.get(cookbook) {
                        if let Some(versions) = cookbook_data.get("versions") {
                            if let Some(versions_arr) = versions.as_array() {
                                for version_info in versions_arr {
                                    if let Some(version) = version_info.get("version") {
                                        println!("  - {}", version.as_str().unwrap_or("unknown"));
                                    }
                                }
                            }
                        }
                    }
                }
                Ok(())
            }
            404 => {
                println!("Cookbook '{}' not found.", cookbook);
                Ok(())
            }
            _ => Err(format!(
                "Failed to get cookbook '{}'. Status: {}",
                cookbook, response.status
            )
            .into()),
        },
        Err(e) => Err(format!("cookbook show: {}", e).into()),
    }
}

/// show_cookbook_version - Shows details of a specific cookbook version
async fn show_cookbook_version(
    config: &KnifeConfig,
    cookbook: &str,
    version: &str,
) -> Result<(), Box<dyn Error>> {
    let request_path = format!(
        "/organizations/{}/cookbooks/{}/{}",
        config.organization, cookbook, version
    );

    match client::request::get(config, &request_path, "").await {
        Ok(response) => match response.status {
            200 => {
                let cookbook_detail: CookbookDetail = serde_json::from_str(&response.body)?;
                cookbook_detail.display();
                Ok(())
            }
            404 => {
                println!("Cookbook '{}' version '{}' not found.", cookbook, version);
                Ok(())
            }
            _ => Err(format!(
                "Failed to get cookbook '{}@{}'. Status: {}",
                cookbook, version, response.status
            )
            .into()),
        },
        Err(e) => Err(format!("cookbook show: {}", e).into()),
    }
}
