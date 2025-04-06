use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "tkrar",
    author = "nuexq",
    about = "Count frequency of words in a file"
)]
pub struct CliArgs {
    /// Path to the target file
    pub target: Option<PathBuf>,

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
}
