mod raytrace;

use raytrace::*;
use rand::Rng;
use std::io::{stderr, stdout, Write, BufWriter};

fn main() {

    let out = stdout();
    let mut out = BufWriter::new(out.lock());

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    // let sample_per_pixel = 100;
    let sample_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut world = HitableList::new();

    let material_ground = Material::Lambertian(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Material::Lambertian(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_left = Material::Metal(Metal::new(Color::new(0.8, 0.8, 0.8)));
    let material_right = Material::Metal(Metal::new(Color::new(0.8, 0.6, 0.2)));

    world.add(&Sphere::new(Point3::new( 0.0, -100.5, -1.0), 100.0, material_ground));
    world.add(&Sphere::new(Point3::new( 0.0,    0.0, -1.0), 0.5, material_center));
    world.add(&Sphere::new(Point3::new(-1.0,    0.0, -1.0), 0.5, material_left));
    world.add(&Sphere::new(Point3::new( 1.0,    0.0, -1.0), 0.5, material_right));

    // Camera
    let camera = Camera::new();

    // Render
    write!(out, "P3\n{:?} {:?}\n255\n", image_width, image_height).unwrap();

    for h in (0..(image_height as i32)).rev() {
        writeln!(stderr(), "\rScanlines remaining: {} ", h);
        for w in 0..(image_width as i32) {
            let mut color = Color::black();
            for i in 0..sample_per_pixel {
                let u = (w as f64 + raytrace::random0()) / (image_width - 1) as f64;
                let v = (h as f64 + raytrace::random0()) / (image_height - 1) as f64;
                let ray = camera.get_ray(u, v);

                color = color + ray.color(&world, max_depth);
            }
            write!(out, "{}\n", color.to_string(sample_per_pixel)).unwrap();
        }
    }
}
