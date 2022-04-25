use std::path::PathBuf;

use tiny_png::{
    utils::{logger, optimize_png},
    TinyResult, TinyWorkspace,
};

#[test]
fn ready() {
    println!("it works!")
}

#[tokio::test]
async fn target() -> TinyResult {
    logger();
    let mut ws = TinyWorkspace::initialize(PathBuf::from("D:\\Python\\tiny-png"), false);
    ws.check_all_pngs().await.unwrap();
    // println!("{:#016X}", hash);
    Ok(())
}

#[test]
pub fn main() {
    let before = include_bytes!("../iphone.test.png");
    let out = optimize_png(before).unwrap();
    println!("before: {}", out.before);
    println!("after: {}", out.after);
    println!("Reduce {:+.2}%", out.reduce);
}
