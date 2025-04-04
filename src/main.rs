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

    let target = matches
        .get_one::<PathBuf>("target")
        .expect("Target file is required");

    let top = matches.get_one::<usize>("top").copied();
    let sort = matches.get_one::<String>("sort");
    let ignore_case = matches.get_flag("ignore_case");

    commands::count_words(target, sort, top, ignore_case)?;

    Ok(())
}
