mod raytrace;

use raytrace::Vec3;
use std::io::{stdout, Write, BufWriter};

type Point3 = Vec3;
type Color = Vec3;

fn main() {

    // Image
    let image_width = 256;
    let image_height = 256;
    let out = stdout();
    let mut out = BufWriter::new(out.lock());

    // Render
    write!(out, "P3\n{:?} {:?}\n255\n", image_width, image_height).unwrap();

    for h in 0..image_height {
        for w in 0..image_width {
            let r = (w as f64) / image_width as f64;
            let g = (h as f64) / image_height as f64;
            let b = 0.25;

            let ir = r * 255.999;
            let ig = g * 255.999;
            let ib = b * 255.999;
            let color = Color::new(ir, ig, ib);
            write!(out, "{}\n", color.to_string());
        }
    }
}
