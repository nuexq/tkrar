use clap::{Arg, ArgAction, Command, value_parser};
use std::path::PathBuf;

pub fn setup_cli() -> Command {
    Command::new("tkrar")
        .author("nuexq")
        .about("Count frequency of words in a file")
        .arg(
            Arg::new("target")
                .help("Path to the target file")
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
            Arg::new("sort")
                .help("Sort order")
                .short('s')
                .long("sort")
                .value_name("ORDER")
                .default_value("desc")
                .value_parser(["asc", "desc"]),
        )
        .arg(
            Arg::new("case_sensitive")
                .help("Enable case sensitivity when counting words")
                .short('c')
                .long("case-sensitive")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("no_stopwords")
                .help("Ignore stopwords when counting words")
                .long("no-stopwords")
                .action(ArgAction::SetTrue),
        )
}
