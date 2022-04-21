use std::{env::current_exe, path::PathBuf};

use find_target::{find_directory, find_directory_or_create};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() -> std::io::Result<()> {
    println!("{:?}", find_directory(&current_exe()?, "target")?);
    Ok(())
}

#[test]
fn test_ext() -> std::io::Result<()> {
    println!("{:?}", find_directory_or_create(&current_exe()?, "target")?);
    Ok(())
}
