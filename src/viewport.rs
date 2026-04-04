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
        let fwd = camera_direction.normalise();

        let (right, up) = if fwd.dot(&UP).abs() < 0.999 {
            // Standard case: Not looking straight up or down
            let r = fwd.cross(&UP).normalise();
            let u = r.cross(&fwd); // Already unit length because fwd and r are orthogonal units
            (r, u)
        } else {
            // Edge case: Looking straight up or down
            // Use RIGHT as the temporary reference instead
            let u = RIGHT.cross(&fwd).normalise();
            let r = fwd.cross(&u);
            (r, u)
        };

        Self {
            half_width: width / 2.0,
            half_height: height / 2.0,
            local_forward: fwd.normalise(),
            local_up: up.normalise(),
            local_right: right.normalise(),
        }
    }
}
