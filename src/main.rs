mod raytrace;

use raytrace::*;
use rand::Rng;
use std::io::{stdout, Write, BufWriter};

fn main() {

    let out = stdout();
    let mut out = BufWriter::new(out.lock());

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let sample_per_pixel = 1;

    // World
    let mut world = HitableList::new();
    world.add(&Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(&Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    // Camera
    let camera = Camera::new();
    let mut rng = rand::thread_rng();

    // Render
    write!(out, "P3\n{:?} {:?}\n255\n", image_width, image_height).unwrap();

    for h in (0..(image_height as i32)).rev() {
        for w in 0..(image_width as i32) {
            let mut color = Color::black();
            let origin = Point3::origin();

            for i in 0..sample_per_pixel {
                let u = (w as f64 + rng.gen::<f64>()) / image_width as f64;
                let v = (h as f64 + rng.gen::<f64>()) / image_height as f64;
                let ray = camera.get_ray(u, v);

                color = color + ray.color(&world);
            }
            color = color / sample_per_pixel as f64;
            write!(out, "{}\n", color.to_string()).unwrap();
        }
    }
}
