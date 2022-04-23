use std::{
    collections::{BTreeMap, BTreeSet},
    env::current_exe,
    fmt::Debug,
    fs::File,
    hash::{BuildHasher, BuildHasherDefault, Hasher},
    io::{BufReader, Read},
    path::{Path, PathBuf},
};

use async_walkdir::{DirEntry, WalkDir};
use futures::StreamExt;
use twox_hash::XxHash64;

use find_target::find_directory_or_create;

use crate::{
    utils::{hash_file, optimize_png},
    TinyResult,
};

pub struct TinyWorkspace {
    workspace: PathBuf,
    write: bool,
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
    pub fn initialize(workspace: PathBuf) -> Self {
        let mut out = TinyWorkspace { workspace, files: Default::default() };
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
            println!("{}", path.display());
        }
        Ok(())
    }
    pub fn optimize_png(&mut self, path: &Path) -> TinyResult {
        let bytes = std::fs::read(path)?;
        let hash = match optimize_png(&bytes) {
            Ok(o) => hash_file(&o.output),
            Err(_) => hash_file(&bytes),
        };
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

#[tokio::test]
async fn target() -> TinyResult {
    let mut ws = TinyWorkspace::initialize(PathBuf::from("D:\\Python\\tiny-png\\projects\\png-gui"));
    ws.check_all_pngs().await.unwrap();
    // println!("{:#016X}", hash);
    Ok(())
}
