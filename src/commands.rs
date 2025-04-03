use std::{collections::HashMap, fs, path::PathBuf};

pub fn count_words(target: &PathBuf) {
    println!("Counting words in file {}", target.display());

    match fs::read_to_string(target) {
        Ok(text) => {
            let mut word_count: HashMap<String, u32> = HashMap::new();

            for word in text.split_whitespace() {
                // removing unwanted characters from the word
                let cleaned_word = word.trim_matches(['.', ',', '!', '?', ';', ':', '"', '\'']);

                // Increment word count
                word_count
                    .entry(cleaned_word.to_string())
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }

            // Sort word count by count
            let mut sorted_word_count: Vec<(_, _)> = word_count.into_iter().collect();
            sorted_word_count.sort_by(|a, b| b.1.cmp(&a.1));

            // print sorted word count
            for (word, count) in sorted_word_count {
                println!("{}: {}", word, count);
            }
        }

        Err(err) => {
            println!("Error reading file: {}", err);
        }
    }
}
