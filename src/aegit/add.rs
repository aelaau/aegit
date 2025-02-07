use std::{io, fs, path::Path};
use io::{ErrorKind};
use crate::paths;

pub fn add(repo_directory: &Path, path: String) -> io::Result<()> {
    let head_directory = paths::head_directory(&repo_directory);
    let stage_directory = paths::stage_directory(&repo_directory);
    let full_path = repo_directory.join(&path);
    let full_commited_path = head_directory.join(&path);
    let attr_result = fs::metadata(&full_commited_path);

    match attr_result {
        Ok(attr) => {
            if attr.is_dir() {
                // TODO: implement recursive file check
                Ok(())
            } else if attr.is_file() {
                // TODO: put out and implement diff
                Ok(())
            } else {
                Ok(())
            }
        }
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                let full_destination_path = stage_directory.join(&path);
                let a = fs::copy(&full_path, full_destination_path);

                Ok(())
            } else {
                return Err(e)
            }
        }
    }

}