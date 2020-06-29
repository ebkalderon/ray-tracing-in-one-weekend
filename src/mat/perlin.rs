use rand::Rng;

use crate::vec3::Point3;

const POINT_COUNT: usize = 256;

#[derive(Clone, Debug, PartialEq)]
pub struct Perlin {
    random_floats: Box<[f64]>,
    perm_x: Box<[usize]>,
    perm_y: Box<[usize]>,
    perm_z: Box<[usize]>,
}

impl Perlin {
    pub fn new() -> Self {
        Perlin::with_rng(rand::thread_rng())
    }

    pub fn with_rng<R: Rng>(mut rng: R) -> Self {
        let random_floats = (&mut rng)
            .sample_iter(rand::distributions::Standard)
            .take(POINT_COUNT)
            .collect();

        Perlin {
            random_floats,
            perm_x: gen_perlin_permutation(&mut rng),
            perm_y: gen_perlin_permutation(&mut rng),
            perm_z: gen_perlin_permutation(&mut rng),
        }
    }

    pub fn noise_at(&self, point: Point3) -> f64 {
        let u = point.x - point.x.floor();
        let v = point.y - point.y.floor();
        let w = point.z - point.z.floor();

        let i = (4.0 * point.x) as usize & 255;
        let j = (4.0 * point.y) as usize & 255;
        let k = (4.0 * point.z) as usize & 255;

        self.random_floats[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
    }
}

fn gen_perlin_permutation<R: Rng>(mut rng: R) -> Box<[usize]> {
    let mut perm: Box<[_]> = (0..POINT_COUNT).collect();

    for i in (0..POINT_COUNT).rev() {
        let target = rng.gen_range(0, i + 1);
        perm.swap(i, target);
    }

    perm
}
