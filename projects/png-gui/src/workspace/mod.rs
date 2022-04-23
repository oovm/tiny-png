use std::{
    collections::BTreeMap,
    env::current_exe,
    fmt::Debug,
    fs::File,
    hash::{BuildHasher, BuildHasherDefault, Hasher},
    io::{BufReader, Read},
    path::{Path, PathBuf},
};

use twox_hash::XxHash64;
use walkdir::WalkDir;

use find_target::find_directory_or_create;

use crate::TinyResult;

pub struct TinyWorkspace {
    workspace: PathBuf,
    files: BTreeMap<u64, TinyCache>,
}

#[derive(Debug)]
pub struct TinyCache {
    hash: u64,
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
    pub fn optimize_pngs(&self) -> TinyResult {
        for entry in WalkDir::new(&self.workspace).follow_links(true) {
            let path = match entry {
                Ok(o) => {
                    if !o.path().is_file() {
                        continue;
                    }
                    match o.path().file_name().and_then(|v| v.to_str()) {
                        Some(s) if s.ends_with(".png") => {}
                        _ => continue,
                    }
                    o.into_path()
                }
                Err(e) => {
                    log::error!("{e}");
                    continue;
                }
            };

            println!("{}", path.display());
            // if path.ends_with(".png") {
            //     println!("{}", path.display());
            // }
        }
        Ok(())
    }
}

fn db_path() -> TinyResult<PathBuf> {
    let dir = find_directory_or_create(&current_exe()?, "target")?;
    Ok(dir.join("tiny-png.db"))
}

#[test]
fn target() -> TinyResult {
    let mut ws = TinyWorkspace::initialize(PathBuf::from("D:\\Python\\tiny-png\\projects\\png-gui"));
    ws.optimize_pngs().unwrap();
    let hash = TinyCache::hash_file(&PathBuf::from("iphone.test.png"))?;
    println!("{:#016X}", hash);
    Ok(())
}

impl TinyCache {
    pub fn hash_file(path: &Path) -> TinyResult<u64> {
        let input = File::open(path)?;
        let mut reader = BufReader::new(input);
        let mut hasher: XxHash64 = BuildHasherDefault::<XxHash64>::default().build_hasher();
        let mut buffer = [0; 1024];
        loop {
            let count = reader.read(&mut buffer)?;
            if count == 0 {
                break;
            }
            hasher.write(&buffer[..count])
        }
        Ok(hasher.finish())
    }
}
