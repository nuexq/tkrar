use std::fmt;

use color_print::cwrite;

#[derive(Debug)]
pub enum CliError {
    Io(std::io::Error),
    MissingRequiredArgument(String),
    Other(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CliError::Io(err) => cwrite!(f, "<bold,red>IO error</>: {}", err),
            CliError::MissingRequiredArgument(msg) => {
                cwrite!(f, "<bold,red>Missing required argument</>: {}", msg)
            }
            CliError::Other(msg) => cwrite!(f, "<bold,red>Error</>: {}", msg),
        }
    }
}

impl std::error::Error for CliError {}

impl From<std::io::Error> for CliError {
    fn from(err: std::io::Error) -> Self {
        CliError::Io(err)
    }
}
