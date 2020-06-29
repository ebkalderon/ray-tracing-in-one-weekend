pub use self::texture::{CheckeredTexture, Texture};

use std::fmt::Debug;

use crate::geom::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

mod perlin;
mod texture;

pub trait Material: Debug + Send + Sync {
    fn scatter(&self, incoming: &Ray, hit: &HitRecord) -> Option<Scatter>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Color,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Lambertian<T: Texture> {
    pub albedo: T,
}

impl<T: Texture> Lambertian<T> {
    pub fn new(albedo: T) -> Self {
        Lambertian { albedo }
    }
}

impl Default for Lambertian<Color> {
    fn default() -> Self {
        Lambertian::new(Color::new(0.5, 0.5, 0.5))
    }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, incoming: &Ray, hit: &HitRecord) -> Option<Scatter> {
        let scatter_direction = hit.normal + Vec3::random_unit();
        Some(Scatter {
            ray: Ray::with_time(hit.point, scatter_direction, incoming.time),
            attenuation: self.albedo.value(hit.texture_u, hit.texture_v, hit.point),
        })
    }
}

/// Implements the simpler hemispherical scattering method.
#[derive(Clone, Debug, PartialEq)]
pub struct SimpleDiffuse<T: Texture> {
    pub albedo: T,
}

impl<T: Texture> SimpleDiffuse<T> {
    pub fn new(albedo: T) -> Self {
        SimpleDiffuse { albedo }
    }
}

impl Default for SimpleDiffuse<Color> {
    fn default() -> Self {
        SimpleDiffuse::new(Color::new(0.5, 0.5, 0.5))
    }
}

impl<T: Texture> Material for SimpleDiffuse<T> {
    fn scatter(&self, incoming: &Ray, hit: &HitRecord) -> Option<Scatter> {
        let scatter_direction = Vec3::random_in_hemisphere(hit.normal);
        Some(Scatter {
            ray: Ray::with_time(hit.point, scatter_direction, incoming.time),
            attenuation: self.albedo.value(hit.texture_u, hit.texture_v, hit.point),
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
    fn scatter(&self, incoming: &Ray, hit: &HitRecord) -> Option<Scatter> {
        let reflected = incoming.direction.to_unit().reflect(hit.normal);
        let scattered = Ray::with_time(
            hit.point,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
            incoming.time,
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
    fn scatter(&self, incoming: &Ray, hit: &HitRecord) -> Option<Scatter> {
        let etai_over_etat = if hit.is_front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let scattered = {
            let unit_direction = incoming.direction.to_unit();
            let cos_theta = (-unit_direction).dot(hit.normal).min(1.0);
            let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

            if etai_over_etat * sin_theta > 1.0 {
                let reflected = unit_direction.reflect(hit.normal);
                Ray::with_time(hit.point, reflected, incoming.time)
            } else if rand::random::<f64>() < schlick(cos_theta, etai_over_etat) {
                let reflected = unit_direction.reflect(hit.normal);
                Ray::with_time(hit.point, reflected, incoming.time)
            } else {
                let refracted = unit_direction.refract(hit.normal, etai_over_etat);
                Ray::with_time(hit.point, refracted, incoming.time)
            }
        };

        Some(Scatter {
            ray: scattered,
            attenuation: Color::ones(),
        })
    }
}

fn schlick(cosine: f64, refraction_index: f64) -> f64 {
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0_squared = r0.powi(2);
    r0_squared + (1.0 - r0_squared) * (1.0 - cosine).powi(5)
}
