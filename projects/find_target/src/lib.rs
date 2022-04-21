use std::{
    fs::create_dir_all,
    io::{Error, Result},
    path::{Path, PathBuf},
};

pub use find_dir::{find_directory, find_directory_or_create};

mod find_dir;
mod find_file;

/// Ensure path is dir
///
/// # Arguments
///
/// * `path`:
///
/// returns: Result<PathBuf, Error>
///
/// # Examples
///
/// ```
/// use find_target::ensure_directory;
/// ```
pub fn ensure_directory(path: &Path) -> Result<PathBuf> {
    if path.is_dir() {
        path.canonicalize()
    }
    else {
        match path.parent() {
            Some(s) => s.canonicalize(),
            None => Err(Error::from_raw_os_error(10006)),
        }
    }
}

/// Ensure path is file
///
/// # Arguments
///
/// * `path`:
/// * `name`:
///
/// returns: Result<PathBuf, Error>
///
/// # Examples
///
/// ```
/// use find_target::ensure_file;
/// ```
pub fn ensure_file(path: &Path, name: &str) -> Result<PathBuf> {
    if path.is_file() { path.canonicalize() } else { Ok(path.canonicalize()?.join(name)) }
}
