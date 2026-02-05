use crate::error::{Error, Result};
use crate::version_manager::list::{list_available_versions, VersionEntry};

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

/// Resolves a version specifier to an exact version and its channel
/// Supports:
/// - Exact: "25.1.2.3" -> ("25.1.2.3", "stable") (assumes stable for exact versions)
/// - Partial: "25.1" -> latest matching "25.1.x.x" with its actual channel
/// - Channel: "stable" -> latest stable, "lts" -> latest lts
pub async fn resolve_version(version_spec: &str) -> Result<VersionEntry> {
    // For all specifiers, fetch available versions to get accurate channel info
    let available = list_available_versions().await?;

    // If it looks like an exact version (4 parts), find its channel from the list
    if version_spec.split('.').count() == 4 {
        let channel = available
            .iter()
            .find(|e| e.version == version_spec)
            .map(|e| e.channel.clone())
            .unwrap_or_else(|| "stable".to_string());
        return Ok(VersionEntry {
            version: version_spec.to_string(),
            channel,
        });
    }

    match version_spec {
        "stable" => {
            available
                .iter()
                .find(|e| e.channel == "stable")
                .cloned()
                .ok_or_else(|| Error::NoMatchingVersion(version_spec.to_string()))
        }
        "lts" => {
            available
                .iter()
                .find(|e| e.channel == "lts")
                .cloned()
                .ok_or_else(|| Error::NoMatchingVersion(version_spec.to_string()))
        }
        partial => {
            // Find the latest version matching the partial spec
            let prefix = format!("{}.", partial);
            available
                .iter()
                .find(|e| e.version.starts_with(&prefix) || e.version == partial)
                .cloned()
                .ok_or_else(|| Error::NoMatchingVersion(partial.to_string()))
        }
    }
}

/// Builds the download URL for a specific version from GitHub releases
/// URL format: https://github.com/ClickHouse/ClickHouse/releases/download/v{version}-{channel}/clickhouse-{os}-{arch}
pub fn build_download_url(version: &str, channel: &str) -> Result<String> {
    let (os, arch) = detect_platform()?;
    Ok(format!(
        "https://github.com/ClickHouse/ClickHouse/releases/download/v{}-{}/clickhouse-{}-{}",
        version, channel, os, arch
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
    fn test_build_download_url_stable() {
        let url = build_download_url("25.12.5.44", "stable").unwrap();
        assert!(url.starts_with("https://github.com/ClickHouse/ClickHouse/releases/download/"));
        assert!(url.contains("v25.12.5.44-stable"));
    }

    #[test]
    fn test_build_download_url_lts() {
        let url = build_download_url("25.8.16.34", "lts").unwrap();
        assert!(url.starts_with("https://github.com/ClickHouse/ClickHouse/releases/download/"));
        assert!(url.contains("v25.8.16.34-lts"));
    }
}
