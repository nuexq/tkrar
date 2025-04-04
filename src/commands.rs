use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::error::CliError;

pub fn count_words(target: &Path, top: Option<usize>) -> Result<(), CliError> {
    let file = File::open(target)?;
    let reader = BufReader::new(file);

    let mut word_count = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        for word in line.split_whitespace() {
            let cleaned = word.trim_matches(|c: char| !c.is_alphanumeric());
            if !cleaned.is_empty() {
                word_count
                    .entry(cleaned.to_string())
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
            }
        }
    }

    let mut sorted: Vec<(String, usize)> = word_count.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));

    let count = top.unwrap_or(sorted.len());
    for (i, (word, freq)) in sorted.into_iter().take(count).enumerate() {
        println!("{:2}. {:<15} {}", i + 1, word, freq);
    }

    Ok(())
}

