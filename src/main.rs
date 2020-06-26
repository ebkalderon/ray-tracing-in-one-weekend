use rand::{thread_rng, Rng};

use camera::Camera;
use geom::{Hittable, Sphere};
use mat::{Dielectric, Lambertian, Metallic, Scatter};
use ray::Ray;
use vec3::{Color, Point3, Vec3};

mod camera;
mod geom;
mod mat;
mod ray;
mod render;
mod scene;
mod vec3;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 384;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: u32 = 100;
const MAX_BOUNCE_DEPTH: u32 = 50;

fn random_scene() -> Vec<Box<dyn Hittable>> {
    let mut world: Vec<Box<dyn Hittable>> = Vec::with_capacity(11 * 11);

    world.push(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Box::new(Lambertian::new(Color::new(0.5, 0.5, 0.5))),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_material: f64 = rand::random();
            let center = Point3::new(
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::random::<f64>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                if choose_material < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let material = Lambertian::new(albedo);
                    world.push(Box::new(Sphere::new(center, 0.2, Box::new(material))));
                } else if choose_material < 0.95 {
                    let albedo = Color::random() * Color::random();
                    let fuzz = rand::random();
                    let material = Metallic::new(albedo, fuzz);
                    world.push(Box::new(Sphere::new(center, 0.2, Box::new(material))));
                } else {
                    let material = Dielectric::new(1.5);
                    world.push(Box::new(Sphere::new(center, 0.2, Box::new(material))));
                }
            }
        }
    }

    let large_spheres: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(
            Point3::new(0.0, 1.0, 0.0),
            1.0,
            Box::new(Dielectric::new(1.5)),
        )),
        Box::new(Sphere::new(
            Point3::new(-4.0, 1.0, 0.0),
            1.0,
            Box::new(Lambertian::new(Color::new(0.4, 0.2, 0.1))),
        )),
        Box::new(Sphere::new(
            Point3::new(4.0, 1.0, 0.0),
            1.0,
            Box::new(Metallic::new(Color::new(0.7, 0.6, 0.5), 0.0)),
        )),
    ];

    world.extend(large_spheres);
    world
}

fn main() {
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let world = random_scene();

    let camera = {
        let up_vec = Vec3::new(0.0, 1.0, 0.0);
        let look_from = Point3::new(13.0, 2.0, 3.0);
        let look_at = Point3::zeros();
        let aperture = 0.1;
        let focus_dist = 10.0;
        Camera::new(
            up_vec,
            look_from,
            look_at,
            20.0,
            ASPECT_RATIO,
            aperture,
            focus_dist,
        )
    };

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {:0>3}", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::zeros();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + thread_rng().gen_range(0.0, 1.0)) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + thread_rng().gen_range(0.0, 1.0)) / (IMAGE_HEIGHT - 1) as f64;
                let ray = camera.ray_at(u, v);
                pixel_color += compute_ray_color(ray, &world[..], MAX_BOUNCE_DEPTH);
            }
            print_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }

    eprintln!("\nDone.");
}

fn compute_ray_color(ray: Ray, world: &[Box<dyn Hittable>], depth: u32) -> Color {
    if depth <= 0 {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        return Color::zeros();
    }

    if let Some(hit_record) = world.hit(ray, (0.001, std::f64::MAX)) {
        if let Some(scatter) = hit_record.material.scatter(ray, &hit_record) {
            let Scatter { ray, attenuation } = scatter;
            return attenuation * compute_ray_color(ray, world, depth - 1);
        } else {
            return Color::zeros();
        }
    }

    let unit_direction = ray.direction.to_unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::ones() + t * Color::new(0.5, 0.7, 1.0)
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
