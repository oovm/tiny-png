use fs::write;
use std::fs;

use bytesize::ByteSize;
use oxipng::{optimize_from_memory, Options};

mod errors;


pub fn main() {
    let before = include_bytes!("../iphone.test.png");
    let mut opts = Options {
        ..Options::default()
    };
    let after = optimize_from_memory(before, &opts).unwrap();
    println!("before: {}", ByteSize::b(before.len() as u64));
    println!("after: {}", ByteSize::b(after.len() as u64));
    println!("Reduce {:+.2}%", calc_reduce(before, &after));
    if is_fully_optimized(before.len(), after.len(), &opts) {} else {
        write("out.test.png", &after).unwrap();
    }
}


fn is_fully_optimized(original_size: usize, optimized_size: usize, opts: &Options) -> bool {
    original_size <= optimized_size && !opts.force && opts.interlace.is_none()
}


pub fn calc_reduce(before: &[u8], after: &[u8]) -> f64 {
    let before = before.len() as f64;
    let after = after.len() as f64;
    (before - after) / -before
}