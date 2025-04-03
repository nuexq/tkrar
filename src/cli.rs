use clap::{Arg, Command};

pub fn setup_cli() -> Command {
    let cli = Command::new("Tkrar")
        .author("nuexq")
        .about("count frequency of words in a file")
        .arg(
            Arg::new("target")
                .help("Path to the target file")
                .required(true)
        )
        .arg(
            Arg::new("say_hello")
                .short('s')
                .long("hello")
                .help("set hello flag")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .help("Turn debugging information on")
                .action(clap::ArgAction::SetTrue),
        );

    return cli;
}
