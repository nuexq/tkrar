use std::fmt;

use color_print::cwrite;

#[derive(Debug)]
#[allow(dead_code)]
pub enum CliError {
    Io(std::io::Error),
    Other(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CliError::Io(err) => cwrite!(f, "<bold,red>IO error</>: {}", err),
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
