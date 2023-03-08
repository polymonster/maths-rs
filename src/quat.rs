use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::ops::Deref;
use std::ops::DerefMut;

use std::fmt::Display;
use std::fmt::Formatter;

use crate::num::*;
use crate::vec::*;
use crate::mat::*;
use crate::swizz::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Quat<T> {
    x: T,
    y: T,
    z: T,
    w: T
}

impl<T> Quat<T> where T: Float + FloatOps<T> + SignedNumberOps<T> {
    /// constructs an identity quaternion which means no rotation and if multiplied with another quat it has no effect
    pub fn identity() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
            w: T::one()
        }
    }

    /// construct a quaternion from 3 euler angles `x`, `y` and `z`
    pub fn from_euler_angles(x: T, y: T, z: T) -> Self {
        let half_z = T::point_five() * z;
        let half_x = T::point_five() * x;
        let half_y = T::point_five() * y;

        let cos_z_2 = T::cos(half_z);
        let cos_y_2 = T::cos(half_y);
        let cos_x_2 = T::cos(half_x);

        let sin_z_2 = T::sin(half_z);
        let sin_y_2 = T::sin(half_y);
        let sin_x_2 = T::sin(half_x);

        Self::normalize( Quat {
            w: cos_z_2 * cos_y_2 * cos_x_2 + sin_z_2 * sin_y_2 * sin_x_2,
            x: cos_z_2 * cos_y_2 * sin_x_2 - sin_z_2 * sin_y_2 * cos_x_2,
            y: cos_z_2 * sin_y_2 * cos_x_2 + sin_z_2 * cos_y_2 * sin_x_2,
            z: sin_z_2 * cos_y_2 * cos_x_2 - cos_z_2 * sin_y_2 * sin_x_2,
        })
    }

    /// constructs quaternion from `axis` and `angle` representation where `angle` is in radians
    pub fn from_axis_angle(axis: Vec3<T>, angle: T) -> Self {
        let half_angle = angle * T::point_five();
        Self::normalize( Quat {
            w: T::cos(half_angle),
            x: axis.x * T::sin(half_angle),
            y: axis.y * T::sin(half_angle),
            z: axis.z * T::sin(half_angle)
        })
    }

    /// constructs quaternion from matrix `m`
    pub fn from_matrix(m: Mat3<T>) -> Self {
        // https://math.stackexchange.com/questions/893984/conversion-of-rotation-matrix-to-quaternion
        let m00 = m.m[0];
        let m01 = m.m[4];
        let m02 = m.m[8];
        let m10 = m.m[1];
        let m11 = m.m[5];
        let m12 = m.m[9];
        let m20 = m.m[2];
        let m21 = m.m[6];
        let m22 = m.m[10];

        let t = T::zero();
        let t0 = T::zero();
        let t1 = T::one();

        let (x, y, z, w) = if m22 < t0 {
            if m00 > m11 {
                let x = t1 + m00 -m11 -m22;
                let y = m01 + m10;
                let z = m20 + m02;
                let w = m12 - m21;
                (x, y, z, w)
            }
            else {
                let x = m01 + m10;
                let y = t1 -m00 + m11 -m22;
                let z = m12 + m21;
                let w = m20 - m02;
                (x, y, z, w)

            }
        }
        else if m00 < -m11 {
            let x = m20+m02;
            let y = m12+m21;
            let z = t1 -m00 -m11 + m22;
            let w = m01-m10;
            (x, y, z, w)
        }
        else {
            let x = m12-m21;
            let y = m20-m02;
            let z = m01-m10;
            let w = t1 + m00 + m11 + m22;
            (x, y, z, w)
        };
        
        let sq = T::point_five() / T::sqrt(t);

        Quat {
            x: x * sq,
            y: y * sq,
            z: z * sq,
            w: w * sq
        }
    }

    /// returns euler angles as tuple `(x, y, z)` from quaternion
    pub fn to_euler_angles(self) -> (T, T, T) {
        let t2 = T::two();
        let t1 = T::one();

        // roll (x-axis rotation)
        let sinr = t2 * (self.w * self.x + self.y * self.z);
        let cosr = t1 - t2 * (self.x * self.x + self.y * self.y);
        let x = T::atan2(sinr, cosr);

        // pitch (y-axis rotation)
        let sinp = t2 * (self.w * self.y - self.z * self.x);
        let y = if T::abs(sinp) >= T::one() {
            // use 90 degrees if out of range
            T::copysign(T::pi() / t2, sinp)
        }
        else {
            T::asin(sinp)
        };

        // yaw (z-axis rotation)
        let siny = t2 * (self.w * self.z + self.x * self.y);
        let cosy = t1 - t2 * (self.y * self.y + self.z * self.z);
        let z = T::atan2(siny, cosy);

        (x, y, z)
    }

    /// returns a 3x3 rotation matrix
    pub fn get_matrix(self) -> Mat3<T> {
        let t1 = T::one();
        let t2 = T::two();
        Mat3::new(
            // row 1
            t1 - t2 * self.y * self.y - t2 * self.z * self.z,
            t2 * self.x * self.y - t2 * self.z * self.w,
            t2 * self.x * self.z + t2 * self.y * self.w,
            // row 2
            t2 * self.x * self.y + t2 * self.z * self.w,
            t1 - t2 * self.x * self.x - t2 * self.z * self.z,
            t2 * self.y * self.z - t2 * self.x * self.w,
            // row 3
            t2 * self.x * self.z - t2 * self.y * self.w,
            t2 * self.y * self.z + t2 * self.x * self.w,
            t1 - t2 * self.x * self.x - t2 * self.y * self.y
        )
    }

    /// returns a slice `T` of the quaternion
    pub fn as_slice(&self) -> &[T] {
        unsafe {
            std::slice::from_raw_parts(&self.x, 4)
        }
    }

    /// returns a mutable slice `T` of the quaternion
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe {
            std::slice::from_raw_parts_mut(&mut self.x, 4)
        }
    }

    /// returns a slice of `u8` bytes for the quaternion
    pub fn as_u8_slice(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts((&self.x as *const T) as *const u8, std::mem::size_of::<Quat<T>>())
        }
    }

    /// returns a quationion which reverses the axis of rotation of self
    pub fn reverse(self) -> Self {
        Quat { 
            x: -self.x, 
            y: -self.y, 
            z: -self.z, 
            w: self.w 
        }
    }

    /// returns a quaternion which is the inverse of self
    pub fn inverse(self) -> Self {
        self.reverse() / Self::mag2(self)
    }
}

impl<T> Add<Self> for Quat<T> where T: Float {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Quat {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl<T> AddAssign<Self> for Quat<T> where T: Float {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self.w += other.w;
    }
}

impl<T> Sub<Self> for Quat<T> where T: Float {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Quat {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl<T> SubAssign<Self> for Quat<T> where T: Float {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self.w -= other.w;
    }
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
        *self = Self::mul(*self, other);
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

/// returns a quaternion spherically interpolated between `e0` and `e1` by percentage `t`
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

/// returns a quaternion lineraly interpolated between `e0` and `e1` by percentage `t`
impl<T> Lerp<T> for Quat<T> where T: Float + FloatOps<T> + NumberOps<T> {
    fn lerp(e0: Self, e1: Self, t: T) -> Self {
        e0 * (T::one() - t) + e1 * t
    }
}

/// returns a quaternion lineraly interpolated between `e0` and `e1` by percentage `t` normalising the return value
impl<T> Nlerp<T> for Quat<T> where T: Float + FloatOps<T> + NumberOps<T> {
    fn nlerp(e0: Self, e1: Self, t: T) -> Self {
        Self::normalize(e0 * (T::one() - t) + e1 * t)
    }
}

/// constructs from a `Vec3`, assuming the `Vec3` is poulated with euler angles `(x, y, z)` where the angles are in radians
impl<T> From<Vec3<T>> for Quat<T> where T: Float + FloatOps<T> + SignedNumberOps<T> {
    fn from(v: Vec3<T>) -> Self {
        Quat::from_euler_angles(v.x, v.y, v.z)
    }
}

/// constructs from a `Vec4`, assuming the `Vec4` is an `axis` - `angle` representation. `xyz = axis, w = radian angle`
impl<T> From<Vec4<T>> for Quat<T> where T: Float + FloatOps<T> + SignedNumberOps<T> {
    fn from(v: Vec4<T>) -> Self {
        Quat::from_axis_angle(v.xyz(), v.w)
    }
}

/// constructs a quaternion from a 3x3 rotation matrix
impl<T> From<Mat3<T>> for Quat<T> where T: Float + FloatOps<T> + SignedNumberOps<T> {
    fn from(m: Mat3<T>) -> Self {
        Quat::from_matrix(m)
    }
}

/// dereference to a slice of `T`
impl<T> Deref for Quat<T> where T: Float + FloatOps<T> + SignedNumberOps<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        self.as_slice()
    }
}

/// mutably dereference to a slice of `T`
impl<T> DerefMut for Quat<T> where T: Float + FloatOps<T> + SignedNumberOps<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

/// displays like [1.0, 2.0, 3.0, 4.0]
impl<T> Display for Quat<T> where T: Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}, {}]", self.x, self.y, self.z, self.w)
    }
}