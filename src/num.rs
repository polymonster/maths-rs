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

macro_rules! number_trait_impl {
    ($($func:ident),*) => {
        /// base number trait for signed or unsigned, floating point or integer numbers.
        pub trait Number: 
            Copy + Default + Display +
            Add<Output=Self> + AddAssign +
            Sub<Output=Self> + SubAssign + 
            Mul<Output=Self> + MulAssign + 
            Div<Output=Self> + DivAssign +
            Rem<Output=Self> + RemAssign +
            PartialEq + PartialOrd {
                fn zero() -> Self;
                fn one() -> Self;
                fn two() -> Self;
                fn min(a: Self, b: Self) -> Self;
                fn max(a: Self, b: Self) -> Self;
                fn min_value() -> Self;
                fn max_value() -> Self;
                fn step(a: Self, b: Self) -> Self;
        }
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
        impl Number for $t {
            $(
                fn $func(v: Self) -> Self {
                    v.$func()
                }
            )*

            fn min(a: Self, b: Self) -> Self {
                a.min(b)
            }

            fn max(a: Self, b: Self) -> Self {
                a.max(b)
            }

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
            $(fn $func(v: Self) -> Self;)*
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
            $(
                fn $func(v: Self) -> Self {
                    v.$func()
                }
            )*
            fn minus_one() -> Self {
                $minus_one
            }
        }
    }
}

macro_rules! float_trait_impl {
    ($($func:ident),*) => {
        /// floating point trait for various levels of fp precision
        pub trait Float: SignedNumber {
            $(fn $func(v: Self) -> Self;)*
            /// returns true if a and b are approximately equal within the designated epsilon
            fn approx(a: Self, b: Self, eps: Self) -> bool;
            /// fused multiply add, m * a + b
            fn mad(m: Self, a: Self, b: Self) -> Self;
            /// checks if value isnan
            fn is_nan(v: Self) -> bool;
            /// checks if value is infinite
            fn is_infinite(v: Self) -> bool;
            /// checks if value is not inf
            fn is_finite(v: Self) -> bool;
            /// performs linear interpolation between e0 and e1, t specifies the ratio to interpolate between the values
            fn lerp(e0: Self, e1: Self, t: Self) -> Self;
            /// performs hermite interpolation between e0 and e1 by t
            fn smoothstep(e0: Self, e1: Self, t: Self) -> Self;
            /// raise v to power of integer exponent 
            fn powi(v: Self, exp: i32) -> Self;
            /// raise v to power of float exponent 
            fn powf(v: Self, exp: Self) -> Self;
            /// floating point remainder of x / y
            fn fmod(x: Self, y: Self) -> Self;
            /// extract fractional (decimal) part of
            fn frac(v: Self) -> Self;
            /// truncate v to integer
            fn trunc(v: Self) -> Self;
            /// split v into fractional and integer part
            fn modf(v: Self) -> (Self, Self);
            /// returns the logarithm of the number with respect to an arbitrary base.
            fn log(v: Self, base: Self) -> Self;
            /// returns (sin(v), cos(v))
            fn sin_cos(v: Self) -> (Self, Self);
            /// atan2 computes arctan of y and x
            fn atan2(y: Self, x: Self) -> Self;
        }
        float_impl!(f64 { $($func),* });
        float_impl!(f32 { $($func),* });
    }
}

macro_rules! float_impl {
    ($t:ident { $($func:ident),* } ) => {
        impl Float for $t {
            $(
                fn $func(v: Self) -> Self {
                    v.$func()
                }
            )*

            fn approx(a: Self, b: Self, eps: Self) -> bool {
                Self::abs(a - b) < eps
            }

            fn mad(m: Self, a: Self, b: Self) -> Self {
                m.mul_add(a, b)
            }

            fn is_nan(v: Self) -> bool {
                v.is_nan()
            }

            fn is_infinite(v: Self) -> bool {
                v.is_infinite()
            }

            fn is_finite(v: Self) -> bool {
                v.is_finite()
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

            fn powi(v: Self, exp: i32) -> Self {
                v.powi(exp)
            }
            
            fn powf(v: Self, exp: Self) -> Self {
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
        }
    }
}

macro_rules! integer_trait_impl {
    ($($func:ident),*) => {
        /// integer point trait for various sized integers
        pub trait Integer: Number {
            fn pow(v: Self, exp: u32) -> Self;
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