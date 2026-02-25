use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Rem;
use std::ops::RemAssign;
use std::ops::Neg;
use std::ops::Deref;
use std::ops::DerefMut;

use std::cmp::PartialEq;

use std::fmt::Display;
use std::fmt::Formatter;

use crate::num::*;

//
// Vec Traits
//

/// generic vec trait to allow sized vectors to be treated generically
pub trait VecN<T: Number>:
    Base<T> + Dot<T> +
    Index<usize, Output=T> + IndexMut<usize> +
    Add<T, Output=Self> + Sub<T, Output=Self> +
    Mul<T, Output=Self> + Div<T, Output=Self> {
    /// returns the count of elements in the vector type
    fn len() -> usize;
    /// returns true if all elements in vector `a` are non-zero
    fn all(a: Self) -> bool;
    /// returns true if any element of the vector `a` is non-zero
    fn any(a: Self) -> bool;
    /// returns a vector initialised as a unit vector in the x-axis `[1, 0, 0, 0]`
    fn unit_x() -> Self;
    /// returns a vector initialised as a unit vector in the y-axis `[0, 1, 0, 0]`
    fn unit_y() -> Self;
    /// returns a vector initialised as a unit vector in the z-axis `[0, 0, 1, 0]` value will be truncated to 0 for vectors < 3 dimension
    fn unit_z() -> Self;
    /// returns a vector initialised as a unit vector in the w-axis `[0, 0, 0, 1]` value will be truncated to 0 for vectors < 4 dimension
    fn unit_w() -> Self;
    /// returns a vector initialised to red `[1, 0, 0, 1]`
    fn red() -> Self;
    /// returns a vector initialised to green `[0, 1, 0, 1]`
    fn green() -> Self;
    /// returns a vector initialised to blue `[0, 0, 1, 1]` value will be truncated to 0 for vectors < 3 dimension
    fn blue() -> Self;
    /// returns a vector initialised to cyan `[0, 1, 1, 1]` value will be truncated to green for vectors < 3 dimension
    fn cyan() -> Self;
    /// returns a vector initialised to magenta `[1, 0, 1, 1]` value will be truncated to red for vectors < 3 dimension
    fn magenta() -> Self;
    /// returns a vector initialised to yellow `[1, 1, 0, 0]`
    fn yellow() -> Self;
    /// returns a vector initialised to black (zero's)
    fn black() -> Self;
    /// returns a vector initialised to white (ones's)
    fn white() -> Self;
    /// returns a slice T of the vector
    fn as_slice(&self) -> &[T];
    /// returns a mutable slice T of the vector
    fn as_mut_slice(&mut self) -> &mut [T];
    /// returns a slice of bytes for the vector
    fn as_u8_slice(&self) -> &[u8];
    /// returns the largest scalar value component contained in the vector `a`
    fn max_scalar(a: Self) -> T;
    /// returns the smallest scalar value component contained in the vector `a`
    fn min_scalar(a: Self) -> T;
}

/// trait for vectors of signed types to allow Neg
pub trait SignedVecN<T: SignedNumber>: Neg<Output=Self> {
    /// returns a vector initialised with -1
    fn minus_one() -> Self;
}

/// trait for operations involve vector magnitude or dot product
pub trait Magnitude<T: Float> {
    /// returns scalar magnitude or length of vector `a`
    fn length(a: Self) -> T;
    /// returns scalar magnitude or length of vector `a`
    fn mag(a: Self) -> T;
    /// returns scalar magnitude or length of vector `a` squared to avoid using sqrt
    fn mag2(a: Self) -> T;
    /// returns a normalized unit vector of `a`
    fn normalize(a: Self) -> Self;
}

/// operations to apply to n-dimensional vectors
pub trait VecFloatOps<T: Float>: SignedVecN<T> + Magnitude<T> {
    /// returns scalar distance between point `a` and `b` (magnitude of the vector between the 2 points)
    fn distance(a: Self, b: Self) -> T;
    /// returns scalar distance between point `a` and `b` (magnitude of the vector between the 2 points)
    fn dist(a: Self, b: Self) -> T;
    /// returns scalar squared distance between point `a` and `b` to avoid using sqrt
    fn dist2(a: Self, b: Self) -> T;
    /// returns a reflection vector using an incident ray `i` and a surface normal `n`
    fn reflect(i: Self, n: Self) -> Self;
    /// returns a refraction vector using an entering ray `i`, a surface normal `n`, and a refraction index `eta`
    fn refract(i: Self, n: Self, eta: T) -> Self;
    /// returns linear interpolation between `e0` and `e1`, `t` specifies the ratio to interpolate between the values, with component-wise `t`
    fn vlerp(e0: Self, e1: Self, t: Self) -> Self;
    /// returns vector with component wise hermite interpolation between `0-1`, with component-wise `t`
    fn vsmoothstep(e0: Self, e1: Self, t: Self) -> Self;
    /// returns vector `a` raised to component wise power `exp`
    fn powfn(a: Self, exp: Self) -> Self;
}

/// trait for dot product
pub trait Dot<T> {
    /// vector dot-product
    fn dot(a: Self, b: Self) -> T;
}

impl<T> Dot<T> for Vec2<T> where T: Number {
    fn dot(a: Self, b: Self) -> T {
        a.x * b.x + a.y * b.y
    }
}

impl<T> Dot<T> for Vec3<T> where T: Number {
    fn dot(a: Self, b: Self) -> T {
        a.x * b.x + a.y * b.y + a.z * b.z
    }
}

impl<T> Dot<T> for Vec4<T> where T: Number {
    fn dot(a: Self, b: Self) -> T {
        a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
    }
}

/// trait for cross product, this is implemented for Vec3
pub trait Cross<T> {
    /// vector cross-product
    fn cross(a: Self, b: Self) -> Self;
}

impl<T> Cross<T> for Vec3<T> where T: Number {
    fn cross(a: Self, b: Self) -> Self {
        Vec3 {
            x: (a.y * b.z) - (a.z * b.y),
            y: (a.z * b.x) - (a.x * b.z),
            z: (a.x * b.y) - (a.y * b.x),
        }
    }
}

/// trait for scalar and vector triple products
pub trait Triple<T> {
    /// scalar triple product
    fn scalar_triple(a: Self, b: Self, c: Self) -> T;
    /// vector triple product
    fn vector_triple(a: Self, b: Self, c: Self) -> Self;
}

/// 3D triple products
impl<T> Triple<T> for Vec3<T> where T: Number {
    fn scalar_triple(a: Self, b: Self, c: Self) -> T {
        a.x * (b.y * c.z - b.z * c.y) + a.y * (b.z * c.x - b.x * c.z) + a.z * (b.x * c.y - b.y * c.x)
    }

    fn vector_triple(a: Self, b: Self, c: Self) -> Self {
        Self::cross(Self::cross(a, b), c)
    }
}

/// 2D triple product specialisation, leveraging z-axis
impl<T> Triple<T> for Vec2<T> where T: Number + SignedNumber {
    fn scalar_triple(_a: Self, b: Self, c: Self) -> T {
        b.x * c.y - b.y * c.x
    }

    fn vector_triple(a: Self, b: Self, c: Self) -> Self {
        let a3 = Vec3::<T>::new(a.x, a.y, T::zero());
        let b3 = Vec3::<T>::new(b.x, b.y, T::zero());
        let c3 = Vec3::<T>::new(c.x, c.y, T::zero());
        let v3 = Vec3::<T>::cross(Vec3::<T>::cross(a3, b3), c3);
        Self {
            x: v3.x,
            y: v3.y
        }
    }
}

/// trait for spherical interpolation, which is applicable with vec and quat
pub trait Slerp<T: Float + FloatOps<T>> {
    // spherically interpolate between edges `e0` and `e1` by percentage `t`
    fn slerp(e0: Self, e1: Self, t: T) -> Self;
}

/// trait for normalized interpolation, which is applicable with vec and quat
pub trait Nlerp<T: Float> {
    // linearly interpolate between edges `e0` and `e1` by percentage `t` and return the normalized value
    fn nlerp(e0: Self, e1: Self, t: T) -> Self;
}

//
// Macro Implementation
//

/// macro to stamp out various n-dimensional vectors, all of their ops and functions
macro_rules! vec_impl {
    ($VecN:ident { $($field:ident, $field_index:expr),* }, $len:expr, $module:ident) => {

        #[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(feature="hash", derive(Hash))]
        #[derive(Debug, Copy, Clone)]
        #[repr(C)]
        pub struct $VecN<T> {
            $(pub $field: T,)+
        }

        impl<T> $VecN<T> where T: Number {
            /// construct a new vector from scalar values
            pub fn new($($field: T,)+) -> $VecN<T> {
                $VecN {
                    $($field,)+
                }
            }
        }

        impl<T> VecN<T> for $VecN<T> where T: Number {
            fn len() -> usize {
                $len
            }

            fn all(a: Self) -> bool {
                $(a.$field != T::zero() &&)+
                true
            }

            fn any(a: Self) -> bool {
                $(a.$field != T::zero() ||)+
                false
            }

            fn max_scalar(a: Self) -> T {
                let mut max = a[0];
                for i in 1..Self::len() {
                    if a[i] > max {
                        max = a[i];
                    }
                }
                max
            }

            fn min_scalar(a: Self) -> T {
                let mut min = a[0];
                for i in 1..Self::len() {
                    if a[i] < min {
                        min = a[i];
                    }
                }
                min
            }

            fn unit_x() -> $VecN<T> {
                let v = [T::one(), T::zero(), T::zero(), T::zero()];
                Self {
                    $($field: v[$field_index],)+
                }
            }

            fn unit_y() -> $VecN<T> {
                let v = [T::zero(), T::one(), T::zero(), T::zero()];
                Self {
                    $($field: v[$field_index],)+
                }
            }

            fn unit_z() -> $VecN<T> {
                let v = [T::zero(), T::zero(), T::one(), T::zero()];
                Self {
                    $($field: v[$field_index],)+
                }
            }

            fn unit_w() -> $VecN<T> {
                let v = [T::zero(), T::zero(), T::zero(), T::one()];
                Self {
                    $($field: v[$field_index],)+
                }
            }

            fn red() -> $VecN<T> {
                let v = [T::one(), T::zero(), T::zero(), T::one()];
                Self {
                    $($field: v[$field_index],)+
                }
            }

            fn green() -> $VecN<T> {
                let v = [T::zero(), T::one(), T::zero(), T::one()];
                Self {
                    $($field: v[$field_index],)+
                }
            }

            fn blue() -> $VecN<T> {
                let v = [T::zero(), T::zero(), T::one(), T::one()];
                Self {
                    $($field: v[$field_index],)+
                }
            }

            fn cyan() -> $VecN<T> {
                let v = [T::zero(), T::one(), T::one(), T::one()];
                Self {
                    $($field: v[$field_index],)+
                }
            }

            fn magenta() -> $VecN<T> {
                let v = [T::one(), T::zero(), T::one(), T::one()];
                Self {
                    $($field: v[$field_index],)+
                }
            }

            fn yellow() -> $VecN<T> {
                let v = [T::one(), T::one(), T::zero(), T::one()];
                Self {
                    $($field: v[$field_index],)+
                }
            }

            fn black() -> $VecN<T> {
                let v = [T::zero(), T::zero(), T::zero(), T::one()];
                Self {
                    $($field: v[$field_index],)+
                }
            }

            fn white() -> $VecN<T> {
                Self::one()
            }

            fn as_slice(&self) -> &[T] {
                // SAFETY: The struct is #[repr(C)] with $len fields of the same type T laid out
                // contiguously. `self.x` is the first field, so the pointer is valid for $len
                // reads of T and the lifetime is bounded by `&self`.
                unsafe {
                    std::slice::from_raw_parts(&self.x, $len)
                }
            }

            fn as_mut_slice(&mut self) -> &mut [T] {
                // SAFETY: The struct is #[repr(C)] with $len fields of the same type T laid out
                // contiguously. `self.x` is the first field, so the pointer is valid for $len
                // writes of T. Exclusive access is guaranteed by `&mut self`.
                unsafe {
                    std::slice::from_raw_parts_mut(&mut self.x, $len)
                }
            }

            fn as_u8_slice(&self) -> &[u8] {
                // SAFETY: Any initialized memory can be viewed as bytes. The struct is #[repr(C)]
                // and T: Number ensures all bytes are initialized. The cast to *const u8 is valid
                // since u8 has alignment 1, and the length is the exact byte size of the struct.
                unsafe {
                    std::slice::from_raw_parts((&self.x as *const T) as *const u8, std::mem::size_of::<$VecN<T>>())
                }
            }
        }

        impl<T> SignedVecN<T> for $VecN<T> where T: SignedNumber  {
            fn minus_one() -> Self {
                $VecN {
                    $($field: T::minus_one(),)+
                }
            }
        }

        impl<T> IntegerOps<T> for $VecN<T> where T: Integer + IntegerOps<T> {
            fn pow(a: Self, exp: u32) -> Self {
                Self {
                    $($field: T::pow(a.$field, exp),)+
                }
            }
        }

        impl<T> NumberOps<T> for $VecN<T> where T: Number + NumberOps<T> {
            fn min(a: Self, b: Self) -> Self {
                Self {
                    $($field: T::min(a.$field, b.$field),)+
                }
            }

            fn max(a: Self, b: Self) -> Self {
                Self {
                    $($field: T::max(a.$field, b.$field),)+
                }
            }

            fn clamp(x: Self, min: Self, max: Self) -> Self {
                Self {
                    $($field: T::max(T::min(x.$field, max.$field), min.$field),)+
                }
            }

            fn step(a: Self, b: Self) -> Self {
                Self {
                    $($field: T::step(a.$field, b.$field),)+
                }
            }
        }

        impl<T> SignedNumberOps<T> for $VecN<T> where T: SignedNumber + SignedNumberOps<T> {
            fn signum(a: Self) -> Self {
                Self {
                    $($field: T::signum(a.$field),)+
                }
            }

            fn abs(a: Self) -> Self {
                Self {
                    $($field: T::abs(a.$field),)+
                }
            }
        }

        impl<T> Magnitude<T> for $VecN<T> where T: Float + FloatOps<T> {
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

        impl<T> VecFloatOps<T> for $VecN<T> where T: Float + FloatOps<T> {
            fn distance(a: Self, b: Self) -> T {
                let c = a-b;
                T::sqrt(Self::dot(c, c))
            }

            fn dist(a: Self, b: Self) -> T {
                Self::distance(a, b)
            }

            fn dist2(a: Self, b: Self) -> T {
                let c = a-b;
                Self::dot(c, c)
            }

            fn reflect(i: Self, n: Self) -> Self {
                // https://docs.microsoft.com/en-us/windows/win32/direct3dhlsl/dx-graphics-hlsl-reflect
                i - Self::from(T::two()) * n * Self::dot(i, n)
            }

            fn refract(i: Self, n: Self, eta: T) -> Self {
                // https://asawicki.info/news_1301_reflect_and_refract_functions.html
                let n_dot_i = Self::dot(n, i);
                let k = T::one() - eta * eta * (T::one() - n_dot_i * n_dot_i);
                if k < T::zero() {
                    Self::zero()
                }
                else {
                    (i * eta) - n * ((n_dot_i + T::sqrt(k)) * eta)
                }
            }

            fn vlerp(e0: Self, e1: Self, t: Self) -> Self {
                Self {
                    $($field: T::lerp(e0.$field, e1.$field, t.$field),)+
                }
            }

            fn vsmoothstep(e0: Self, e1: Self, t: Self) -> Self {
                Self {
                    $($field: T::smoothstep(e0.$field, e1.$field, t.$field),)+
                }
            }

            fn powfn(a: Self, exp: Self) -> Self {
                Self {
                    $($field: T::powf(a.$field, exp.$field),)+
                }
            }
        }

        impl<T> Slerp<T> for $VecN<T> where T: Float + FloatOps<T> + NumberOps<T> {
            fn slerp(e0: Self, e1: Self, t: T) -> Self {
                // https://blog.demofox.org/2016/02/19/normalized-vector-interpolation-tldr/
                let dot = Self::dot(e0, e1);
                let dot = T::clamp(dot, T::minus_one(), T::one());
                let theta = T::acos(dot) * t;
                let v = Self::normalize(e1 - e0 * dot);
                ((e0 * T::cos(theta)) + (v * T::sin(theta)))
            }
        }

        impl<T> Nlerp<T> for $VecN<T> where T: Float + FloatOps<T> + NumberOps<T> {
            fn nlerp(e0: Self, e1: Self, t: T) -> Self {
                Self::normalize( Self {
                    $($field: T::lerp(e0.$field, e1.$field, t),)+
                })
            }
        }

        impl<T> Base<T> for $VecN<T> where T: Number {
            fn zero() -> Self {
                $VecN {
                    $($field: T::zero(),)+
                }
            }

            fn one() -> Self {
                $VecN {
                    $($field: T::one(),)+
                }
            }

            fn two() -> Self {
                $VecN {
                    $($field: T::two(),)+
                }
            }

            fn three() -> Self {
                $VecN {
                    $($field: T::three(),)+
                }
            }

            fn four() -> Self {
                $VecN {
                    $($field: T::four(),)+
                }
            }

            fn min_value() -> Self {
                $VecN {
                    $($field: T::min_value(),)+
                }
            }

            fn max_value() -> Self {
                $VecN {
                    $($field: T::max_value(),)+
                }
            }
        }

        impl<T> Lerp<T> for $VecN<T> where T: Float + Lerp<T> {
            fn lerp(e0: Self, e1: Self, t: T) -> Self {
                Self {
                    $($field: T::lerp(e0.$field, e1.$field, t),)+
                }
            }
        }

        impl<T> FloatOps<T> for $VecN<T> where T: Float + SignedNumberOps<T> + NumberOps<T> + FloatOps<T> {
            fn point_five() -> Self {
                $VecN {
                    $($field: T::point_five(),)+
                }
            }

            fn pi() -> Self {
                $VecN {
                    $($field: T::pi(),)+
                }
            }

            fn two_pi() -> Self {
                $VecN {
                    $($field: T::two_pi(),)+
                }
            }

            fn inv_pi() -> Self {
                $VecN {
                    $($field: T::inv_pi(),)+
                }
            }

            fn phi() -> Self {
                $VecN {
                    $($field: T::phi(),)+
                }
            }

            fn inv_phi() -> Self {
                $VecN {
                    $($field: T::inv_phi(),)+
                }
            }

            fn tau() -> Self {
                $VecN {
                    $($field: T::tau(),)+
                }
            }

            fn sqrt(a: Self) -> Self {
                Self {
                    $($field: T::sqrt(a.$field),)+
                }
            }

            fn rsqrt(a: Self) -> Self {
                Self {
                    $($field: T::recip(T::sqrt(a.$field)),)+
                }
            }

            fn recip(a: Self) -> Self {
                Self {
                    $($field: T::recip(a.$field),)+
                }
            }

            fn powi(a: Self, exp: i32) -> Self {
                Self {
                    $($field: T::powi(a.$field, exp),)+
                }
            }

            fn powf(a: Self, exp: T) -> Self {
                Self {
                    $($field: T::powf(a.$field, exp),)+
                }
            }

            fn mad(m: Self, a: Self, b: Self) -> Self {
                Self {
                    $($field: T::mad(m.$field, a.$field, b.$field),)+
                }
            }

            fn approx(a: Self, b: Self, eps: T) -> bool {
                $(T::abs(a.$field - b.$field) < eps &&)+
                true
            }

            fn floor(a: Self) -> Self {
                Self {
                    $($field: T::floor(a.$field),)+
                }
            }

            fn ceil(a: Self) -> Self {
                Self {
                    $($field: T::ceil(a.$field),)+
                }
            }

            fn copysign(a: Self, sign: T) -> Self {
                Self {
                    $($field: T::copysign(a.$field, sign),)+
                }
            }

            fn smoothstep(e0: Self, e1: Self, t: T) -> Self {
                Self {
                    $($field: T::smoothstep(e0.$field, e1.$field, t),)+
                }
            }

            fn round(a: Self) -> Self {
                Self {
                    $($field: T::round(a.$field),)+
                }
            }

            fn is_nan(a: Self) -> Self {
                Self {
                    $($field: T::is_nan(a.$field),)+
                }
            }

            fn is_infinite(a: Self) -> Self {
                Self {
                    $($field: T::is_infinite(a.$field),)+
                }
            }

            fn is_finite(a: Self) -> Self {
                Self {
                    $($field: T::is_finite(a.$field),)+
                }
            }

            fn saturate(x: Self) -> Self {
                Self::clamp(x, Self::zero(), Self::one())
            }

            fn deg_to_rad(a: Self) -> Self {
                Self {
                    $($field: T::deg_to_rad(a.$field),)+
                }
            }

            fn rad_to_deg(a: Self) -> Self {
                Self {
                    $($field: T::rad_to_deg(a.$field),)+
                }
            }

            fn fmod(x: Self, y: Self) -> Self {
                x % y
            }

            fn frac(v: Self) -> Self {
                Self {
                    $($field: T::frac(v.$field),)+
                }
            }

            fn trunc(v: Self) -> Self {
                Self {
                    $($field: T::trunc(v.$field),)+
                }
            }

            fn modf(v: Self) -> (Self, Self) {
                (
                    Self {
                        $($field: T::frac(v.$field),)+
                    },
                    Self {
                        $($field: T::trunc(v.$field),)+
                    }
                )
            }

            fn cos(v: Self) -> Self {
                Self {
                    $($field: T::cos(v.$field),)+
                }
            }

            fn sin(v: Self) -> Self {
                Self {
                    $($field: T::sin(v.$field),)+
                }
            }

            fn tan(v: Self) -> Self {
                Self {
                    $($field: T::tan(v.$field),)+
                }
            }

            fn acos(v: Self) -> Self {
                Self {
                    $($field: T::acos(v.$field),)+
                }
            }

            fn asin(v: Self) -> Self {
                Self {
                    $($field: T::asin(v.$field),)+
                }
            }

            fn atan(v: Self) -> Self {
                Self {
                    $($field: T::atan(v.$field),)+
                }
            }

            fn cosh(v: Self) -> Self {
                Self {
                    $($field: T::cosh(v.$field),)+
                }
            }

            fn sinh(v: Self) -> Self {
                Self {
                    $($field: T::sinh(v.$field),)+
                }
            }

            fn tanh(v: Self) -> Self {
                Self {
                    $($field: T::tanh(v.$field),)+
                }
            }

            fn sin_cos(v: Self) -> (Self, Self) {
                (
                    Self {
                        $($field: T::sin(v.$field),)+
                    },
                    Self {
                        $($field: T::cos(v.$field),)+
                    }
                )
            }

            fn atan2(y: Self, x: Self) -> Self {
                Self {
                    $($field: T::atan2(y.$field, x.$field),)+
                }
            }

            fn exp(v: Self) -> Self {
                Self {
                    $($field: T::exp(v.$field),)+
                }
            }

            fn exp2(v: Self) -> Self {
                Self {
                    $($field: T::exp2(v.$field),)+
                }
            }

            fn log2(v: Self) -> Self {
                Self {
                    $($field: T::log2(v.$field),)+
                }
            }

            fn log10(v: Self) -> Self {
                Self {
                    $($field: T::log10(v.$field),)+
                }
            }

            fn log(v: Self, base: T) -> Self {
                Self {
                    $($field: T::log(v.$field, base),)+
                }
            }
        }

        impl<T> Index<usize> for $VecN<T> {
            type Output = T;
            fn index(&self, i: usize) -> &Self::Output {
                match i {
                    $($field_index => &self.$field, )+
                    _ => panic!()
                }
            }
        }

        impl<T> IndexMut<usize> for $VecN<T> {
            fn index_mut(&mut self, i: usize) -> &mut T {
                match i {
                    $($field_index => &mut self.$field, )+
                    _ => panic!()
                }
            }
        }

        /// displays like [10.0, 12.0, 13.0]
        impl<T> Display for $VecN<T> where T: Display {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                let mut output = String::from("[");
                $(
                    output += &self.$field.to_string();
                    if $field_index < $len-1 {
                        output += &String::from(", ");
                    }
                )+
                output += "]";
                write!(f, "{}", output)
            }
        }

        /// default to zero
        impl<T> Default for $VecN<T> where T: Number {
            fn default() -> Self {
                Self::zero()
            }
        }

        //
        // ops
        //

        impl<T> Deref for $VecN<T> where T: Number {
            type Target = [T];
            fn deref(&self) -> &Self::Target {
                self.as_slice()
            }
        }

        impl<T> DerefMut for $VecN<T> where T: Number {
            fn deref_mut(&mut self) -> &mut [T] {
                self.as_mut_slice()
            }
        }

        impl<T> Add<Self> for $VecN<T> where T: Number {
            type Output = Self;
            fn add(self, other: Self) -> Self {
                Self {
                    $($field: self.$field + other.$field,)+
                }
            }
        }

        impl<T> Add<T> for $VecN<T> where T: Number {
            type Output = Self;
            fn add(self, other: T) -> Self {
                Self {
                    $($field: self.$field + other,)+
                }
            }
        }

        // add refs
        impl<T> Add<&Self> for $VecN<T> where T: Number {
            type Output = Self;
            fn add(self, other: &Self) -> Self {
                self.add(*other)
            }
        }

        impl<T> Add<$VecN<T>> for &$VecN<T> where T: Number {
            type Output = $VecN<T>;
            fn add(self, other: $VecN<T>) -> $VecN<T> {
                *self + other
            }
        }

        impl<T> Add<Self> for &$VecN<T> where T: Number {
            type Output = $VecN<T>;
            fn add(self, other: Self) -> $VecN<T> {
                *self + other
            }
        }

        impl<T> AddAssign<Self> for $VecN<T> where T: Number {
            fn add_assign(&mut self, other: Self) {
                $(self.$field += other.$field;)+
            }
        }

        impl<T> AddAssign<T> for $VecN<T> where T: Number {
            fn add_assign(&mut self, other: T) {
                $(self.$field += other;)+
            }
        }

        // add assign ref
        impl<T> AddAssign<&Self> for $VecN<T> where T: Number {
            fn add_assign(&mut self, other: &Self) {
                $(self.$field += other.$field;)+
            }
        }

        impl<T> Sub<Self> for $VecN<T> where T: Number {
            type Output = Self;
            fn sub(self, other: Self) -> Self {
                Self {
                    $($field: self.$field - other.$field,)+
                }
            }
        }

        impl<T> Sub<T> for $VecN<T> where T: Number {
            type Output = Self;
            fn sub(self, other: T) -> Self {
                Self {
                    $($field: self.$field - other,)+
                }
            }
        }

        // sub refs
        impl<T> Sub<&Self> for $VecN<T> where T: Number {
            type Output = Self;
            fn sub(self, other: &Self) -> Self {
                self.sub(*other)
            }
        }

        impl<T> Sub<$VecN<T>> for &$VecN<T> where T: Number {
            type Output = $VecN<T>;
            fn sub(self, other: $VecN<T>) -> $VecN<T> {
                *self - other
            }
        }

        impl<T> Sub<Self> for &$VecN<T> where T: Number {
            type Output = $VecN<T>;
            fn sub(self, other: Self) -> $VecN<T> {
                *self - other
            }
        }

        impl<T> SubAssign<Self> for $VecN<T> where T: Number {
            fn sub_assign(&mut self, other: Self) {
                $(self.$field -= other.$field;)+
            }
        }

        impl<T> SubAssign<T> for $VecN<T> where T: Number {
            fn sub_assign(&mut self, other: T) {
                $(self.$field -= other;)+
            }
        }

        // sub assign ref
        impl<T> SubAssign<&Self> for $VecN<T> where T: Number {
            fn sub_assign(&mut self, other: &Self) {
                $(self.$field -= other.$field;)+
            }
        }

        impl<T> Mul<Self> for $VecN<T> where T: Number {
            type Output = Self;
            fn mul(self, other: Self) -> Self {
                Self {
                    $($field: self.$field * other.$field,)+
                }
            }
        }

        impl<T> Mul<T> for $VecN<T> where T: Number {
            type Output = Self;
            fn mul(self, other: T) -> Self {
                Self {
                    $($field: self.$field * other,)+
                }
            }
        }

        // mul refs
        impl<T> Mul<&Self> for $VecN<T> where T: Number {
            type Output = Self;
            fn mul(self, other: &Self) -> Self {
                self.mul(*other)
            }
        }

        impl<T> Mul<$VecN<T>> for &$VecN<T> where T: Number {
            type Output = $VecN<T>;
            fn mul(self, other: $VecN<T>) -> $VecN<T> {
                *self * other
            }
        }

        impl<T> Mul<Self> for &$VecN<T> where T: Number {
            type Output = $VecN<T>;
            fn mul(self, other: Self) -> $VecN<T> {
                *self * other
            }
        }

        impl<T> MulAssign<Self> for $VecN<T> where T: Number {
            fn mul_assign(&mut self, other: Self) {
                $(self.$field *= other.$field;)+
            }
        }

        impl<T> MulAssign<T> for $VecN<T> where T: Number {
            fn mul_assign(&mut self, other: T) {
                $(self.$field *= other;)+
            }
        }

        // mul assign ref
        impl<T> MulAssign<&Self> for $VecN<T> where T: Number {
            fn mul_assign(&mut self, other: &Self) {
                $(self.$field *= other.$field;)+
            }
        }

        impl<T> Div<Self> for $VecN<T> where T: Number {
            type Output = Self;
            fn div(self, other: Self) -> Self {
                Self {
                    $($field: self.$field / other.$field,)+
                }
            }
        }

        impl<T> Div<T> for $VecN<T> where T: Number {
            type Output = Self;
            fn div(self, other: T) -> Self {
                Self {
                    $($field: self.$field / other,)+
                }
            }
        }

        // div refs
        impl<T> Div<&Self> for $VecN<T> where T: Number {
            type Output = Self;
            fn div(self, other: &Self) -> Self {
                self.div(*other)
            }
        }

        impl<T> Div<$VecN<T>> for &$VecN<T> where T: Number {
            type Output = $VecN<T>;
            fn div(self, other: $VecN<T>) -> $VecN<T> {
                *self / other
            }
        }

        impl<T> Div<Self> for &$VecN<T> where T: Number {
            type Output = $VecN<T>;
            fn div(self, other: Self) -> $VecN<T> {
                *self / other
            }
        }

        impl<T> DivAssign<Self> for $VecN<T> where T: Number {
            fn div_assign(&mut self, other: Self) {
                $(self.$field /= other.$field;)+
            }
        }

        impl<T> DivAssign<T> for $VecN<T> where T: Number {
            fn div_assign(&mut self, other: T) {
                $(self.$field /= other;)+
            }
        }

        // div assign ref
        impl<T> DivAssign<&Self> for $VecN<T> where T: Number {
            fn div_assign(&mut self, other: &Self) {
                $(self.$field /= other.$field;)+
            }
        }

        impl<T> Rem<Self> for $VecN<T> where T: Number {
            type Output = Self;
            fn rem(self, other: Self) -> Self {
                Self {
                    $($field: self.$field % other.$field,)+
                }
            }
        }

        impl<T> Rem<T> for $VecN<T> where T: Number {
            type Output = Self;
            fn rem(self, other: T) -> Self {
                Self {
                    $($field: self.$field % other,)+
                }
            }
        }

        impl<T> RemAssign<Self> for $VecN<T> where T: Number {
            fn rem_assign(&mut self, other: Self) {
                $(self.$field %= other.$field;)+
            }
        }

        impl<T> RemAssign<T> for $VecN<T> where T: Number {
            fn rem_assign(&mut self, other: T) {
                $(self.$field %= other;)+
            }
        }

        impl<T> Neg for $VecN<T> where T: SignedNumber {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self {
                    $($field: -self.$field,)+
                }
            }
        }

        impl<T> Eq for $VecN<T> where T: Eq  {}
        impl<T> PartialEq for $VecN<T> where T: PartialEq  {
            fn eq(&self, other: &Self) -> bool {
                $(self.$field == other.$field &&)+
                true
            }
        }
    }
}

/// macro to stamp out various typed c-style constructors. ie. let v = vec3f(0.0, 1.0, 0.0);
macro_rules! vec_ctor {
    ($VecN:ident { $($field:ident),+ }, $ctor:ident, $splat:ident, $t:ident) => {
        pub fn $ctor($($field: $t,)+) -> $VecN<$t> {
            $VecN {
                $($field,)+
            }
        }
        pub fn $splat(v: $t) -> $VecN<$t> {
            $VecN {
                $($field: v,)+
            }
        }
    }
}

/// macro to stamp out all arithmetic ops for lhs scalars
macro_rules! vec_scalar_lhs {
    ($VecN:ident { $($field:ident),+ }, $t:ident) => {
        impl Add<$VecN<$t>> for $t {
            type Output = $VecN<$t>;
            fn add(self, other: $VecN<$t>) -> $VecN<$t> {
                $VecN {
                    $($field: self + other.$field,)+
                }
            }
        }

        impl Sub<$VecN<$t>> for $t {
            type Output = $VecN<$t>;
            fn sub(self, other: $VecN<$t>) -> $VecN<$t> {
                $VecN {
                    $($field: self - other.$field,)+
                }
            }
        }

        impl Mul<$VecN<$t>> for $t {
            type Output = $VecN<$t>;
            fn mul(self, other: $VecN<$t>) -> $VecN<$t> {
                $VecN {
                    $($field: self * other.$field,)+
                }
            }
        }

        impl Div<$VecN<$t>> for $t {
            type Output = $VecN<$t>;
            fn div(self, other: $VecN<$t>) -> $VecN<$t> {
                $VecN {
                    $($field: self / other.$field,)+
                }
            }
        }

        impl Rem<$VecN<$t>> for $t {
            type Output = $VecN<$t>;
            fn rem(self, other: $VecN<$t>) -> $VecN<$t> {
                $VecN {
                    $($field: self % other.$field,)+
                }
            }
        }
    }
}

//
// From
//

/// constructs vec2 from scalar T splatting to x and y
impl<T> From<T> for Vec2<T> where T: Number {
    fn from(other: T) -> Vec2<T> {
        Vec2 {
            x: other,
            y: other,
        }
    }
}

// constructs vec2 from tuple of 2 scalars x: .0, y: .1
impl<T> From<(T, T)> for Vec2<T> where T: Number {
    fn from(other: (T, T)) -> Vec2<T> {
        Vec2 {
            x: other.0,
            y: other.1,
        }
    }
}

/// constructs vec2 from vec3 copying the x,y and truncating the z
impl<T> From<Vec3<T>> for Vec2<T> where T: Number {
    fn from(other: Vec3<T>) -> Vec2<T> {
        Vec2 {
            x: other.x,
            y: other.y,
        }
    }
}

/// constructs vec2 from vec4 copying the x,y and truncating the z,w
impl<T> From<Vec4<T>> for Vec2<T> where T: Number {
    fn from(other: Vec4<T>) -> Vec2<T> {
        Vec2 {
            x: other.x,
            y: other.y,
        }
    }
}

/// constructs vec3 from scalar T splatting to x,y,z
impl<T> From<T> for Vec3<T> where T: Number {
    fn from(other: T) -> Vec3<T> {
        Vec3 {
            x: other,
            y: other,
            z: other
        }
    }
}

/// constructs vec3 from vec2 copying the x,y and zeroing the z
impl<T> From<Vec2<T>> for Vec3<T> where T: Number {
    fn from(other: Vec2<T>) -> Vec3<T> {
        Vec3 {
            x: other.x,
            y: other.y,
            z: T::zero()
        }
    }
}

/// constructs vec3 from tuple of 3 scalars
impl<T> From<(T, T, T)> for Vec3<T> where T: Number {
    fn from(other: (T, T, T)) -> Vec3<T> {
        Vec3 {
            x: other.0,
            y: other.1,
            z: other.2
        }
    }
}

/// construct from a tuple of vec2 into x,y and scalar into z
impl<T> From<(Vec2<T>, T)> for Vec3<T> where T: Number {
    fn from(other: (Vec2<T>, T)) -> Vec3<T> {
        Vec3 {
            x: other.0.x,
            y: other.0.y,
            z: other.1
        }
    }
}

/// constructs vec3 from vec4 copying the x,y,z and truncating the w
impl<T> From<Vec4<T>> for Vec3<T> where T: Number {
    fn from(other: Vec4<T>) -> Vec3<T> {
        Vec3 {
            x: other.x,
            y: other.y,
            z: other.z,
        }
    }
}

/// constructs vec4 from scalar T splatting to x,y,z,w
impl<T> From<T> for Vec4<T> where T: Number {
    fn from(other: T) -> Vec4<T> {
        Vec4 {
            x: other,
            y: other,
            z: other,
            w: other
        }
    }
}

/// constructs vec4 from vec2 copying the x,y and zeroing the z,w
impl<T> From<Vec2<T>> for Vec4<T> where T: Number {
    fn from(other: Vec2<T>) -> Vec4<T> {
        Vec4 {
            x: other.x,
            y: other.y,
            z: T::zero(),
            w: T::zero()
        }
    }
}

/// constructs vec4 from vec2 copying the x,y,z and zeroing the w
impl<T> From<Vec3<T>> for Vec4<T> where T: Number {
    fn from(other: Vec3<T>) -> Vec4<T> {
        Vec4 {
            x: other.x,
            y: other.y,
            z: other.z,
            w: T::zero()
        }
    }
}

/// construct from a tuple of vec2 into x,y 2 scalars into z and w
impl<T> From<(Vec2<T>, T, T)> for Vec4<T> where T: Number {
    fn from(other: (Vec2<T>, T, T)) -> Vec4<T> {
        Vec4 {
            x: other.0.x,
            y: other.0.y,
            z: other.1,
            w: other.2
        }
    }
}

/// construct from a tuple of vec2 into x,y and vec2 into z,w
impl<T> From<(Vec2<T>, Vec2<T>)> for Vec4<T> where T: Number {
    fn from(other: (Vec2<T>, Vec2<T>)) -> Vec4<T> {
        Vec4 {
            x: other.0.x,
            y: other.0.y,
            z: other.1.x,
            w: other.1.y
        }
    }
}

/// construct from a tuple of vec3 into x,y,z and a scalar into w
impl<T> From<(Vec3<T>, T)> for Vec4<T> where T: Number {
    fn from(other: (Vec3<T>, T)) -> Vec4<T> {
        Vec4 {
            x: other.0.x,
            y: other.0.y,
            z: other.0.z,
            w: other.1
        }
    }
}

/// constructs vec4 from tuple of 4 scalars
impl<T> From<(T, T, T, T)> for Vec4<T> where T: Number {
    fn from(other: (T, T, T, T)) -> Vec4<T> {
        Vec4 {
            x: other.0,
            y: other.1,
            z: other.2,
            w: other.3
        }
    }
}

macro_rules! vec_cast {
    ($VecN:ident { $($field:ident),+ }, $t:ident, $u:ident) => {
        impl From<$VecN<$u>> for $VecN<$t> {
            fn from(other: $VecN<$u>) -> $VecN<$t> {
                $VecN {
                    $($field: other.$field as $t,)+
                }
            }
        }

        impl From<$VecN<$t>> for $VecN<$u> {
            fn from(other: $VecN<$t>) -> $VecN<$u> {
                $VecN {
                    $($field: other.$field as $u,)+
                }
            }
        }
    }
}

//
// Macro Decl
//

vec_impl!(Vec2 { x, 0, y, 1 }, 2, v2);
vec_impl!(Vec3 { x, 0, y, 1, z, 2 }, 3, v3);
vec_impl!(Vec4 { x, 0, y, 1, z, 2, w, 3 }, 4, v4);

#[cfg(feature = "lhs_scalar_vec_ops")]
vec_scalar_lhs!(Vec2 { x, y }, f32);

#[cfg(feature = "lhs_scalar_vec_ops")]
vec_scalar_lhs!(Vec3 { x, y, z }, f32);

#[cfg(feature = "lhs_scalar_vec_ops")]
vec_scalar_lhs!(Vec4 { x, y, z, w }, f32);

#[cfg(feature = "lhs_scalar_vec_ops")]
vec_scalar_lhs!(Vec2 { x, y }, f64);

#[cfg(feature = "lhs_scalar_vec_ops")]
vec_scalar_lhs!(Vec3 { x, y, z }, f64);

#[cfg(feature = "lhs_scalar_vec_ops")]
vec_scalar_lhs!(Vec4 { x, y, z, w }, f64);

#[cfg(feature = "lhs_scalar_vec_ops")]
vec_scalar_lhs!(Vec2 { x, y }, i32);

#[cfg(feature = "lhs_scalar_vec_ops")]
vec_scalar_lhs!(Vec3 { x, y, z }, i32);

#[cfg(feature = "lhs_scalar_vec_ops")]
vec_scalar_lhs!(Vec4 { x, y, z, w }, i32);

#[cfg(feature = "lhs_scalar_vec_ops")]
vec_scalar_lhs!(Vec2 { x, y }, u32);

#[cfg(feature = "lhs_scalar_vec_ops")]
vec_scalar_lhs!(Vec3 { x, y, z }, u32);

#[cfg(feature = "lhs_scalar_vec_ops")]
vec_scalar_lhs!(Vec4 { x, y, z, w }, u32);

#[cfg(feature = "short_hand_constructors")]
vec_ctor!(Vec2 { x, y }, vec2f, splat2f, f32);

#[cfg(feature = "short_hand_constructors")]
vec_ctor!(Vec3 { x, y, z }, vec3f, splat3f, f32);

#[cfg(feature = "short_hand_constructors")]
vec_ctor!(Vec4 { x, y, z, w }, vec4f, splat4f, f32);

#[cfg(feature = "short_hand_constructors")]
vec_ctor!(Vec2 { x, y }, vec2d, splat2d, f64);

#[cfg(feature = "short_hand_constructors")]
vec_ctor!(Vec3 { x, y, z }, vec3d, splat3d, f64);

#[cfg(feature = "short_hand_constructors")]
vec_ctor!(Vec4 { x, y, z, w }, vec4d, splat4d, f64);

#[cfg(feature = "short_hand_constructors")]
vec_ctor!(Vec2 { x, y }, vec2i, splat2i, i32);

#[cfg(feature = "short_hand_constructors")]
vec_ctor!(Vec3 { x, y, z }, vec3i, splat3i, i32);

#[cfg(feature = "short_hand_constructors")]
vec_ctor!(Vec4 { x, y, z, w }, vec4i, splat4i, i32);

#[cfg(feature = "short_hand_constructors")]
vec_ctor!(Vec2 { x, y }, vec2u, splat2u, u32);

#[cfg(feature = "short_hand_constructors")]
vec_ctor!(Vec3 { x, y, z }, vec3u, splat3u, u32);

#[cfg(feature = "short_hand_constructors")]
vec_ctor!(Vec4 { x, y, z, w }, vec4u, splat4u, u32);

#[cfg(feature = "casts")]
vec_cast!(Vec2 { x, y }, f64, i32);

#[cfg(feature = "casts")]
vec_cast!(Vec2 { x, y }, f64, u32);

#[cfg(feature = "casts")]
vec_cast!(Vec2 { x, y }, f32, f64);

#[cfg(feature = "casts")]
vec_cast!(Vec2 { x, y }, f32, i32);

#[cfg(feature = "casts")]
vec_cast!(Vec2 { x, y }, f32, u32);

#[cfg(feature = "casts")]
vec_cast!(Vec2 { x, y }, i32, u32);

#[cfg(feature = "casts")]
vec_cast!(Vec3 {x, y, z}, f64, i32);

#[cfg(feature = "casts")]
vec_cast!(Vec3 {x, y, z}, f64, u32);

#[cfg(feature = "casts")]
vec_cast!(Vec3 {x, y, z}, f32, f64);

#[cfg(feature = "casts")]
vec_cast!(Vec3 {x, y, z}, f32, i32);

#[cfg(feature = "casts")]
vec_cast!(Vec3 {x, y, z}, f32, u32);

#[cfg(feature = "casts")]
vec_cast!(Vec3 {x, y, z}, i32, u32);

#[cfg(feature = "casts")]
vec_cast!(Vec4 {x, y, z, w}, f64, i32);

#[cfg(feature = "casts")]
vec_cast!(Vec4 {x, y, z, w}, f64, u32);

#[cfg(feature = "casts")]
vec_cast!(Vec4 {x, y, z, w}, f32, f64);

#[cfg(feature = "casts")]
vec_cast!(Vec4 {x, y, z, w}, f32, i32);

#[cfg(feature = "casts")]
vec_cast!(Vec4 {x, y, z, w}, f32, u32);

#[cfg(feature = "casts")]
vec_cast!(Vec4 {x, y, z, w}, i32, u32);