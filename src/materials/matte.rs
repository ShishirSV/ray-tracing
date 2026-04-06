use crate::{color::Color, materials::Material, vec3::Vec3};

pub struct Matte {
    pub base_color: Color,
    pub diffuse_intensity: f64,
}

impl Matte {
    pub fn new(base_color: Color, diffuse_intensity: f64) -> Self {
        Self {
            base_color,
            diffuse_intensity,
        }
    }
}

impl Material for Matte {
    fn shade(
        &self,
        normal: &Vec3,
        point_to_light: &Vec3,
        _point_to_camera: &Vec3,
        incoming_light: &Vec3,
    ) -> Vec3 {
        // Only diffuse component
        let normal_alignment = normal.dot(&point_to_light).max(0.0);

        self.base_color
            .rgb
            .component_multiply(&incoming_light)
            .multiply(normal_alignment)
            .multiply(self.diffuse_intensity)
    }

    fn ambient(&self, ambient_light: &Vec3) -> Vec3 {
        self.base_color.rgb.component_multiply(ambient_light)
    }
}
