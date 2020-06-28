use super::{HitRecord, Hittable};
use crate::aabb::Aabb;
use crate::ray::Ray;

#[derive(Debug)]
enum Node {
    Branch { left: Box<Bvh>, right: Box<Bvh> },
    Leaf(Box<dyn Hittable>),
}

#[derive(Debug)]
pub struct Bvh {
    tree: Node,
    bounding_box: Aabb,
}

impl Bvh {
    pub fn new(world: Vec<Box<dyn Hittable>>, time0: f64, time1: f64) -> Self {
        unimplemented!()
    }
}

impl Hittable for Bvh {
    fn hit(&self, ray: &Ray, t_range: (f64, f64)) -> Option<HitRecord> {
        if self.bounding_box.hit(ray, t_range) {
            match &self.tree {
                Node::Leaf(object) => object.hit(ray, t_range),
                Node::Branch { left, right } => {
                    let hit_left = left.hit(ray, t_range);
                    let hit_right = {
                        let (t_min, mut t_max) = t_range;
                        t_max = hit_left.as_ref().map(|hit| hit.t).unwrap_or(t_max);
                        right.hit(ray, (t_min, t_max))
                    };

                    hit_right.or(hit_left)
                }
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        Some(self.bounding_box)
    }
}
