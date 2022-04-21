use std::path::PathBuf;

use find_target::find_directory;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    let path = PathBuf::from("cargo.toml");
    println!("{:?}", find_directory(&path, "target"))
}
