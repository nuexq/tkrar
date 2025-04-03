mod cli;

fn main() {
    let matches = cli::setup_cli().get_matches();

    if let Some(target) = matches.get_one::<String>("target") {
        println!("Value for target: {target}");
    }

    if matches.get_flag("say_hello") {
        println!("Hello flag is set");
    }

    if matches.get_flag("debug") {
        println!("Debug mode is on");
    } else {
        println!("Debug mode is off");
    }
}
