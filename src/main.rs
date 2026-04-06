use ray_tracing::camera::Camera;
use ray_tracing::canvas::Canvas;
use ray_tracing::color::Color;
use ray_tracing::lights::point_light::PointLight;
use ray_tracing::materials::metallic::Metallic;
use ray_tracing::shapes::sphere::Sphere;
use ray_tracing::vec3::Vec3;
use ray_tracing::world::World;

fn main() {
    // Image dimensions
    let width = 400;
    let height = 400;

    // Setup Camera
    let cam_pos = Vec3::new(0.0, 0.0, 0.0);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let camera = Camera::new(cam_pos, look_at, 2.0, 2.0, 1.0);

    // Initialize World and Canvas
    let canvas = Canvas::new(width, height);
    let mut world = World::new(camera, canvas);

    // Setup Lighting and Background
    world.set_ambient_color(Color::new(1.0, 1.0, 1.0).unwrap());
    world.set_ambient_intensity(0.1);
    world.set_background(Color::new(0.1, 0.1, 0.2).unwrap());

    // Point light
    let light = PointLight {
        position: Vec3::new(-5.0, 5.0, 0.0),
        color: Color::new(1.0, 1.0, 1.0).unwrap(),
        intensity: 50.0, // High intensity for falloff distance
    };
    world.lights.push(Box::new(light));

    // Add a Sphere
    let red_metallic = Metallic::new(
        Color::new(1.0, 0.0, 0.0).unwrap(), // base_color: Red
        0.8,                                // diffuse_intensity
        20.0,                               // shininess
        2.0,                                // specular intensity
    );
    let red_sphere = Sphere::new(Vec3::new(0.0, 0.0, -5.0), 1.0, Box::new(red_metallic));
    world.objects.push(Box::new(red_sphere));

    // Render Loop
    for row in 0..height {
        for col in 0..width {
            let ray = world.camera.get_ray(row, col, width, height);
            let pixel_color = world.trace(&ray);
            world.canvas.set_pixel(row, col, pixel_color);
        }
    }

    // Save the output
    match world.canvas.write_ppm("output.ppm") {
        Ok(_) => println!("Successfully saved output.ppm"),
        Err(e) => eprintln!("Error saving file: {}", e),
    }
}
