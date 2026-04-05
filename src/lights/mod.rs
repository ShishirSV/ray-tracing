use crate::{color::Color, vec3::Vec3};

pub mod point_light;

pub struct Illumination {
    pub direction: Vec3, // Vector from the surface point to the light
    pub distance: f64,   // Distance between light and the point
    pub color: Color,    // The intensity reaching the point(falloff)
}

pub trait Light {
    // Given a point on a surface, what light arrives there?
    fn get_illumination(&self, point: &Vec3) -> Illumination;
}
