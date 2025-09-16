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
    Cookbook {
        #[command(subcommand)]
        command: CookbookCommands,
    },

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

// Data bag enum

#[derive(Subcommand, Debug)]
pub enum DataCommands {
    Bag {
        #[command(subcommand)]
        command: DataBagCommands,

        /// Secret to encrypt
        #[arg(long = "secret")]
        secret: Option<String>,

        /// Path to the secret_file
        #[arg(long = "secret-file")]
        secret_file: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum CookbookCommands {
    List,
}

#[derive(Subcommand, Debug)]
pub enum NodeCommands {
    /// List all the nodes from Chef Server
    List,

    /// Show node information
    Show { node_id: String },

    /// SSH into the node
    Ssh {
        node_id: String,
        #[arg(short = 'x', long = "ssh-user")]
        user: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum EnvironmentCommands {
    /// List environments
    List,

    /// Show environment
    Show { environment_id: String },
}

#[derive(Subcommand, Debug)]
pub enum RoleCommands {
    /// List roles
    List,

    /// Show role
    Show { role_id: String },
}

#[derive(Subcommand, Debug)]
pub enum DataBagCommands {
    /// Encrypts the file using the secret-file
    EncryptFile { json_file: String },

    /// Decrypts the file using the secret-file
    DecryptFile { encrypted_file: String },

    /// Reads the encrypted file, decrypts it and uploads it as a databag item to Chef
    /// using the secret-file
    UploadEncryptedItem {
        databag: String,
        encrypted_file: String,
    },

    /// List data bags
    List,

    /// Show data bag
    Show {
        databag_id: String,

        #[arg(default_value = None)]
        item_id: Option<String>,
    },
}
