use clap::{Args, Parser, Subcommand};

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

    /// Initialize a project-local ClickHouse configuration
    Init,

    /// Run ClickHouse commands
    Run(RunArgs),

    /// ClickHouse Cloud API commands
    Cloud(CloudArgs),
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

#[derive(Args)]
pub struct CloudArgs {
    /// API key (or set CLICKHOUSE_CLOUD_API_KEY)
    #[arg(long, global = true)]
    pub api_key: Option<String>,

    /// API secret (or set CLICKHOUSE_CLOUD_API_SECRET)
    #[arg(long, global = true)]
    pub api_secret: Option<String>,

    /// Output as JSON
    #[arg(long, global = true)]
    pub json: bool,

    #[command(subcommand)]
    pub command: CloudCommands,
}

#[derive(Subcommand)]
pub enum CloudCommands {
    /// Organization commands
    Org {
        #[command(subcommand)]
        command: OrgCommands,
    },

    /// Service commands
    Service {
        #[command(subcommand)]
        command: ServiceCommands,
    },

    /// Backup commands
    Backup {
        #[command(subcommand)]
        command: BackupCommands,
    },
}

#[derive(Subcommand)]
pub enum OrgCommands {
    /// List organizations
    List,

    /// Get organization details
    Get {
        /// Organization ID
        org_id: String,
    },
}

#[derive(Subcommand)]
pub enum ServiceCommands {
    /// List all services
    List {
        /// Organization ID (auto-detected if not specified)
        #[arg(long)]
        org_id: Option<String>,
    },

    /// Get service details
    Get {
        /// Service ID
        service_id: String,

        /// Organization ID (auto-detected if not specified)
        #[arg(long)]
        org_id: Option<String>,
    },

    /// Create a new service
    Create {
        /// Service name (required)
        #[arg(long)]
        name: String,

        /// Cloud provider: aws, gcp, azure (required)
        #[arg(long, default_value = "aws")]
        provider: String,

        /// Region (required). Examples: us-east-1, eu-west-1, us-central1
        #[arg(long, default_value = "us-east-1")]
        region: String,

        /// Minimum memory per replica in GB (8-356, multiple of 4)
        #[arg(long)]
        min_replica_memory_gb: Option<u32>,

        /// Maximum memory per replica in GB (8-356, multiple of 4)
        #[arg(long)]
        max_replica_memory_gb: Option<u32>,

        /// Number of replicas (1-20)
        #[arg(long)]
        num_replicas: Option<u32>,

        /// Allow scale to zero when idle (default: true)
        #[arg(long)]
        idle_scaling: Option<bool>,

        /// Minimum idle timeout in minutes (>= 5)
        #[arg(long)]
        idle_timeout_minutes: Option<u32>,

        /// IP addresses to allow (CIDR format, e.g., "0.0.0.0/0"). Can be specified multiple times
        #[arg(long = "ip-allow")]
        ip_allow: Vec<String>,

        /// Backup ID to restore from
        #[arg(long)]
        backup_id: Option<String>,

        /// Release channel: slow, default, fast
        #[arg(long)]
        release_channel: Option<String>,

        /// Data warehouse ID (for creating read replicas)
        #[arg(long)]
        data_warehouse_id: Option<String>,

        /// Make service read-only (requires --data-warehouse-id)
        #[arg(long)]
        readonly: bool,

        /// Customer-provided disk encryption key
        #[arg(long)]
        encryption_key: Option<String>,

        /// Role ARN for disk encryption
        #[arg(long)]
        encryption_role: Option<String>,

        /// Enable Transparent Data Encryption (enterprise only)
        #[arg(long)]
        enable_tde: bool,

        /// BYOC region ID
        #[arg(long)]
        byoc_id: Option<String>,

        /// Compliance type: hipaa, pci
        #[arg(long)]
        compliance_type: Option<String>,

        /// Instance profile (enterprise only): v1-default, v1-highmem-xs, etc.
        #[arg(long)]
        profile: Option<String>,

        /// Organization ID (auto-detected if not specified)
        #[arg(long)]
        org_id: Option<String>,
    },

    /// Delete a service
    Delete {
        /// Service ID
        service_id: String,

        /// Organization ID (auto-detected if not specified)
        #[arg(long)]
        org_id: Option<String>,
    },

    /// Start a service
    Start {
        /// Service ID
        service_id: String,

        /// Organization ID (auto-detected if not specified)
        #[arg(long)]
        org_id: Option<String>,
    },

    /// Stop a service
    Stop {
        /// Service ID
        service_id: String,

        /// Organization ID (auto-detected if not specified)
        #[arg(long)]
        org_id: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum BackupCommands {
    /// List backups for a service
    List {
        /// Service ID
        service_id: String,

        /// Organization ID (auto-detected if not specified)
        #[arg(long)]
        org_id: Option<String>,
    },

    /// Get backup details
    Get {
        /// Service ID
        service_id: String,

        /// Backup ID
        backup_id: String,

        /// Organization ID (auto-detected if not specified)
        #[arg(long)]
        org_id: Option<String>,
    },
}
