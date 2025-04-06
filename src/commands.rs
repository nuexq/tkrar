use indexmap::IndexMap;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use color_print::cprintln;
use stopwords::{Language, Spark, Stopwords};
use unicode_segmentation::UnicodeSegmentation;

use crate::{cli::CliArgs, error::CliError};

pub fn count_words_from_file(target: &Path, args: &CliArgs) -> Result<(), CliError> {
    let file = File::open(target)?;
    let reader = BufReader::new(file);
    count_words_from_reader(reader, args)
}

pub fn count_words_from_reader<R: BufRead>(reader: R, args: &CliArgs) -> Result<(), CliError> {
    let stopwords = if args.no_stopwords {
        Some(load_stopwords()?)
    } else {
        None
    };

    let word_count = process_words(reader, args.case_sensitive, &stopwords);

    let sorted = sort_word_counts(&args.sort, word_count);

    print_results(args.top, sorted);

    Ok(())
}

fn process_words<R: BufRead>(
    reader: R,
    case_sensitive: bool,
    stopwords: &Option<HashSet<String>>,
) -> IndexMap<String, i32> {
    let mut word_count = IndexMap::new();

    for line in reader.lines().flatten() {
        for word in line.unicode_words() {
            if let Some(cleaned) = preprocess_word(word, case_sensitive, stopwords) {
                *word_count.entry(cleaned).or_insert(0) += 1;
            }
        }
    }

    word_count
}

fn preprocess_word(
    word: &str,
    case_sensitive: bool,
    stopwords: &Option<HashSet<String>>,
) -> Option<String> {
    let cleaned = if case_sensitive {
        word.to_string()
    } else {
        word.to_lowercase()
    };

    if let Some(stops) = stopwords {
        if stops.contains(&cleaned.to_lowercase()) {
            return None;
        }
    }

    Some(cleaned)
}

fn load_stopwords() -> Result<HashSet<String>, CliError> {
    Ok(Spark::stopwords(Language::English)
        .ok_or_else(|| CliError::Other("Failed to load stopwords".into()))?
        .into_iter()
        .map(|s| s.to_string())
        .collect())
}

fn sort_word_counts(order: &str, word_count: IndexMap<String, i32>) -> Box<[(String, i32)]> {
    let mut sorted: Box<[_]> = word_count.into_iter().collect();

    match order {
        "asc" => sorted.sort_by(|a, b| a.1.cmp(&b.1)),
        _ => sorted.sort_by(|a, b| b.1.cmp(&a.1)),
    }

    sorted
}

fn print_results(top: Option<usize>, sorted: Box<[(String, i32)]>) {
    let count = top.unwrap_or(sorted.len());

    for (i, (word, freq)) in sorted.into_iter().take(count).enumerate() {
        cprintln!(
            "<k!>{:>2}.</> <w!>{:<15}</> <bold,y>{}</>",
            i + 1,
            word,
            freq,
        );
    }
}
