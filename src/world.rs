use crate::{
    camera::Camera, canvas::Canvas, color::Color, lights::point_light::PointLight,
    shapes::sphere::Sphere,
};

pub struct World {
    pub camera: Camera,
    pub canvas: Canvas,
    pub objects: Vec<Sphere>,
    pub lights: Vec<PointLight>,

    // Color returned when the ray hit nothing
    pub background: Color,

    // Minimum light in the world
    pub ambient_color: Color,
    pub ambient_intensity: f64,
}

impl World {
    pub fn new(camera: Camera, canvas: Canvas) -> Self {
        Self {
            camera,
            canvas,
            objects: Vec::new(),
            lights: Vec::new(),
            background: Color::black(),
            ambient_color: Color::black(),
            ambient_intensity: 1.0,
        }
    }

    pub fn set_background(&mut self, color: Color) {
        self.background = color;
    }

    pub fn set_ambient_intensity(&mut self, intensity: f64) {
        self.ambient_intensity = intensity;
    }

    pub fn set_ambient_color(&mut self, color: Color) {
        self.ambient_color = color;
    }
}
