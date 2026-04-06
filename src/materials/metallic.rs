use crate::{color::Color, materials::Material, vec3::Vec3};

pub struct Metallic {
    pub base_color: Color,
    pub diffuse_intensity: f64,
    pub shininess: f64,          // How sharp/small the highlight is
    pub specular_intensity: f64, // How bright the highlight is
}

impl Metallic {
    pub fn new(
        base_color: Color,
        diffuse_intensity: f64,
        shininess: f64,
        specular_intensity: f64,
    ) -> Self {
        Self {
            base_color,
            diffuse_intensity,
            shininess,
            specular_intensity,
        }
    }
}

impl Material for Metallic {
    fn shade(
        &self,
        normal: &Vec3,
        point_to_light: &Vec3,
        point_to_camera: &Vec3,
        incoming_light: &Vec3,
    ) -> Vec3 {
        // Diffuse component
        let diffuse_alignment = normal.dot(&point_to_light).max(0.0);
        let diffuse_vector = self
            .base_color
            .rgb
            .component_multiply(&incoming_light)
            .multiply(diffuse_alignment)
            .multiply(self.diffuse_intensity);

        // Specular component(shiny/reflection part)
        let normal_component = point_to_light.dot(&normal);
        let reflection_vector = normal
            .multiply(2.0 * normal_component)
            .subtract(&point_to_light)
            .normalise();
        let camera_alignment = reflection_vector.dot(&point_to_camera).max(0.0);
        let specular_factor = camera_alignment.powf(self.shininess);
        let specular_vector = incoming_light
            .multiply(specular_factor)
            .multiply(self.specular_intensity);

        diffuse_vector.add(&specular_vector)
    }

    fn ambient(&self, ambient_light: &Vec3) -> Vec3 {
        self.base_color.rgb.component_multiply(ambient_light)
    }
}
