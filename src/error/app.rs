use chrono::ParseError;
use glob::{GlobError, PatternError};
use inquire::InquireError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errors {
    /// an I/O error has occurred
    #[error("an I/O error has occurred")]
    IOError(#[from] tokio::io::Error),

    /// error occurred while displaying options
    #[error("error occurred while displaying options")]
    PromptError(#[from] InquireError),

    /// error occurred while deserializing the TOML file
    #[error("error occurred while deserializing the TOML file")]
    TomlDeserializingError(#[from] toml::de::Error),

    /// failed to parse the pattern
    #[error("failed to parse the pattern")]
    ParsePatternError(#[from] PatternError),

    /// unable to read the path for iteration
    #[error("unable to read the path for iteration")]
    IterationError(#[from] GlobError),

    /// failed to parse the date
    #[error("failed to parse the date")]
    ParseDateError(#[from] ParseError),

    /// file does not exist
    #[error("file does not exist")]
    FileNotExistError,
}
