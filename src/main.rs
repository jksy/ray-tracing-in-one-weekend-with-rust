mod raytrace;

use raytrace::*;
use std::io::{stdout, Write, BufWriter};

fn main() {

    let out = stdout();
    let mut out = BufWriter::new(out.lock());

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    // World
    let mut world = HitableList::new();
    world.add(&Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(&Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    // Camera
    let camera = Camera::new();

    // Render
    write!(out, "P3\n{:?} {:?}\n255\n", image_width, image_height).unwrap();

    for h in (0..(image_height as i32)).rev() {
        for w in 0..(image_width as i32) {
            let origin = Point3::origin();
            let u = (w as f64) / image_width as f64;
            let v = (h as f64) / image_height as f64;
            let ray = camera.get_ray(u, v);

            let color = ray.color(&world);
            write!(out, "{}\n", color.to_string()).unwrap();
        }
    }
}
