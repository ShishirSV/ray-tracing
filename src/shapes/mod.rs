use crate::{materials::Material, ray::Ray, vec3::Vec3};

pub mod sphere;

pub struct HitRecord<'a> {
    pub distance: f64,              // Distance from ray origin(t)
    pub point: Vec3,                // 3D corrdinate of intersection
    pub normal: Vec3,               // Normal vector at intersection
    pub material: &'a dyn Material, // Color of the shape
}

pub trait Shape: Send + Sync {
    fn hit(&self, ray: &Ray) -> Option<HitRecord<'_>>;
}
