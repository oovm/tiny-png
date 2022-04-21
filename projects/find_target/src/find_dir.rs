use super::*;

pub fn find_dir(start: &Path, name: &str) -> std::io::Result<PathBuf> {
    let normed = if start.is_dir() { start.canonicalize()?.join(name) } else { start.canonicalize()? };
    let mut here = normed.as_path();
    while let Some(dir) = here.parent() {
        let path = here.join(name);
        if path.exists() && path.is_dir() {
            return Ok(path);
        }
        else {
            here = dir;
        }
    }
    Err(Error::from_raw_os_error(10006))
}

pub fn find_dir_or_create(start: &Path, name: &str) -> std::io::Result<PathBuf> {
    match find_dir(start, name) {
        Ok(o) => return Ok(o),
        Err(_) => {}
    }
    let dir = ensure_dir()
}


#[test]
fn test() {
    let path = PathBuf::from("cargo.toml");
    println!("{:?}", find_dir(&path, "target"))
}
