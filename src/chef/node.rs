use serde_json::Value;
use std::{error::Error, process::Stdio};

use crate::{chef::search::SearchNode, client, config::KnifeConfig};

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
                    let node: SearchNode = serde_json::from_str(&n.body)?;
                    node.display(&Vec::with_capacity(0));
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
                    let node: SearchNode = serde_json::from_str(&n.body)?;

                    open_ssh_connection(node.ipaddress, user).await?;
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
