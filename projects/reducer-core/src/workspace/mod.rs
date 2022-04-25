use std::{
    collections::BTreeSet,
    env::current_exe,
    fs::{read, write},
    path::{Path, PathBuf},
};

use async_walkdir::{DirEntry, WalkDir};
use futures::StreamExt;

use find_target::find_directory_or_create;

use crate::{
    utils::{hash_file, optimize_png},
    TinyResult,
};

pub struct TinyWorkspace {
    workspace: PathBuf,
    writable: bool,
    files: BTreeSet<u64>,
}

impl Drop for TinyWorkspace {
    fn drop(&mut self) {
        if let Err(e) = self.on_drop() {
            eprintln!("{}", e)
        }
    }
}

impl TinyWorkspace {
    pub fn initialize(workspace: PathBuf, writable: bool) -> Self {
        let mut out = TinyWorkspace { workspace, writable, files: Default::default() };
        match out.load_database() {
            Ok(_) => {}
            Err(_) => {}
        }
        out
    }
    fn load_database(&mut self) -> TinyResult<PathBuf> {
        let db = db_path()?;
        Ok(db)
    }
    fn on_drop(&self) -> TinyResult {
        let _ = db_path()?;
        Ok(())
    }
}

impl TinyWorkspace {
    pub async fn check_all_pngs(&mut self) -> TinyResult {
        let mut entries = WalkDir::new(&self.workspace);
        loop {
            let path = match entries.next().await {
                Some(out) => match continue_search(out) {
                    Some(path) => path,
                    None => continue,
                },
                None => break,
            };
            if let Err(e) = self.optimize_png(&path) {
                log::error!("{e}")
            }
        }
        Ok(())
    }
    pub fn optimize_png(&mut self, path: &Path) -> TinyResult {
        let bytes = read(path)?;
        let hash = hash_file(&bytes);
        if self.files.contains(&hash) {
            log::info!("Skip Optimized \n{}", path.display());
            return Ok(());
        }
        let hash = match optimize_png(&bytes) {
            Ok(o) => {
                log::info!("{} => {} ({:+.2}%)\n{}", o.before, o.after, o.reduce, path.display());
                hash_file(&o.output)
            }
            Err(_) => hash_file(&bytes),
        };

        if self.writable {
            write(path, bytes)?;
            self.files.insert(hash);
        }
        Ok(())
    }
}

fn continue_search(r: Result<DirEntry, std::io::Error>) -> Option<PathBuf> {
    let path = match r {
        Ok(o) => o.path(),
        Err(e) => {
            log::error!("{e}");
            return None;
        }
    };
    if !path.is_file() {
        return None;
    }
    let name = path.file_name()?.to_str()?;
    if name.ends_with(".png") { Some(path) } else { None }
}

fn db_path() -> TinyResult<PathBuf> {
    let dir = find_directory_or_create(&current_exe()?, "target")?;
    Ok(dir.join("tiny-png.db"))
}
