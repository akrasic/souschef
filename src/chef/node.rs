use serde_json::Value;
use std::error::Error;

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
                    println!("Node not found");
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
