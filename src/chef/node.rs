use super::search::ChefSearchResponseRaw;
use crate::{chef::search::SearchNode, client, config::KnifeConfig};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{error::Error, process::Stdio};

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Clone)]
pub struct ChefNode {
    pub automatic: ChefNodeAutomatic,
    pub chef_environment: String,
    pub name: String,
    pub run_list: Vec<String>,
}

/// Collects relevant fields from the `automatic` key from Chef API response
#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Clone)]
pub struct ChefNodeAutomatic {
    #[serde(default)]
    pub ipaddress: String,

    #[serde(default)]
    pub macaddress: String,

    #[serde(default)]
    pub hostname: String,

    #[serde(default)]
    pub os: String,

    #[serde(default)]
    pub os_version: String,

    #[serde(default)]
    pub machinename: String,

    #[serde(default)]
    pub fqdn: String,

    #[serde(default)]
    pub platform: String,

    #[serde(default)]
    pub platform_family: String,

    #[serde(default)]
    pub platform_version: String,

    #[serde(default)]
    pub recipes: Vec<String>,

    #[serde(default)]
    pub roles: Vec<String>,
}

impl ChefNode {
    pub fn display(&self) {
        println!("{}:        {}", "Node name".green().bold(), self.name);
        println!(
            "{}:       {}",
            "IP Address".green().bold(),
            self.automatic.ipaddress
        );
        println!(
            "{}: {}",
            "Chef Environment".green().bold(),
            self.chef_environment
        );
        println!(
            "{}:            {}",
            "Roles".green().bold(),
            self.automatic.roles.join(", ")
        );
        println!(
            "{}:         {}",
            "Run List".green().bold(),
            self.run_list.join(", ")
        );

        println!(
            "{}:          {}",
            "Recipes".green().bold(),
            self.automatic.recipes.join(", ")
        );

        println!(
            "{}:          {}",
            "Platform".green().bold(),
            self.automatic.platform_family
        );

        println!(
            "{}:  {}",
            "Platform version".green().bold(),
            self.automatic.platform_version
        );
        println!("\n");
    }
}

pub async fn node_list(config: &KnifeConfig) -> Result<(), Box<dyn Error>> {
    let request_path = format!("/organizations/{}/nodes", config.organization);

    match client::request::get(config, &request_path, "").await {
        Ok(n) => {
            let nodes: Value = serde_json::from_str(&n.body)?;

            for (k, _) in nodes.as_object().unwrap() {
                println!("{k}");
            }
            Ok(())
        }
        Err(e) => Err(format!("node list: {e}").into()),
    }
}

// node_show queries Chef server to display information about the node object
pub async fn node_show(config: &KnifeConfig, node_id: &str) -> Result<(), Box<dyn Error>> {
    let request_path = format!("/organizations/{}/nodes/{}", config.organization, node_id);

    match client::request::get(config, &request_path, "").await {
        Ok(n) => {
            match n.status {
                404 => {
                    println!("Node not found: {}", node_id);
                }
                200 => {
                    let node: ChefNode = serde_json::from_str(&n.body)?;
                    node.display();
                }
                _ => {
                    println!("Unkown status code: {}", n.status)
                }
            }

            Ok(())
        }
        Err(e) => Err(format!("node show: {e}").into()),
    }
}

/// node_ssh - handles the CLI call for opening a SSH connection
pub async fn node_ssh(
    config: &KnifeConfig,
    node_id: &str,
    user: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let request_path: String = format!("/organizations/{}/nodes/{}", config.organization, node_id);

    match client::request::get(config, &request_path, "").await {
        Ok(n) => {
            match n.status {
                200 => {
                    let node: ChefNode = serde_json::from_str(&n.body)?;

                    open_ssh_connection(node.automatic.ipaddress, user).await?;
                }
                404 => {
                    println!("Node not found: {}", node_id);
                }
                _ => {
                    println!("Unknown status code: {}", n.status)
                }
            }
            Ok(())
        }
        Err(e) => Err(format!("node show: {}", e).into()),
    }
}

/// open_ssh_connection - Opens a SSH client and establishes and SSH connection
/// Wraps around the ~/.ssh/config that uses the set username in
async fn open_ssh_connection(
    node_ipaddress: String,
    user: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let ssh_command = match user {
        Some(u) => format!("{}@{}", u, node_ipaddress),
        None => node_ipaddress,
    };

    let mut child = std::process::Command::new("ssh")
        .arg(ssh_command)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    let status = child.wait().unwrap();
    println!("SSH process existed with: {}", status);
    Ok(())
}
