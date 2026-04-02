use crate::vec3::Vec3;

pub struct Camera {
    pub position: Vec3,
    pub look_direction: Vec3,
    pub up: Vec3,
    pub right: Vec3,
}
