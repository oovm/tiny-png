use clap::Parser;
use std::env::current_dir;
use tiny_png::{TinyResult, TinyWorkspace};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = true)]
    dry_run: bool,
}

#[allow(dead_code)]
fn main() -> TinyResult {
    let args = Args::parse();
    let mut ws = TinyWorkspace::initialize(current_dir()?, !args.dry_run);
    ws.check_all_pngs();
    Ok(())
}
