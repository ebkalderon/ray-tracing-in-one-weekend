use crate::vec3::{Point3, Vec3};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub const fn new(origin: Point3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn point_at(self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_at() {
        let ray = {
            let origin = Point3::new(0.0, 0.0, 0.0);
            let direction = Vec3::new(4.0, 2.0, 5.0);
            Ray::new(origin, direction)
        };

        assert_eq!(ray.point_at(0.0), Point3::zeros());
        assert_eq!(ray.point_at(2.0), Point3::new(8.0, 4.0, 10.0));
        assert_eq!(ray.point_at(4.0), Point3::new(16.0, 8.0, 20.0));
    }
}
