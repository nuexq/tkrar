use clap::{Arg, Command};

pub fn setup_cli() -> Command {
    Command::new("Tkrar")
        .author("nuexq")
        .about("Count frequency of words in a file")
        .arg(
            Arg::new("target")
                .help("Path to the target file")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .help("Turn debugging information on")
                .action(clap::ArgAction::SetTrue),
        )
}

