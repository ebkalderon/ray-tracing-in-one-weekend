use std::fmt::Debug;

use crate::vec3::{Color, Point3};

pub trait Texture: Debug + Send + Sync {
    fn value(&self, u: f64, v: f64, point: Point3) -> Color;
}

impl Texture for Color {
    fn value(&self, _: f64, _: f64, _: Point3) -> Color {
        *self
    }
}
