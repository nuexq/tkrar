use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub fn count_words(target: &PathBuf) {
    println!("Counting words in file {}", target.display());
    match fs::read_to_string(target) {
        Ok(text) => {
            let mut word_count: HashMap<String, u32> = HashMap::new();
            for word in text.split_whitespace() {
                word_count
                    .entry(word.trim_matches(['.', ',', '!', '?']).to_string())
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }

            let mut sorted_word_count: Vec<(_, _)> = word_count.into_iter().collect();
            sorted_word_count.sort_by(|a, b| b.1.cmp(&a.1));

            for (word, count) in sorted_word_count {
                println!("{}: {}", word, count);
            }
        }

        Err(err) => {
            println!("Error reading file: {}", err);
        }
    }
}

