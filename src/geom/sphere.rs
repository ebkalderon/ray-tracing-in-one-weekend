use super::{HitRecord, Hittable};
use crate::aabb::{self, Aabb};
use crate::mat::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

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
    #[inline(always)]
    fn hit(&self, ray: &Ray, (t_min, t_max): (f64, f64)) -> Option<HitRecord> {
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
                    *ray,
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
                    *ray,
                    point,
                    outward_normal,
                    &self.material,
                    t,
                ));
            }
        }

        None
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        Some(Aabb {
            min: self.center - Vec3::new(self.radius, self.radius, self.radius),
            max: self.center + Vec3::new(self.radius, self.radius, self.radius),
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MovingSphere<M: Material> {
    pub center: (Point3, Point3),
    pub time: (f64, f64),
    pub radius: f64,
    pub material: M,
}

impl<M: Material> MovingSphere<M> {
    pub fn new(center: (Point3, Point3), time: (f64, f64), radius: f64, material: M) -> Self {
        MovingSphere {
            center,
            time,
            radius,
            material,
        }
    }

    pub fn center_at(&self, time_value: f64) -> Point3 {
        let time_scale = (time_value - self.time.0) / (self.time.1 - self.time.0);
        self.center.0 + time_scale * (self.center.1 - self.center.0)
    }
}

impl<M: Material> Hittable for MovingSphere<M> {
    #[inline(always)]
    fn hit(&self, ray: &Ray, (t_min, t_max): (f64, f64)) -> Option<HitRecord> {
        let origin_to_center = ray.origin - self.center_at(ray.time);
        let a = ray.direction.len_squared();
        let half_b = origin_to_center.dot(ray.direction);
        let c = origin_to_center.len_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();

            let t = (-half_b - root) / a;
            if t < t_max && t > t_min {
                let point = ray.point_at(t);
                let outward_normal = (point - self.center_at(ray.time)) / self.radius;
                return Some(HitRecord::with_face_normal(
                    *ray,
                    point,
                    outward_normal,
                    &self.material,
                    t,
                ));
            }

            let t = (-half_b + root) / a;
            if t < t_max && t > t_min {
                let point = ray.point_at(t);
                let outward_normal = (point - self.center_at(ray.time)) / self.radius;
                return Some(HitRecord::with_face_normal(
                    *ray,
                    point,
                    outward_normal,
                    &self.material,
                    t,
                ));
            }
        }

        None
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        let box0 = Aabb {
            min: self.center_at(time0) - Vec3::new(self.radius, self.radius, self.radius),
            max: self.center_at(time0) + Vec3::new(self.radius, self.radius, self.radius),
        };
        let box1 = Aabb {
            min: self.center_at(time1) - Vec3::new(self.radius, self.radius, self.radius),
            max: self.center_at(time1) + Vec3::new(self.radius, self.radius, self.radius),
        };

        Some(aabb::surrounding_box(box0, box1))
    }
}
