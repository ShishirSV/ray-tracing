use crate::{ray::Ray, utils::RAY_COUNT, vec3::Vec3, viewport::Viewport};
use rand::Rng;

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
    ) -> Vec<Ray> {
        let mut rng = rand::thread_rng();
        let mut rays: Vec<Ray> = Vec::with_capacity(RAY_COUNT);

        // Corresponding x, y in the viewport
        let width_ratio = (2.0 * self.viewport.half_width as f64) / canvas_width as f64;
        let height_ratio = (2.0 * self.viewport.half_height as f64) / canvas_height as f64;

        // Creating the ray from camera through viewport(vx, vy)
        let origin = self.position;
        let middle_viewport = origin.add(&self.look_direction.multiply(self.focal_length));

        for _ in 0..RAY_COUNT {
            // Generating offsets
            let row_offset = rng.gen_range(-0.99..1.00);
            let col_offset = rng.gen_range(-0.99..1.00);

            // Offsets from the middle of the viewport
            let vp_row = (row as f64 + row_offset) * height_ratio;
            let vp_col = (col as f64 + col_offset) * width_ratio;
            let up_offset = self.viewport.half_height - vp_row;
            let right_offset = vp_col - self.viewport.half_width; // Since column 0 is left(-Right)
            let mut viewport_coordinate =
                middle_viewport.add(&self.viewport.local_up.multiply(up_offset));
            viewport_coordinate =
                viewport_coordinate.add(&self.viewport.local_right.multiply(right_offset));
            let ray_direction = viewport_coordinate.subtract(&self.position).normalise();

            rays.push(Ray::new(self.position, ray_direction));
        }

        rays
    }
}
