# chv

A fast ClickHouse version manager.

## Installation

```bash
cargo install --path .
```

## Usage

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
