use clap::Parser;
use regex::Regex;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "tkrar",
    author = "nuexq",
    about = "Count frequency of words in a file or a directory"
)]
pub struct CliArgs {
    /// Path to the target file
    pub target: Option<Vec<PathBuf>>,

    /// Show the N most frequent words
    #[arg(short, long, value_name = "N")]
    pub top: Option<usize>,

    /// Exclude words with less than N characters
    #[arg(short = 'm', long = "min-char", value_name = "N")]
    pub min_char: Option<usize>,

    /// Sort order (asc or desc)
    #[arg(short, long, default_value = "desc", value_parser = ["asc", "desc"])]
    pub sort: String,

    /// Enable case sensitivity when counting words
    #[arg(short = 'c', long)]
    pub case_sensitive: bool,

    /// Ignore stopwords when counting words
    #[arg(long)]
    pub no_stopwords: bool,

    /// Ignore words provided with regex patterns
    #[arg(long = "ignore-words", short = 'i', value_name = "REGEX")]
    pub ignore_words: Option<Regex>,

    /// Ignore provided files
    #[arg(
        long = "ignore-files",
        short = 'I',
        value_name = "FILE",
        value_delimiter = ','
    )]
    pub ignore_files: Option<Vec<String>>,

    /// Ignore non-alphanumeric characters
    #[arg(long)]
    pub alphabetic_only: bool,

    /// Output with the specified format
    #[arg(long = "output-format", short = 'o', value_name = "FORMAT", default_value = "text", value_parser = ["text", "json", "csv"])]
    pub output_format: String,

    /// Path to the config file
    #[arg(short = 'C', long)]
    pub config: Option<PathBuf>,
}
