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
                .value_parser(value_parser!(usize)),
        )
        .arg(
            Arg::new("case-sensitive")
                .help("Count words case-sensitively")
                .short('c')
                .long("case-sensitive")
                .action(ArgAction::SetTrue)
        )
}
