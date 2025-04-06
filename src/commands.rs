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
    let mut word_count = IndexMap::new();

    let stops: HashSet<String> = Spark::stopwords(Language::English)
        .unwrap()
        .into_iter()
        .map(|s| s.to_string())
        .collect();

    for line in reader.lines() {
        let line = line?;

        for word in line.unicode_words() {
            let cleaned = handle_case_sensitive(word, args.case_sensitive);

            if args.no_stopwords && stops.contains(&cleaned.to_lowercase()) {
                continue;
            }

            *word_count.entry(cleaned).or_insert(0) += 1;
        }
    }

    let mut sorted: Box<[_]> = word_count.into_iter().collect();

    if args.sort == "asc" {
        sorted.sort_by(|a, b| a.1.cmp(&b.1));
    } else {
        sorted.sort_by(|a, b| b.1.cmp(&a.1));
    }

    let count = args.top.unwrap_or(sorted.len());

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

fn handle_case_sensitive(word: &str, case_sensitive: bool) -> String {
    if case_sensitive {
        word.to_string()
    } else {
        word.to_lowercase()
    }
}

