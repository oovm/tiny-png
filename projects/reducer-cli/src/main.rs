use std::env::current_dir;

use clap::Parser;

use image_reducer::{TinyConfig, TinyResult};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of times to greet
    #[arg(short, long, default_value_t = false)]
    execute: bool,
}

#[tokio::main]
async fn main() -> TinyResult {
    let args = Args::parse();
    let mut ws = TinyConfig::default().with_writable(args.execute).initialize(current_dir()?)?;
    ws.check_all_pngs().await?;
    Ok(())
}
