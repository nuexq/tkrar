use crate::{cli::CliArgs, error::CliError};
use atty;
use color_print::{ceprintln, cprintln};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    collections::{HashMap, HashSet},
    fs::{self, File},
    io::{BufRead, BufReader, StdinLock},
    path::PathBuf,
    sync::Arc,
};
use stopwords::{Language, Spark, Stopwords};
use unicode_segmentation::UnicodeSegmentation;

static STOPWORDS: Lazy<Arc<HashSet<String>>> = Lazy::new(|| {
    Arc::new(
        Spark::stopwords(Language::English)
            .expect("Failed to load embedded English stopwords")
            .into_iter()
            .map(|s| s.to_string())
            .collect(),
    )
});

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
        Some(Arc::clone(&STOPWORDS))
    } else {
        None
    };

    let all_files = collect_files(target)?
        .into_iter()
        .filter(|file| {
            !args.ignore_files.as_ref().map_or(false, |ignore_files| {
                ignore_files.contains(
                    &file
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string(),
                )
            })
        })
        .collect::<Vec<PathBuf>>();

    let word_counts = all_files
        .iter()
        .filter_map(|file| {
            let file_name = file.file_name().unwrap_or_default().to_string_lossy();
            match File::open(file) {
                Ok(open_file) => {
                    let reader = BufReader::new(open_file);
                    match count_freq_of_words_from_reader(reader, args, &stopwords_set) {
                        Ok(wc) => Some(wc),
                        Err(e) => {
                            ceprintln!("<yellow>warn: {} failed - {}</>", file_name, e);
                            None
                        }
                    }
                }
                Err(e) => {
                    ceprintln!("<yellow>warn: {} not opened - {}</>", file_name, e);
                    None
                }
            }
        })
        .reduce(|mut a, b| {
            merge_word_counts(&mut a, b);
            a
        });

    let word_counts = word_counts.ok_or_else(|| CliError::Other("no valid files found".into()))?;

    output_results(args.top, &args.sort, &args.output_format, word_counts);

    Ok(())
}

pub fn count_words_from_stdin(reader: StdinLock, args: &CliArgs) -> Result<(), CliError> {
    let stopwords_set = if args.no_stopwords {
        Some(Arc::clone(&STOPWORDS))
    } else {
        None
    };
    let word_count = count_freq_of_words_from_reader(reader, args, &stopwords_set)?;

    output_results(args.top, &args.sort, &args.output_format, word_count);

    Ok(())
}

fn merge_word_counts(main_map: &mut HashMap<String, i32>, other_map: HashMap<String, i32>) {
    for (word, count) in other_map {
        *main_map.entry(word).or_insert(0) += count;
    }
}

pub fn count_freq_of_words_from_reader<R: BufRead>(
    reader: R,
    args: &CliArgs,
    stopwords: &Option<Arc<HashSet<String>>>,
) -> Result<HashMap<String, i32>, CliError> {
    let word_count = process_words(reader, args, stopwords);
    Ok(word_count)
}

fn output_results(
    top: Option<usize>,
    sort: &str,
    output_format: &str,
    word_count: HashMap<String, i32>,
) {
    let sorted = sort_word_counts(sort, word_count);
    print_results(top, sorted, output_format);
}

fn process_words<R: BufRead>(
    reader: R,
    args: &CliArgs,
    stopwords: &Option<Arc<HashSet<String>>>,
) -> HashMap<String, i32> {
    let mut word_count = HashMap::new();
    let case_sensitive = args.case_sensitive;

    for line in reader.lines().flatten() {
        for word in line.unicode_words() {
            let (original, lowered) = (word, word.to_lowercase());
            let key = if case_sensitive { original } else { &lowered };

            if args.alphabetic_only && !key.chars().all(|c| c.is_alphabetic()) {
                continue;
            }
            if let Some(min) = args.min_char {
                if key.graphemes(true).count() < min {
                    continue;
                }
            }
            if let Some(stop) = stopwords {
                if stop.contains(&lowered) {
                    continue;
                }
            }
            if let Some(regex) = &args.ignore_words {
                if regex.is_match(key) {
                    continue;
                }
            }
            *word_count.entry(key.to_string()).or_insert(0) += 1;
        }
    }
    word_count
}

fn sort_word_counts(order: &str, word_count: HashMap<String, i32>) -> Vec<(String, i32)> {
    let mut sorted: Vec<(String, i32)> = word_count.into_iter().collect();
    match order {
        "asc" => sorted.sort_by(|a, b| a.1.cmp(&b.1)),
        _ => sorted.sort_by(|a, b| b.1.cmp(&a.1)),
    }
    sorted
}

#[derive(Serialize, Deserialize, Debug)]
struct WordFrequency {
    word: String,
    frequency: i32,
}

fn print_results(top: Option<usize>, sorted: Vec<(String, i32)>, output_format: &str) {
    let count = top.unwrap_or(sorted.len());
    let results: Vec<WordFrequency> = sorted
        .into_iter()
        .take(count)
        .map(|(word, freq)| WordFrequency {
            word,
            frequency: freq,
        })
        .collect();

    let is_tty = atty::is(atty::Stream::Stdout);

    match output_format {
        "text" => {
            for (i, result) in results.iter().enumerate() {
                if is_tty {
                    cprintln!(
                        "<w!>{:>2}. {:<15} <g>{}</>",
                        i + 1,
                        result.word,
                        result.frequency
                    );
                } else {
                    println!("{:>2}. {:<15} {}", i + 1, result.word, result.frequency);
                }
            }
        }
        "json" => {
            let json_output = serde_json::to_string(&results).unwrap();
            println!("{}", json_output);
        }
        "csv" => {
            println!("Rank,Word,Frequency");
            for (i, result) in results.iter().enumerate() {
                println!("{},{},{}", i + 1, result.word, result.frequency);
            }
        }
        _ => {
            eprintln!("Unsupported output format: {}", output_format);
        }
    }
}
