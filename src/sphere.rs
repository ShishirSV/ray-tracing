use crate::{color::Color, vec3::Vec3};

pub struct Sphere {
    pub centre: Vec3,
    pub radius: f64,
    pub color: Color,
}

impl Sphere {
    pub fn new(centre: Vec3, radius: f64, color: Color) -> Self {
        Self {
            centre,
            radius,
            color,
        }
    }
}
