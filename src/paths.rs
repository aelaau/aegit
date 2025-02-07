use std::path::{Path, PathBuf};
use crate::constants;

pub fn git_directory(repo_directory: &Path) -> PathBuf {
    repo_directory.join(constants::GIT_DIRECTORY)
}

pub fn commits_directory(repo_directory: &Path) -> PathBuf {
    git_directory(repo_directory).join(constants::COMMITS_DIRECTORY)
}

pub fn zero_commit_directory(repo_directory: &Path) -> PathBuf {
    commits_directory(repo_directory).join(constants::ZERO_COMMIT_DIRECTORY)
}

pub fn stage_directory(repo_directory: &Path) -> PathBuf {
    git_directory(repo_directory).join(constants::STAGE_DIRECTORY)
}

pub fn head_directory(repo_directory: &Path) -> PathBuf {
    commits_directory(repo_directory).join(constants::HEAD_DIRECTORY)
}