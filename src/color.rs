use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub rgb: Vec3,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self {
            rgb: Vec3::new(red, green, blue),
        }
    }
}

impl From<Vec3> for Color {
    fn from(vector: Vec3) -> Self {
        Self { rgb: vector }
    }
}
