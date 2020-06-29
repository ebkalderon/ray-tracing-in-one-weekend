use std::fmt::Debug;

use super::perlin::Perlin;
use crate::vec3::{Color, Point3};

pub trait Texture: Debug + Send + Sync {
    fn value(&self, u: f64, v: f64, point: Point3) -> Color;
}

impl Texture for Color {
    fn value(&self, _: f64, _: f64, _: Point3) -> Color {
        *self
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CheckeredTexture<T: Texture, U: Texture> {
    pub odd: T,
    pub even: U,
}

impl<T: Texture, U: Texture> CheckeredTexture<T, U> {
    pub fn new(odd: T, even: U) -> Self {
        CheckeredTexture { odd, even }
    }
}

impl<T: Texture, U: Texture> Texture for CheckeredTexture<T, U> {
    fn value(&self, u: f64, v: f64, point: Point3) -> Color {
        let sines = (10.0 * point.x).sin() * (10.0 * point.y).sin() * (10.0 * point.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, point)
        } else {
            self.even.value(u, v, point)
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NoiseTexture(Perlin);

impl NoiseTexture {
    pub fn new() -> Self {
        NoiseTexture(Perlin::new())
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _: f64, _: f64, point: Point3) -> Color {
        Color::ones() * self.0.noise_at(point)
    }
}
