use clap::Parser;
use cli::CliArgs;
use std::io::{self, IsTerminal};

mod cli;
mod commands;
mod error;

use error::CliError;

fn main() {
    if let Err(e) = real_main() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn real_main() -> Result<(), CliError> {
    let args = CliArgs::parse();
    let stdin = io::stdin();

    match &args.target {
        Some(target) => commands::count_words_from_file(target, &args)?,
        None if !stdin.is_terminal() => {
            let reader = stdin.lock();
            commands::count_words_from_stdin(reader, &args)?;
        }
        None => {
            return Err(CliError::MissingRequiredArgument(
                "No target file or stdin found.".to_string(),
            ));
        }
    }

    Ok(())
}
