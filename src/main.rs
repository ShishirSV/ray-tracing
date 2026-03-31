use ray_tracing::canvas::Canvas;
use ray_tracing::color::Color;

fn main() {
    let rows = 100;
    let cols = 100;
    let mut my_canvas = Canvas::new(rows, cols);
    for i in 0..rows {
        for j in 0..cols {
            let c = Color::new(i as f64 / rows as f64, j as f64 / cols as f64, 0.25)
                .unwrap_or(Color::black());
            my_canvas.set_pixel(i, j, c);
        }
    }

    let _write = my_canvas.write_ppm("test.ppm");
}
