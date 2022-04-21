use std::{
    io::{Error, Result},
    path::{Path, PathBuf},
};

pub use find_dir::*;

mod find_dir;
mod find_file;

pub fn ensure_dir(path: &Path) -> Result<PathBuf> {
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
