use std::ops::{Neg, SubAssign, AddAssign, MulAssign, DivAssign, Add, Sub, Mul, Div};
use std::fmt;
use rand::prelude::random;

#[derive(Debug, Copy, Clone)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vec3({}, {}, {})", self.0, self.1, self.2)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2);
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self(self.0 + other.0, self.1 + other.1, self.2 + other.2);
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self(self.0 * rhs, self.1 * rhs, self.2 * rhs);
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = f64;

    fn mul(self, rhs: Self) -> f64 {
        self.0*rhs.0 + self.1*rhs.1 + self.2*rhs.2
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = Self(self.0 / rhs, self.1 / rhs, self.2 / rhs);
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}


fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

impl Vec3 {
    pub fn length_squared(&self) -> f64 {
        self.0*self.0 + self.1*self.1 + self.2*self.2
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn write_color(&self, samples_per_pixel: i32) -> String {
        let scale = 1. / samples_per_pixel as f64;
        let r = (scale * self.0).sqrt();
        let g = (scale * self.1).sqrt();
        let b = (scale * self.2).sqrt();

        format!("{} {} {}\n", (clamp(r, 0., 0.999) * 256.) as i32, (clamp(g, 0., 0.999) * 256.) as i32, (clamp(b, 0., 0.999) * 256.) as i32)
    }

    pub fn cross(&self, v: &Vec3) -> Vec3 {
        Vec3(self.1*v.2 - self.2*v.1, 
             self.2*v.0 - self.0*v.2, 
             self.0*v.1 - self.1*v.0)
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn random_vec(min: f64, max: f64) -> Vec3 {
        Vec3(min+(max-min)*random::<f64>(), min+(max-min)*random::<f64>(), min+(max-min)*random::<f64>())
    }

    pub fn random_unit_vector() -> Vec3 {
        let a: f64 = 2.*std::f64::consts::PI*random::<f64>();
        let z: f64 = -1. + 2.*random::<f64>();
        let r = (1.-z*z).sqrt();
        Vec3(r*a.cos(), r*a.sin(), z)
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_vec(-1., 1.);
            if p.length_squared() >= 1. {
                continue;
            }
            return p;
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3(2.*random::<f64>() - 1., 2.*random::<f64>() - 1., 0.);
            if p.length_squared() >= 1. {continue};
            return p
        }
    }
}