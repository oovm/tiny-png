use std::{
    collections::BTreeSet,
    env::current_exe,
    fs::{read, write},
    path::{Path, PathBuf},
    time::SystemTime,
};

use async_walkdir::{DirEntry, WalkDir};
use bytesize::ByteSize;
use futures::StreamExt;
use log::LevelFilter;

use colored::Colorize;
use find_target::find_directory_or_create;

use crate::{
    utils::{hash_file, logger, optimize_png},
    TinyResult,
};

mod config;

pub struct TinyConfig {
    pub writable: bool,
    pub database: bool,
    pub log_level: LevelFilter,
}

pub struct TinyWorkspace {
    workspace: PathBuf,
    writable: bool,
    database: PathBuf,
    reduced: u64,
    start: SystemTime,
    files: BTreeSet<u64>,
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
        let reduced = ByteSize::b(self.reduced);
        let timing = SystemTime::now().duration_since(self.start)?;
        log::info!("Reduce {} in {:?}", reduced, timing);
        Ok(())
    }
    pub fn optimize_png(&mut self, path: &Path) -> TinyResult {
        let bytes = read(path)?;
        let hash = hash_file(&bytes);
        if self.files.contains(&hash) {
            log::info!("Skip Optimized \n{}", path.display());
            return Ok(());
        }
        match optimize_png(&bytes) {
            Ok(o) => {
                self.reduced += o.before.0 - o.after.0;
                let reduce = format!("({:+.2}%)", o.reduce).green();
                let file = self.relative_path(&path);
                if self.writable {
                    let overwrite = "(overwrite)".bold();
                    log::info!("{} => {} {reduce}\n{} {overwrite}", o.before, o.after, file.display());
                    write(path, &o.output)?;
                    self.files.insert(hash_file(&o.output));
                }
                else {
                    log::info!("{} => {} {reduce}\n{}", o.before, o.after, file.display());
                }
            }
            Err(_) => {
                if self.writable {
                    self.files.insert(hash_file(&bytes));
                }
            }
        };
        Ok(())
    }
    fn relative_path(&self, target: &Path) -> PathBuf {
        match pathdiff::diff_paths(target, &self.workspace) {
            Some(s) => s,
            None => target.to_path_buf(),
        }
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
