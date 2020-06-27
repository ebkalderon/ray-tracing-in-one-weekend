pub use self::sphere::{MovingSphere, Sphere};

use std::fmt::Debug;

use crate::mat::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

mod sphere;

pub trait Hittable: Debug + Send + Sync {
    fn hit(&self, ray: &Ray, t_range: (f64, f64)) -> Option<HitRecord>;
}

impl<T: AsRef<[Box<dyn Hittable>]> + Debug + Send + Sync> Hittable for T {
    fn hit(&self, ray: &Ray, (t_min, t_max): (f64, f64)) -> Option<HitRecord> {
        let mut closest_so_far: Option<HitRecord> = None;
        let mut t_max = t_max;

        for object in self.as_ref().iter() {
            if let Some(record) = object.hit(ray, (t_min, t_max)) {
                t_max = record.t;
                closest_so_far = Some(record);
            }
        }

        closest_so_far
    }
}

#[derive(Clone, Debug)]
pub struct HitRecord<'a> {
    pub point: Point3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
    pub t: f64,
    pub is_front_face: bool,
}

impl<'a> HitRecord<'a> {
    #[inline]
    pub fn new(
        point: Point3,
        normal: Vec3,
        material: &'a dyn Material,
        t: f64,
        is_front_face: bool,
    ) -> Self {
        HitRecord {
            point,
            normal,
            material,
            t,
            is_front_face,
        }
    }

    #[inline]
    pub fn with_face_normal(
        ray: Ray,
        point: Point3,
        outward_normal: Vec3,
        material: &'a dyn Material,
        t: f64,
    ) -> Self {
        let is_front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if is_front_face {
            outward_normal
        } else {
            -outward_normal
        };

        HitRecord::new(point, normal, material, t, is_front_face)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mat::Lambertian;

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 384;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f64 = 1.0;

    fn generate_world() -> Vec<Box<dyn Hittable>> {
        vec![
            Box::new(Sphere::new(
                Point3::new(0.0, 0.0, -1.0),
                0.5,
                Lambertian::default(),
            )),
            Box::new(Sphere::new(
                Point3::new(0.0, -100.5, -1.0),
                100.0,
                Lambertian::default(),
            )),
        ]
    }

    fn compute_ray(pixel_x: u32, pixel_y: u32) -> Ray {
        let origin = Point3::zeros();
        let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
        let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
        let lower_left_corner =
            origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

        let screen_x = pixel_x as f64 / (IMAGE_WIDTH - 1) as f64;
        let screen_y = pixel_y as f64 / (IMAGE_HEIGHT - 1) as f64;

        Ray::new(
            origin,
            lower_left_corner + (screen_x * horizontal) + (screen_y * vertical) - origin,
        )
    }

    #[test]
    fn selects_closest_hit() {
        let world = generate_world();
        let center_of_image = compute_ray(IMAGE_WIDTH / 2, IMAGE_HEIGHT / 2);

        let hit_record = world
            .hit(&center_of_image, (0.0, f64::MAX))
            .expect("ray didn't hit any objects");

        assert!(hit_record.is_front_face);
        assert!((0.00232..0.00233).contains(&hit_record.point.x));
        assert!((0.00232..0.00233).contains(&hit_record.point.y));
        assert!((-0.50002..-0.50001).contains(&hit_record.point.z));
    }

    #[test]
    fn returns_none_if_hits_nothing() {
        let world = generate_world();
        let random_ray = Ray::new(Point3::new(0.0, 200.0, 0.0), Vec3::new(0.0, 200.0, 0.0));

        let hit_record = world.hit(&random_ray, (0.0, f64::MAX));
        assert!(hit_record.is_none());
    }
}
