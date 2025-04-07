use once_cell::sync::Lazy;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, StdinLock},
    path::PathBuf,
    sync::Arc,
};

use color_print::cprintln;
use stopwords::{Language, Spark, Stopwords};
use unicode_segmentation::UnicodeSegmentation;

use crate::{cli::CliArgs, error::CliError};

static STOPWORDS: Lazy<Arc<HashSet<String>>> = Lazy::new(|| {
    Arc::new(
        Spark::stopwords(Language::English)
            .unwrap()
            .into_iter()
            .map(|s| s.to_string())
            .collect(),
    )
});

// count_words fn for single or multiple files
pub fn count_words_from_file(target: &Vec<PathBuf>, args: &CliArgs) -> Result<(), CliError> {
    // defind a variable to store the word count of each file
    let mut files_word_count = HashMap::new();

    // iterate over the files in the target vector
    for file in target {
        let open_file = File::open(file)?;
        let reader = BufReader::new(open_file);
        let word_count = count_words_from_reader(reader, args)?;
        files_word_count.extend(word_count);
    }

    output_results(args, files_word_count);

    Ok(())
}

// count_words fn for stdin
pub fn count_words_from_stdin(reader: StdinLock, args: &CliArgs) -> Result<(), CliError> {
    let word_count = count_words_from_reader(reader, args)?;

    output_results(args, word_count);

    Ok(())
}

// return a map of word counts
pub fn count_words_from_reader<R: BufRead>(
    reader: R,
    args: &CliArgs,
) -> Result<HashMap<String, i32>, CliError> {
    let stopwords = if args.no_stopwords {
        Some(load_stopwords()?)
    } else {
        None
    };

    let word_count = process_words(
        reader,
        args.case_sensitive,
        args.min_char,
        &args.ignore_words,
        &stopwords,
    );

    Ok(word_count)
}

fn output_results(args: &CliArgs, word_count: HashMap<String, i32>) {
    let sorted = sort_word_counts(&args.sort, word_count);

    print_results(args.top, sorted);
}

fn process_words<R: BufRead>(
    reader: R,
    case_sensitive: bool,
    min_char: Option<usize>,
    ignore_words: &Option<Vec<String>>,
    stopwords: &Option<HashSet<String>>,
) -> HashMap<String, i32> {
    let mut word_count = HashMap::new();

    for line in reader.lines().flatten() {
        for word in line.unicode_words() {
            if let Some(cleaned) =
                preprocess_word(word, case_sensitive, min_char, &ignore_words, stopwords)
            {
                *word_count.entry(cleaned).or_insert(0) += 1;
            }
        }
    }

    word_count
}

fn preprocess_word(
    word: &str,
    case_sensitive: bool,
    min_char: Option<usize>,
    ignore_words: &Option<Vec<String>>,
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

    if let Some(min) = min_char {
        if cleaned.graphemes(true).count() < min {
            return None;
        }
    }

    if let Some(ignore_words) = ignore_words {
        if ignore_words.contains(&cleaned) {
            return None;
        }
    }

    Some(cleaned)
}

fn load_stopwords() -> Result<HashSet<String>, CliError> {
    Ok(STOPWORDS.as_ref().clone()) // Clone the HashSet only when needed
}

fn sort_word_counts(order: &str, word_count: HashMap<String, i32>) -> Vec<(String, i32)> {
    let mut sorted: Vec<(String, i32)> = word_count.into_iter().collect();

    match order {
        "asc" => sorted.sort_by(|a, b| a.1.cmp(&b.1)),
        _ => sorted.sort_by(|a, b| b.1.cmp(&a.1)),
    }

    sorted
}

fn print_results(top: Option<usize>, sorted: Vec<(String, i32)>) {
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
