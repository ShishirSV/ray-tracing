use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, distance: f64) -> Vec3 {
        let d = self.direction.multiply(distance);

        Vec3::new(
            self.origin.x + d.x,
            self.origin.y + d.y,
            self.origin.z + d.z,
        )
    }
}

impl From<&[Vec3; 2]> for Ray {
    fn from(vector: &[Vec3; 2]) -> Self {
        Self {
            origin: vector[0],
            direction: vector[1],
        }
    }
}
