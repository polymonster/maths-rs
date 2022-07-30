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
    /// returns 0
    fn zero() -> Self;
    /// returns 1
    fn one() -> Self;
    /// returns 2
    fn two() -> Self;
    /// returns 3
    fn three() -> Self;
    /// returns 4
    fn four() -> Self;
    /// returns the smallest representable number with the available precision
    fn min_value() -> Self;
    /// returns the largest representable number with the available precision
    fn max_value() -> Self;
}

pub trait NumberOps<T: Number> {
    /// returns the minimum value of a and b
    fn min(a: Self, b: Self) -> Self;
    /// returns the maximum value of a and b
    fn max(a: Self, b: Self) -> Self;

    /// returns a vector with elements of x clamped component wise to min and max
    fn clamp(x: Self, min: Self, max: Self) -> Self;
    /// returns a vector stepped component wise; 1 if a is >= b, 0 otherwise
    fn step(a: Self, b: Self) -> Self;
}

pub trait SignedNumberOps<T: SignedNumber>: Neg<Output=Self> {
    /// returns sign value of a; -1 = negative, 1 = positive or 0 (integers only)
    fn signum(a: Self) -> Self;
    /// returns the absolute (postive) value of a
    fn abs(a: Self) -> Self;
}

pub trait IntegerOps<T: Integer> {
    /// returns vector with component-wise values raised to unsigned integer power
    fn pow(a: Self, exp: u32) -> Self;
}

pub trait FloatOps<T: Float> where Self: Sized {
    /// returns 0.5
    fn point_five() -> Self;
    /// returns pi
    fn pi() -> Self;
    /// returns square root of a
    fn sqrt(a: Self) -> Self;
    /// returns reciprocal square root of a (1/sqrt(a))
    fn rsqrt(a: Self) -> Self;
    /// returns the reciprocal of a
    fn recip(a: Self) -> Self;
    /// returns a raised to integer power
    fn powi(a: Self, exp: i32) -> Self;
    /// returns a raised to float power
    fn powf(a: Self, exp: T) -> Self;
    /// returns fused multiply add a * m + b
    fn mad(m: Self, a: Self, b: Self) -> Self;
    /// returns true if a and b are approximately equal within the designated epsilon
    fn approx(a: Self, b: Self, eps: T) -> bool;
    /// returns the greatest integer which is less than or equal to a
    fn floor(a: Self) -> Self;
    /// returns the smallest integer which is greater than or equal to a
    fn ceil(a: Self) -> Self;
    /// returns linear interpolation of t between e0 and e1, t specifies the ratio to interpolate between the values
    fn lerp(e0: Self, e1: Self, t: T) -> Self;
    /// returns hermite interpolation between 0-1 of t between edges e0 and e1
    fn smoothstep(e0: Self, e1: Self, t: T) -> Self;
    /// returns a rounded component wise
    fn round(a: Self) -> Self;
    /// returns true if a is not a number
    fn is_nan(a: Self) -> Self;
    /// returns true if a is inf
    fn is_infinite(a: Self) -> Self;
    /// returns true is a is finite
    fn is_finite(a: Self) -> Self;
    /// returns the value of a saturated (clamped between 0-1). equivalent to clamp (x, 0, 1)
    fn saturate(x: Self) -> Self;
    /// returns theta converted from degrees to radians
    fn deg_to_rad(theta: Self) -> Self;
    /// returns theta converted from radians to degrees
    fn rad_to_deg(theta: Self) -> Self;
    fn fmod(x: Self, y: Self) -> Self;
    fn frac(v: Self) -> Self;
    fn trunc(v: Self) -> Self;
    fn modf(v: Self) -> (Self, Self);
    fn cos(v: Self) -> Self;
    fn sin(v: Self) -> Self;
    fn tan(v: Self) -> Self;
    fn acos(v: Self) -> Self;
    fn asin(v: Self) -> Self;
    fn atan(v: Self) -> Self;
    fn cosh(v: Self) -> Self;
    fn sinh(v: Self) -> Self;
    fn tanh(v: Self) -> Self;
    fn sin_cos(v: Self) -> (Self, Self);
    fn atan2(y: Self, x: Self) -> Self;
    fn exp(v: Self) -> Self;
    fn exp2(v: Self) -> Self;
    fn log2(v: Self) -> Self;
    fn log10(v: Self) -> Self;
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

            fn lerp(e0: Self, e1: Self, t: Self) -> Self {
                e0 + t * (e1 - e0)
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
        pub trait Integer: Number { 
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

signed_number_trait_impl!(signum, abs);
number_trait_impl!();

float_trait_impl!(
    floor, ceil, round, sqrt, recip,
    cos, sin, tan, acos, asin, atan, cosh, sinh, tanh,
    exp, exp2, log2, log10
);

integer_trait_impl!();