use vec3::Color;

mod ray;
mod vec3;

fn main() {
    let (image_width, image_height) = (256, 256);

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {:0>3}", j);
        for i in 0..image_width {
            print_color(Color {
                x: i as f64 / (image_width - 1) as f64,
                y: j as f64 / (image_height - 1) as f64,
                z: 0.25f64,
            });
        }
    }

    eprintln!("\nDone.");
}

fn print_color(pixel: Color) {
    let ir = (255.999 * pixel.x) as i64;
    let ig = (255.999 * pixel.y) as i64;
    let ib = (255.999 * pixel.z) as i64;
    println!("{} {} {}", ir, ig, ib);
}
