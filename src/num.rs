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
use std::ops::Shl;
use std::ops::ShlAssign;
use std::ops::Shr;
use std::ops::ShrAssign;
use std::ops::BitOr;
use std::ops::BitOrAssign;
use std::ops::BitAnd;
use std::ops::BitAndAssign;
use std::ops::BitXor;
use std::ops::BitXorAssign;

use std::cmp::PartialEq;
use std::cmp::PartialOrd;

use std::fmt::Display;

/// base trait for scalar and vector numerical operations, arithmetic and generic constants 
pub trait Base<T: Number>:
    Copy + Display +
    Add<Output=Self> + AddAssign +
    Sub<Output=Self> + SubAssign + 
    Mul<Output=Self> + MulAssign + 
    Div<Output=Self> + DivAssign +
    Rem<Output=Self> + RemAssign
    where Self: Sized {
    /// returns `0`
    fn zero() -> Self;
    /// returns `1`
    fn one() -> Self;
    /// returns `2`
    fn two() -> Self;
    /// returns `3`
    fn three() -> Self;
    /// returns `4`
    fn four() -> Self;
    /// returns the smallest representable number with the available precision
    fn min_value() -> Self;
    /// returns the largest representable number with the available precision
    fn max_value() -> Self;
}

/// operations applicable to both floating point, integer and unsigned types
pub trait NumberOps<T: Number> {
    /// returns the minimum value of `a` and `b`
    fn min(a: Self, b: Self) -> Self;
    /// returns the maximum value of `a` and `b`
    fn max(a: Self, b: Self) -> Self;
    /// returns value `x` clamped to the range `min` - `max`
    fn clamp(x: Self, min: Self, max: Self) -> Self;
    /// returns a vector stepped component wise; `1` if `a` is >= `b`, `0` otherwise
    fn step(a: Self, b: Self) -> Self;
}

/// operations applicable to signed types
pub trait SignedNumberOps<T: SignedNumber>: Neg<Output=Self> {
    /// returns sign value of `a`; `-1` = negative, `1` = positive or `0` (integers only)
    fn signum(a: Self) -> Self;
    /// returns the absolute (postive) value of `a`
    fn abs(a: Self) -> Self;
}

/// operations applicable to integer types
pub trait IntegerOps<T: Integer> {
    /// returns value `a` raised to unsigned integer power `exp`
    fn pow(a: Self, exp: u32) -> Self;
}

/// trait for performing linear interpolation on scalars, vectors or quaternions
pub trait Lerp<T: Float> {
    /// returns linear interpolation of `t` between `e0` and `e1`, `t` specifies the ratio to interpolate between the values
    fn lerp(e0: Self, e1: Self, t: T) -> Self;
}

pub trait Cast<T: Number> where Self: Sized {
    fn from_f32(v: f32) -> Self;
    fn from_f64(v: f64) -> Self;
    fn from_u32(v: u32) -> Self;
    fn from_i32(v: i32) -> Self;
    fn from_u64(v: u64) -> Self;
    fn from_i64(v: i64) -> Self;
    fn from_usize(v: usize) -> Self;
    fn as_f32(&self) -> f32;
    fn as_f64(&self) -> f64;
    fn as_u32(&self) -> u32;
    fn as_i32(&self) -> i32;
    fn as_u64(&self) -> u64;
    fn as_i64(&self) -> i64;
    fn as_usize(&self) -> usize;
}

/// operations applicable to floating point types
pub trait FloatOps<T: Float>: Lerp<T> where Self: Sized {
    /// returns `0.5`
    fn point_five() -> Self;
    /// returns `pi`
    fn pi() -> Self;
    /// returns `2.0 * pi`
    fn two_pi() -> Self;
    /// returns `1.0 / pi`
    fn inv_pi() -> Self;
    /// returns `phi` (the golden constant)
    fn phi() -> Self;
    /// returns `1.0 / phi` (the golden constant)
    fn inv_phi() -> Self;
    /// returns square root of `a`
    fn sqrt(a: Self) -> Self;
    /// returns reciprocal square root of `a` (`1/sqrt(a)`)
    fn rsqrt(a: Self) -> Self;
    /// returns the reciprocal of `a`
    fn recip(a: Self) -> Self;
    /// returns `a` raised to integer power `exp`
    fn powi(a: Self, exp: i32) -> Self;
    /// returns `a` raised to float power `exp`
    fn powf(a: Self, exp: T) -> Self;
    /// returns fused multiply add `a * m + b`
    fn mad(m: Self, a: Self, b: Self) -> Self;
    /// returns true if `a` and `b` are approximately equal within the designated epsilon `eps`
    fn approx(a: Self, b: Self, eps: T) -> bool;
    /// returns the greatest integer which is less than or equal to a
    fn floor(a: Self) -> Self;
    /// returns the smallest integer which is greater than or equal to `a`
    fn ceil(a: Self) -> Self;
    /// returns value `a` with the same sign as the second parameter `sign`
    fn copysign(a: Self, sign: T) -> Self;
    /// returns hermite interpolation between `0-1` of `t` between edges `e0` and `e1`
    fn smoothstep(e0: Self, e1: Self, t: T) -> Self;
    /// returns `a` rounded component wise
    fn round(a: Self) -> Self;
    /// returns true if `a` is not a number (`nan`)
    fn is_nan(a: Self) -> Self;
    /// returns true if `a` is `inf`
    fn is_infinite(a: Self) -> Self;
    /// returns true is `a` is finite
    fn is_finite(a: Self) -> Self;
    /// returns the value of `a` saturated (clamped between `0-1`). equivalent to `clamp(x, 0, 1)`
    fn saturate(x: Self) -> Self;
    /// returns `theta` converted from degrees to radians
    fn deg_to_rad(theta: Self) -> Self;
    /// returns `theta` converted from radians to degrees
    fn rad_to_deg(theta: Self) -> Self;
    /// returns the floating-point remainder of `x/y`
    fn fmod(x: Self, y: Self) -> Self;
    /// returns the fractional part of floating point number `v` removing the integer part
    fn frac(v: Self) -> Self;
    /// returns the integer part of float value `v` truncating the decimal part
    fn trunc(v: Self) -> Self;
    /// returns a tuple containing `(frac(v), trunc(v))` breaking the float into 2 parts 
    fn modf(v: Self) -> (Self, Self);
    /// returns the cosine of `v` where the value `v` is in radians
    fn cos(v: Self) -> Self;
    /// returns the sine of `v` where the value `v` is in radians
    fn sin(v: Self) -> Self;
    /// returns the tangent of `v` where the value `v` is in radians
    fn tan(v: Self) -> Self;
    /// returns the arc cosine of `v` where the value `v` is in radians
    fn acos(v: Self) -> Self;
    /// returns the arc sine of `v` where the value `v` is in radians
    fn asin(v: Self) -> Self;
    /// returns the arc tangent of `v` where the value `v` is in radians
    fn atan(v: Self) -> Self;
    /// returns the hyperbolic cosine of `v` where the value `v` is in radians
    fn cosh(v: Self) -> Self;
    /// returns the hyperbolic sine of `v` where the value `v` is in radians
    fn sinh(v: Self) -> Self;
    /// returns the hyperbolic tangent of `v` where the value `v` is in radians
    fn tanh(v: Self) -> Self;
    /// returns a tuple of `(sin(v), cos(v))` of `v` where the value `v` is in radians
    fn sin_cos(v: Self) -> (Self, Self);
    // returns the value of the arc tangent of `y/x`, expressed in radians
    fn atan2(y: Self, x: Self) -> Self;
    // returns the base-e exponential function of `v`, which is `e` raised to the power `v`
    fn exp(v: Self) -> Self;
    /// returns 2 raised to the given power `v`
    fn exp2(v: Self) -> Self;
    /// returns the binary (base-2) logarithm of `v`
    fn log2(v: Self) -> Self;
    /// returns the common (base-10) logarithm of `v`
    fn log10(v: Self) -> Self;
    /// returns the logarithm of `v` in base
    fn log(v: Self, base: T) -> Self;
}

macro_rules! number_trait_impl {
    ($($func:ident),*) => {
        /// base number trait for signed or unsigned, floating point or integer numbers.
        pub trait Number: Base<Self> + Default + PartialEq + PartialOrd  {}
        number_impl!(f64 { $($func),* }, 0.0, 1.0);
        number_impl!(f32 { $($func),* }, 0.0, 1.0);
        number_impl!(usize { $($func),* }, 0, 1);
        number_impl!(isize { $($func),* }, 0, 1);
        number_impl!(i64 { $($func),* }, 0, 1);
        number_impl!(u64 { $($func),* }, 0, 1);
        number_impl!(i32 { $($func),* }, 0, 1);
        number_impl!(u32 { $($func),* }, 0, 1);
        number_impl!(i16 { $($func),* }, 0, 1);
        number_impl!(u16 { $($func),* }, 0, 1);
        number_impl!(i8 { $($func),* }, 0, 1);
        number_impl!(u8 { $($func),* }, 0, 1);
    }
}

macro_rules! number_impl {
    ($t:ident { $($func:ident),* }, $zero:literal, $one:literal) => {
        impl Base<$t> for $t {
            fn min_value() -> Self {
                $t::MIN
            }

            fn max_value() -> Self {
                $t::MAX
            }

            fn zero() -> Self {
                $zero
            }

            fn one() -> Self {
                $one
            }

            fn two() -> Self {
                2 as Self
            }

            fn three() -> Self {
                3 as Self
            }

            fn four() -> Self {
                4 as Self
            }

        }

        impl Number for $t {}

        impl NumberOps<$t> for $t {
            fn min(a: Self, b: Self) -> Self {
                a.min(b)
            }

            fn max(a: Self, b: Self) -> Self {
                a.max(b)
            }

            fn clamp(x: Self, min: Self, max: Self) -> Self {
                NumberOps::max(NumberOps::min(x, max), min)
            }

            fn step(a: Self, b: Self) -> Self {
                if a >= b {
                    Self::one()
                } 
                else {
                    Self::zero()
                }
            }
        }

        impl Cast<$t> for $t {
            fn from_f32(v: f32) -> Self {
                v as Self 
            }

            fn from_f64(v: f64) -> Self {
                v as Self 
            }

            fn from_u32(v: u32) -> Self {
                v as Self 
            }

            fn from_i32(v: i32) -> Self {
                v as Self 
            }

            fn from_u64(v: u64) -> Self {
                v as Self 
            }

            fn from_i64(v: i64) -> Self {
                v as Self 
            }

            fn from_usize(v: usize) -> Self {
                v as Self 
            }

            fn as_f32(&self) -> f32 {
                *self as f32
            }

            fn as_f64(&self) -> f64 {
                *self as f64
            }

            fn as_u32(&self) -> u32 {
                *self as u32
            }

            fn as_i32(&self) -> i32 {
                *self as i32
            }

            fn as_u64(&self) -> u64 {
                *self as u64
            }
            
            fn as_i64(&self) -> i64 {
                *self as i64
            }

            fn as_usize(&self) -> usize {
                *self as usize
            }
        }
    }
}

macro_rules! signed_number_trait_impl {
    ($($func:ident),*) => {
        /// signed number trait for signed integers or floats.
        pub trait SignedNumber: Number + Neg<Output=Self> {
            fn minus_one() -> Self;
        }
        signed_number_impl!(f64 { $($func),* }, -1.0);
        signed_number_impl!(f32 { $($func),* }, -1.0);
        signed_number_impl!(i64 { $($func),* }, -1);
        signed_number_impl!(i32 { $($func),* }, -1);
        signed_number_impl!(i16 { $($func),* }, -1);
        signed_number_impl!(i8 { $($func),* }, -1);
    }
}

macro_rules! signed_number_impl {
    ($t:ident { $($func:ident),* }, $minus_one:literal) => {
        impl SignedNumber for $t {
            fn minus_one() -> Self {
                $minus_one
            }
        }

        impl SignedNumberOps<$t> for $t {
            $(
                fn $func(v: Self) -> Self {
                    v.$func()
                }
            )*
        }
    }
}

macro_rules! float_trait_impl {
    ($($func:ident),*) => {
        /// floating point trait for various levels of fp precision
        pub trait Float: SignedNumber {
            fn small_epsilon() -> Self;
        }
        float_impl!(f64 { $($func),* });
        float_impl!(f32 { $($func),* });
    }
}

macro_rules! float_impl {
    ($t:ident { $($func:ident),* } ) => {
        impl Float for $t {
            fn small_epsilon() -> Self {
                1e-30
            }
        }

        impl Lerp<$t> for $t {
            fn lerp(e0: Self, e1: Self, t: Self) -> Self {
                e0 + t * (e1 - e0)
            }
        }

        impl FloatOps<$t> for $t {
            $(
                fn $func(v: Self) -> Self {
                    v.$func()
                }
            )*

            fn point_five() -> Self {
                0.5 as Self
            }

            #[allow(clippy::excessive_precision)] 
            fn pi() -> Self {
                3.14159265358979323846264338327950288 as Self
            }

            fn two_pi() -> Self {
                2.0 as Self * Self::pi() as Self
            }

            fn inv_pi() -> Self {
                1.0 as Self / Self::pi() as Self
            }

            #[allow(clippy::excessive_precision)] 
            fn phi() -> Self {
                1.618033988749894 as Self
            }

            fn inv_phi() -> Self {
                1.0 as Self / Self::phi() as Self
            }

            fn rsqrt(a: Self) -> Self {
                Self::one()/Self::sqrt(a)
            }

            fn approx(a: Self, b: Self, eps: Self) -> bool {
                Self::abs(a - b) < eps
            }

            fn mad(m: Self, a: Self, b: Self) -> Self {
                m.mul_add(a, b)
            }

            fn is_nan(v: Self) -> $t {
                if v.is_nan() { $t::one() } else { $t::zero() }
            }

            fn is_infinite(v: Self) -> $t {
                if v.is_infinite() { $t::one() } else { $t::zero() }
            }

            fn is_finite(v: Self) -> $t {
                if v.is_finite() { $t::one() } else { $t::zero() }
            }

            fn copysign(a: Self, sign: $t) -> Self {
                a.copysign(sign)
            }

            fn smoothstep(e0: Self, e1: Self, t: Self) -> Self {
                if t < e0 { return Self::zero(); }
                if (t >= e1) { return Self::one(); }
                let x = (t - e0) / (e1 - e0);
                x * x * (3 as Self - 2 as Self * x)
            }

            fn saturate(v: Self) -> Self {
                Self::max(Self::min(v, 1.0), 0.0)
            }

            fn powi(v: Self, exp: i32) -> Self {
                v.powi(exp)
            }
            
            fn powf(v: Self, exp: $t) -> Self {
                v.powf(exp)
            }

            fn fmod(x: Self, y: Self) -> Self {
                x % y
            }

            fn frac(v: Self) -> Self {
                v.fract()
            }

            fn trunc(v: Self) -> Self {
                v.trunc()
            }

            fn modf(v: Self) -> (Self, Self) {
                (Self::frac(v), Self::trunc(v))
            }

            fn log(v: Self, base: Self) -> Self {
                v.log(base)
            }

            fn sin_cos(v: Self) -> (Self, Self) {
                (v.sin(), v.cos())
            }

            fn atan2(y: Self, x: Self) -> Self {
                y.atan2(x)
            }

            fn deg_to_rad(theta: Self) -> Self {
                theta.to_radians()
            }

            fn rad_to_deg(theta: Self) -> Self {
                theta.to_degrees()
            }
        }
    }
}

macro_rules! integer_trait_impl {
    ($($func:ident),*) => {
        /// integer point trait for various sized integers
        pub trait Integer: Number + 
            Shl<Output=Self> + ShlAssign +
            Shr<Output=Self> + ShrAssign + 
            BitOr<Output=Self> + BitOrAssign +
            BitAnd<Output=Self> + BitAndAssign + 
            BitXor<Output=Self> + BitXorAssign { 
        }
        integer_impl!(i8 { $($func),* });
        integer_impl!(u8 { $($func),* });
        integer_impl!(i16 { $($func),* });
        integer_impl!(u16 { $($func),* });
        integer_impl!(i32 { $($func),* });
        integer_impl!(u32 { $($func),* });
        integer_impl!(i64 { $($func),* });
        integer_impl!(u64 { $($func),* });
    }
}

macro_rules! integer_impl {
    ($t:ident { $($func:ident),* } ) => {
        impl Integer for $t {
        }

        impl IntegerOps<$t> for $t {
            fn pow(v: Self, exp: u32) -> Self {
                v.pow(exp)
            }
        }
    }
}

number_trait_impl!();
integer_trait_impl!();

signed_number_trait_impl!(signum, abs);
float_trait_impl!(
    floor, ceil, round, sqrt, recip,
    cos, sin, tan, acos, asin, atan, cosh, sinh, tanh,
    exp, exp2, log2, log10
);