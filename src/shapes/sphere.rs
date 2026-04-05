use crate::{
    color::Color,
    ray::Ray,
    shapes::{HitRecord, Shape},
    vec3::Vec3,
};

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

impl Shape for Sphere {
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        // Solving intersection equations
        // at^2 + bt + c = 0
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * (ray.origin.dot(&ray.direction) - self.centre.dot(&ray.direction));
        let c = ray.origin.dot(&ray.origin) - 2.0 * ray.origin.dot(&self.centre)
            + self.centre.dot(&self.centre)
            - self.radius.powi(2);

        let d = b.powi(2) - 4.0 * a * c;
        if d < 0.0 {
            return None;
        }

        // Solutions for time(t) when the ray intersects the object
        let s1 = (-b + d.sqrt()) / (2.0 * a);
        let s2 = (-b - d.sqrt()) / (2.0 * a);

        let mut t = s1.min(s2);
        if t < 0.0 {
            t = s1.max(s2);
        }

        // Both negative
        if t < 0.0 {
            return None;
        }

        let point = ray.at(t);
        let normal = point.subtract(&self.centre).normalise();

        Some(HitRecord {
            distance: t,
            point,
            normal,
            color: self.color,
        })
    }
}
