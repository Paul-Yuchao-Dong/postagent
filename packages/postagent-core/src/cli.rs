use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "postagent-core",
    version,
    about = "CLI collection tool for agents",
    disable_help_subcommand = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Output format: markdown / json
    #[arg(long, default_value = "markdown", global = true)]
    pub format: String,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Search projects by keyword
    Search {
        /// Search query
        query: String,
    },
    /// Get project/resource/action details (progressive discovery)
    Help {
        /// Project name
        project: Option<String>,
        /// Resource name
        resource: Option<String>,
        /// Action name
        action: Option<String>,
    },
    /// Save API key for a project
    Auth {
        /// Project name
        project: String,
    },
    /// Send an HTTP request
    Request {
        /// Request URL
        url: String,
        /// HTTP method
        #[arg(short = 'X', long)]
        method: Option<String>,
        /// Request header (repeatable)
        #[arg(short = 'H', long, num_args = 1)]
        header: Vec<String>,
        /// Request body
        #[arg(short = 'd', long)]
        data: Option<String>,
    },
}
