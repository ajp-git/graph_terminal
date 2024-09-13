use clap::Parser;
mod libs;
use anyhow::{Context, Result};
use libs::terminal::Terminal3d;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of x points
    #[arg(short, long)]
    x: usize,

    /// Number of y points
    #[arg(short, long)]
    y: usize,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let terminal = Terminal3d::new()?;
    terminal.fill_terminal();
    Ok(())
}
