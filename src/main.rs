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
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::origin();
    let horizontal = Vec3::new(viewport_width, 0f64, 0f64);
    let vertical = Vec3::new(0f64, viewport_height, 0f64);
    let lower_left_corner = origin - horizontal / 2f64 - vertical / 2f64 - Vec3::new(0f64, 0f64, focal_length);

    // Render
    write!(out, "P3\n{:?} {:?}\n255\n", image_width, image_height).unwrap();

    for h in (0..(image_height as i32)).rev() {
        for w in 0..(image_width as i32) {
            let origin = Point3::origin();
            let u = (w as f64) / image_width as f64;
            let v = (h as f64) / image_height as f64;
            let ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);

            let color = ray.color(&world);
            write!(out, "{}\n", color.to_string()).unwrap();
        }
    }
}
