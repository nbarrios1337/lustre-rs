use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vec3f32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3f32 {
    /// Creates a new [`Vec3f32`].
    pub fn new() -> Vec3f32 {
        Vec3f32 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

// unary neg
impl Neg for Vec3f32 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// binary math ops with vec

// +
impl Add for Vec3f32 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

// -
impl Sub for Vec3f32 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

// binary math ops with scalar

// +
impl Add<f32> for Vec3f32 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

// -
impl Sub<f32> for Vec3f32 {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

// *
impl Mul<f32> for Vec3f32 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

// /
impl Div<f32> for Vec3f32 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

// +=
impl AddAssign for Vec3f32 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

// -=
impl SubAssign for Vec3f32 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

// *=
impl AddAssign<f32> for Vec3f32 {
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}

// *=
impl SubAssign<f32> for Vec3f32 {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
    }
}

// *=
impl MulAssign<f32> for Vec3f32 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

// /=
impl DivAssign<f32> for Vec3f32 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3f32;

    // use super::Vec3f32::*;
    #[test]
    fn new() {
        let v = Vec3f32::new();
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
        assert_eq!(v.z, 0.0);
    }

    #[test]
    fn neg() {
        let v = Vec3f32 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        assert_eq!(
            -v,
            Vec3f32 {
                x: -1.0,
                y: -2.0,
                z: -3.0,
            }
        );
    }

    #[test]
    fn add_vec() {
        let a = Vec3f32 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        let b = Vec3f32 {
            x: 3.0,
            y: 2.0,
            z: 1.0,
        };

        assert_eq!(
            a + b,
            Vec3f32 {
                x: 4.0,
                y: 4.0,
                z: 4.0,
            }
        );
    }

    #[test]
    fn sub_vec() {
        let a = Vec3f32 {
            x: 5.0,
            y: 6.0,
            z: 7.0,
        };

        let b = Vec3f32 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };

        assert_eq!(
            a - b,
            Vec3f32 {
                x: 2.0,
                y: 2.0,
                z: 2.0,
            }
        );
    }

    #[test]
    fn add_scalar() {
        let a = Vec3f32 {
            x: 2.0,
            y: 3.0,
            z: 4.0,
        };

        assert_eq!(
            a + 2.0,
            Vec3f32 {
                x: 4.0,
                y: 5.0,
                z: 6.0,
            }
        );
    }

    #[test]
    fn sub_scalar() {
        let a = Vec3f32 {
            x: 2.0,
            y: 3.0,
            z: 4.0,
        };

        assert_eq!(
            a - 2.0,
            Vec3f32 {
                x: 0.0,
                y: 1.0,
                z: 2.0,
            }
        );
    }

    #[test]
    fn mul_scalar() {
        let a = Vec3f32 {
            x: 2.0,
            y: 3.0,
            z: 4.0,
        };

        assert_eq!(
            a * 2.0,
            Vec3f32 {
                x: 4.0,
                y: 6.0,
                z: 8.0,
            }
        );
    }

    #[test]
    fn div_scalar() {
        let a = Vec3f32 {
            x: 4.0,
            y: 6.0,
            z: 8.0,
        };

        assert_eq!(
            a / 2.0,
            Vec3f32 {
                x: 2.0,
                y: 3.0,
                z: 4.0,
            }
        );
    }
}
