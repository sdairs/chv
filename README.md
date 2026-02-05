# chv

A fast ClickHouse version manager and cloud CLI.

## Installation

```bash
cargo install --path .
```

## Usage

### Version Management

```bash
# Install a version
chv install stable          # Latest stable release
chv install lts             # Latest LTS release
chv install 25.12           # Latest 25.12.x.x
chv install 25.12.5.44      # Exact version

# List versions
chv list                    # Installed versions
chv list --available        # Available for download

# Manage default version
chv use 25.12.5.44          # Set default
chv which                   # Show current default

# Remove a version
chv remove 25.12.5.44
```

### Running ClickHouse

```bash
# Quick SQL query (uses clickhouse local)
chv run --sql "SELECT 1"
chv run -s "SELECT * FROM system.functions LIMIT 5"

# Run clickhouse local with full options
chv run local --query "SELECT 1"
chv run local -- --help

# Run clickhouse client
chv run client
chv run client -- --host localhost --query "SHOW DATABASES"

# Run clickhouse server
chv run server
chv run server -- --config-file=/path/to/config.xml
```

### ClickHouse Cloud

Manage ClickHouse Cloud services via the API.

#### Authentication

Set environment variables:
```bash
export CLICKHOUSE_CLOUD_API_KEY=your-key
export CLICKHOUSE_CLOUD_API_SECRET=your-secret
```

Or pass via flags:
```bash
chv cloud --api-key KEY --api-secret SECRET ...
```

#### Organizations

```bash
chv cloud org list              # List organizations
chv cloud org get <org-id>      # Get organization details
```

#### Services

```bash
# List services
chv cloud service list

# Get service details
chv cloud service get <service-id>

# Create a service (minimal)
chv cloud service create --name my-service

# Create with scaling options
chv cloud service create --name my-service \
  --provider aws \
  --region us-east-1 \
  --min-replica-memory-gb 8 \
  --max-replica-memory-gb 32 \
  --num-replicas 2

# Create with specific IP allowlist
chv cloud service create --name my-service \
  --ip-allow 10.0.0.0/8 \
  --ip-allow 192.168.1.0/24

# Create from backup
chv cloud service create --name restored-service --backup-id <backup-uuid>

# Create with release channel
chv cloud service create --name my-service --release-channel fast

# Start/stop a service
chv cloud service start <service-id>
chv cloud service stop <service-id>

# Delete a service
chv cloud service delete <service-id>
```

**Service Create Options:**
| Option | Description |
|--------|-------------|
| `--name` | Service name (required) |
| `--provider` | Cloud provider: aws, gcp, azure (default: aws) |
| `--region` | Region (default: us-east-1) |
| `--min-replica-memory-gb` | Min memory per replica in GB (8-356, multiple of 4) |
| `--max-replica-memory-gb` | Max memory per replica in GB (8-356, multiple of 4) |
| `--num-replicas` | Number of replicas (1-20) |
| `--idle-scaling` | Allow scale to zero (default: true) |
| `--idle-timeout-minutes` | Min idle timeout in minutes (>= 5) |
| `--ip-allow` | IP CIDR to allow (repeatable, default: 0.0.0.0/0) |
| `--backup-id` | Backup ID to restore from |
| `--release-channel` | Release channel: slow, default, fast |
| `--data-warehouse-id` | Data warehouse ID (for read replicas) |
| `--readonly` | Make service read-only |
| `--encryption-key` | Customer disk encryption key |
| `--encryption-role` | Role ARN for disk encryption |
| `--enable-tde` | Enable Transparent Data Encryption |
| `--byoc-id` | BYOC region ID |
| `--compliance-type` | Compliance: hipaa, pci |
| `--profile` | Instance profile (enterprise) |

#### Backups

```bash
chv cloud backup list <service-id>
chv cloud backup get <service-id> <backup-id>
```

#### JSON Output

Add `--json` for machine-readable output (useful for AI agents):

```bash
chv cloud --json service list
chv cloud --json service get <service-id>
```

## Storage

Versions are stored in `~/.clickhouse/`:

```
~/.clickhouse/
├── versions/
│   └── 25.12.5.44/
│       └── clickhouse
└── default
```

## Requirements

- macOS (aarch64, x86_64) or Linux (aarch64, x86_64)
- Binaries are downloaded from [ClickHouse GitHub releases](https://github.com/ClickHouse/ClickHouse/releases)
- Cloud commands require a [ClickHouse Cloud API key](https://clickhouse.com/docs/en/cloud/manage/api)
