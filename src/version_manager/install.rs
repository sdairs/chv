use crate::error::{Error, Result};
use crate::paths;
use crate::version_manager::download::download_version;
use std::os::unix::fs::PermissionsExt;

/// Installs a ClickHouse version
pub async fn install_version(version: &str, channel: &str) -> Result<()> {
    paths::ensure_dirs()?;

    let version_dir = paths::version_dir(version)?;

    // Check if already installed
    if version_dir.exists() {
        return Err(Error::VersionAlreadyInstalled(version.to_string()));
    }

    // Create the version directory
    std::fs::create_dir_all(&version_dir)?;

    // Download the binary directly to the destination
    let binary_path = version_dir.join("clickhouse");

    println!("Downloading ClickHouse {}...", version);
    download_version(version, channel, &binary_path).await?;

    // Make the binary executable
    let mut perms = std::fs::metadata(&binary_path)?.permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(&binary_path, perms)?;

    println!("ClickHouse {} installed successfully", version);
    Ok(())
}
