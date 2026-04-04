use std::f64;

use crate::{
    camera::Camera, canvas::Canvas, color::Color, lights::point_light::PointLight, ray::Ray,
    shapes::sphere::Sphere, vec3::Vec3,
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

    fn solve(&self, ray: &Ray, object: &Sphere) -> (Option<Vec3>, f64) {
        // Solving intersection equations
        // at^2 + bt + c = 0
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * (ray.origin.dot(&ray.direction) - object.centre.dot(&ray.direction));
        let c = ray.origin.dot(&ray.origin) - 2.0 * ray.origin.dot(&object.centre)
            + object.centre.dot(&object.centre)
            - object.radius.powi(2);

        let d = b.powi(2) - 4.0 * a * c;
        if d < 0.0 {
            return (None, 0.0);
        }

        // Solutions for time(t) when the ray intersects the object
        let s1 = (-b + d.sqrt()) / (2.0 * a);
        let s2 = (-b - d.sqrt()) / (2.0 * a);

        let mut t = s1.min(s2);
        if t < 0.0 {
            t = s1.max(s2);
        }

        // Both negative
        if t < 0.0 {
            return (None, 0.0);
        }

        (Some(ray.at(t)), t)
    }

    fn check_ligthing(&self, point: &Vec3, object_color: &Color, normal: &Vec3) -> Color {
        let mut final_color = object_color
            .rgb
            .component_multiply(&self.ambient_color.rgb)
            .multiply(self.ambient_intensity);

        for light in &self.lights {
            let los = point.subtract(&light.position);
            let los_distance = los.magnitude();
            let light_ray = Ray::new(light.position, los.normalise());
            let mut reaches = true;

            for object in &self.objects {
                let (point, distance) = self.solve(&light_ray, object);

                if let Some(_point) = point {
                    // It is blocked by some other object before reaching the point
                    // Subtracting 0.0001 to prevent cases where object seems to block itself due
                    // to float precision
                    if distance > 0.0 && distance < (los_distance - 0.0001) {
                        reaches = false;
                        break;
                    }
                }
            }

            if !reaches {
                continue;
            }

            let point_to_light = &light.position.subtract(point).normalise();
            let diffuse_intensity = normal.dot(&point_to_light).max(0.0);
            let falloff = 1.0 / los_distance.powi(2);
            let light_contribution = object_color
                .rgb
                .component_multiply(&light.color.rgb)
                .multiply(light.intensity)
                .multiply(diffuse_intensity)
                .multiply(falloff);
            final_color = final_color.add(&light_contribution);
        }

        Color::tone(&final_color)
    }

    pub fn trace(&self, ray: &Ray) -> Color {
        // Initialising the intersection variables
        let mut intersection_point: Option<Vec3> = None; // Nearest point of intersection
        let mut intersection_distance = f64::INFINITY; // Distance from camera to nearest point
        let mut object_color = Color::black(); // Color at the intersected point
        let mut normal = Vec3::new(-1.0, -1.0, -1.0); // Surface normal for object at intersection

        // Check if ray hits any of the objects(take the first one)
        for object in &self.objects {
            let (point, distance) = self.solve(ray, object);

            if let Some(point) = point {
                if distance < intersection_distance {
                    intersection_distance = distance;
                    intersection_point = Some(point);
                    object_color = object.color;
                    normal = point.subtract(&object.centre).normalise();
                }
            }
        }

        // Check if light from the light sources reaches the above pixel
        if let Some(point) = intersection_point {
            let pixel_color = self.check_ligthing(&point, &object_color, &normal);
            return pixel_color;
        }

        self.background
    }
}
