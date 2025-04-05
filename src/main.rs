use std::io::{self, IsTerminal};
use std::path::PathBuf;

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
    let matches = cli::setup_cli().get_matches();

    let top = matches.get_one::<usize>("top").copied();
    let sort = matches.get_one::<String>("sort");
    let case_sensitive = matches.get_flag("case_sensitive");
    let no_stopwords = matches.get_flag("no_stopwords");

    let stdin = io::stdin();
    match matches.get_one::<PathBuf>("target") {
        Some(target) => commands::count_words(target, sort, top, case_sensitive, no_stopwords)?,
        None if !stdin.is_terminal() => {
            let reader = stdin.lock();
            commands::count_words_from_reader(reader, sort, top, case_sensitive, no_stopwords)?;
        }
        None => return Err(CliError::Other("No target file or stdin".to_string())),
    }

    Ok(())
}
