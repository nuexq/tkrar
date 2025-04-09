use once_cell::sync::Lazy;
use std::{
    collections::{HashMap, HashSet},
    fs::{self, File},
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
            .expect("Failed to load embedded English stopwords") // Panic with message
            .into_iter()
            .map(|s| s.to_string()) // Assumes stopwords library provides owned Strings or &str
            .collect(),
    )
});

// collect file paths recursively
fn collect_files(target: &Vec<PathBuf>) -> Result<Vec<PathBuf>, CliError> {
    let mut files = Vec::new();
    for path in target {
        if path.is_file() {
            files.push(path.clone());
        } else if path.is_dir() {
            match fs::read_dir(path) {
                Ok(entries) => {
                    for entry in entries.flatten() {
                        let subpath = entry.path();
                        if subpath.is_file() {
                            files.push(subpath);
                        } else if subpath.is_dir() {
                            files.extend(collect_files(&vec![subpath])?);
                        }
                    }
                }
                Err(e) => return Err(CliError::Other(e.to_string())),
            }
        }
    }
    Ok(files)
}

pub fn count_freq_of_words(target: &Vec<PathBuf>, args: &CliArgs) -> Result<(), CliError> {
    let stopwords_set = if args.no_stopwords {
        Some(Arc::clone(&STOPWORDS)) // Clone the Arc, cheap
    } else {
        None
    };

    let all_files = collect_files(target)?;

    let word_counts = all_files
        .iter()
        .filter_map(|file| match File::open(file) {
            Ok(open_file) => {
                let reader = BufReader::new(open_file);
                match count_freq_of_words_from_reader(reader, args, &stopwords_set) {
                    Ok(wc) => Some(wc),
                    Err(e) => {
                        eprintln!("Error processing file {}: {}", file.display(), e);
                        None
                    }
                }
            }
            Err(e) => {
                eprintln!("Error opening file {}: {}", file.display(), e);
                None
            }
        })
        .reduce(|mut a, b| {
            merge_word_counts(&mut a, b);
            a
        });

    let word_counts = match word_counts {
        Some(counts) => counts,
        None => return Err(CliError::Other("empty file".to_string())),
    };

    // let filtered = filter_words(word_counts, args)?;

    output_results(args.top, &args.sort, word_counts);

    Ok(())
}

// count_words fn for stdin
pub fn count_words_from_stdin(reader: StdinLock, args: &CliArgs) -> Result<(), CliError> {
    let stopwords_set = if args.no_stopwords {
        Some(Arc::clone(&STOPWORDS))
    } else {
        None
    };

    let word_count = count_freq_of_words_from_reader(reader, args, &stopwords_set)?;

    output_results(args.top, &args.sort, word_count);

    Ok(())
}

// merge two word counts hashmaps
fn merge_word_counts(main_map: &mut HashMap<String, i32>, other_map: HashMap<String, i32>) {
    for (word, count) in other_map {
        *main_map.entry(word).or_insert(0) += count;
    }
}

// return a map of word counts
pub fn count_freq_of_words_from_reader<R: BufRead>(
    reader: R,
    args: &CliArgs,
    stopwords: &Option<Arc<HashSet<String>>>,
) -> Result<HashMap<String, i32>, CliError> {
    let word_count = process_words(reader, args, stopwords);

    Ok(word_count)
}

fn output_results(top: Option<usize>, sort: &str, word_count: HashMap<String, i32>) {
    let sorted = sort_word_counts(sort, word_count);

    print_results(top, sorted);
}

fn process_words<R: BufRead>(
    reader: R,
    args: &CliArgs,
    stopwords: &Option<Arc<HashSet<String>>>,
) -> HashMap<String, i32> {
    let mut word_count = HashMap::new();

    for line in reader.lines().flatten() {
        for word in line.unicode_words() {
            if !filter_words(word, args, stopwords) {
                continue;
            }

            let key = if args.case_sensitive {
                word.to_string()
            } else {
                word.to_lowercase()
            };

            *word_count.entry(key).or_insert(0) += 1;
        }
    }

    word_count
}

fn filter_words(
    word: &str,
    args: &CliArgs,
    stopwords: &Option<Arc<HashSet<String>>>, // Accept stopwords
) -> bool {
    let check_word = if args.case_sensitive {
        word.to_string()
    } else {
        word.to_lowercase()
    };

    if args.alphabetic_only {
        let word_for_alpha_check = if args.case_sensitive {
            word
        } else {
            &check_word
        };
        if !word_for_alpha_check.chars().all(|c| c.is_alphabetic()) {
            return false;
        }
    }

    if let Some(stops) = stopwords {
        if stops.contains(&word.to_lowercase()) {
            return false;
        }
    }

    if let Some(min) = args.min_char {
        let word_for_len_check = if args.case_sensitive {
            word
        } else {
            &check_word
        };
        if word_for_len_check.graphemes(true).count() < min {
            return false;
        }
    }

    if let Some(ignore_regex) = &args.ignore_words {
        let word_for_regex_check = if args.case_sensitive {
            word
        } else {
            &check_word
        };
        if ignore_regex.is_match(word_for_regex_check) {
            return false;
        }
    }

    true
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
            "<k!>{:>2}.</> <w!>{:<15}</> <bold,g>{}</>",
            i + 1,
            word,
            freq,
        );
    }
}
