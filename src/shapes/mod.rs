use crate::{color::Color, ray::Ray, vec3::Vec3};

pub mod sphere;

pub struct HitRecord {
    pub distance: f64, // Distance from ray origin(t)
    pub point: Vec3,   // 3D corrdinate of intersection
    pub normal: Vec3,  // Normal vector at intersection
    pub color: Color,  // Color of the shape
}

pub trait Shape {
    fn hit(&self, ray: Ray) -> Option<HitRecord>;
}
