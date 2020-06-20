use ray::Ray;
use vec3::{Color, Point3, Vec3};

mod ray;
mod vec3;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 384;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f64 = 1.0;

fn main() {
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let origin = Point3::zeros();
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner =
        origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {:0>3}", j);
        for i in 0..IMAGE_WIDTH {
            let screen_x = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let screen_y = j as f64 / (IMAGE_HEIGHT - 1) as f64;
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
    let (center, radius) = (Point3::new(0.0, 0.0, -1.0), 0.5);

    if ray_hits_sphere(center, radius, ray) {
        return Color::new(1.0, 0.0, 0.0);
    }

    let unit_direction = ray.direction.to_unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::ones() + t * Color::new(0.5, 0.7, 1.0)
}

fn ray_hits_sphere(center: Point3, radius: f64, ray: Ray) -> bool {
    let origin_to_center = ray.origin - center;
    let a = ray.direction.dot(ray.direction);
    let b = 2.0 * origin_to_center.dot(ray.direction);
    let c = origin_to_center.dot(origin_to_center) - radius.powi(2);
    let discriminant = b.powi(2) - 4.0 * (a * c);
    discriminant > 0.0
}

fn print_color(pixel: Color) {
    let ir = (255.999 * pixel.x) as i64;
    let ig = (255.999 * pixel.y) as i64;
    let ib = (255.999 * pixel.z) as i64;
    println!("{} {} {}", ir, ig, ib);
}
