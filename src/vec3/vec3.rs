use crate::utils::helpers::random_f32;
use crate::utils::{constants::EPSILON, helpers::random_f32_with_range};
use std::{
    fmt,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

pub type Point3 = Vec3;

#[derive(Debug, Copy, Clone, Default, PartialOrd)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    #[inline(always)]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    #[inline]
    pub fn fromi(x: i32, y: i32, z: i32) -> Self {
        Vec3 {
            x: x as f32,
            y: y as f32,
            z: z as f32,
        }
    }

    #[inline]
    pub fn from(&self) -> Self {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    #[inline(always)]
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    #[inline(always)]
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline(always)]
    pub fn normalize(&self) -> Vec3 {
        *self / self.length()
    }

    #[inline]
    pub fn random() -> Vec3 {
        Vec3 {
            x: random_f32(),
            y: random_f32(),
            z: random_f32(),
        }
    }

    #[inline]
    pub fn random_range(min: f32, max: f32) -> Vec3 {
        Vec3 {
            x: random_f32_with_range(min, max),
            y: random_f32_with_range(min, max),
            z: random_f32_with_range(min, max),
        }
    }

    #[inline]
    pub fn random_unit_vector() -> Vec3 {
        loop {
            let random_uv = Vec3::random_range(-1.0, 1.0);
            let lensq = random_uv.length_squared();
            if 1e-160 < lensq && lensq <= 1.0 {
                return random_uv / lensq.sqrt();
            }
        }
    }

    #[inline]
    pub fn random_unit_vector_on_hemisphere(normal: Vec3) -> Vec3 {
        let v = Vec3::random_unit_vector();
        if Vec3::dot_product(v, normal) < 0.0 {
            -v
        } else {
            v
        }
    }

    #[inline(always)]
    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - (2.0 * Vec3::dot_product(v, n)) * n
    }

    #[inline]
    pub fn refract(
        uv: Vec3,
        n: Vec3,
        source_medium_ref_index: f32,
        dest_medium_ref_index: f32,
    ) -> Vec3 {
        let cos_theta: f32 = f32::min(Vec3::dot_product(-uv, n), 1.0);
        let perp_component =
            (source_medium_ref_index / dest_medium_ref_index) * (uv + cos_theta * n);
        let parallel_component = -1.0 * ((1.0 - perp_component.length_squared()).abs()).sqrt() * n;
        perp_component + parallel_component
    }

    #[inline]
    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(
                random_f32_with_range(-1.0, 1.0),
                random_f32_with_range(-1.0, 1.0),
                0.0,
            );
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    #[inline(always)]
    pub fn dot_product(lhs: Vec3, rhs: Vec3) -> f32 {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }

    #[inline(always)]
    pub fn cross_product(lhs: Vec3, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: lhs.y * rhs.z - lhs.z * rhs.y,
            y: lhs.z * rhs.x - lhs.x * rhs.z,
            z: lhs.x * rhs.y - lhs.y * rhs.x,
        }
    }

    #[inline(always)]
    pub fn unit(v: Vec3) -> Vec3 {
        v / v.length()
    }
}

impl PartialEq for Vec3 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        let x_equal = (self.x - other.x).abs() < EPSILON;
        let y_equal = (self.y - other.y).abs() < EPSILON;
        let z_equal = (self.z - other.z).abs() < EPSILON;

        x_equal && y_equal && z_equal
    }
}

impl fmt::Display for Point3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn div(self, rhs: f32) -> Self::Output {
        1.0 / rhs * self
    }
}

impl AddAssign for Vec3 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl SubAssign for Vec3 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl MulAssign<f32> for Vec3 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl DivAssign<f32> for Vec3 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn test_add() {
        let first_vec = Vec3::new(0.1, 0.0, 0.0);
        let second_vec = Vec3::new(0.2, 0.0, 0.0);
        assert_eq!(first_vec + second_vec, Vec3::new(0.3, 0.0, 0.0));
    }

    #[test]
    fn test_sub() {
        let first_vec = Vec3::new(0.1, 0.0, 0.0);
        let second_vec = Vec3::new(0.2, 0.0, 0.0);
        assert_eq!(second_vec - first_vec, Vec3::new(0.1, 0.0, 0.0));
    }

    #[test]
    fn test_mul() {
        let first_vec = Vec3::new(0.1, 0.0, 0.0);
        let second_vec = Vec3::new(0.2, 0.0, 0.0);
        assert_eq!(first_vec * second_vec, Vec3::new(0.02, 0.0, 0.0));
    }

    #[test]
    fn test_mul_scalar() {
        let first_vec = Vec3::new(0.1, 0.0, 0.0);
        let scalar = 5.0;
        assert_eq!(first_vec * scalar, Vec3::new(0.5, 0.0, 0.0));
    }

    #[test]
    fn test_div() {
        let first_vec = Vec3::new(0.1, 0.0, 0.0);
        let scalar = 5.0;
        assert_eq!(first_vec / scalar, Vec3::new(0.02, 0.0, 0.0));
    }

    #[test]
    fn test_add_assign() {
        let mut first_vec = Vec3::new(0.1, 0.0, 0.0);
        let second_vec = Vec3::new(0.2, 0.0, 0.0);
        first_vec += second_vec;
        assert_eq!(first_vec, Vec3::new(0.3, 0.0, 0.0));
    }

    #[test]
    fn test_sub_assign() {
        let mut first_vec = Vec3::new(0.1, 0.0, 0.0);
        let second_vec = Vec3::new(0.2, 0.0, 0.0);
        first_vec -= second_vec;
        assert_eq!(first_vec, Vec3::new(-0.1, 0.0, 0.0));
    }

    #[test]
    fn test_mul_assign() {
        let mut first_vec = Vec3::new(0.1, 0.0, 0.0);
        let scalar = 5.0;
        first_vec *= scalar;
        assert_eq!(first_vec, Vec3::new(0.5, 0.0, 0.0));
    }

    #[test]
    fn test_div_assign() {
        let mut first_vec = Vec3::new(0.1, 0.0, 0.0);
        let scalar = 5.0;
        first_vec /= scalar;
        assert_eq!(first_vec, Vec3::new(0.02, 0.0, 0.0));
    }

    #[test]
    fn test_length() {
        let first_vec = Vec3::new(0.1, 0.1, 0.0);
        assert_eq!(first_vec.length(), 0.14142136);
    }

    #[test]
    fn test_length_squared() {
        let first_vec = Vec3::new(0.1, 0.2, 0.0);
        assert_eq!(first_vec.length_squared(), 0.050000004); // meh
    }
}
