use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use color_print::cprintln;

use crate::error::CliError;

pub fn count_words(
    target: &Path,
    sort: Option<&String>,
    top: Option<usize>,
    ignore_case: bool,
) -> Result<(), CliError> {
    let file = File::open(target)?;
    let reader = BufReader::new(file);

    let mut word_count = HashMap::new();

    for line in reader.lines() {
        let line = line?;

        for word in line.split_whitespace() {
            let cleaned = clean_word(word, ignore_case);

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
    if let Some(sort) = sort {
        if sort == "asc" {
            sorted.sort_by(|a, b| a.1.cmp(&b.1));
        } else {
            sorted.sort_by(|a, b| b.1.cmp(&a.1));
        }
    }

    // Print the top N words
    let count = top.unwrap_or(sorted.len());

    for (i, (word, freq)) in sorted.into_iter().take(count).enumerate() {
        cprintln!(
            "<k!>{:>2}.</> <w!>{:<15}</> <bold,y>{}</>",
            i + 1,
            word,
            freq,
        );
    }
    Ok(())
}

// clean word from unnecessary characters and implement case-insensitive
fn clean_word(word: &str, ignore_case: bool) -> String {
    let cleaned = word.trim_matches(|c: char| !c.is_alphanumeric());

    // handle case-insensitive
    if ignore_case {
        cleaned.to_lowercase()
    } else {
        cleaned.to_string()
    }
}
