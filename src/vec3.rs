use rand::prelude::*;
use std::{fmt::Display, ops};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vec3(f64, f64, f64);

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn len_squared(&self) -> f64 {
        self[0].powi(2) + self[1].powi(2) + self[2].powi(2)
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self[0] < s && self[1] < s && self[2] < s
    }

    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self(
            self.1 * other.2 - self.2 * other.2,
            self.2 * other.0 - self.2 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn unit_vector(&self) -> Self {
        (*self).clone() / self.len()
    }

    pub fn random() -> Self {
        let mut rng = thread_rng();
        let x: f64 = rng.gen();
        let y: f64 = rng.gen();
        let z: f64 = rng.gen();
        Vec3::new(x, y, z)
    }

    pub fn random_from_range(min: f64, max: f64) -> Self {
        let mut rng = thread_rng();
        let x: f64 = rng.gen_range(min..max);
        let y: f64 = rng.gen_range(min..max);
        let z: f64 = rng.gen_range(min..max);
        Vec3::new(x, y, z)
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Vec3::random_from_range(-1., 1.);
            if p.len_squared() < 1. {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        Vec3::random_in_unit_sphere().unit_vector()
    }

    pub fn random_on_hemisphere(&self) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit_vector();
        match on_unit_sphere.dot(self) > 0. {
            true => on_unit_sphere,
            false => -on_unit_sphere,
        }
    }

    pub fn reflect(&self, n: Vec3) -> Self {
        *self - (n * self.dot(&n) * 2.)
    }

    pub fn refract(&self, n: Vec3, etai_over_etat: f64) -> Self {
        let cos_theta = (-(*self)).dot(&n).min(1.0);
        let r_out_perp = (n * cos_theta + *self) * etai_over_etat;
        let r_out_parallel = n * -((1. - r_out_perp.len_squared()).abs().sqrt());
        r_out_perp + r_out_parallel
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("Index out of bonds. Consider index in range [0;2]"),
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = (*self).clone() + rhs
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = (*self).clone() / rhs
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = (*self).clone() * rhs
    }
}
