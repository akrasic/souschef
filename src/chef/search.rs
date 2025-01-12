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
            Self::IPAddress => node.ipaddress.clone(),
            Self::ChefEnvironment => node.chef_environment.clone(),
            Self::Name => node.name.clone(),
            Self::Platform => node.platform.clone(),
            Self::Roles => node.roles.join(", "),
        }
    }
}

/// Chef API reponse mapping for search object
#[derive(Deserialize, Serialize, Debug, Eq, PartialEq)]
pub struct SearchResult {
    pub total: u16,
    pub start: u16,
    pub rows: Vec<SearchNode>,
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Clone)]
pub struct SearchNode {
    pub chef_environment: String,
    pub name: String,
    pub run_list: Vec<String>,
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
/*
#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Clone)]
pub struct SearchNodeOld {
    pub automatic: SearchNodeAutomatic,
    pub chef_environment: String,
    pub name: String,
    pub run_list: Vec<String>,
}
*/

/// Collects relevant fields from the `automatic` key from Chef API response
#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Clone)]
pub struct SearchNodeAutomatic {
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

impl SearchNode {
    pub fn display(&self, attributes: &[String]) {
        if attributes.is_empty() {
            println!("{}:        {}", "Node name".green().bold(), self.name);
            println!("{}:       {}", "IP Address".green().bold(), self.ipaddress);
            println!(
                "{}: {}",
                "Chef Environment".green().bold(),
                self.chef_environment
            );
            println!(
                "{}:            {}",
                "Roles".green().bold(),
                self.roles.join(", ")
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

// Temp
#[derive(Serialize, Deserialize)]
pub struct ChefSearchResponseRaw {
    #[serde(default)]
    pub total: i32,

    #[serde(default)]
    pub start: i32,

    pub rows: Vec<ChefNodeRowRaw>,
}

#[derive(Serialize, Deserialize)]
pub struct ChefNodeRowRaw {
    #[serde(default)]
    pub url: String,
    pub data: NodeDocumentRaw,
}

#[derive(Serialize, Deserialize)]
pub struct NodeDocumentRaw {
    #[serde(default)]
    pub chef_environment: Option<String>,

    #[serde(default)]
    pub node_name: Option<String>,

    #[serde(default)]
    pub run_list: Vec<String>,

    #[serde(default)]
    pub ipaddress: Option<String>,

    #[serde(default)]
    pub macaddress: Option<String>,

    #[serde(default)]
    pub hostname: Option<String>,

    #[serde(default)]
    pub os: Option<String>,

    #[serde(default)]
    pub os_version: Option<String>,

    #[serde(default)]
    pub fqdn: Option<String>,

    #[serde(default)]
    pub platform: Option<String>,

    #[serde(default)]
    pub platform_family: Option<String>,

    #[serde(default)]
    pub platform_version: Option<String>,

    #[serde(default)]
    pub recipes: Option<Vec<String>>,

    #[serde(default)]
    pub roles: Option<Vec<String>>,
}

impl From<ChefNodeRowRaw> for SearchNode {
    fn from(raw: ChefNodeRowRaw) -> Self {
        SearchNode {
            chef_environment: raw.data.chef_environment.unwrap_or_default(),
            name: raw.data.node_name.unwrap_or_default(), // mapping node_name to name
            run_list: raw.data.run_list,
            ipaddress: raw.data.ipaddress.unwrap_or_default(),
            macaddress: raw.data.macaddress.unwrap_or_default(),
            hostname: raw.data.hostname.clone().unwrap_or_default(),
            os: raw.data.os.unwrap_or_default(),
            os_version: raw.data.os_version.unwrap_or_default(),
            machinename: raw.data.hostname.unwrap_or_default(), // assuming machinename is same as hostname
            fqdn: raw.data.fqdn.unwrap_or_default(),
            platform: raw.data.platform.unwrap_or_default(),
            platform_family: raw.data.platform_family.unwrap_or_default(),
            platform_version: raw.data.platform_version.unwrap_or_default(),
            recipes: raw.data.recipes.unwrap_or_default(),
            roles: raw.data.roles.unwrap_or_default(),
        }
    }
}

/// display_search_nodes- Calls Chef server and issues as search for the node objects, and displaysthem
pub async fn display_search_nodes(config: &KnifeConfig, query: &str, attributes: &[String]) {
    match search_nodes(config, query).await {
        Ok(nodes) => {
            // nodes.rows.iter().for_each(|n| n.display(attributes))
            for n in nodes {
                println!("{}", n.name);
            }
        }
        Err(e) => {
            println!("Error during search: {}", e);
        }
    }
}

/// search_nodes - Calls Chef server and issues as search for the node objects
pub async fn search_nodes(
    config: &KnifeConfig,
    query: &str,
) -> Result<Vec<SearchNode>, Box<dyn Error + Send + Sync>> {
    let request_path = format!("/organizations/{}/search/node", config.organization);

    match client::request::post(config, &request_path, query).await {
        Ok(k) => match k.status {
            200 => {
                //:w
                // println!("{:#?}", k.body);
                let body: ChefSearchResponseRaw = match serde_json::from_str(&k.body) {
                    Ok(body) => body,
                    Err(e) => return Err(format!("parsing return JSON: {}", e).into()),
                };

                let nodes: Vec<SearchNode> = body.rows.into_iter().map(SearchNode::from).collect();

                Ok(nodes)
            }
            _ => {
                println!("HTTP Status code: {}", k.status);
                println!("Body returned: {:#?}", k.body);
                Err(format!("HTTP Status: {}", k.status).into())
            }
        },
        Err(e) => Err(format!("search {}: {}", query, e).into()),
    }

    /*
    match client::request::get(config, &request_path, query).await {
        Ok(k) => match k.status {
            200 => {
                let body: SearchResult = match serde_json::from_str(&k.body) {
                    Ok(body) => body,
                    Err(e) => return Err(format!("parsing return JSON: {}", e).into()),
                };

                Ok(body)
            }
            _ => {
                println!("HTTP Status code: {}", k.status);
                println!("Body returned: {:#?}", k.body);
                Err(format!("HTTP Status: {}", k.status).into())
            }
        },
        Err(e) => Err(format!("search {}: {}", query, e).into()),
    }
    */
}
