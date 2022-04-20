use bytesize::ByteSize;
use eta::{Eta, TimeAcc};
use oxipng::{optimize_from_memory, Options};

use crate::errors::TinyError;

pub use self::errors::Result;

mod errors;

fn main() {
    println!("GGG")
}

pub struct TinyPNG {
    value: i32,
    language: String,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    ClearAll,
    DryRun,
    Run,
    Pause,
    Stop,
}

pub struct ImageBuffer {
    output: Vec<u8>,
    before: ByteSize,
    after: ByteSize,
    reduce: f64,
}

pub fn optimize_png(png: &[u8]) -> Result<ImageBuffer> {
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

fn calculate_square(number: usize) -> usize {
    number * number
}

#[test]
fn test() {
    let count = 100;
    let numbers = Vec::from_iter(0..count);
    let mut eta = Eta::new(count, TimeAcc::MILLI);

    for number in numbers {
        calculate_square(number);
        eta.step();
        if (number % 10) == 0 {
            println!("{}", eta);
        }
    }
}
