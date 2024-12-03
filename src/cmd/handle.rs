use crate::error::app::Errors;
use anyhow::Result;
use inquire::Select;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

async fn create_config_file(path: &PathBuf) -> Result<(), Errors> {
    let mut config_file = File::create(path).await?;

    config_file
        .write_all(include_bytes!("katharsis.config.toml"))
        .await?;

    Ok(())
}

/// Generate a config file.
///
/// # Examples
///
/// ```no_run
/// use anyhow::Result;
/// use katharsis::cmd;
/// use std::path::PathBuf;
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     let path = PathBuf::from("katharsis.config.toml");
///
///     cmd::handle::init(&path).await?;
///
///     Ok(())
/// }
/// ```
///
/// # Errors
///
/// - When the file’s existence cannot be verified.
/// - When the file cannot be created.
/// - When the options cannot be displayed.
/// - When the file cannot be written to.
pub async fn init(path: &PathBuf) -> Result<(), Errors> {
    if path.try_exists()? {
        let options: Vec<&str> = vec!["Yes", "No"];
        let ans: &str = Select::new("A katharsis.config.toml already exists in the current directory. Do you want to overwrite it?", options).prompt()?;

        if ans == "yes" {
            create_config_file(path).await?;
        }
    } else {
        create_config_file(path).await?;
    }

    Ok(())
}
