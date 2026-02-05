mod cli;
mod error;
mod paths;
mod version_manager;

use clap::Parser;
use cli::{Cli, Commands, RunArgs, RunCommands};
use error::{Error, Result};
use std::os::unix::process::CommandExt;
use std::process::Command;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let result = run(cli.command).await;

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

async fn run(cmd: Commands) -> Result<()> {
    match cmd {
        Commands::Install { version } => install(&version).await,
        Commands::List { available } => {
            if available {
                list_available().await
            } else {
                list_installed()
            }
        }
        Commands::Use { version } => use_version(&version),
        Commands::Remove { version } => remove(&version),
        Commands::Which => which(),
        Commands::Run(args) => run_clickhouse(args),
    }
}

async fn install(version_spec: &str) -> Result<()> {
    println!("Resolving version {}...", version_spec);
    let version = version_manager::resolve_version(version_spec).await?;
    println!("Resolved to version {}", version);

    version_manager::install_version(&version).await?;
    Ok(())
}

fn list_installed() -> Result<()> {
    let versions = version_manager::list_installed_versions()?;
    let default = version_manager::get_default_version().ok();

    if versions.is_empty() {
        println!("No versions installed");
        println!("Run: chv install stable");
        return Ok(());
    }

    println!("Installed versions:");
    for v in versions {
        if Some(&v) == default.as_ref() {
            println!("  {} (default)", v);
        } else {
            println!("  {}", v);
        }
    }

    Ok(())
}

async fn list_available() -> Result<()> {
    println!("Fetching available versions...");
    let versions = version_manager::list_available_versions().await?;

    if versions.is_empty() {
        println!("No versions available");
        return Ok(());
    }

    let installed = version_manager::list_installed_versions().unwrap_or_default();

    println!("Available versions:");
    for v in versions.iter().take(20) {
        if installed.contains(v) {
            println!("  {} (installed)", v);
        } else {
            println!("  {}", v);
        }
    }

    if versions.len() > 20 {
        println!("  ... and {} more", versions.len() - 20);
    }

    Ok(())
}

fn use_version(version: &str) -> Result<()> {
    version_manager::set_default_version(version)?;
    println!("Default version set to {}", version);
    Ok(())
}

fn remove(version: &str) -> Result<()> {
    let version_dir = paths::version_dir(version)?;

    if !version_dir.exists() {
        return Err(Error::VersionNotFound(version.to_string()));
    }

    // Check if this is the default version
    if let Ok(default) = version_manager::get_default_version() {
        if default == version {
            let default_file = paths::default_file()?;
            let _ = std::fs::remove_file(default_file);
        }
    }

    std::fs::remove_dir_all(&version_dir)?;
    println!("Removed version {}", version);
    Ok(())
}

fn which() -> Result<()> {
    let version = version_manager::get_default_version()?;
    let binary = paths::binary_path(&version)?;
    println!("{} ({})", version, binary.display());
    Ok(())
}

fn run_clickhouse(args: RunArgs) -> Result<()> {
    let version = version_manager::get_default_version()?;
    let binary = paths::binary_path(&version)?;

    if !binary.exists() {
        return Err(Error::VersionNotFound(version));
    }

    // If --sql is provided, run clickhouse local with the query
    if let Some(sql) = args.sql {
        let mut cmd = Command::new(&binary);
        cmd.arg("local").arg("--query").arg(&sql);
        let err = cmd.exec();
        return Err(Error::Exec(err.to_string()));
    }

    // Otherwise, handle subcommands
    match args.command {
        Some(RunCommands::Server { args }) => {
            let mut cmd = Command::new(&binary);
            cmd.arg("server").args(&args);
            let err = cmd.exec();
            Err(Error::Exec(err.to_string()))
        }
        Some(RunCommands::Client { args }) => {
            let mut cmd = Command::new(&binary);
            cmd.arg("client").args(&args);
            let err = cmd.exec();
            Err(Error::Exec(err.to_string()))
        }
        Some(RunCommands::Local { args }) => {
            let mut cmd = Command::new(&binary);
            cmd.arg("local").args(&args);
            let err = cmd.exec();
            Err(Error::Exec(err.to_string()))
        }
        None => {
            eprintln!("Usage: chv run --sql <QUERY>");
            eprintln!("       chv run server [ARGS...]");
            eprintln!("       chv run client [ARGS...]");
            eprintln!("       chv run local [ARGS...]");
            std::process::exit(1);
        }
    }
}
