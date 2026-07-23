use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Configuration file path
    /// Path to the knife.rb configuration, by default ~/.chef/knife.rb is used
    #[clap(short, long, verbatim_doc_comment)]
    #[arg(default_value = "~/.chef/knife.rb")]
    pub config: String,

    /// Configuration profile (chef, cinc, or default)
    #[clap(short, long, verbatim_doc_comment)]
    #[arg(default_value = "default")]
    pub profile: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage cookbooks
    Cookbook {
        #[command(subcommand)]
        command: CookbookCommands,
    },

    /// Manage data bags and data bag items
    Data {
        #[command(subcommand)]
        command: DataCommands,
    },

    /// Manage environments
    Environment {
        #[command(subcommand)]
        command: EnvironmentCommands,
    },

    /// Manage nodes
    Node {
        #[command(subcommand)]
        command: NodeCommands,
    },

    /// Manage roles
    Role {
        #[command(subcommand)]
        command: RoleCommands,
    },

    /// Search for nodes
    Search {
        /// Search query (e.g., "role:webserver" or "platform:ubuntu")
        query: String,

        /// Attributes to display (e.g., -a ipaddress -a platform)
        #[arg(short = 'a', long = "attribute", num_args = 1..)]
        attributes: Vec<String>,
    },

    /// Open an interactive SSH session to a node, resolving its IP from the Chef server
    Login {
        /// Node name
        node_id: String,

        /// SSH username
        #[arg(short = 'x', long = "ssh-user")]
        user: Option<String>,
    },

    /// Execute SSH commands on nodes matching a search query
    Ssh {
        /// Search query to find nodes
        query: String,

        /// Command to execute on each node
        command: String,

        /// SSH username
        #[arg(short = 'x', long = "ssh-user")]
        user: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum DataCommands {
    /// Manage data bags
    Bag {
        #[command(subcommand)]
        command: DataBagCommands,

        /// Secret passphrase for encryption/decryption
        #[arg(long = "secret", global = true)]
        secret: Option<String>,

        /// Path to the secret file for encryption/decryption
        #[arg(long = "secret-file", global = true)]
        secret_file: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum CookbookCommands {
    /// List all cookbooks
    List,

    /// Show cookbook details or versions
    Show {
        /// Cookbook name
        cookbook_id: String,

        /// Specific version to show (shows all versions if not specified)
        #[arg(short = 'v', long = "version")]
        version: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum NodeCommands {
    /// List all nodes from Chef Server
    List,

    /// Show node information
    Show {
        /// Node name
        node_id: String,
    },

    /// SSH into a node
    Ssh {
        /// Node name
        node_id: String,

        /// SSH username
        #[arg(short = 'x', long = "ssh-user")]
        user: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum EnvironmentCommands {
    /// List all environments
    List,

    /// Show environment details
    Show {
        /// Environment name
        environment_id: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum RoleCommands {
    /// List all roles
    List,

    /// Show role details
    Show {
        /// Role name
        role_id: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum DataBagCommands {
    /// Create a new data bag
    Create {
        /// Data bag name
        databag: String,
    },

    /// Delete a data bag or data bag item
    Delete {
        /// Data bag name
        databag: String,

        /// Item name (if omitted, deletes the entire data bag)
        #[arg(default_value = None)]
        item: Option<String>,
    },

    /// Encrypt a JSON file using the secret file
    EncryptFile {
        /// Path to the JSON file to encrypt
        json_file: String,
    },

    /// Decrypt an encrypted file using the secret file
    DecryptFile {
        /// Path to the encrypted file
        encrypted_file: String,
    },

    /// Upload a JSON file as a data bag item
    UploadItem {
        /// Data bag name
        databag: String,

        /// Path to the JSON file
        json_file: String,
    },

    /// Decrypt and upload an encrypted file as a data bag item
    UploadEncryptedItem {
        /// Data bag name
        databag: String,

        /// Path to the encrypted file
        encrypted_file: String,
    },

    /// List all data bags
    List,

    /// Show data bag contents or a specific item
    Show {
        /// Data bag name
        databag_id: String,

        /// Item name (shows all items if not specified)
        #[arg(default_value = None)]
        item_id: Option<String>,
    },
}
