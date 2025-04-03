use std::path::PathBuf;

mod cli;
mod commands;

fn main() {
    let matches = cli::setup_cli().get_matches();

    let target = matches
        .get_one::<PathBuf>("target")
        .expect("Target file is required");

    let top = matches.get_one::<usize>("top").copied();

    if let Err(e) = commands::count_words(target, top) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

