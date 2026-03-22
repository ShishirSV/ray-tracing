struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    fn add(&self, other: &Vec3) -> Self {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    fn subtract(&self, other: &Vec3) -> Self {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn multiply(&self, other: &Vec3, scalar: f32) -> Self {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    fn divide(&self, other: &Vec3, scalar: f32) -> Self {
        Vec3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }

    fn dot(&self, other: &Vec3) -> f32 {
        let result = (self.x * other.x) + (self.y * other.y) + (self.z * other.y);

        result.sqrt()
    }

    fn cross(&self, other: &Vec3) -> Self {
        let x = self.y * other.z - self.z * other.y;
        let y = self.z * other.x - self.x * other.z;
        let z = self.x * other.y - self.y * other.x;

        Vec3 { x, y, z }
    }

    fn negate(&self) -> Self {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    fn componentMultipy(&self, other: &Vec3) -> Self {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}
