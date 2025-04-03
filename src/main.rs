use std::path::PathBuf;

mod cli;
mod commands;

fn main() {
    let matches = cli::setup_cli().get_matches();

    if let Some(target) = matches.get_one::<PathBuf>("target") {
        commands::count_words(target);
    }

    if matches.get_flag("debug") {
        println!("Debug mode is ON.");
    }
}

