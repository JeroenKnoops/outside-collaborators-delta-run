use super::*;

use crate::searcher;
use crate::searcher::runner::{FileStatus, OutsideCollaboratorFileTypes};

use std::collections::HashSet;

use log::{debug, error, info};

/// Read a file and execute the correct search on it.
#[derive(Debug, Parser)]
pub struct Args {
    /// Input-File with definition of organization
    #[clap(short, long, required(true))]
    input_file: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum FileType {
    Group,
    Repository,
    Other,
}

pub fn type_of_configuration(file_path: &str) -> FileType {
    if !file_path.ends_with(".yml") {
        return FileType::Other;
    }
    if file_path.contains("groups/") {
        return FileType::Group;
    }
    if file_path.contains("repos/") {
        return FileType::Repository;
    }
    FileType::Other
}

pub fn process(input_file: String) -> Result<()> {
    let file_type = type_of_configuration(&input_file);

    let initial_file_status = FileStatus::new(HashSet::new(), vec![input_file]);

    let mut outside_collaborator_file_types = match file_type {
        FileType::Group => {
            debug!("Group file");
            OutsideCollaboratorFileTypes::new(FileStatus::empty(), initial_file_status)
        }
        FileType::Repository => {
            debug!("Repository file");
            OutsideCollaboratorFileTypes::new(initial_file_status, FileStatus::empty())
        }
        _ => {
            error!("Input file is not a Group or Repository configuration file.");
            OutsideCollaboratorFileTypes::new(FileStatus::empty(), FileStatus::empty())
        }
    };

    searcher::runner::get_files(&mut outside_collaborator_file_types)?;

    info!("touched_files");
    for file in outside_collaborator_file_types.repositories.touched_files {
        println!("{file}");
    }
    for file in outside_collaborator_file_types.groups.touched_files {
        println!("{file}");
    }

    Ok(())
}

pub async fn command(args: Args) -> Result<()> {
    let input_file = args.input_file.expect("input_file is not set");
    process(input_file)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn configuration_check_repository() {
        assert_eq!(
            type_of_configuration(&"../repos/test.yml".to_string()),
            FileType::Repository
        );
        assert_eq!(
            type_of_configuration(&"repos/test.yml".to_string()),
            FileType::Repository
        );
    }

    #[test]
    fn configuration_check_group() {
        assert_eq!(
            type_of_configuration(&"../groups/test.yml".to_string()),
            FileType::Group
        );
        assert_eq!(
            type_of_configuration(&"groups/test.yml".to_string()),
            FileType::Group
        );
    }

    #[test]
    fn configuration_check_other() {
        assert_eq!(
            type_of_configuration(&"README.md".to_string()),
            FileType::Other
        );
        assert_eq!(
            type_of_configuration(&"groups/INFO.md".to_string()),
            FileType::Other
        );
    }
}
