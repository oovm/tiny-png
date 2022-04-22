use bytesize::ByteSize;
use oxipng::{optimize_from_memory, Options};

use crate::{errors::TinyError, TinyResult};

pub struct ImageBuffer {
    output: Vec<u8>,
    before: ByteSize,
    after: ByteSize,
    reduce: f64,
}

pub fn optimize_png(png: &[u8]) -> TinyResult<ImageBuffer> {
    let mut opts = Options { ..Options::default() };
    let image = optimize_from_memory(png, &opts)?;
    let before = ByteSize::b(png.len() as u64);
    let after = ByteSize::b(image.len() as u64);
    let reduce = calc_reduce(png, &image);
    let output = if is_fully_optimized(png.len(), image.len(), &opts) { return Err(TinyError::ImageOptimized) } else { image };
    Ok(ImageBuffer { output, before, after, reduce })
}

pub fn is_fully_optimized(original_size: usize, optimized_size: usize, opts: &Options) -> bool {
    original_size <= optimized_size && opts.interlace.is_none()
}

pub fn calc_reduce(before: &[u8], after: &[u8]) -> f64 {
    let before = before.len() as f64;
    let after = after.len() as f64;
    (before - after) / -before
}

#[test]
pub fn main() {
    let before = include_bytes!("../../iphone.test.png");
    let mut out = optimize_png(before).unwrap();
    println!("before: {}", out.before);
    println!("after: {}", out.after);
    println!("Reduce {:+.2}%", out.reduce);
}
