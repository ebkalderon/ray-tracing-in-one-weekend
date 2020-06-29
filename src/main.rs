use std::time::Duration;

use camera::Camera;
use geom::{Hittable, Sphere};
use mat::{Lambertian, NoiseTexture};
use scene::Scene;
use vec3::{Color, Point3, Vec3};

mod aabb;
mod camera;
mod geom;
mod mat;
mod ray;
mod render;
mod scene;
mod vec3;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 384;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

fn two_perlin_spheres() -> Vec<Box<dyn Hittable>> {
    let noise = NoiseTexture::new();
    vec![
        Box::new(Sphere::new(
            Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            Lambertian::new(noise.clone()),
        )),
        Box::new(Sphere::new(
            Point3::new(0.0, 2.0, 0.0),
            2.0,
            Lambertian::new(noise),
        )),
    ]
}

fn main() {
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let scene = Scene {
        world: two_perlin_spheres(),
        ..Default::default()
    };

    let camera = {
        let up_vec = Vec3::new(0.0, 1.0, 0.0);
        let look_from = Point3::new(13.0, 2.0, 3.0);
        let look_at = Point3::zeros();
        let aperture = 0.0;
        let focus_dist = 10.0;
        let shutter_duration = Duration::from_secs(1);
        Camera::new(
            up_vec,
            look_from,
            look_at,
            20.0,
            ASPECT_RATIO,
            aperture,
            focus_dist,
            shutter_duration,
        )
    };

    for pixel in render::render(&scene, &camera, IMAGE_WIDTH, IMAGE_HEIGHT) {
        print_color(pixel, scene.samples_per_pixel);
    }
}

fn print_color(pixel: Color, samples_per_pixel: u32) {
    let scale = 1.0 / samples_per_pixel as f64;
    let (r, g, b) = {
        // Divide the color total by the number of samples and gamma-correct for gamma=2.0.
        let r = (pixel.x * scale).sqrt();
        let g = (pixel.y * scale).sqrt();
        let b = (pixel.z * scale).sqrt();
        (r, g, b)
    };

    let ir = (256.0 * clamp(r, 0.0, 0.999)) as i64;
    let ig = (256.0 * clamp(g, 0.0, 0.999)) as i64;
    let ib = (256.0 * clamp(b, 0.0, 0.999)) as i64;
    println!("{} {} {}", ir, ig, ib);
}

fn clamp(mut x: f64, min: f64, max: f64) -> f64 {
    debug_assert!(min <= max);
    if x < min {
        x = min;
    }
    if x > max {
        x = max;
    }
    x
}
