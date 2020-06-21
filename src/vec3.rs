use std::fmt::{self, Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

use rand::Rng;

pub type Color = Vec3;
pub type Point3 = Vec3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub const fn ones() -> Self {
        Vec3::new(1.0, 1.0, 1.0)
    }

    pub const fn zeros() -> Self {
        Vec3::new(0.0, 0.0, 0.0)
    }

    pub fn random() -> Self {
        Vec3::new(rand::random(), rand::random(), rand::random())
    }

    pub fn random_in_range(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        Vec3::new(
            rng.gen_range(min, max),
            rng.gen_range(min, max),
            rng.gen_range(min, max),
        )
    }

    pub fn random_unit() -> Self {
        let mut rng = rand::thread_rng();
        let a: f64 = rng.gen_range(0.0, 2.0 * std::f64::consts::PI);
        let z: f64 = rng.gen_range(-1.0, 1.0);
        let r = (1.0 - z.powi(2)).sqrt();
        Vec3::new(r * a.cos(), r * a.sin(), z)
    }

    pub fn random_in_hemisphere(normal: Self) -> Self {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Vec3::random_in_range(-1.0, 1.0);
            if p.len_squared() < 1.0 {
                return p;
            }
        }
    }

    #[inline]
    pub fn len(self) -> f64 {
        self.len_squared().sqrt()
    }

    #[inline]
    pub fn len_squared(self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    #[inline]
    pub fn dot(self, rhs: Self) -> f64 {
        let Vec3 { x, y, z } = rhs;
        (self.x * x) + (self.y * y) + (self.z * z)
    }

    #[inline]
    pub fn cross(self, rhs: Self) -> Self {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    #[inline]
    pub fn to_unit(self) -> Self {
        self / self.len()
    }

    #[inline]
    pub fn reflect(self, surface_normal: Self) -> Self {
        self - 2.0 * self.dot(surface_normal) * surface_normal
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        (1f64 / rhs) * self
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1f64 / rhs;
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            i => panic!("index {} out of bounds", i),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            i => panic!("index {} out of bounds", i),
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cross_product_simple() {
        let lhs = Vec3::new(1.0, 0.0, 0.0);
        let rhs = Vec3::new(0.0, 1.0, 0.0);
        let Vec3 { x, y, z } = lhs.cross(rhs);

        assert_eq!(x, 0.0);
        assert_eq!(y, 0.0);
        assert_eq!(z, 1.0);
    }

    #[test]
    fn cross_product_advanced() {
        let lhs = Vec3::new(5.0, 7.0, 1.0);
        let rhs = Vec3::new(4.0, 4.0, 3.0);
        let Vec3 { x, y, z } = lhs.cross(rhs);

        assert_eq!(x, 17.0);
        assert_eq!(y, -11.0);
        assert_eq!(z, -8.0);
    }

    #[test]
    fn unit() {
        use float_eq::assert_float_eq;

        let foo = Vec3::new(1.0, 5.0, -1.0);
        let Vec3 { x, y, z } = foo.to_unit();

        assert_float_eq!(x, 1.0 / (3.0 * 3.0f64.sqrt()), rel <= f64::EPSILON);
        assert_float_eq!(y, 5.0 / (3.0 * 3.0f64.sqrt()), rel <= f64::EPSILON);
        assert_float_eq!(z, -1.0 / (3.0 * 3.0f64.sqrt()), rel <= f64::EPSILON);
    }

    #[test]
    fn indexing() {
        let mut foo = Vec3::new(0.0, 0.0, 0.0);

        foo[0] = 1.0;
        foo[1] = 2.0;
        foo[2] = 3.0;

        assert_eq!(foo[0], 1.0);
        assert_eq!(foo[1], 2.0);
        assert_eq!(foo[2], 3.0);

        assert_eq!(foo[0], foo.x);
        assert_eq!(foo[1], foo.y);
        assert_eq!(foo[2], foo.z);
    }
}
