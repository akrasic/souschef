use crate::client;
use crate::config::KnifeConfig;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

/// NodeAttribute enumerator has the filtering deny_unknown_fields
/// for the search action.
#[derive(Debug)]
enum NodeAttribute {
    IPAddress,
    ChefEnvironment,
    Name,
    Platform,
    Roles,
}

/// Display implementation to give a &str value to a Enum type
/// needed for displaying the type name.
impl std::fmt::Display for NodeAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_str = match self {
            NodeAttribute::IPAddress => "IP Address",
            NodeAttribute::ChefEnvironment => "Chef Environment",
            NodeAttribute::Name => "Name",
            NodeAttribute::Platform => "Platform",
            NodeAttribute::Roles => "Roles",
        };

        write!(f, "{}", display_str)
    }
}

impl NodeAttribute {
    // from_str takes the attribute name and returns associated Enum field.
    // Returns as an Option<>
    fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "ipaddress" => Some(Self::IPAddress),
            "chef_environment" => Some(Self::ChefEnvironment),
            "name" => Some(Self::Name),
            "platform" => Some(Self::Platform),
            "roles" => Some(Self::Roles),
            _ => None,
        }
    }

    // get_value returns the value from the SearchNode struct
    // to the associated Enum field
    fn get_value(&self, node: &SearchNode) -> String {
        match self {
            Self::IPAddress => node.automatic.ipaddress.clone(),
            Self::ChefEnvironment => node.chef_environment.clone(),
            Self::Name => node.name.clone(),
            Self::Platform => node.automatic.platform.clone(),
            Self::Roles => node.automatic.roles.join(", "),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SearchResult {
    pub total: u16,
    pub start: u16,
    pub rows: Vec<SearchNode>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SearchNode {
    pub automatic: SearchNodeAutomatic,
    pub chef_environment: String,
    pub name: String,
    pub run_list: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SearchNodeAutomatic {
    #[serde(default)]
    pub ipaddress: String,

    #[serde[default]]
    pub hostname: String,

    #[serde[default]]
    pub platform: String,

    #[serde[default]]
    pub platform_family: String,

    #[serde[default]]
    pub platform_version: String,

    #[serde[default]]
    pub roles: Vec<String>,
}

impl SearchNode {
    pub fn display(&self, attributes: &[String]) {
        if attributes.is_empty() {
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

            println!("\n");
        } else {
            for attribute in attributes {
                if let Some(attr) = NodeAttribute::from_str(attribute) {
                    println!("{}: {}", attr, attr.get_value(self));
                }
            }
        }
    }
}

/// display_search_nodes- Calls Chef server and issues as search for the node objects, and displaysthem
pub async fn display_search_nodes(config: &KnifeConfig, query: &str, attributes: &[String]) {
    match search_nodes(config, query).await {
        Ok(nodes) => nodes.rows.iter().for_each(|n| n.display(attributes)),
        Err(e) => {
            println!("Error during search: {}", e);
        }
    }
}

/// search_nodes - Calls Chef server and issues as search for the node objects
pub async fn search_nodes(
    config: &KnifeConfig,
    query: &str,
) -> Result<SearchResult, Box<dyn Error>> {
    let request_path = format!("/organizations/{}/search/node", config.organization);

    match client::request::get(config, &request_path, query).await {
        Ok(k) => {
            let body: SearchResult = match serde_json::from_str(&k.body) {
                Ok(body) => body,
                Err(e) => return Err(format!("parsing return JSON: {}", e).into()),
            };

            Ok(body)
        }
        Err(e) => Err(format!("erro sending search request {}: {}", query, e).into()),
    }
}
