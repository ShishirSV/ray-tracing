use crate::utils::{RIGHT, UP};
use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Viewport {
    // All directions in absolute sense
    // Up: y, Right: x, Outward: z
    pub half_width: f64,  // Along local right
    pub half_height: f64, // Along local up

    // All following local directions also w.r.t to above conventions
    pub local_forward: Vec3,
    pub local_up: Vec3,
    pub local_right: Vec3,
}

impl Viewport {
    pub fn new(width: f64, height: f64, camera_direction: Vec3) -> Self {
        let right: Vec3;
        let up: Vec3;

        // According to right hand rule
        if !camera_direction.equals(&UP) {
            right = camera_direction.cross(&UP);
            up = right.cross(&camera_direction);
        } else {
            up = RIGHT.cross(&camera_direction);
            right = camera_direction.cross(&up);
        }

        Self {
            half_width: width / 2.0,
            half_height: height / 2.0,
            local_forward: camera_direction,
            local_up: up,
            local_right: right,
        }
    }
}
