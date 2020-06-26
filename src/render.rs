use indicatif::{ProgressBar, ProgressStyle};

use crate::camera::Camera;
use crate::geom::Hittable;
use crate::mat::Scatter;
use crate::ray::Ray;
use crate::scene::{Scene, Sky};
use crate::vec3::Color;

pub fn render<S: Sky>(scene: &Scene<S>, camera: &Camera, w: usize, h: usize) -> Vec<Color> {
    let mut pixels = Vec::with_capacity(w * h);

    console::set_colors_enabled(true);
    let progress = ProgressBar::new(h as u64).with_style(
        ProgressStyle::default_bar()
            .template("Rendering: [{eta_precise}] {bar:40.cyan/blue} {pos:>7}/{len:} scanlines"),
    );

    for j in (0..h).rev() {
        for i in 0..w {
            let mut pixel_color = Color::zeros();
            for _ in 0..scene.samples_per_pixel {
                let u = (i as f64 + rand::random::<f64>()) / (w - 1) as f64;
                let v = (j as f64 + rand::random::<f64>()) / (h - 1) as f64;
                let ray = camera.ray_at(u, v);
                pixel_color += compute_ray_color(scene, ray, scene.max_bounce_depth);
            }
            pixels.push(pixel_color);
        }
        progress.inc(1);
    }

    progress.finish();
    pixels
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
