use crate::vec3::Vec3;

pub mod matte;
pub mod metallic;

pub trait Material: Send + Sync {
    fn shade(
        &self,
        normal: &Vec3,
        point_to_light: &Vec3,
        point_to_camera: &Vec3,
        incoming_light: &Vec3,
    ) -> Vec3;

    fn ambient(&self, ambient_light: &Vec3) -> Vec3;
}
