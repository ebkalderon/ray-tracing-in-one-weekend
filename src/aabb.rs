use crate::ray::Ray;
use crate::vec3::Point3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Aabb {
    pub min: Point3,
    pub max: Point3,
}

impl Aabb {
    pub const fn new(min: Point3, max: Point3) -> Self {
        Aabb { min, max }
    }

    pub fn hit(&self, ray: &Ray, (mut t_min, mut t_max): (f64, f64)) -> bool {
        for a in 0..3 {
            let time0 = ((self.min[a] - ray.origin[a]) / ray.direction[a])
                .min((self.max[a] - ray.origin[a]) / ray.direction[a]);
            let time1 = ((self.min[a] - ray.origin[a]) / ray.direction[a])
                .max((self.max[a] - ray.origin[a]) / ray.direction[a]);

            t_min = time0.max(t_min);
            t_max = time1.min(t_max);

            if t_max <= t_min {
                return false;
            }
        }

        true
    }
}
