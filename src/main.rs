use anyhow::Result;
use clap::{Parser, Subcommand};
use katharsis::{arg, cmd};
use std::path::PathBuf;

#[derive(Subcommand)]
enum Commands {
    /// Generate a katharsis.config.toml file in the current directory
    Init,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Use a katharsis.config.toml file as the config
    #[arg(env, short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut path = PathBuf::from("katharsis.config.toml");

    if let Some(Commands::Init) = cli.command {
        cmd::handle::init(&path).await?;
        return Ok(());
    }

    if let Some(config_path) = cli.config {
        path = config_path;
    }

    arg::handle::builder(&path).await?;

    Ok(())
}
