use clap::{Arg, Command, value_parser};
use std::path::PathBuf;

pub fn setup_cli() -> Command {
    Command::new("Tkrar")
        .author("nuexq")
        .about("Count frequency of words in a file")
        .arg(
            Arg::new("target")
                .help("Path to the target file")
                .required(true)
                .index(1)
                .value_parser(value_parser!(PathBuf)),
        )
}
