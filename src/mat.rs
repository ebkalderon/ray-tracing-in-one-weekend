use crate::geom::HitRecord;
use crate::ray::Ray;
use crate::vec3::Color;

pub trait Material {
    fn scatter(&self, incoming: Ray, hit: &HitRecord) -> Option<Scatter>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Color,
}
