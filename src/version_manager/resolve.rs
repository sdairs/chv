use crate::error::{Error, Result};
use crate::version_manager::list::list_available_versions;

/// Detects the current platform and returns (os, arch) for download URLs
/// Returns values matching GitHub release naming: (macos|linux, aarch64|x86_64)
pub fn detect_platform() -> Result<(&'static str, &'static str)> {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;

    let os_name = match os {
        "macos" => "macos",
        "linux" => "linux",
        _ => {
            return Err(Error::UnsupportedPlatform {
                os: os.to_string(),
                arch: arch.to_string(),
            })
        }
    };

    let arch_name = match arch {
        "x86_64" => "x86_64",
        "aarch64" => "aarch64",
        _ => {
            return Err(Error::UnsupportedPlatform {
                os: os.to_string(),
                arch: arch.to_string(),
            })
        }
    };

    Ok((os_name, arch_name))
}

/// Resolves a version specifier to an exact version string
/// Supports:
/// - Exact: "25.1.2.3" -> "25.1.2.3"
/// - Partial: "25.1" -> latest matching "25.1.x.x"
/// - Channel: "stable" or "lts" -> latest from channel
pub async fn resolve_version(version_spec: &str) -> Result<String> {
    // If it looks like an exact version (4 parts), return as-is
    if version_spec.split('.').count() == 4 {
        return Ok(version_spec.to_string());
    }

    // For channels or partial versions, fetch available versions
    let available = list_available_versions().await?;

    match version_spec {
        "stable" | "lts" => {
            // Return the latest version (first in the sorted list)
            available
                .first()
                .cloned()
                .ok_or_else(|| Error::NoMatchingVersion(version_spec.to_string()))
        }
        partial => {
            // Find the latest version matching the partial spec
            let prefix = format!("{}.", partial);
            available
                .iter()
                .find(|v| v.starts_with(&prefix) || *v == partial)
                .cloned()
                .ok_or_else(|| Error::NoMatchingVersion(partial.to_string()))
        }
    }
}

/// Builds the download URL for a specific version from GitHub releases
/// URL format: https://github.com/ClickHouse/ClickHouse/releases/download/v{version}-stable/clickhouse-{os}-{arch}
pub fn build_download_url(version: &str) -> Result<String> {
    let (os, arch) = detect_platform()?;
    Ok(format!(
        "https://github.com/ClickHouse/ClickHouse/releases/download/v{}-stable/clickhouse-{}-{}",
        version, os, arch
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_platform() {
        let result = detect_platform();
        assert!(result.is_ok());
        let (os, arch) = result.unwrap();
        assert!(os == "macos" || os == "linux");
        assert!(arch == "x86_64" || arch == "aarch64");
    }

    #[test]
    fn test_build_download_url() {
        let url = build_download_url("25.12.5.44").unwrap();
        assert!(url.starts_with("https://github.com/ClickHouse/ClickHouse/releases/download/"));
        assert!(url.contains("v25.12.5.44-stable"));
    }
}
