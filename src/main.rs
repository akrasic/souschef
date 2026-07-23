use clap::Parser;
use std::error::Error;

// Load modules
mod chef;
mod cli;
mod client;
mod config;
mod parse;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli_options = cli::Cli::parse();

    let config = config::KnifeConfig::from_file(&cli_options.config, &cli_options.profile)?;

    match cli_options.command {
        cli::Commands::Data { command } => match command {
            cli::DataCommands::Bag {
                command,
                secret,
                secret_file,
            } => match command {
                cli::DataBagCommands::Create { databag } => {
                    chef::databag::create_databag(&config, &databag).await?;
                }
                cli::DataBagCommands::Delete { databag, item } => match item {
                    Some(item_name) => {
                        chef::databag::delete_item(&config, &databag, &item_name).await?;
                    }
                    None => {
                        chef::databag::delete_databag(&config, &databag).await?;
                    }
                },
                cli::DataBagCommands::EncryptFile { json_file } => {
                    chef::crypt::encrypt_file(
                        &config,
                        &json_file,
                        secret.as_deref(),
                        secret_file.as_deref(),
                    )?;
                }
                cli::DataBagCommands::DecryptFile { encrypted_file } => {
                    chef::crypt::decrypt_file(
                        &config,
                        &encrypted_file,
                        secret.as_deref(),
                        secret_file.as_deref(),
                    )?;
                }
                cli::DataBagCommands::UploadItem { databag, json_file } => {
                    chef::databag::upload_item(&config, &databag, &json_file).await?;
                }
                cli::DataBagCommands::UploadEncryptedItem {
                    databag,
                    encrypted_file,
                } => {
                    chef::databag::upload_encrypted_item(
                        &config,
                        &databag,
                        &encrypted_file,
                        secret.as_deref(),
                        secret_file.as_deref(),
                    )
                    .await?;
                }
                cli::DataBagCommands::List => {
                    chef::databag::list(&config).await?;
                }
                cli::DataBagCommands::Show {
                    databag_id,
                    item_id,
                } => {
                    chef::databag::show(&config, &databag_id, item_id).await?;
                }
            },
        },

        cli::Commands::Environment { command } => match command {
            cli::EnvironmentCommands::List => {
                chef::environment::list(&config).await?;
            }
            cli::EnvironmentCommands::Show { environment_id } => {
                chef::environment::show(&config, &environment_id).await?;
            }
        },

        cli::Commands::Node { command } => match command {
            cli::NodeCommands::List => {
                chef::node::node_list(&config).await?;
            }
            cli::NodeCommands::Show { node_id } => {
                chef::node::node_show(&config, &node_id).await?;
            }
            cli::NodeCommands::Ssh { node_id, user } => {
                chef::node::node_ssh(&config, &node_id, user).await?;
            }
        },

        cli::Commands::Role { command } => match command {
            cli::RoleCommands::List => {
                chef::role::list(&config).await?;
            }
            cli::RoleCommands::Show { role_id } => {
                chef::role::show(&config, &role_id).await?;
            }
        },

        cli::Commands::Search { query, attributes } => {
            chef::search::display_search_nodes(&config, &query, &attributes).await;
        }

        cli::Commands::Login { node_id, user } => {
            chef::node::node_ssh(&config, &node_id, user).await?;
        }

        cli::Commands::Ssh {
            query,
            command,
            user,
        } => {
            chef::ssh::ssh_nodes(&config, &query, &command, user).await?;
        }

        cli::Commands::Cookbook { command } => match command {
            cli::CookbookCommands::List => {
                chef::cookbook::list(&config).await?;
            }
            cli::CookbookCommands::Show {
                cookbook_id,
                version,
            } => {
                chef::cookbook::show(&config, &cookbook_id, version).await?;
            }
        },
    }

    Ok(())
}
