use crate::{errors::TinyError, TinyResult};
use bytesize::ByteSize;
use chrono::Local;
use env_logger::fmt::Formatter;
use log::{LevelFilter, Record};
use oxipng::{optimize_from_memory, Options};
use std::{
    hash::{BuildHasher, BuildHasherDefault, Hasher},
    io::Write,
};
use twox_hash::XxHash64;

pub struct ImageBuffer {
    pub output: Vec<u8>,
    pub before: ByteSize,
    pub after: ByteSize,
    pub reduce: f64,
}

pub fn optimize_png(png: &[u8]) -> TinyResult<ImageBuffer> {
    let mut opts = Options { ..Options::default() };
    let image = optimize_from_memory(png, &opts)?;
    let before = ByteSize::b(png.len() as u64);
    let after = ByteSize::b(image.len() as u64);
    let reduce = calc_reduce(png, &image);
    if is_fully_optimized(png.len(), image.len(), &opts) {
        return Err(TinyError::ImageOptimized);
    }
    Ok(ImageBuffer { output: image, before, after, reduce })
}

pub fn is_fully_optimized(original_size: usize, optimized_size: usize, opts: &Options) -> bool {
    original_size <= optimized_size && opts.interlace.is_none()
}

pub fn calc_reduce(before: &[u8], after: &[u8]) -> f64 {
    let before = before.len() as f64;
    let after = after.len() as f64;
    (before - after) / (before * -100.0)
}

pub fn hash_file(image: &[u8]) -> u64 {
    let mut hasher: XxHash64 = BuildHasherDefault::default().build_hasher();
    hasher.write(image);
    hasher.finish()
}

pub fn logger() {
    let _ = env_logger::builder()
        .format_module_path(false)
        .format(log_writter)
        .filter(Some("oxipng"), LevelFilter::Off)
        .filter_level(LevelFilter::Trace)
        // .is_test(false)
        .try_init();
}

fn log_writter(w: &mut Formatter, record: &Record) -> std::io::Result<()> {
    let logs = format!("[{} {}] {}", record.level(), Local::now(), record.args());
    for (i, line) in logs.lines().enumerate() {
        if i != 0 {
            w.write(b"\n")?;
            w.write(b"    ")?;
        }
        w.write(line.as_bytes())?;
    }
    w.write(b"\n")?;
    Ok(())
}
