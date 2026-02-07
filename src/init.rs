use crate::error::Result;
use std::path::PathBuf;

pub fn local_dir() -> PathBuf {
    std::env::current_dir()
        .expect("failed to get current directory")
        .join(".clickhouse")
}

pub fn is_initialized() -> bool {
    local_dir().exists()
}

pub fn init() -> Result<()> {
    let dir = local_dir();

    if is_initialized() {
        println!("Already initialized at {}", dir.display());
        return Ok(());
    }

    std::fs::create_dir_all(&dir)?;
    std::fs::write(dir.join(".gitignore"), "*\n")?;

    println!("Initialized ClickHouse project in {}", dir.display());
    Ok(())
}

pub fn version_data_dir(version: &str) -> PathBuf {
    local_dir().join(version)
}

pub fn ensure_initialized(version: &str) -> Result<()> {
    if !is_initialized() {
        init()?;
    }
    let vdir = version_data_dir(version);
    std::fs::create_dir_all(&vdir)?;
    Ok(())
}

/// Returns CLI flags that point ClickHouse data into `.clickhouse/`.
pub fn server_flags() -> Vec<String> {
    vec!["--".into(), "--path=./".into()]
}
