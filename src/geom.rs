pub use self::sphere::Sphere;

use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

mod sphere;

pub trait Hittable {
    fn hit(&self, ray: Ray, t_range: (f64, f64)) -> Option<HitRecord>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub is_front_face: bool,
}

impl HitRecord {
    pub const fn new(point: Point3, normal: Vec3, t: f64, is_front_face: bool) -> Self {
        HitRecord {
            point,
            normal,
            t,
            is_front_face,
        }
    }

    pub fn with_face_normal(ray: Ray, point: Point3, outward_normal: Vec3, t: f64) -> Self {
        let is_front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if is_front_face {
            outward_normal
        } else {
            -outward_normal
        };

        HitRecord::new(point, normal, t, is_front_face)
    }
}
