use std::io::{stdout, Write, BufWriter};

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
            let r = (w as f32) / image_width as f32;
            let g = (h as f32) / image_height as f32;
            let b = 0.25;

            let ir = (r * 255.999) as i32;
            let ig = (g * 255.999) as i32;
            let ib = (b * 255.999) as i32;

            write!(out, "{:?} {:?} {:?}\n", ir, ig, ib).unwrap();
        }
    }
}
