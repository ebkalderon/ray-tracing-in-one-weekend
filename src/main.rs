use rand::Rng;

use camera::Camera;
use geom::{Hittable, MovingSphere, Sphere};
use mat::{Dielectric, Lambertian, Metallic};
use scene::Scene;
use vec3::{Color, Point3, Vec3};

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

fn random_scene() -> Vec<Box<dyn Hittable>> {
    let mut world: Vec<Box<dyn Hittable>> = Vec::with_capacity(22 * 22 + 4);

    world.push(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(Color::new(0.5, 0.5, 0.5)),
    )));

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_material: f64 = rng.gen();
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                if choose_material < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let material = Lambertian::new(albedo);
                    let center2 = center + Vec3::new(0.0, rng.gen_range(0.0, 0.5), 0.0);
                    world.push(Box::new(MovingSphere::new(
                        (center, center2),
                        (0.0, 1.0),
                        0.2,
                        material,
                    )));
                } else if choose_material < 0.95 {
                    let albedo = Color::random() * Color::random();
                    let fuzz = rng.gen();
                    let material = Metallic::new(albedo, fuzz);
                    world.push(Box::new(Sphere::new(center, 0.2, material)));
                } else {
                    let material = Dielectric::new(1.5);
                    world.push(Box::new(Sphere::new(center, 0.2, material)));
                }
            }
        }
    }

    let large_spheres: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(
            Point3::new(0.0, 1.0, 0.0),
            1.0,
            Dielectric::new(1.5),
        )),
        Box::new(Sphere::new(
            Point3::new(-4.0, 1.0, 0.0),
            1.0,
            Lambertian::new(Color::new(0.4, 0.2, 0.1)),
        )),
        Box::new(Sphere::new(
            Point3::new(4.0, 1.0, 0.0),
            1.0,
            Metallic::new(Color::new(0.7, 0.6, 0.5), 0.0),
        )),
    ];

    world.extend(large_spheres);
    world
}

fn main() {
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let scene = Scene {
        world: random_scene(),
        ..Default::default()
    };

    let camera = {
        let up_vec = Vec3::new(0.0, 1.0, 0.0);
        let look_from = Point3::new(13.0, 2.0, 3.0);
        let look_at = Point3::zeros();
        let aperture = 0.1;
        let focus_dist = 10.0;
        let shutter_open = 0.0;
        let shutter_closed = 1.0;
        Camera::new(
            up_vec,
            look_from,
            look_at,
            20.0,
            ASPECT_RATIO,
            aperture,
            focus_dist,
            shutter_open,
            shutter_closed,
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
