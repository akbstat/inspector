use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
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
