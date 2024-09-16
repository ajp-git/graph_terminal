use anyhow::{Context, Result};
use crossterm::{
    cursor::MoveTo,
    event, execute, queue,
    style::{Color, Colors, Print, SetColors},
    terminal, ExecutableCommand,
};
use std::{
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};

pub struct Terminal3d {
    x: u16,
    y: u16,
    proportion: f64,
}

impl Terminal3d {
    pub fn new() -> Result<Self> {
        let (x, y) = crossterm::terminal::size().context("Failed to get terminal size")?;
        let proportion = 0.5;
        Ok(Self {
            x: ((x as f64 / proportion) as u16).min(y),
            y: y.min((x as f64 / proportion) as u16),
            proportion,
        })
    }

    pub fn get_size(&self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
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
    ) -> Result<&mut Terminal3d, anyhow::Error> {
        let mut stdout = stdout();
        for theta in 0..360 {
            let z_depth = (theta as f64 / 360.0) * 10.0; // simulate depth
            let x_wave = x + (amplitude * (theta as f64).to_radians().sin()) * frequency;
            let y_wave =
                y + (amplitude * (theta as f64).to_radians().cos()) * frequency * self.proportion;
            let shade = (z_depth / 10.0) * 255.0; // calculate shade based on depth
            let shade_char = match shade as u8 {
                0..=63 => ' ',    // dark shade
                64..=127 => '.',  // medium shade
                128..=191 => '*', // light shade
                _ => '+',         // very light shade
            };
            queue!(
                stdout,
                MoveTo(x_wave as u16, y_wave as u16),
                Print(shade_char)
            );
        }
        stdout.flush()?;
        Ok(self)
    }

    pub fn draw_donut(&mut self) -> Result<&mut Self, anyhow::Error> {
        let mut azimuth_angle: f64 = 0.0;
        let mut polar_angle: f64 = 0.0;
        self.clear_terminal()?;
        loop {
            let buffer_len = self.get_size().0 * self.get_size().1;
            let mut depth_buffer = vec![0.0; buffer_len];
            let mut pixel_values = vec![0u8; buffer_len];

            for theta in (0..628).step_by(7) {
                for phi in (0..628).step_by(2) {
                    let theta = theta as f64 / 100.0;
                    let phi = phi as f64 / 100.0;
                    let cosine_phi = phi.cos();
                    let sine_theta = theta.sin();
                    let sine_azimuth_angle = azimuth_angle.sin();
                    let cosine_theta = theta.cos();
                    let cosine_azimuth_angle = azimuth_angle.cos();
                    let h = cosine_theta + 2.0;
                    let distance = 0.80
                        / (cosine_phi * h * sine_azimuth_angle
                            + sine_theta * cosine_azimuth_angle
                            + 5.0);
                    let sine_phi = phi.sin();
                    let cosine_polar_angle = polar_angle.cos();
                    let sine_polar_angle = polar_angle.sin();
                    let t = cosine_phi * h * cosine_azimuth_angle - sine_theta * sine_azimuth_angle;
                    let screen_x: usize = (self.get_size().0 as f64 / 2.0
                        + self.get_size().0 as f64 / 1.2
                            * distance
                            * (sine_phi * h * cosine_polar_angle - t * sine_polar_angle))
                        as usize;
                    let screen_y: usize = (self.get_size().1 as f64 / 2.0
                        + self.get_size().1 as f64 / 1.2
                            * distance
                            * (sine_phi * h * sine_polar_angle + t * cosine_polar_angle)
                            * self.proportion) as usize;
                    let offset = screen_x + self.get_size().0 * screen_y;
                    let illumination_index = ((256.0
                        * ((sine_theta * sine_azimuth_angle
                            - cosine_phi * cosine_theta * cosine_azimuth_angle)
                            * cosine_polar_angle
                            - cosine_phi * cosine_theta * sine_azimuth_angle
                            - sine_theta * cosine_azimuth_angle
                            - sine_phi * cosine_theta * sine_polar_angle)
                        + 256.0)
                        / 2.0) as u8;
                    if self.get_size().1 > screen_y
                        && screen_y > 0
                        && screen_x > 0
                        && self.get_size().0 > screen_x
                        && offset < depth_buffer.len()
                        && distance > depth_buffer[offset]
                    {
                        depth_buffer[offset] = distance;
                        /*pixel_values[offset] = ".,-~:;=!*#$@"
                        .chars()
                        .nth(illumination_index.max(0.0) as usize)
                        .unwrap();*/
                        pixel_values[offset] = illumination_index as u8;
                    }
                }
            }
            //self.clear_terminal()?;
            let mut stdout = stdout();

            for index in 0..buffer_len {
                if index % self.get_size().0 == 0 {
                    queue!(stdout, Print("\n"))?;
                } else {
                    if pixel_values[index] == 0 {
                        queue!(
                            stdout,
                            SetColors(Colors::new(
                                crossterm::style::Color::Black,
                                crossterm::style::Color::Black
                            ))
                        )?;
                        queue!(stdout, Print(" "))?;
                    } else {
                        queue!(
                            stdout,
                            SetColors(Colors::new(
                                crossterm::style::Color::Rgb {
                                    r: pixel_values[index].max(0),
                                    g: pixel_values[index].max(0),
                                    b: pixel_values[index].max(0)
                                },
                                crossterm::style::Color::Black
                            ))
                        )?;
                        queue!(stdout, Print("*"))?;
                    }
                }
                azimuth_angle += 0.000008;
                polar_angle += 0.000005;
            }
            stdout.flush()?;
            sleep(Duration::from_millis(200));
        }
    }
}
