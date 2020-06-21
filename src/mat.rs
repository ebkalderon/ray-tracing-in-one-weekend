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

#[derive(Clone, Debug, PartialEq)]
pub struct Metallic {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metallic {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metallic {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Default for Metallic {
    fn default() -> Self {
        Metallic::new(Color::new(0.8, 0.8, 0.8), 0.0)
    }
}

impl Material for Metallic {
    fn scatter(&self, incoming: Ray, hit: &HitRecord) -> Option<Scatter> {
        let reflected = incoming.direction.to_unit().reflect(hit.normal);
        let scattered = Ray::new(
            hit.point,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
        );
        if scattered.direction.dot(hit.normal) > 0.0 {
            Some(Scatter {
                ray: scattered,
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Dielectric {
    pub refraction_index: f64,
}

impl Dielectric {
    pub const fn new(refraction_index: f64) -> Self {
        Dielectric { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, incoming: Ray, hit: &HitRecord) -> Option<Scatter> {
        let etai_over_etat = if hit.is_front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = incoming.direction.to_unit();
        let refracted = unit_direction.refract(hit.normal, etai_over_etat);
        Some(Scatter {
            ray: Ray::new(hit.point, refracted),
            attenuation: Color::ones(),
        })
    }
}
