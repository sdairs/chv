use crate::error::Result;
use std::path::PathBuf;

pub fn local_dir() -> PathBuf {
    std::env::current_dir()
        .expect("failed to get current directory")
        .join(".clickhouse")
}

pub fn project_dir() -> PathBuf {
    std::env::current_dir()
        .expect("failed to get current directory")
        .join("clickhouse")
}

pub fn is_initialized() -> bool {
    local_dir().exists()
}

pub fn init() -> Result<()> {
    let dir = local_dir();

    if is_initialized() {
        println!("Already initialized at {}", dir.display());
    } else {
        std::fs::create_dir_all(&dir)?;
        std::fs::write(dir.join(".gitignore"), "*\n")?;
        println!("Initialized ClickHouse project in {}", dir.display());
    }

    create_project_scaffold()?;

    Ok(())
}

fn create_project_scaffold() -> Result<()> {
    let dir = project_dir();
    let subdirs = ["tables", "materialized_views", "queries", "seed"];

    let mut created = false;
    for subdir in &subdirs {
        let path = dir.join(subdir);
        if !path.exists() {
            std::fs::create_dir_all(&path)?;
            std::fs::write(path.join(".gitkeep"), "")?;
            created = true;
        }
    }

    if created {
        println!(
            "Created project scaffold in {}/ (tables, materialized_views, queries, seed)",
            dir.display()
        );
    }

    Ok(())
}

pub fn version_data_dir(version: &str) -> PathBuf {
    local_dir().join(version)
}

pub fn ensure_initialized(version: &str) -> Result<()> {
    let dir = local_dir();
    if !dir.exists() {
        std::fs::create_dir_all(&dir)?;
        std::fs::write(dir.join(".gitignore"), "*\n")?;
    }
    let vdir = version_data_dir(version);
    std::fs::create_dir_all(&vdir)?;
    Ok(())
}

/// Returns CLI flags that point ClickHouse data into `.clickhouse/`.
pub fn server_flags() -> Vec<String> {
    vec!["--".into(), "--path=./".into()]
}
