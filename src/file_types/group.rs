use super::*;

use crate::searcher;

use serde_yaml;

use std::collections::HashSet;
use std::process;

use log::{error, info};

pub fn get_files_from_repos_defined_in_group_file(input_file: &String) -> Result<HashSet<String>> {
    let f = std::fs::File::open(input_file)?;
    let document: serde_yaml::Value = serde_yaml::from_reader(f)?;
    // Get all filenames from repos files with a group definition
    search_group(document)
}

fn search_group(document: serde_yaml::Value) -> Result<HashSet<String>> {
    let mut groups = HashSet::new();

    document
        .as_mapping()
        .expect("Expects mapping")
        .keys()
        .for_each(|group| {
            groups.insert(group.as_str().expect("Should be a String"));
        });

    let mut total_matches = HashSet::new();

    for group in &groups {
        info!("Looking for group: {group}");
        let matches = match searcher::core::grep(group, "repos/") {
            Ok(result) => result,
            Err(err) => {
                error!("error: {}", err);
                process::exit(1);
            }
        };
        for m in matches {
            info!("Found '{group}' in '{m}'");
            total_matches.insert(m);
        }
    }

    info!("Unique matches");

    for file in &total_matches {
        info!("found: {}", file);
    }

    Ok(total_matches)
}
