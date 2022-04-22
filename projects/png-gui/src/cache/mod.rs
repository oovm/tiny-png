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

use find_target::find_directory_or_create;

use crate::TinyResult;

pub struct TinyWorkspace {
    current: PathBuf,
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
    pub fn initialize(workspace: PathBuf) {
        let mut out = TinyWorkspace { current: workspace, files: Default::default() };
        match out.load_database() {
            Ok(_) => {}
            Err(_) => {}
        }
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

fn db_path() -> TinyResult<PathBuf> {
    let dir = find_directory_or_create(&current_exe()?, "target")?;
    Ok(dir.join("tiny-png.db"))
}

#[test]
fn target() -> TinyResult {
    TinyWorkspace::initialize(PathBuf::from("../"));
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
