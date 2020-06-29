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
        let mut u = point.x - point.x.floor();
        let mut v = point.y - point.y.floor();
        let mut w = point.z - point.z.floor();
        u = u.powi(2) * (3.0 - 2.0 * u);
        v = v.powi(2) * (3.0 - 2.0 * v);
        w = w.powi(2) * (3.0 - 2.0 * w);

        let i = point.x.floor();
        let j = point.y.floor();
        let k = point.z.floor();

        let mut c = [[[0.0; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.random_floats[self.perm_x[(i as usize + di) & 255]
                        ^ self.perm_y[(j as usize + dj) & 255]
                        ^ self.perm_z[(k as usize + dk) & 255]];
                }
            }
        }

        trilinear_interp(c, u, v, w)
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

#[inline]
fn trilinear_interp(interp_point: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let mut accum = 0.0;

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                accum += (i as f64 * u + (1.0 - i as f64) * (1.0 - u))
                    * (j as f64 * v + (1.0 - j as f64) * (1.0 - v))
                    * (k as f64 * w + (1.0 - k as f64) * (1.0 - w))
                    * interp_point[i][j][k];
            }
        }
    }

    accum
}
