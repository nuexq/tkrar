use once_cell::sync::Lazy;
use std::{
    collections::{HashMap, HashSet},
    fs::{self, File},
    io::{BufRead, BufReader, StdinLock},
    path::PathBuf,
    sync::Arc,
};

use color_print::cprintln;
use regex::Regex;
use stopwords::{Language, Spark, Stopwords};
use unicode_segmentation::UnicodeSegmentation;
use rayon::prelude::*;

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

pub struct Filters<'a> {
    case_sensitive: bool,
    alphabetic_only: bool,
    min_char: Option<usize>,
    ignore_words: &'a Option<Regex>,
    stopwords: &'a Option<Arc<HashSet<String>>>,
}

impl<'a> Filters<'a> {
    pub fn filter_word(&self, word: &str) -> Option<String> {
        let Filters {
            case_sensitive,
            alphabetic_only,
            min_char,
            ignore_words,
            stopwords,
        } = self;

        let cleaned = if *case_sensitive {
            word.to_string()
        } else {
            word.to_lowercase()
        };

        if *alphabetic_only {
            if !cleaned.chars().all(|c| c.is_alphabetic()) {
                return None;
            }
        }
        if let Some(stops) = stopwords {
            if stops.contains(&cleaned.to_lowercase()) {
                return None;
            }
        }

        if let Some(min) = *min_char {
            if cleaned.graphemes(true).count() < min {
                return None;
            }
        }

        if let Some(ignore_words) = ignore_words {
            if ignore_words.is_match(&cleaned) {
                return None;
            }
        }

        Some(cleaned)
    }
}

// collect file paths recursively
fn collect_files(target: &Vec<PathBuf>) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for path in target {
        if path.is_file() {
            files.push(path.clone());
        } else if path.is_dir() {
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries.flatten() {
                    let subpath = entry.path();
                    if subpath.is_file() {
                        files.push(subpath);
                    } else if subpath.is_dir() {
                        files.extend(collect_files(&vec![subpath]));
                    }
                }
            }
        }
    }
    files
}

pub fn count_words(target: &Vec<PathBuf>, args: &CliArgs) -> Result<(), CliError> {
    let all_files = collect_files(target);

    let word_counts: Vec<HashMap<String, i32>> = all_files
        .par_iter()
        .filter_map(|file| match File::open(file) {
            Ok(open_file) => {
                let reader = BufReader::new(open_file);
                match count_words_from_reader(reader, args) {
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
        .collect();

    let mut total_word_count = HashMap::new();
    for wc in word_counts {
        merge_word_counts(&mut total_word_count, wc);
    }

    output_results(args.top, &args.sort, total_word_count);

    Ok(())
}

fn merge_word_counts(main_map: &mut HashMap<String, i32>, other_map: HashMap<String, i32>) {
    for (word, count) in other_map {
        *main_map.entry(word).or_insert(0) += count;
    }
}

// count_words fn for stdin
pub fn count_words_from_stdin(reader: StdinLock, args: &CliArgs) -> Result<(), CliError> {
    let word_count = count_words_from_reader(reader, args)?;

    output_results(args.top, &args.sort, word_count);

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
    let filters = Filters {
        case_sensitive: args.case_sensitive,
        alphabetic_only: args.alphabetic_only,
        min_char: args.min_char,
        ignore_words: &args.ignore_words,
        stopwords: &stopwords,
    };

    let word_count = process_words(reader, filters);

    Ok(word_count)
}

fn output_results(top: Option<usize>, sort: &str, word_count: HashMap<String, i32>) {
    let sorted = sort_word_counts(sort, word_count);

    print_results(top, sorted);
}

fn process_words<R: BufRead>(reader: R, filters: Filters) -> HashMap<String, i32> {
    let mut word_count = HashMap::new();

    for line in reader.lines().flatten() {
        for word in line.unicode_words() {
            if let Some(cleaned) = filters.filter_word(word) {
                *word_count.entry(cleaned).or_insert(0) += 1;
            }
        }
    }

    word_count
}

fn load_stopwords() -> Result<Arc<HashSet<String>>, CliError> {
    Ok(Arc::clone(&STOPWORDS))
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
