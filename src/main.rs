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
    let mut terminal = Terminal3d::new()?;
    //terminal.draw_sample_function()?;

    terminal
        .clear_terminal()?
        .draw_3d_wave(5.0, 2.0, 20.0, 20.0, 0.0)?;

    Ok(())
}
