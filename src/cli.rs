use clap::{Arg, ArgAction, Command, value_parser};
use std::path::PathBuf;

pub fn setup_cli() -> Command {
    Command::new("tkrar")
        .author("nuexq")
        .about("Count frequency of words in a file")
        .arg(
            Arg::new("target")
                .help("Path to the target file")
                .required(true)
                .index(1)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            Arg::new("top")
                .help("Show the N most frequent words")
                .short('t')
                .long("top")
                .value_name("N")
                .value_parser(value_parser!(usize)),
        )
        .arg(
            Arg::new("ignore_case")
                .help("Ignore case when counting words")
                .short('i')
                .long("ignore_case")
                .action(ArgAction::SetTrue),
        )
}
