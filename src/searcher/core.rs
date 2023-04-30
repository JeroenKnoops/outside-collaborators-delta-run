use super::*;

use std::error::Error;

use grep::regex::RegexMatcher;
use grep::searcher::sinks::UTF8;
use grep::searcher::{BinaryDetection, SearcherBuilder};
use std::collections::HashSet;
use walkdir::WalkDir;

pub fn grep(pattern: &str, path: &str) -> Result<HashSet<String>, Box<dyn Error>> {
    let mut matches = HashSet::new();

    let matcher = RegexMatcher::new_line_matcher(pattern)?;
    let mut searcher = SearcherBuilder::new()
        .binary_detection(BinaryDetection::quit(b'\x00'))
        .line_number(true)
        .build();

    for result in WalkDir::new(path) {
        let dent = match result {
            Ok(dent) => dent,
            Err(err) => {
                eprintln!("{}", err);
                continue;
            }
        };
        if !dent.file_type().is_file() {
            continue;
        }

        let result = searcher.search_path(
            &matcher,
            dent.path(),
            UTF8(|_lnum, _line| {
                matches.insert(dent.path().display().to_string());
                Ok(true)
            }),
        );
        if let Err(err) = result {
            eprintln!("{}: {}", dent.path().display(), err);
        }
    }
    Ok(matches)
}
