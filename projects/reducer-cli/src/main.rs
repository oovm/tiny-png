use std::env::current_dir;

use clap::{Parser, Subcommand};

use image_reducer::{TinyConfig, TinyResult};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Whether to overwrite image files
    #[arg(short, long, default_value_t = false)]
    execute: bool,
    #[command(subcommand)]
    command: ArgSubs,
}

#[derive(Subcommand, Debug)]
enum ArgSubs {
    /// Clear database and cache
    Clear,
}

#[tokio::main]
async fn main() -> TinyResult {
    let args = Args::parse();
    let mut ws = TinyConfig::default().with_writable(args.execute).with_database(true).initialize(current_dir()?)?;
    ws.check_all_pngs().await?;
    Ok(())
}
