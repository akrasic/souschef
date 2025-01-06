use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Configuration file path
    /// Path to the knife.rb configuraiton, by default ~/.chef/knife.rb is used
    #[clap(short, long, verbatim_doc_comment)]
    #[arg(default_value = "~/.chef/knife.rb")]
    pub config: String,

    #[clap(short, long, verbatim_doc_comment)]
    #[arg(default_value = "default")]
    pub profile: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Data {
        #[command(subcommand)]
        command: DataCommands,
    },

    Environment {
        #[command(subcommand)]
        command: EnvironmentCommands,
    },
    Node {
        #[command(subcommand)]
        command: NodeCommands,
    },

    Role {
        #[command(subcommand)]
        command: RoleCommands,
    },

    Search {
        query: String,

        #[arg(short = 'a', long="attribute", num_args = 1..)]
        attributes: Vec<String>,
    },

    Ssh {
        query: String,
        command: String,

        #[arg(short = 'x', long = "ssh-user")]
        user: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum DataCommands {
    Bag {
        #[command(subcommand)]
        command: DataBagCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum NodeCommands {
    List,

    Show {
        node_id: String,
    },

    Ssh {
        node_id: String,
        #[arg(short = 'x', long = "ssh-user")]
        user: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum EnvironmentCommands {
    List,

    Show { environment_id: String },
}

#[derive(Subcommand, Debug)]
pub enum RoleCommands {
    List,

    Show { role_id: String },
}

#[derive(Subcommand, Debug)]
pub enum DataBagCommands {
    List,
    Show {
        databag_id: String,

        #[arg(default_value = None)]
        item_id: Option<String>,
    },
}
