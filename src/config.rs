use core::error::Error;
use regex::Regex;
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
enum KnifeConfigError {
    #[error("Error reading {config_file}: {reason}")]
    ReadingConfigurationFile {
        config_file: PathBuf,
        reason: String,
    },

    #[error("Extracting organization from Chef server URL failed: {server_url}")]
    OrganizationExtractFailed { server_url: String },
}

#[derive(Debug)]
pub struct KnifeConfig {
    pub node_name: String,
    pub client_key: String,
    pub chef_server_url: String,
    pub organization: String,
}

impl KnifeConfig {
    /// from_file - reads the configuration file from file based on the path or profile CLI
    /// argument passed.
    ///
    /// If profile is defined, it would take priority over path.
    pub fn from_file(path: &str, profile: &str) -> Result<Self, Box<dyn Error>> {
        let homedir = match dirs::home_dir() {
            Some(p) => p,
            None => return Err("can't determing $HOME.".into()),
        };
        let off_path = match profile {
            "chef" => homedir.join(".chef/knife.rb"),
            "cinc" => homedir.join(".cinc/knife.rb"),
            _ => {
                if path.starts_with("~/") {
                    let clean_path = path.strip_prefix("~/").unwrap_or("");
                    homedir.join(clean_path)
                } else {
                    PathBuf::from(path)
                }
            }
        };

        let content = match fs::read_to_string(off_path.clone()) {
            Ok(c) => c,
            Err(e) => {
                return Err(KnifeConfigError::ReadingConfigurationFile {
                    config_file: off_path,
                    reason: e.to_string(),
                }
                .to_string()
                .into())
            }
        };

        // Extract Chef server configuration files from knife.rb ,f
        let node_name_re = Regex::new(r#"node_name\s+['"](.+?)['"]"#)?;
        let client_key_re = Regex::new(r#"client_key\s+['"](.+?)['"]"#)?;
        let server_url_re = Regex::new(r#"chef_server_url\s+['"](.+?)['"]"#)?;

        let node_name = node_name_re
            .captures(&content)
            .ok_or("node_name not found")?[1]
            .to_string();

        let client_key = client_key_re
            .captures(&content)
            .ok_or("client_key not found")?[1]
            .to_string();

        let chef_server_url = server_url_re
            .captures(&content)
            .ok_or("chef_server_url not found")?[1]
            .to_string();

        // Extract the organization from the chef_server_url
        let organization = match chef_server_url.split("/").last() {
            Some(org) => org.to_string(),
            None => {
                return Err(KnifeConfigError::OrganizationExtractFailed {
                    server_url: chef_server_url,
                }
                .into())
            }
        };

        // Expand ~ to home directory if present
        let client_key = if client_key.starts_with('~') {
            homedir
                .join(&client_key[2..])
                .to_string_lossy()
                .into_owned()
        } else {
            client_key
        };

        Ok(KnifeConfig {
            node_name,
            client_key,
            chef_server_url,
            organization,
        })
    }
}
