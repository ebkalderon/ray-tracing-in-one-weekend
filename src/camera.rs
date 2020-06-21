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
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        vertical_fov_deg: f64,
        aspect_ratio: f64,
    ) -> Self {
        let up_vec = Vec3::new(0.0, 1.0, 0.0);
        Camera::with_up_vec(up_vec, look_from, look_at, vertical_fov_deg, aspect_ratio)
    }

    pub fn with_up_vec(
        up_vec: Vec3,
        look_from: Point3,
        look_at: Point3,
        vertical_fov_deg: f64,
        aspect_ratio: f64,
    ) -> Self {
        let theta = vertical_fov_deg.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).to_unit();
        let u = up_vec.cross(w).to_unit();
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin - horizontal / 2.0 - vertical / 2.0 - w,
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
        Camera::new(
            Point3::new(-2.0, 2.0, 1.0),
            Point3::new(0.0, 0.0, -1.0),
            90.0,
            16.0 / 9.0,
        )
    }
}
