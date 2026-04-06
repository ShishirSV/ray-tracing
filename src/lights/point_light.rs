use crate::{
    color::Color,
    lights::{Illumination, Light},
    vec3::Vec3,
};

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

impl Light for PointLight {
    fn get_illumination(&self, point: &Vec3) -> Illumination {
        let point_to_light = self.position.subtract(point);
        let distance = point_to_light.magnitude();
        let falloff = 1.0 / distance.powi(2);
        let final_light = self.color.rgb.multiply(falloff).multiply(self.intensity);

        Illumination {
            source_position: self.position,
            point_to_light: point_to_light.normalise(),
            distance,
            light: final_light,
        }
    }
}
