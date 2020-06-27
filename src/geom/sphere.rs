use super::{HitRecord, Hittable};
use crate::mat::Material;
use crate::ray::Ray;
use crate::vec3::Point3;

#[derive(Clone, Debug, PartialEq)]
pub struct Sphere<M: Material> {
    pub center: Point3,
    pub radius: f64,
    pub material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Point3, radius: f64, material: M) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, ray: Ray, (t_min, t_max): (f64, f64)) -> Option<HitRecord> {
        let origin_to_center = ray.origin - self.center;
        let a = ray.direction.len_squared();
        let half_b = origin_to_center.dot(ray.direction);
        let c = origin_to_center.len_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();

            let t = (-half_b - root) / a;
            if t < t_max && t > t_min {
                let point = ray.point_at(t);
                let outward_normal = (point - self.center) / self.radius;
                return Some(HitRecord::with_face_normal(
                    ray,
                    point,
                    outward_normal,
                    &self.material,
                    t,
                ));
            }

            let t = (-half_b + root) / a;
            if t < t_max && t > t_min {
                let point = ray.point_at(t);
                let outward_normal = (point - self.center) / self.radius;
                return Some(HitRecord::with_face_normal(
                    ray,
                    point,
                    outward_normal,
                    &self.material,
                    t,
                ));
            }
        }

        None
    }
}
