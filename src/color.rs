use crate::vec3::Vec3;
use std::convert::TryFrom;
use std::{error::Error, fmt};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub rgb: Vec3,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Result<Self, ColorError> {
        if red < 0.0 || blue < 0.0 || green < 0.0 || red > 1.0 || blue > 1.0 || green > 1.0 {
            return Err(ColorError::OutOfBounds(format!(
                "RGB values should be between 0.0 and 1.0 inclusive"
            )));
        }

        Ok(Self {
            rgb: Vec3::new(red, green, blue),
        })
    }
    pub fn get_red(&self) -> u8 {
        (self.rgb.x * 255.0).round() as u8
    }

    pub fn get_green(&self) -> u8 {
        (self.rgb.y * 255.0).round() as u8
    }

    pub fn get_blue(&self) -> u8 {
        (self.rgb.z * 255.0).round() as u8
    }

    pub fn black() -> Self {
        Self {
            rgb: Vec3::new(1.0, 1.0, 1.0),
        }
    }
}

impl TryFrom<Vec3> for Color {
    type Error = ColorError;

    fn try_from(v: Vec3) -> Result<Self, Self::Error> {
        if v.x < 0.0 || v.y < 0.0 || v.z < 0.0 || v.x > 1.0 || v.y > 1.0 || v.z > 1.0 {
            return Err(ColorError::OutOfBounds(format!(
                "RGB values should be between 0.0 and 1.0 inclusive"
            )));
        }
        Ok(Self { rgb: v })
    }
}

#[derive(Debug)]
pub enum ColorError {
    OutOfBounds(String),
}

impl fmt::Display for ColorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ColorError::OutOfBounds(msg) => write!(f, "RGB out of bounds: {}", msg),
        }
    }
}

impl Error for ColorError {}
