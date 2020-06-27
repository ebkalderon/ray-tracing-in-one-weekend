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

    #[inline]
    pub fn hit(&self, ray: &Ray, (mut t_min, mut t_max): (f64, f64)) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / ray.direction[a];
            let mut time0 = (self.min[a] - ray.origin[a]) * inv_d;
            let mut time1 = (self.max[a] - ray.origin[a]) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut time0, &mut time1);
            }

            t_min = if time0 > t_min { time0 } else { t_min };
            t_max = if time1 < t_max { time1 } else { t_max };

            if t_max <= t_min {
                return false;
            }
        }

        true
    }
}
