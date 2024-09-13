use anyhow::{Context, Result};
use crossterm::{
    event, execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand,
};

pub struct Terminal3d {
    x: u16,
    y: u16,
}

impl Terminal3d {
    pub fn new() -> Result<Self, anyhow::Error> {
        let (x, y) = crossterm::terminal::size()?;
        Ok(Self { x: x, y: y })
    }

    pub fn fill_terminal(&self) {
        for y in 0..self.y {
            for x in 0..self.x {
                print!("#");
            }
            println!();
        }
    }

    pub fn draw_sample_function(&self) {}
}
