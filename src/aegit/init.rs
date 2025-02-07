use crate::paths;

use std::{fs, io, path::Path};

fn create_directory(directory: &Path) -> io::Result<()> {
    if !directory.exists() {
        fs::create_dir(directory)?
    }

    return Ok(())
}

pub fn init(repo_directory: &Path) -> io::Result<()> {
    let commits_path = paths::commits_directory(&repo_directory);
    let zero_commit_path = paths::zero_commit_directory(&repo_directory);

    create_directory(&paths::git_directory(&repo_directory))?;
    create_directory(&commits_path)?;
    create_directory(&zero_commit_path)?;
    create_directory(&paths::stage_directory(&repo_directory))?;

    std::os::unix::fs::symlink(zero_commit_path, paths::head_directory(&repo_directory))?;

    println!("Aegit repository initialized");
    Ok(())
}