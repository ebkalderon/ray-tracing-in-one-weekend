use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Clone, Debug, PartialEq)]
pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f64,
}

impl Camera {
    pub fn new(
        up_vec: Vec3,
        look_from: Point3,
        look_at: Point3,
        vertical_fov_deg: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = vertical_fov_deg.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).to_unit();
        let u = up_vec.cross(w).to_unit();
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;

        Camera {
            origin,
            horizontal,
            vertical,
            u,
            v,
            w,
            lower_left_corner: origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn ray_at(&self, screen_x: f64, screen_y: f64) -> Ray {
        let (s, t) = (screen_x, screen_y);
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        let look_from = Point3::new(3.0, 3.0, 2.0);
        let look_at = Point3::new(0.0, 0.0, -1.0);
        Camera::new(
            Vec3::new(0.0, 1.0, 0.0),
            look_from,
            look_at,
            20.0,
            16.0 / 9.0,
            2.0,
            (look_from - look_at).len(),
        )
    }
}
