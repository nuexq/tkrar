use clap::Parser;
use cli::CliArgs;
use regex::Regex;
use serde::Deserialize;
use std::io::{self, IsTerminal};
use std::{fs, path::PathBuf};
use toml;

mod cli;
mod commands;
mod error;

use error::CliError;

#[derive(Deserialize, Debug)]
struct Config {
    top: Option<usize>,
    min_char: Option<usize>,
    ignore_words: Option<String>,
    ignore_files: Option<Vec<String>>,
}

fn main() {
    if let Err(e) = real_main() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn real_main() -> Result<(), CliError> {
    let args = CliArgs::parse();

    let args = match &args.config {
        Some(config_path) => {
            let config = read_config(config_path).unwrap_or_else(|_| Config {
                top: None,
                min_char: None,
                ignore_words: None,
                ignore_files: None,
            });
            let args = CliArgs {
                top: args.top.or(config.top),
                min_char: args.min_char.or(config.min_char),
                sort: args.sort,
                case_sensitive: args.case_sensitive,
                no_stopwords: args.no_stopwords,
                ignore_words: args.ignore_words.or_else(|| {
                    config
                        .ignore_words
                        .as_ref()
                        .map(|regex| Regex::new(regex).unwrap())
                }),
                ignore_files: args.ignore_files.or(config.ignore_files),
                alphabetic_only: args.alphabetic_only,
                output_format: args.output_format,
                ..args
            };
            args
        }
        None => args,
    };

    let stdin = io::stdin();

    match &args.target {
        Some(target) => commands::count_freq_of_words(target, &args)?,
        None if !stdin.is_terminal() => {
            let reader = stdin.lock();
            commands::count_words_from_stdin(reader, &args)?;
        }
        None => {
            return Err(CliError::MissingRequiredArgument(
                "No target file or stdin found.".to_string(),
            ));
        }
    }

    Ok(())
}

fn read_config(config_path: &PathBuf) -> Result<Config, CliError> {
    let config_content = fs::read_to_string(config_path)
        .map_err(|e| CliError::Other(format!("Failed to read config file: {}", e)))?;
    let config: Config = toml::de::from_str(&config_content)
        .map_err(|e| CliError::Other(format!("Failed to parse config file: {}", e)))?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_config() {
        let config_path = PathBuf::from("example.toml");
        let config = read_config(&config_path).unwrap();
        assert_eq!(config.top, Some(10));
        assert_eq!(config.min_char, Some(3));
        assert_eq!(config.ignore_words, Some("ignored|hi|hidden".to_string()));
        assert_eq!(
            config.ignore_files,
            Some(vec!["src/ignored.txt".to_string(), "dummy.txt".to_string()])
        );
    }
}
