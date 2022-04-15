use std::fs;
use oxipng::{optimize_from_memory, Options};

mod errors;

pub fn main() {
    let png = include_bytes!("../iphone.test.png");
    let mut opt = Options {
        ..Options::default()
    };
    let out = optimize_from_memory(png, &opt).unwrap();
    fs::write("out.test.png", &out).unwrap();

}