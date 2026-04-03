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
