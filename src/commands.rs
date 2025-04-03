use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

pub fn count_words(target: &Path, top: Option<usize>) -> io::Result<()> {
    let file = File::open(target)?;
    let reader = BufReader::new(file);

    let mut word_count = HashMap::new();

    for line in reader.lines() {
        for word in line?.split_whitespace() {
            let cleaned_word = word.trim_matches(|c: char| !c.is_alphanumeric());
            if !cleaned_word.is_empty() {
                word_count
                    .entry(cleaned_word.to_string())
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
            }
        }
    }

    let mut sorted: Vec<_> = word_count.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));

    let count = top.unwrap_or(sorted.len());
    for (i, (word, freq)) in sorted.into_iter().take(count).enumerate() {
        println!("{:2}. {:<15} {}", i + 1, word, freq);
    }

    Ok(())
}

