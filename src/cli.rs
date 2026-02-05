use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[command(name = "chv")]
#[command(about = "ClickHouse version manager", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Install a ClickHouse version
    Install {
        /// Version to install (e.g., 25.1.2.3, 25.1, stable, lts)
        version: String,
    },

    /// List installed versions
    List {
        /// List versions available for download
        #[arg(long)]
        available: bool,
    },

    /// Set the default version
    Use {
        /// Version to use as default
        version: String,
    },

    /// Remove an installed version
    Remove {
        /// Version to remove
        version: String,
    },

    /// Show the current default version
    Which,

    /// Run ClickHouse commands
    Run(RunArgs),
}

#[derive(Args)]
pub struct RunArgs {
    /// Execute SQL query using clickhouse local
    #[arg(long, short)]
    pub sql: Option<String>,

    #[command(subcommand)]
    pub command: Option<RunCommands>,
}

#[derive(Subcommand)]
pub enum RunCommands {
    /// Run clickhouse-server
    Server {
        /// Arguments to pass to clickhouse-server
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// Run clickhouse-client
    Client {
        /// Arguments to pass to clickhouse-client
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// Run clickhouse-local
    Local {
        /// Arguments to pass to clickhouse-local
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
}
