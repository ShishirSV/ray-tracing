use crate::{color::Color, vec3::Vec3};

pub struct PointLight {
    pub position: Vec3,
    pub intensity: f64,
    pub color: Color,
}

impl PointLight {
    pub fn new(position: Vec3, intensity: f64, color: Color) -> Self {
        Self {
            position,
            intensity,
            color,
        }
    }
}
