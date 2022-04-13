use ::std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// TODO Maybe use the newType pattern with the Deref trait rather than a type alias
pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn i() -> Self {
        Self::new(1., 0., 0.)
    }

    pub fn j() -> Self {
        Self::new(0., 1., 0.)
    }

    pub fn k() -> Self {
        Self::new(0., 0., 1.)
    }

    pub fn zeros() -> Self {
        Self::new(0., 0., 0.)
    }

    pub fn ones() -> Self {
        Self::new(1., 1., 1.)
    }

    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Self) -> Self {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn unit_vector(self) -> Self {
        self / self.length()
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Vec3) -> Self::Output {
        Self::Output {
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
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
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

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
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
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn can_default_construct_vec3() {
        let v3 = Vec3::default();
        assert_eq!(v3.x, 0.);
        assert_eq!(v3.y, 0.);
        assert_eq!(v3.z, 0.);
    }

    #[test]
    fn can_construct_vec3() {
        let v3 = Vec3::new(1., 2., 3.);
        assert_eq!(v3.x, 1.);
        assert_eq!(v3.y, 2.);
        assert_eq!(v3.z, 3.);
    }

    #[test]
    fn can_can_create_key_vectors() {
        assert_eq!(Vec3::i(), Vec3::new(1., 0., 0.));
        assert_eq!(Vec3::j(), Vec3::new(0., 1., 0.));
        assert_eq!(Vec3::k(), Vec3::new(0., 0., 1.));
        assert_eq!(Vec3::zeros(), Vec3::default());
        assert_eq!(Vec3::ones(), Vec3::new(1., 1., 1.));
    }
    #[test]
    fn can_add_vec3() {
        let lhs = Vec3::new(1., 2., 3.);
        let rhs = Vec3::new(4., 5., 6.);

        let outpt = lhs + rhs;
        let expected = Vec3::new(5., 7., 9.);

        assert_eq!(outpt, expected);
    }

    #[test]
    fn can_add_assign_vec3() {
        let mut lhs = Vec3::default();
        let rhs = Vec3::new(1., 2., 3.0);
        let expected = Vec3::new(1., 2., 3.0);

        lhs += rhs;

        assert_eq!(lhs, expected);
    }

    #[test]
    fn can_subtract_vec3() {
        let lhs = Vec3::new(4., 5., 6.);
        let rhs = Vec3::new(1., 2., 3.);

        let outpt = lhs - rhs;
        let expected = Vec3::new(3., 3., 3.);

        assert_eq!(outpt, expected);
    }

    #[test]
    fn can_subtract_assign_vec3() {
        let mut lhs = Vec3::default();
        let rhs = Vec3::new(1., 2., 3.0);
        let expected = Vec3::new(-1., -2., -3.0);

        lhs -= rhs;

        assert_eq!(lhs, expected);
    }

    #[test]
    fn can_multiply_vec3_by_scalar() {
        let v3 = Vec3::new(1., 2., 3.);
        let scalar = 2.;
        let expected = Vec3::new(2., 4., 6.);

        assert_eq!(v3 * scalar, expected);
        assert_eq!(scalar * v3, expected);
    }

    #[test]
    fn can_multiply_assign_vec3_by_scalar() {
        let mut v3 = Vec3::new(1., 2., 3.);
        let scalar = 2.;
        let expected = Vec3::new(2., 4., 6.);

        v3 *= scalar;

        assert_eq!(v3, expected);
    }

    #[test]
    fn can_multiply_vec3() {
        let lhs = Vec3::new(1., 2., 3.);
        let rhs = Vec3::new(2., 2., 2.);
        let expected = Vec3::new(2., 4., 6.);

        let outpt = lhs * rhs;

        assert_eq!(outpt, expected);
    }

    #[test]
    fn can_divide_vec3_by_scalar() {
        let v3 = Vec3::new(2., 4., 6.);
        let scalar = 2.;
        let expected = Vec3::new(1., 2., 3.);

        assert_eq!(v3 / scalar, expected);
    }

    #[test]
    fn can_divide_assign_vec3_by_scalar() {
        let mut v3 = Vec3::new(2., 4., 6.);
        let scalar = 2.;
        let expected = Vec3::new(1., 2., 3.);

        v3 /= scalar;

        assert_eq!(v3, expected);
    }

    #[test]
    fn can_negate_vec3() {
        let v3 = Vec3::new(1., 2., 3.);
        let expected = Vec3::new(-1., -2., -3.);

        assert_eq!(-v3, expected);
    }

    #[test]
    fn can_take_length_of_vec3() {
        let v3 = Vec3::new(2., 3., 6.);

        assert_eq!(v3.length_squared(), 49.);
        assert_eq!(v3.length(), 7.);
    }

    #[test]
    fn can_take_vec3_dot_product() {
        let a = Vec3::new(1., 2., 3.);
        let b = Vec3::new(4., 5., 6.);
        let expected = 32.;

        assert_eq!(a.dot(b), expected);
        assert_eq!(b.dot(a), expected);
    }

    #[test]
    fn can_take_vec3_cross_product() {
        let a = Vec3::new(1., 2., 3.);
        let b = Vec3::new(4., 5., 6.);
        let expected_1 = Vec3::new(-3., 6., -3.);
        let expected_2 = Vec3::new(3., -6., 3.);

        assert_eq!(a.cross(b), expected_1);
        assert_eq!(b.cross(a), expected_2);
    }

    #[test]
    fn can_get_vec3_unit_vector() {
        let v3 = Vec3::new(1., 2., 2.);
        let expected = Vec3::new(1. / 3., 2. / 3., 2. / 3.);

        assert_eq!(expected, v3.unit_vector())
    }
}
