use std::fmt::Debug;

use crate::geom::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

pub trait Material: Debug {
    fn scatter(&self, incoming: Ray, hit: &HitRecord) -> Option<Scatter>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Color,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub const fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Default for Lambertian {
    fn default() -> Self {
        Lambertian::new(Color::new(0.5, 0.5, 0.5))
    }
}

impl Material for Lambertian {
    fn scatter(&self, _incoming: Ray, hit: &HitRecord) -> Option<Scatter> {
        let scatter_direction = hit.normal + Vec3::random_unit();
        Some(Scatter {
            ray: Ray::new(hit.point, scatter_direction),
            attenuation: self.albedo,
        })
    }
}

/// Implements the simpler hemispherical scattering method.
#[derive(Clone, Debug, PartialEq)]
pub struct SimpleDiffuse {
    pub albedo: Color,
}

impl SimpleDiffuse {
    pub const fn new(albedo: Color) -> Self {
        SimpleDiffuse { albedo }
    }
}

impl Default for SimpleDiffuse {
    fn default() -> Self {
        SimpleDiffuse::new(Color::new(0.5, 0.5, 0.5))
    }
}

impl Material for SimpleDiffuse {
    fn scatter(&self, _incoming: Ray, hit: &HitRecord) -> Option<Scatter> {
        let scatter_direction = Vec3::random_in_hemisphere(hit.normal);
        Some(Scatter {
            ray: Ray::new(hit.point, scatter_direction),
            attenuation: self.albedo,
        })
    }
}
