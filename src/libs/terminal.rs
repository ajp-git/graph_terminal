use anyhow::{Context, Result};
use crossterm::{cursor::MoveTo, event, execute, queue, style::Print, terminal, ExecutableCommand};
use std::io::{stdout, Write};

pub struct Terminal3d {
    x: u16,
    y: u16,
    proportion: f64,
}

impl Terminal3d {
    pub fn new() -> Result<Self> {
        let (x, y) = crossterm::terminal::size().context("Failed to get terminal size")?;
        Ok(Self {
            x,
            y,
            proportion: 0.5,
        })
    }

    pub fn fill_terminal(&self) -> Result<&Self> {
        for _ in 0..self.y {
            for _ in 0..self.x {
                print!("#");
            }
            println!();
        }
        Ok(self)
    }

    pub fn clear_terminal(&mut self) -> Result<&mut Self, anyhow::Error> {
        let mut stdout = stdout();
        stdout.execute(terminal::Clear(terminal::ClearType::All))?;
        Ok(self)
    }

    pub fn draw_sample_function(&mut self) -> Result<&mut Terminal3d, anyhow::Error> {
        let mut stdout = stdout();
        stdout
            .execute(terminal::Clear(terminal::ClearType::All))
            .map_err(|e| anyhow::Error::from(e))?;

        self.draw_3d_sphere(10.0, 20.0, 20.0, 0.0)?;

        stdout.flush()?;
        Ok(self)
    }

    pub fn draw_3d_sphere(
        &mut self,
        radius: f64,
        x: f64,
        y: f64,
        z: f64,
    ) -> Result<&mut Terminal3d, anyhow::Error> {
        let mut stdout = stdout();
        for theta in 0..360 {
            for phi in -90..91 {
                let theta = theta as f64;
                let phi = phi as f64;
                let x_sphere = x + radius * theta.to_radians().sin() * phi.to_radians().cos();
                let y_sphere = y + radius * phi.to_radians().sin() * self.proportion;
                let z_sphere = z + radius * theta.to_radians().cos() * phi.to_radians().cos();
                queue!(stdout, MoveTo(x_sphere as u16, y_sphere as u16), Print("*"));
            }
        }
        stdout.flush()?;
        Ok(self)
    }
    pub fn draw_3d_wave(
        &mut self,
        amplitude: f64,
        frequency: f64,
        x: f64,
        y: f64,
        z: f64,
    ) -> Result<&mut Terminal3d, anyhow::Error> {
        let mut stdout = stdout();
        for theta in 0..360 {
            for phi in -90..91 {
                let theta = theta as f64;
                let phi = phi as f64;
                let x_wave = x + (amplitude * theta.to_radians().sin()) * phi.to_radians().cos();
                let y_wave = y
                    + (amplitude * frequency * theta.to_radians().cos())
                        * phi.to_radians().sin()
                        * self.proportion;
                let z_wave =
                    z + (amplitude * frequency * theta.to_radians().sin()) * phi.to_radians().cos();
                queue!(stdout, MoveTo(x_wave as u16, y_wave as u16), Print("*"));
            }
        }
        stdout.flush()?;
        Ok(self)
    }
}
