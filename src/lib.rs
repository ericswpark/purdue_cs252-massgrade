use std::{fs, io};
use std::path::{Path, PathBuf};
use tempfile::tempdir;
use thiserror::Error;
use cs252chkr::check as cs252chkr;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    CHKRError(#[from] cs252chkr::Error),

}

/// Checks each directory to make sure they exist and returns an array of directory PathBufs
///
/// * `raw_dirs` - array of directory strings to check
pub fn check_directories(raw_dirs: &[String]) -> Vec<PathBuf> {
    let mut directories = Vec::with_capacity(raw_dirs.len());

    for raw_dir in raw_dirs {
        let path = PathBuf::from(raw_dir);
        if !path.exists() {
            panic!("Directory {raw_dir} does not exist!");
        }

        directories.push(path);
    }

    directories
}

/// Copies all files in the given directory to the target directory
///
/// Shamelessly taken from here: https://stackoverflow.com/a/65192210
///
/// * `src` - source directory to copy from
/// * `dst` - target directory to copy to
fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else if ty.is_file() {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}


/// Main check function of massgrade
///
/// * `dirs` - directories to check
pub fn check(dirs: &[PathBuf]) -> Result<(), Error> {
    for dir in dirs {
        let temp_dir = tempdir()?;
        copy_dir_all(dir, &temp_dir)?;

        // Run cs252chkr against copied directory
        println!("Checking {}...", dir.display());
        cs252chkr(temp_dir.path().to_str().unwrap())?;
    }

    Ok(())
}