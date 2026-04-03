use crate::color::Color;
use std::fs::File;
use std::io::Write;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Vec<Color>>,
    pub is_set: Vec<Vec<bool>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![vec![Color::black(); width]; height],
            is_set: vec![vec![false; width]; height],
        }
    }

    pub fn reset(&mut self) {
        for row in self.is_set.iter_mut() {
            row.fill(false);
        }

        for row in self.pixels.iter_mut() {
            row.fill(Color::black());
        }
    }

    pub fn set_pixel(&mut self, row: usize, col: usize, color: Color) {
        self.pixels[row][col] = color;
    }

    pub fn write_ppm(&self, path: &str) -> Result<(), std::io::Error> {
        let mut file = File::create(path)?;
        writeln!(file, "P3\n{} {}\n255", self.width, self.height)?; // setting the file header

        for row in &self.pixels {
            for color in row {
                writeln!(
                    file,
                    "{} {} {}",
                    color.get_red(),
                    color.get_green(),
                    color.get_blue()
                )?;
            }
        }

        Ok(())
    }
}
