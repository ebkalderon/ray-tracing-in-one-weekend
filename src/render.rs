use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rand::Rng;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

use crate::camera::Camera;
use crate::geom::Hittable;
use crate::mat::Scatter;
use crate::ray::Ray;
use crate::scene::{Scene, Sky};
use crate::vec3::Color;

pub fn render<S: Sky>(scene: &Scene<S>, camera: &Camera, w: usize, h: usize) -> Vec<Color> {
    console::set_colors_enabled(true);

    let bar = ProgressBar::new(h as u64).with_style(
        ProgressStyle::default_bar()
            .template("Rendering: [{eta_precise}] {bar:40.cyan/blue} {pos:>7}/{len:} scanlines"),
    );

    (0..h)
        .into_par_iter()
        .rev()
        .progress_with(bar)
        .flat_map(|j| {
            (0..w).into_par_iter().map(move |i| {
                (0..scene.samples_per_pixel)
                    .into_par_iter()
                    .map_init(rand::thread_rng, move |rng, _| {
                        let u = (i as f64 + rng.gen::<f64>()) / (w - 1) as f64;
                        let v = (j as f64 + rng.gen::<f64>()) / (h - 1) as f64;
                        let ray = camera.ray_at(u, v);
                        compute_ray_color(scene, ray, scene.max_bounce_depth)
                    })
                    .sum()
            })
        })
        .collect()
}

fn compute_ray_color<S: Sky>(scene: &Scene<S>, ray: Ray, depth: u32) -> Color {
    if depth <= 0 {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        return Color::zeros();
    }

    if let Some(hit_record) = scene.world.hit(ray, (0.001, std::f64::MAX)) {
        if let Some(scatter) = hit_record.material.scatter(ray, &hit_record) {
            let Scatter { ray, attenuation } = scatter;
            return attenuation * compute_ray_color(scene, ray, depth - 1);
        } else {
            return Color::zeros();
        }
    }

    scene.sky.color(ray)
}
