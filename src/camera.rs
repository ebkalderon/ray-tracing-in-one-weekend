use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Clone, Debug, PartialEq)]
pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn with_aspect(aspect_ratio: f64, viewport_height: f64, focal_length: f64) -> Self {
        let viewport_width = aspect_ratio * viewport_height;

        let origin = Point3::zeros();
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin
                - horizontal / 2.0
                - vertical / 2.0
                - Vec3::new(0.0, 0.0, focal_length),
        }
    }

    pub fn ray_at(&self, screen_x: f64, screen_y: f64) -> Ray {
        let (u, v) = (screen_x, screen_y);
        let dir = self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin;
        Ray::new(self.origin, dir)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera::with_aspect(16.0 / 9.0, 2.0, 1.0)
    }
}
