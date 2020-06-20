use ray::Ray;
use vec3::{Color, Point3, Vec3};

mod ray;
mod vec3;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 384;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    println!("P3\n{} {}\n255", image_width, image_height);

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::zeros();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, focal_length);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {:0>3}", j);
        for i in 0..image_width {
            let screen_x = i as f64 / (image_width - 1) as f64;
            let screen_y = j as f64 / (image_height - 1) as f64;
            let ray = Ray::new(
                origin,
                lower_left_corner + (screen_x * horizontal) + (screen_y * vertical) - origin,
            );
            let pixel_color = compute_ray_color(ray);
            print_color(pixel_color);
        }
    }

    eprintln!("\nDone.");
}

fn compute_ray_color(ray: Ray) -> Color {
    let unit_direction = ray.direction.to_unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::ones() + t * Color::new(0.5, 0.7, 1.0)
}

fn print_color(pixel: Color) {
    let ir = (255.999 * pixel.x) as i64;
    let ig = (255.999 * pixel.y) as i64;
    let ib = (255.999 * pixel.z) as i64;
    println!("{} {} {}", ir, ig, ib);
}
