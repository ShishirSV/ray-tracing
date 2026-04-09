use crate::vec3::Vec3;

pub mod point_light;

pub struct Illumination {
    pub source_position: Vec3,
    pub point_to_light: Vec3, // Vector from the surface point to the light
    pub distance: f64,        // Distance between light and the point
    pub light: Vec3,          // The intensity reaching the point(falloff)
}

pub trait Light: Send + Sync {
    // Given a point on a surface, what light arrives there?
    fn get_illumination(&self, point: &Vec3) -> Illumination;
}
