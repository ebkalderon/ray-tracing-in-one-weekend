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
}

impl HitRecord {
    pub const fn new(point: Point3, normal: Vec3, t: f64) -> Self {
        HitRecord { point, normal, t }
    }
}
