use super::*;

use serde_yaml;

use crate::searcher;

use std::collections::HashSet;

use log::{error, info};
use std::process;

pub fn get_files_with_groups_defined_in_repo_file(input_file: &String) -> Result<HashSet<String>> {
    let f = std::fs::File::open(input_file)?;

    let document: serde_yaml::Value = serde_yaml::from_reader(f)?;
    let mut groups = HashSet::new();

    document
        .as_mapping()
        .expect("Expects mapping")
        .iter()
        .for_each(|repos| {
            repos
                .1
                .as_mapping()
                .expect("Expects mapping")
                .keys()
                .for_each(|group| {
                    groups.insert(group.as_str().expect("Should be a String"));
                });
        });

    let mut files = HashSet::new();

    for group in &groups {
        info!("Looking for {group}");
        let matches = match searcher::core::grep(group, "groups/") {
            Ok(result) => result,
            Err(err) => {
                error!("error: {}", err);
                process::exit(1);
            }
        };
        for file in matches {
            info!("Found '{group}' in '{file}'");
            files.insert(file);
        }
    }

    info!("Unique files");
    for file in &files {
        info!("found: {}", file);
    }

    Ok(files)
}
