use std::f64;

use crate::{
    camera::Camera, canvas::Canvas, color::Color, lights::Light, materials::Material, ray::Ray,
    shapes::Shape, vec3::Vec3,
};

pub struct World {
    pub camera: Camera,
    pub canvas: Canvas,
    pub objects: Vec<Box<dyn Shape>>,
    pub lights: Vec<Box<dyn Light>>,

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

    fn check_lighting(&self, point: &Vec3, normal: &Vec3, material: &dyn Material) -> Color {
        let point_to_camera = self.camera.position.subtract(point).normalise();
        let ambient_light = self.ambient_color.rgb.multiply(self.ambient_intensity);
        let mut total_light_energy = material.ambient(&ambient_light);

        for light in &self.lights {
            let illumination = light.get_illumination(point);
            let light_to_point = illumination.point_to_light.negate();
            let los_distance = illumination.distance;
            let light_ray = Ray::new(illumination.source_position, light_to_point);
            let mut reaches = true;

            for object in &self.objects {
                let record = object.hit(&light_ray);

                if let Some(record) = record {
                    // It is blocked by some other object before reaching the point
                    // Subtracting 0.0001 to prevent cases where object seems to block itself due
                    // to float precision
                    if record.distance > 0.0 && record.distance < (los_distance - 0.0001) {
                        reaches = false;
                        break;
                    }
                }
            }

            if !reaches {
                continue;
            }

            let energy = material.shade(
                normal,
                &illumination.point_to_light,
                &point_to_camera,
                &illumination.light,
            );
            total_light_energy = total_light_energy.add(&energy);
        }

        Color::tone(&total_light_energy)
    }

    pub fn trace(&self, rays: &Vec<Ray>) -> Color {
        let mut colors: Vec<Color> = Vec::with_capacity(rays.len());

        for ray in rays {
            // Initialising the intersection variables
            let mut intersection_point: Option<Vec3> = None; // Nearest point of intersection
            let mut intersection_distance = f64::INFINITY; // Distance from camera to nearest point
            let mut object_material: Option<&dyn Material> = None; // Material at the intersected point
            let mut normal = Vec3::new(-1.0, -1.0, -1.0); // Surface normal for object at intersection

            // Check if ray hits any of the objects(take the first one)
            for object in &self.objects {
                let record = object.hit(ray);

                if let Some(record) = record {
                    if record.distance < intersection_distance {
                        intersection_distance = record.distance;
                        intersection_point = Some(record.point);
                        normal = record.normal;
                        object_material = Some(record.material);
                    }
                }
            }

            // Check if light from the light sources reaches the above pixel
            let color = if let (Some(point), Some(material)) = (intersection_point, object_material)
            {
                let pixel_color = self.check_lighting(&point, &normal, material);
                pixel_color
            } else {
                self.background
            };

            colors.push(color);
        }

        self.color_average(&colors)
    }

    fn color_average(&self, colors: &Vec<Color>) -> Color {
        let mut all = Vec3::new(0.0, 0.0, 0.0);

        for color in colors {
            all = all.add(&color.rgb);
        }

        let averge = all.divide(colors.len() as f64);

        Color::try_from(averge).unwrap_or(self.background)
    }
}
