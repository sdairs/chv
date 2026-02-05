use clap::{Parser, Subcommand};

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
}
