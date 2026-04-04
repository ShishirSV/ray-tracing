use crate::{ray::Ray, vec3::Vec3, viewport::Viewport};

pub struct Camera {
    pub position: Vec3,
    pub viewport: Viewport,
    pub focal_length: f64,

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
        focal_length: f64,
    ) -> Self {
        Self {
            position,
            viewport: Viewport::new(viewport_width, viewport_height, look_direction),
            focal_length,
            look_direction: look_direction.normalise(),
        }
    }

    pub fn get_ray(
        &self,
        row: usize,
        col: usize,
        canvas_width: usize,
        canvas_height: usize,
    ) -> Ray {
        // Corresponding x, y in the viewport
        let width_ratio = (2.0 * self.viewport.half_width as f64) / canvas_width as f64;
        let height_ratio = (2.0 * self.viewport.half_height as f64) / canvas_height as f64;
        let v_row = (row as f64 + 0.5) * height_ratio; // 0.5 is for centering
        let v_col = (col as f64 + 0.5) * width_ratio;

        // Creating the ray from camera through viewport(vx, vy)
        let origin = self.position;
        let middle_viewport = origin.add(&self.look_direction.multiply(self.focal_length));

        // Offsets from the middle of the viewport
        let up_offset = self.viewport.half_height - v_row;
        let right_offset = v_col - self.viewport.half_width; // Since column 0 is left(-Right)
        let mut viewport_coordinate =
            middle_viewport.add(&self.viewport.local_up.multiply(up_offset));
        viewport_coordinate =
            viewport_coordinate.add(&self.viewport.local_right.multiply(right_offset));
        let ray_direction = viewport_coordinate.subtract(&self.position).normalise();

        Ray::new(self.position, ray_direction)
    }
}
