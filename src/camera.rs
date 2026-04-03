use crate::{vec3::Vec3, viewport::Viewport};

pub struct Camera {
    pub position: Vec3,
    pub viewport: Viewport,

    // All directions in absolute sense
    // Up: y, Right: x, Outward: z
    pub look_direction: Vec3,
}

impl Camera {
    pub fn new(
        position: Vec3,
        look_direction: Vec3,
        viewport_height: f64,
        viewport_width: f64,
    ) -> Self {
        Self {
            position,
            viewport: Viewport::new(viewport_width, viewport_height, look_direction),
            look_direction: look_direction.normalise(),
        }
    }
}
