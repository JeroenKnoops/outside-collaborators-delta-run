use super::*;

use crate::file_types::group::get_files_from_repos_defined_in_group_file;
use crate::file_types::repos::get_files_with_groups_defined_in_repo_file;

use std::collections::HashSet;

use log::info;

#[derive(Debug)]
pub struct FileStatus {
    pub touched_files: HashSet<String>,
    pub to_check_files: Vec<String>,
}

impl FileStatus {
    pub fn new(touched_files: HashSet<String>, to_check_files: Vec<String>) -> Self {
        Self {
            touched_files,
            to_check_files,
        }
    }

    pub fn empty() -> Self {
        Self {
            touched_files: HashSet::new(),
            to_check_files: vec![],
        }
    }
}

#[derive(Debug)]
pub struct OutsideCollaboratorFileTypes {
    pub repositories: FileStatus,
    pub groups: FileStatus,
}

impl OutsideCollaboratorFileTypes {
    pub fn new(repositories: FileStatus, groups: FileStatus) -> Self {
        Self {
            repositories,
            groups,
        }
    }
}

fn groups_to_check_empty(outside_collaborator_file_types: &OutsideCollaboratorFileTypes) -> bool {
    outside_collaborator_file_types
        .groups
        .to_check_files
        .is_empty()
}
fn repositories_to_check_empty(
    outside_collaborator_file_types: &OutsideCollaboratorFileTypes,
) -> bool {
    outside_collaborator_file_types
        .repositories
        .to_check_files
        .is_empty()
}

pub fn get_files(outside_collaborator_file_types: &mut OutsideCollaboratorFileTypes) -> Result<()> {
    info!("getting_files");
    if groups_to_check_empty(outside_collaborator_file_types)
        & repositories_to_check_empty(outside_collaborator_file_types)
    {
        info!("There are no more files to check");
        return Ok(());
    }
    if !groups_to_check_empty(outside_collaborator_file_types) {
        let input_file = outside_collaborator_file_types
            .groups
            .to_check_files
            .pop()
            .expect("The list is not empty so there is an entry");
        let repos_files = get_files_from_repos_defined_in_group_file(&input_file)?;

        outside_collaborator_file_types
            .groups
            .touched_files
            .insert(input_file);
        for repos_file in repos_files {
            if outside_collaborator_file_types
                .repositories
                .touched_files
                .contains(&repos_file)
            {
                info!("already checked! {repos_file}");
            } else {
                outside_collaborator_file_types
                    .repositories
                    .to_check_files
                    .push(repos_file.clone());
            }
        }
    }
    if !repositories_to_check_empty(outside_collaborator_file_types) {
        let input_file = outside_collaborator_file_types
            .repositories
            .to_check_files
            .pop()
            .expect("The list is not empty so there is an entry");
        let group_files = get_files_with_groups_defined_in_repo_file(&input_file)?;

        outside_collaborator_file_types
            .repositories
            .touched_files
            .insert(input_file);
        for group_file in group_files {
            if outside_collaborator_file_types
                .groups
                .touched_files
                .contains(&group_file)
            {
                info!("already checked! {group_file}");
            } else {
                info!("add to groups check list: {group_file}");
                outside_collaborator_file_types
                    .groups
                    .to_check_files
                    .push(group_file.clone());
            };
        }
    }
    get_files(outside_collaborator_file_types)
}
