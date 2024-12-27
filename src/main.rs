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
            cli::DataCommands::Bag { command } => match command {
                cli::DataBagCommands::List => {
                    chef::databag::list(&config).await?;
                    println!("List data bag");
                }

                cli::DataBagCommands::Show {
                    databag_id,
                    item_id,
                } => {
                    chef::databag::show(&config, databag_id, item_id).await?;
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
            chef::search::search_nodes(&config, &query, &attributes).await?;
        }
    }

    Ok(())
}
