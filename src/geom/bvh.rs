use std::cmp::Ordering;

use anyhow::format_err;
use rand::Rng;
use rayon::slice::ParallelSliceMut;

use super::{HitRecord, Hittable};
use crate::aabb::{self, Aabb};
use crate::ray::Ray;

const MAX_SEQUENTIAL: usize = 250;

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
    pub fn new(mut world: Vec<Box<dyn Hittable>>, time0: f64, time1: f64) -> anyhow::Result<Self> {
        #[inline]
        fn box_compare(axis: usize) -> impl Fn(&Box<dyn Hittable>, &Box<dyn Hittable>) -> Ordering {
            move |left, right| match (left.bounding_box(0.0, 0.0), right.bounding_box(0.0, 0.0)) {
                (Some(left), Some(right)) => left.min[axis].partial_cmp(&right.min[axis]).unwrap(),
                _ => panic!("No bounding box in Bvh::new() constructor"),
            }
        }

        let axis = rand::thread_rng().gen_range(0, 3);
        world.par_sort_unstable_by(box_compare(axis));

        match world.len() {
            0 => Err(format_err!("Scene cannot be empty")),
            1 => {
                let leaf = world.remove(0);
                let bounding_box = leaf
                    .bounding_box(time0, time1)
                    .ok_or_else(|| format_err!("Element is missing bounding box"))?;

                Ok(Bvh {
                    bounding_box,
                    tree: Node::Leaf(leaf),
                })
            }
            len => {
                let half = world.drain(len / 2..).collect();
                let (right, left) = if len < MAX_SEQUENTIAL {
                    let right = Bvh::new(half, time0, time1)?;
                    let left = Bvh::new(world, time0, time1)?;
                    (right, left)
                } else {
                    let (right, left) = rayon::join(
                        || Bvh::new(half, time0, time1),
                        || Bvh::new(world, time0, time1),
                    );
                    (right?, left?)
                };

                Ok(Bvh {
                    bounding_box: aabb::surrounding_box(left.bounding_box, right.bounding_box),
                    tree: Node::Branch {
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                })
            }
        }
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
