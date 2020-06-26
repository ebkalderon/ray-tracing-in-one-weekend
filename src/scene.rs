use crate::geom::Hittable;
use crate::ray::Ray;
use crate::vec3::Color;

const MAX_BOUNCE_DEPTH: u32 = 50;
const SAMPLES_PER_PIXEL: u32 = 100;

pub trait Sky: Send + Sync {
    fn color(&self, incoming: Ray) -> Color;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GradientSky {
    pub color: Color,
}

impl Default for GradientSky {
    fn default() -> Self {
        GradientSky {
            color: Color::new(0.5, 0.7, 1.0),
        }
    }
}

impl Sky for GradientSky {
    fn color(&self, incoming: Ray) -> Color {
        let unit_direction = incoming.direction.to_unit();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Color::ones() + t * self.color
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SolidSky {
    pub color: Color,
}

impl Default for SolidSky {
    fn default() -> Self {
        SolidSky {
            color: Color::zeros(),
        }
    }
}

impl Sky for SolidSky {
    fn color(&self, _: Ray) -> Color {
        self.color
    }
}

#[derive(Debug)]
pub struct Scene<S: Sky> {
    pub world: Vec<Box<dyn Hittable>>,
    pub sky: S,
    pub max_bounce_depth: u32,
    pub samples_per_pixel: u32,
}

impl<S: Sky> Scene<S> {
    pub fn new(world: Vec<Box<dyn Hittable>>, sky: S) -> Self {
        Scene {
            world,
            sky,
            max_bounce_depth: MAX_BOUNCE_DEPTH,
            samples_per_pixel: SAMPLES_PER_PIXEL,
        }
    }

    #[allow(unused)]
    pub fn with_max_bounces(mut self, val: u32) -> Self {
        self.max_bounce_depth = val;
        self
    }

    #[allow(unused)]
    pub fn with_samples_per_pixel(mut self, val: u32) -> Self {
        self.samples_per_pixel = val;
        self
    }
}

impl Default for Scene<GradientSky> {
    fn default() -> Self {
        Scene::new(Vec::new(), GradientSky::default())
    }
}
