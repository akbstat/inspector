use std::path::Path;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let args = Args::parse();
    let mut i = inspector::Inspector::new();
    i.collect(Path::new(&args.target))?;
    i.status().unwrap();
    Ok(())
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    target: String,
    #[arg(short, long)]
    dest: String,
}
