use crate::{errors::TinyError, TinyResult};
use find_target::find_directory_or_create;
use std::env::current_exe;

#[test]
fn target() -> TinyResult {
    find_directory_or_create(&current_exe()?, "target")?;

    Ok(())
}
