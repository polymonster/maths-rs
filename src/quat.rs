use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;

use crate::vec::*;
use crate::num::*;

#[derive(Debug, Copy, Clone)]
pub struct Quat<T> {
    x: T,
    y: T,
    z: T,
    w: T
}

impl<T> Div<T> for Quat<T> where T: Number {
    type Output = Self;
    fn div(self, other: T) -> Self {
        Quat {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}

impl<T> DivAssign<T> for Quat<T> where T: Number {
    fn div_assign(&mut self, other: T) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
        self.w /= other;
    }
}

impl<T> Mul<T> for Quat<T> where T: Number {
    type Output = Self;
    fn mul(self, other: T) -> Self {
        Quat {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl<T> MulAssign<T> for Quat<T> where T: Number {
    fn mul_assign(&mut self, other: T) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
        self.w *= other;
    }
}

impl<T> Mul<Self> for Quat<T> where T: Number {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Quat {
            w: self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
            x: self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y,
            y: self.w * other.y - self.x * other.z + self.y * other.w + self.z * other.z,
            z: self.w * other.z + self.x * other.y - self.y * other.x + self.z * other.w
        }
    }
}

impl<T> MulAssign<Self> for Quat<T> where T: Number {
    fn mul_assign(&mut self, other: Self) {
        self.x *= self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z;
        self.y *= self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y;
        self.z *= self.w * other.y - self.x * other.z + self.y * other.w + self.z * other.z;
        self.w *= self.w * other.z + self.x * other.y - self.y * other.x + self.z * other.w;
    }
}

impl<T> Neg for Quat<T> where T: Float {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Quat {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl<T> Dot<T> for Quat<T> where T: Float + FloatOps<T> {
    fn dot(a: Self, b: Self) -> T {
        a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
    }
}

impl<T> Magnitude<T> for Quat<T> where T: Float + FloatOps<T> {
    fn length(a: Self) -> T {
        T::sqrt(Self::dot(a, a))
    }

    fn mag(a: Self) -> T {
        T::sqrt(Self::dot(a, a))
    }

    fn mag2(a: Self) -> T {
        Self::dot(a, a)
    }

    fn normalize(a: Self) -> Self {
        let m = Self::mag(a);
        a / m
    }
}

impl<T> Slerp<T> for Quat<T> where T: Float + FloatOps<T> + NumberOps<T> + From<f64> {
    fn slerp(e0: Self, e1: Self, t: T) -> Self {
        let q1 = e0;
        // reverse the sign of e1 if q1.q2 < 0.
        let q2 = if Self::dot(q1, e1) < T::zero() {
            -e1
        }
        else {
            e1
        };
        
        let theta = T::acos(Self::dot(q1, q2));
        let k_epsilon = T::from(0.000001);
        let (m1, m2) = if theta > k_epsilon {
            (
                T::sin((T::one() - t) * theta) / T::sin(theta),
                T::sin(t * theta) / T::sin(theta)
            )
        }
        else {
            (
                T::one() - t,
                t
            )
        };

        Quat { 
            w: m1 * q1.w + m2 * q2.w, 
            x: m1 * q1.x + m2 * q2.x, 
            y: m1 * q1.y + m2 * q2.y, 
            z: m1 * q1.z + m2 * q2.z 
        }
    }
}