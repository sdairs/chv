# Agent Guide for chv

## Overview

`chv` is a Rust CLI for working with ClickHouse, that can:

- Download/manage local ClickHouse binaries
- Interact with local ClickHouse
- Manage ClickHouse Cloud services via REST API

chv is inspired by `uv`, `pnpm` and the AWS and LocalStack CLIs.

## CLI Structure

Commands for managing local installs are at the top level.
Commands for interacting with local ClickHouse are under the `run` subcommand.
Commands for interacting with ClickHouse Cloud are under the `cloud` subcommand.

## Key Patterns

### Adding a CLI Command

1. Define in `cli.rs` using clap derive macros:
```rust
#[derive(Subcommand)]
pub enum ServiceCommands {
    /// Command description
    NewCommand {
        #[arg(long)]
        some_option: Option<String>,
    },
}
```

2. Handle in `main.rs`:
```rust
ServiceCommands::NewCommand { some_option } => {
    cloud::commands::new_command(&client, some_option, json).await
}
```

3. Implement in `cloud/commands.rs`:
```rust
pub async fn new_command(...) -> Result<(), Box<dyn std::error::Error>> {
    // Implementation
}
```

### Cloud API Requests

The `CloudClient` in `cloud/client.rs` handles auth and provides typed methods:

```rust
// GET request
async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T>

// POST request
async fn post<T: DeserializeOwned, B: Serialize>(&self, path: &str, body: &B) -> Result<T>

// PATCH request
async fn patch<T: DeserializeOwned, B: Serialize>(&self, path: &str, body: &B) -> Result<T>

// DELETE request
async fn delete(&self, path: &str) -> Result<()>
```

Add new API methods to `CloudClient`:
```rust
pub async fn do_something(&self, org_id: &str) -> Result<SomeType> {
    self.get(&format!("/organizations/{}/something", org_id)).await
}
```

### API Types

Define request/response types in `cloud/types.rs`:

```rust
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]  // API uses camelCase
pub struct MyType {
    pub id: String,
    pub some_field: Option<String>,  // Optional fields
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MyRequest {
    pub required_field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional_field: Option<String>,
}
```

### Output Format

All cloud commands support `--json` flag. Pattern:

```rust
if json {
    println!("{}", serde_json::to_string_pretty(&data)?);
} else {
    println!("Human readable: {}", data.field);
}
```

## ClickHouse Cloud OpenAPI Spec

https://api.clickhouse.cloud/v1


## Dependencies

| Crate | Purpose |
|-------|---------|
| clap | CLI argument parsing (derive macros) |
| tokio | Async runtime |
| reqwest | HTTP client |
| serde/serde_json | JSON serialization |
| thiserror | Error types |
| indicatif | Progress bars |
| base64 | API auth encoding |
| dirs | Home directory |
| futures-util | Async stream utilities |

## Storage

```
~/.clickhouse/
├── versions/
│   └── {version}/
│       └── clickhouse    # Single binary
└── default               # Contains default version string
```

## Binary Source

ClickHouse binaries from GitHub releases:
```
https://github.com/ClickHouse/ClickHouse/releases/download/v{version}-stable/clickhouse-{os}-{arch}
```
- os: `macos` or `linux`
- arch: `aarch64` or `x86_64`

## Common Tasks

### Add a new cloud subcommand

1. Add to `CloudCommands` enum in `cli.rs`
2. Add match arm in `run_cloud()` in `main.rs`
3. Implement handler in `cloud/commands.rs`
4. Add any new types to `cloud/types.rs`

### Add a new API endpoint

1. Add method to `CloudClient` in `cloud/client.rs`
2. Add request/response types to `cloud/types.rs`
3. Create command handler in `cloud/commands.rs`

### Extend service create options

1. Add field to `CreateServiceRequest` in `types.rs`
2. Add CLI arg to `ServiceCommands::Create` in `cli.rs`
3. Add to `CreateServiceOptions` struct in `commands.rs`
4. Wire up in `main.rs` match arm

## Error Handling

Use `thiserror` for error types in `error.rs`. Cloud errors wrapped as `Error::Cloud(String)`.

Commands return `Result<(), Box<dyn std::error::Error>>` and errors are printed in `main()`.
