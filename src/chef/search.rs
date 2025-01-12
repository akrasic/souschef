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
    Recipes,
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
            NodeAttribute::Recipes => "Recipes",
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
            "recipes" => Some(Self::Recipes),
            _ => None,
        }
    }

    /// display - pretty prints the SearchNode items
    fn display(&self, node: &SearchNode) {
        match self {
            Self::Name => node.display_node_name(),

            Self::IPAddress => node.display_ipaddress(),
            Self::ChefEnvironment => node.display_chef_environment(),
            Self::Roles => node.display_roles(),
            Self::Platform => node.display_platform(),
            Self::Recipes => node.display_recipes(),
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

// Flat Search

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Clone)]
pub struct SearchNode {
    /// Chef environment
    pub chef_environment: String,

    /// Node name
    pub name: String,

    /// Node run_list items
    pub run_list: Vec<String>,

    /// Node IP address
    #[serde(default)]
    pub ipaddress: String,

    /// MAC Address
    #[serde(default)]
    pub macaddress: String,

    /// Detected hostname
    #[serde(default)]
    pub hostname: String,

    /// Operating system
    #[serde(default)]
    pub os: String,

    /// Operating Sytem version
    #[serde(default)]
    pub os_version: String,

    /// Machine name
    #[serde(default)]
    pub machinename: String,

    /// FQDN
    #[serde(default)]
    pub fqdn: String,

    /// OS Platform
    #[serde(default)]
    pub platform: String,

    /// Platform Family
    #[serde(default)]
    pub platform_family: String,

    /// Platform version
    #[serde(default)]
    pub platform_version: String,

    /// List of recipes in the node
    #[serde(default)]
    pub recipes: Vec<String>,

    /// List of roles assigned to the node
    #[serde(default)]
    pub roles: Vec<String>,
}

impl SearchNode {
    /// display - A CLI friendy CLI display the Node information
    pub fn display(&self, attributes: &[String]) {
        if attributes.is_empty() {
            self.display_node_name();
            self.display_ipaddress();
            self.display_chef_environment();
            self.display_roles();
            self.display_runlist();
            self.display_recipes();
            self.display_platform_family();
            self.display_platform_version();

            println!("\n");
        } else {
            println!("{}", self.name);
            for attribute in attributes {
                if let Some(attr) = NodeAttribute::from_str(attribute) {
                    attr.display(self);
                }
            } // end forloop
            println!("\n");
        }
    }

    fn display_node_name(&self) {
        println!("{}:        {}", "Node name".green().bold(), self.name);
    }

    fn display_ipaddress(&self) {
        println!("{}:       {}", "IP Address".green().bold(), self.ipaddress);
    }

    fn display_chef_environment(&self) {
        println!(
            "{}: {}",
            "Chef Environment".green().bold(),
            self.chef_environment
        );
    }

    fn display_roles(&self) {
        println!(
            "{}:            {}",
            "Roles".green().bold(),
            self.roles.join(", ")
        );
    }

    fn display_platform(&self) {
        println!(
            "{}:          {}",
            "Platform".green().bold(),
            self.platform_family
        );
    }

    fn display_runlist(&self) {
        println!(
            "{}:         {}",
            "Run List".green().bold(),
            self.run_list.join(", ")
        );
    }

    fn display_recipes(&self) {
        println!(
            "{}:          {}",
            "Recipes".green().bold(),
            self.recipes.join(", ")
        );
    }

    fn display_platform_family(&self) {
        println!(
            "{}:          {}",
            "Platform".green().bold(),
            self.platform_family
        );
    }

    fn display_platform_version(&self) {
        println!(
            "{}:  {}",
            "Platform version".green().bold(),
            self.platform_version
        );
    }
}

// Chef Search POST mapping body.
#[derive(Serialize, Deserialize)]
pub struct ChefSearchResponseRaw {
    #[serde(default)]
    pub total: i32,

    #[serde(default)]
    pub start: i32,

    pub rows: Vec<ChefNodeRowRaw>,
}

/// Row mapping, cotains the URL and data fields
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

/// Implement From<> to convert the multi-layer `ChefNodeRowRaw` into a flat `SearchNode` struct
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
        // Print the output
        Ok(nodes) => nodes.iter().for_each(|n| n.display(attributes)),

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
                let body: ChefSearchResponseRaw = match serde_json::from_str(&k.body) {
                    Ok(body) => body,
                    Err(e) => return Err(format!("parsing return JSON: {}", e).into()),
                };

                let nodes: Vec<SearchNode> = body.rows.into_iter().map(SearchNode::from).collect();

                Ok(nodes)
            }

            400 => Err(
                "HTTP Status: 400 - Request parameters or body have missing or invalid fields"
                    .into(),
            ),

            401 => Err("HTTP Status: 401 - failed authentication!".into()),
            403 => Err("HTTP Code: 403 . Permission denied".into()),
            404 => Err("HTTP Code: 404 . Resource does not exist.".into()),
            406 => Err("HTTP Code: 406. Accept header does not include application/json".into()),
            _ => {
                println!("HTTP Status code: {}", k.status);
                println!("Body returned: {:#?}", k.body);
                Err(format!("HTTP Status: {}", k.status).into())
            }
        },
        Err(e) => Err(format!("search {}: {}", query, e).into()),
    }
}
