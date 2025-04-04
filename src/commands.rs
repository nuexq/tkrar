use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::error::CliError;

fn clean_word(word: &str, case_sensitive: bool) -> String {
    let cleaned = word.trim_matches(|c: char| !c.is_alphanumeric());

    // handle case-insensitive
    if case_sensitive {
        cleaned.to_string()
    } else {
        cleaned.to_lowercase()
    }
}

pub fn count_words(
    target: &Path,
    top: Option<usize>,
    case_sensitive: bool,
) -> Result<(), CliError> {
    let file = File::open(target)?;
    let reader = BufReader::new(file);

    let mut word_count = HashMap::new();

    for line in reader.lines() {
        let line = line?;

        for word in line.split_whitespace() {
            let cleaned = clean_word(word, case_sensitive);

            if !cleaned.is_empty() {
                word_count
                    .entry(cleaned)
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
            }
        }
    }

    // Sort the words by frequency
    let mut sorted: Vec<(String, usize)> = word_count.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));

    // Print the top N words
    let count = top.unwrap_or(sorted.len());

    for (i, (word, freq)) in sorted.into_iter().take(count).enumerate() {
        println!("{:2}. {:<15} {}", i + 1, word, freq);
    }

    Ok(())
}
