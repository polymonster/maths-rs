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
pub trait VecN<T: Number>: Index<usize, Output=T> + IndexMut<usize> {
    /// returns the count of elements in the vector
    fn len() -> usize;
    /// returns true if all elements in vector are non-zero
    fn all(a: Self) -> bool;
    /// returns true if any element of the vector is non-zero
    fn any(a: Self) -> bool;
    /// returns scalar value which is the vector dot product a . b
    fn dot(a: Self, b: Self) -> T;
}

pub trait VecFloatOps<T: Float> {
    /// returns scalar magnitude or length of vector
    fn length(a: Self) -> T;
    /// returns scalar magnitude or length of vector
    fn mag(a: Self) -> T;
    /// returns scalar magnitude or length of vector squared to avoid using sqrt
    fn mag2(a: Self) -> T;
    /// returns a normalized unit vector of a
    fn normalize(a: Self) -> Self;
    /// returns scalar distance between 2 points (magnitude of the vector between the 2 points)
    fn distance(a: Self, b: Self) -> T;
    /// returns scalar distance between 2 points (magnitude of the vector between the 2 points)
    fn dist(a: Self, b: Self) -> T;
    /// returns scalar squared distance between 2 points to avoid using sqrt
    fn dist2(a: Self, b: Self) -> T;
    /// returns a reflection vector using an incident ray and a surface normal
    fn reflect(i: Self, n: Self) -> Self;
    /// returns a refraction vector using an entering ray, a surface normal, and a refraction index
    fn refract(i: Self, n: Self, eta: T) -> Self;
}

pub trait IntegerOps<T: Integer, Exp> {
    /// returns vector with component-wise values raised to unsigned integer power
    fn pow(a: Self, exp: Exp) -> Self;
}

pub trait NumberOps<T: Number> {
    /// returns a vector containing component wise min of a and b
    fn min(a: Self, b: Self) -> Self;
    /// returns a vector containing component wise max of a and b
    fn max(a: Self, b: Self) -> Self;
    /// returns a vector with elements of x clamped component wise to min and max
    fn clamp(x: Self, min: Self, max: Self) -> Self;
    /// returns a vector stepped component wise; 1 if a is >= b, 0 otherwise
    fn step(a: Self, b: Self) -> Self;
}

pub trait SignedNumberOps<T: SignedNumber> {
    /// returns component wise sign value; -1 = negative, 1 = positive or 0 (integers only)
    fn sign(a: Self) -> Self;
    /// returns component wise sign value; -1 = negative, 1 = positive or 0 (integers only)
    fn signum(a: Self) -> Self;
    /// returns a omponent wise vector containing the absolute (postive) value of a
    fn abs(a: Self) -> Self;
}

pub trait FloatOps<T: Float, Exp, Tuple> {
    /// returns vector with component-wise square root
    fn sqrt(a: Self) -> Self;
    /// returns vector with component-wise reciprocal square root (1/sqrt(a))
    fn rsqrt(a: Self) -> Self;
    /// returns vector with component-wise reciprocal
    fn recip(a: Self) -> Self;
    /// returns vector with component-wise values raised to integer power
    fn powi(a: Self, exp: Exp) -> Self;
    /// returns vector with component-wise values raised to float power
    fn powf(a: Self, exp: Self) -> Self;
    /// returns vector with fused multiply add component wise
    fn mad(m: Self, a: Self, b: Self) -> Self;
    /// returns true if all elements in vectors a and b are approximately equal within the designated epsilon
    fn approx(a: Self, b: Self, eps: T) -> bool;
    /*
    /// returns the greatest integer which is less than or equal to each vector element component wise
    fn floor(a: Self) -> Self;
    /// returns the smallest integer which is greater than or equal to each vector element component wise
    fn ceil(a: Self) -> Self;
    /// performs linear interpolation between e0 and e1, t specifies the ratio to interpolate between the values
    fn lerpn(e0: Self, e1: Self, t: Self) -> Self;
    /// performs linear interpolation between e0 and e1, t specifies the ratio to interpolate between the values
    fn lerp(e0: Self, e1: Self, t: T) -> Self;
    /// returns vector with component wise hermite interpolation between 0-1
    fn smoothstep(e0: Self, e1: Self, t: T) -> Self;
    /// returns vector with component wise hermite interpolation between 0-1
    fn smoothstepn(e0: Self, e1: Self, t: Self) -> Self;
    /// returns vector with values from a rounded component wise
    fn round(a: Self) -> Self;
    /// returns true if a is not a number
    fn is_nan(a: Self) -> ComponentBool;
    /// returns true if a is inf
    fn is_infinite(a: Self) -> ComponentBool;
    /// returns true is a is finite
    fn is_finite(a: Self) -> ComponentBool;
    /// returns a vector with saturated elements clamped between 0-1. equivalent to clamp (x, 0, 1)
    fn saturate(x: Self) -> Self;
    */
    /*
    fn fmod(x: Self, y: Self) -> Self;
    fn frac(v: Self) -> Self;
    fn trunc(v: Self) -> Self;
    fn modf(v: Self) -> Tuple;
    fn cos(v: Self) -> Self;
    fn sin(v: Self) -> Self;
    fn tan(v: Self) -> Self;
    fn acos(v: Self) -> Self;
    fn asin(v: Self) -> Self;
    fn atan(v: Self) -> Self;
    fn cosh(v: Self) -> Self;
    fn sinh(v: Self) -> Self;
    fn tanh(v: Self) -> Self;
    fn sin_cos(v: Self) -> Tuple;
    fn atan2(y: Self, x: Self) -> Self;
    fn exp(v: Self) -> Self;
    fn exp2(v: Self) -> Self;
    fn log2(v: Self) -> Self;
    fn log10(v: Self) -> Self;
    fn log(v: Self, base: T) -> Self;
    */
}

// 
// Macro Implementation
//

/// macro to stamp out various n-dimensional vectors, all of their ops and functions
macro_rules! vec_impl {
    ($VecN:ident { $($field:ident, $field_index:expr),* }, $len:expr, $module:ident) => {
        #[derive(Debug, Copy, Clone)]
        pub struct $VecN<T> {
            $(pub $field: T,)+
        }

        /// Vec4::<f32>::new(1.0, 2.0, 3.0, 4.0) or with abbreviated type Vec4f::new(1.0, 2.0, 3.0, 4.0)
        impl<T> $VecN<T> where T: Number {
            pub fn new($($field: T,)+) -> $VecN<T> {
                $VecN {
                    $($field: $field,)+
                }
            }

            /// returns a vector with all members set to 0
            pub fn zero() -> $VecN<T> {
                $VecN {
                    $($field: T::zero(),)+
                }
            }

            /// returns a vector with all members set to 1
            pub fn one() -> $VecN<T> {
                $VecN {
                    $($field: T::one(),)+
                }
            }

            /// returns a vector initialised as a unit vector in the x-axis [1, 0, 0, 0]
            pub fn unit_x() -> $VecN<T> {
                let v = [T::one(), T::zero(), T::zero(), T::zero()];
                Self {
                    $($field: v[$field_index],)+
                }
            }

            /// returns a vector initialised as a unit vector in the y-axis [0, 1, 0, 0]
            pub fn unit_y() -> $VecN<T> {
                let v = [T::zero(), T::one(), T::zero(), T::zero()];
                Self {
                    $($field: v[$field_index],)+
                }
            }

            /// returns a vector initialised as a unit vector in the z-axis [0, 0, 1, 0] value will be truncated to 0 for vectors < 3 dimension
            pub fn unit_z() -> $VecN<T> {
                let v = [T::zero(), T::zero(), T::one(), T::zero()];
                Self {
                    $($field: v[$field_index],)+
                }
            }

            /// returns a vector initialised as a unit vector in the w-axis [0, 0, 0, 1] value will be truncated to 0 for vectors < 4 dimension
            pub fn unit_w() -> $VecN<T> {
                let v = [T::zero(), T::zero(), T::zero(), T::one()];
                Self {
                    $($field: v[$field_index],)+
                }
            }

            /// returns a vector initialised to red [1, 0, 0, 1]
            pub fn red() -> $VecN<T> {
                let v = [T::one(), T::zero(), T::zero(), T::one()];
                Self {
                    $($field: v[$field_index],)+
                }
            }

            /// returns a vector initialised to green [0, 1, 0, 1]
            pub fn green() -> $VecN<T> {
                let v = [T::zero(), T::one(), T::zero(), T::one()];
                Self {
                    $($field: v[$field_index],)+
                }
            }

            /// returns a vector initialised to blue [0, 0, 1, 1] value will be truncated to 0 for vectors < 3 dimension
            pub fn blue() -> $VecN<T> {
                let v = [T::zero(), T::zero(), T::one(), T::one()];
                Self {
                    $($field: v[$field_index],)+
                }
            }

            /// returns a vector initialised to cyan [0, 1, 1, 1] value will be truncated to green for vectors < 3 dimension
            pub fn cyan() -> $VecN<T> {
                let v = [T::zero(), T::one(), T::one(), T::one()];
                Self {
                    $($field: v[$field_index],)+
                }
            }

            /// returns a vector initialised to magenta [1, 0, 1, 1] value will be truncated to red for vectors < 3 dimension
            pub fn magenta() -> $VecN<T> {
                let v = [T::one(), T::zero(), T::one(), T::one()];
                Self {
                    $($field: v[$field_index],)+
                }
            }

            /// returns a vector initialised to yellow [1, 1, 0, 0]
            pub fn yellow() -> $VecN<T> {
                let v = [T::one(), T::one(), T::zero(), T::one()];
                Self {
                    $($field: v[$field_index],)+
                }
            }

            /// returns a vector initialised to black (zero's)
            pub fn black() -> $VecN<T> {
                let v = [T::zero(), T::zero(), T::zero(), T::one()];
                Self {
                    $($field: v[$field_index],)+
                }
            }

            /// returns a vector initialised to white (ones's)
            pub fn white() -> $VecN<T> {
                Self::one()
            }

            /// returns a vector initialised to the max supported value for type <T>
            pub fn min_value() -> $VecN<T> {
                let v = [T::min_value(), T::min_value(), T::min_value(), T::min_value()];
                Self {
                    $($field: v[$field_index],)+
                }
            }

            /// returns a vector initialised to the max supported value for type <T>
            pub fn max_value() -> $VecN<T> {
                let v = [T::max_value(), T::max_value(), T::max_value(), T::max_value()];
                Self {
                    $($field: v[$field_index],)+
                }
            }

            /// returns a slice T of the vector
            pub fn as_slice(&self) -> &[T] {
                unsafe {
                    std::slice::from_raw_parts(&self.x, $len)
                }
            }

            /// returns a mutable slice T of the vector
            pub fn as_mut_slice(&mut self) -> &mut [T] {
                unsafe {
                    std::slice::from_raw_parts_mut(&mut self.x, $len)
                }
            }

            /// returns a slice of bytes for the vector
            pub fn as_u8_slice(&self) -> &[u8] {
                unsafe {
                    std::slice::from_raw_parts((&self.x as *const T) as *const u8, std::mem::size_of::<$VecN<T>>())
                }
            }
        }

        /// for n-dimensional functionality Self::len()
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

            fn dot(a: Self, b: Self) -> T {
                T::zero()
                $( 
                    +(a.$field * b.$field)
                )+
            }
        }

        impl<T> IntegerOps<T, $VecN<u32>> for $VecN<T> where T: Integer {
            fn pow(a: Self, exp: $VecN<u32>) -> Self {
                Self {
                    $($field: T::pow(a.$field, exp.$field as u32),)+
                }
            }
        }

        impl<T> NumberOps<T> for $VecN<T> where T: Number {
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

        impl<T> SignedNumberOps<T> for $VecN<T> where T: SignedNumber {
            fn sign(a: Self) -> Self {
                Self {
                    $($field: T::signum(a.$field),)+
                }
            }

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

        impl<T> VecFloatOps<T> for $VecN<T> where T: Float {
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
                (i - T::two()) * n * Self::dot(i, n)
            }

            fn refract(i: Self, n: Self, eta: T) -> Self {
                // https://asawicki.info/news_1301_reflect_and_refract_functions.html
                let n_dot_i = Self::dot(n, i);
                let k = T::one() - eta * eta * (T::one() - n_dot_i * n_dot_i);
                if k < T::zero() {
                    return Self::zero();
                }
                (i * eta) - n * ((n_dot_i + T::sqrt(k)) * eta)
            }
        }

        impl<T> FloatOps<T, $VecN<i32>, (Self, Self)> for $VecN<T> where T: Float {
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

            fn powi(a: Self, exp: $VecN<i32>) -> Self {
                Self {
                    $($field: T::powi(a.$field, exp.$field),)+
                }
            }

            fn powf(a: Self, exp: Self) -> Self {
                Self {
                    $($field: T::powf(a.$field, exp.$field),)+
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

            /*
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

            fn lerpn(e0: Self, e1: Self, t: Self) -> Self {
                Self {
                    $($field: T::lerp(e0.$field, e1.$field, t.$field),)+
                }
            }

            fn lerp(e0: Self, e1: Self, t: T) -> Self {
                Self {
                    $($field: T::lerp(e0.$field, e1.$field, t),)+
                }
            }

            fn smoothstep(e0: Self, e1: Self, t: T) -> Self {
                Self {
                    $($field: T::smoothstep(e0.$field, e1.$field, t),)+
                }
            }

            fn smoothstepn(e0: Self, e1: Self, t: Self) -> Self {
                Self {
                    $($field: T::smoothstep(e0.$field, e1.$field, t.$field),)+
                }
            }

            fn round(a: Self) -> Self {
                Self {
                    $($field: T::round(a.$field),)+
                }
            }
        
            fn is_nan(a: Self) -> $VecN<bool> {
                $VecN::<bool> {
                    $($field: T::is_nan(a.$field),)+
                }
            }

            fn is_infinite(a: Self) -> $VecN<bool> {
                $VecN::<bool> {
                    $($field: T::is_infinite(a.$field),)+
                }
            }

            fn is_finite(a: Self) -> $VecN<bool> {
                $VecN::<bool> {
                    $($field: T::is_finite(a.$field),)+
                }
            }

            fn saturate(x: Self) -> Self {
                Self::clamp(x, super::$VecN::zero(), super::$VecN::one())
            }
            */

            /*
            fn fmod(x: Self, y: Self) -> Self {

            }

            fn frac(v: Self) -> Self;
            fn trunc(v: Self) -> Self;
            fn modf(v: Self) -> Tuple;
            fn cos(v: Self) -> Self;
            fn sin(v: Self) -> Self;
            fn tan(v: Self) -> Self;
            fn acos(v: Self) -> Self;
            fn asin(v: Self) -> Self;
            fn atan(v: Self) -> Self;
            fn cosh(v: Self) -> Self;
            fn sinh(v: Self) -> Self;
            fn tanh(v: Self) -> Self;
            fn sin_cos(v: Self) -> Tuple;
            fn atan2(y: Self, x: Self) -> Self;
            fn exp(v: Self) -> Self;
            fn exp2(v: Self) -> Self;
            fn log2(v: Self) -> Self;
            fn log10(v: Self) -> Self;
            fn log(v: Self, base: T) -> Self;
            */
        }

        /// for n-dimensional functionality
        impl<T> Index<usize> for $VecN<T> {
            type Output = T;
            fn index(&self, i: usize) -> &Self::Output {
                match i {
                    $($field_index => &self.$field, )+
                    _ => &self.x
                }
            }
        }

        impl<T> IndexMut<usize> for $VecN<T> {
            fn index_mut(&mut self, i: usize) -> &mut T {
                match i {
                    $($field_index => &mut self.$field, )+
                    _ => &mut self.x
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

        /// collection of functions and operations for n dimensional vector
        pub mod $module {
            /// returns scalar value which is the vector dot product a . b
            pub fn dot<T: super::Number>(a: super::$VecN<T>, b: super::$VecN<T>) -> T {
                T::zero()
                $( 
                    +(a.$field * b.$field)
                )+
            }

            /// returns vector with component-wise square root
            pub fn sqrt<T: super::Float>(a: super::$VecN<T>) -> super::$VecN<T> {
                super::$VecN {
                    $($field: T::sqrt(a.$field),)+
                }
            }

            /// returns vector with component-wise reciprocal square root (1/sqrt(a))
            pub fn rsqrt<T: super::Float>(a: super::$VecN<T>) -> super::$VecN<T> {
                super::$VecN {
                    $($field: T::recip(T::sqrt(a.$field)),)+
                }
            }

            /// returns vector with component-wise reciprocal
            pub fn recip<T: super::Float>(a: super::$VecN<T>) -> super::$VecN<T> {
                super::$VecN {
                    $($field: T::recip(a.$field),)+
                }
            }

            /// returns vector with component-wise values raised to unsigned integer power
            pub fn pow<T: super::Integer>(a: super::$VecN<T>, exp: super::$VecN<u32>) -> super::$VecN<T> {
                super::$VecN {
                    $($field: T::pow(a.$field, exp.$field),)+
                }
            }

            /// returns vector with component-wise values raised to integer power
            pub fn powi<T: super::Float>(a: super::$VecN<T>, exp: super::$VecN<i32>) -> super::$VecN<T> {
                super::$VecN {
                    $($field: T::powi(a.$field, exp.$field),)+
                }
            }

            /// returns vector with component-wise values raised to float power
            pub fn powf<T: super::Float>(a: super::$VecN<T>, exp: super::$VecN<T>) -> super::$VecN<T> {
                super::$VecN {
                    $($field: T::powf(a.$field, exp.$field),)+
                }
            }
            
            /// returns scalar magnitude or length of vector
            pub fn length<T: super::Float>(a: super::$VecN<T>) -> T {
                T::sqrt(dot(a, a))
            }

            /// returns scalar magnitude or length of vector
            pub fn mag<T: super::Float>(a: super::$VecN<T>) -> T {
                T::sqrt(dot(a, a))
            }

            /// returns scalar magnitude or length of vector squared to avoid using sqrt
            pub fn mag2<T: super::Float>(a: super::$VecN<T>) -> T {
                dot(a, a)
            }

            /// returns a normalized unit vector of a
            pub fn normalize<T: super::Float>(a: super::$VecN<T>) -> super::$VecN<T> {
                let m = mag(a);
                a / m
            }

            /// returns vector with fused multiply add component wise
            pub fn mad<T: super::Float>(m: super::$VecN<T>, a: super::$VecN<T>, b: super::$VecN<T>) -> super::$VecN<T> {
                super::$VecN {
                    $($field: T::mad(m.$field, a.$field, b.$field),)+
                }
            }

            /// returns scalar distance between 2 points (magnitude of the vector between the 2 points)
            pub fn distance<T: super::Float>(a: super::$VecN<T>, b: super::$VecN<T>) -> T {
                let c = a-b;
                T::sqrt(dot(c, c))
            }

            /// returns scalar distance between 2 points (magnitude of the vector between the 2 points)
            pub fn dist<T: super::Float>(a: super::$VecN<T>, b: super::$VecN<T>) -> T {
                distance(a, b)
            }

            /// returns scalar squared distance between 2 points to avoid using sqrt
            pub fn dist2<T: super::Float>(a: super::$VecN<T>, b: super::$VecN<T>) -> T {
                let c = a-b;
                dot(c, c)
            }

            /// returns true if all elements in vector are non-zero
            pub fn all<T: super::Number>(a: super::$VecN<T>) -> bool {
                $(a.$field != T::zero() &&)+
                true
            }

            /// returns true if any element of the vector is non-zero
            pub fn any<T: super::Number>(a: super::$VecN<T>) -> bool {
                $(a.$field != T::zero() ||)+
                false
            }

            /// returns true if all elements in vectors a and b are approximately equal within the designated epsilon
            pub fn approx<T: super::Float>(a: super::$VecN<T>, b: super::$VecN<T>, eps: T) -> bool {
                $(T::abs(a.$field - b.$field) < eps &&)+
                true
            }

            /// returns the greatest integer which is less than or equal to each vector element component wise
            pub fn floor<T: super::Float>(a: super::$VecN<T>) -> super::$VecN<T> {
                super::$VecN {
                    $($field: super::Float::floor(a.$field),)+
                }
            }

            /// returns the smallest integer which is greater than or equal to each vector element component wise
            pub fn ceil<T: super::Float>(a: super::$VecN<T>) -> super::$VecN<T> {
                super::$VecN {
                    $($field: super::Float::ceil(a.$field),)+
                }
            }

            /// performs linear interpolation between e0 and e1, t specifies the ratio to interpolate between the values
            pub fn lerpn<T: super::Float>(e0: super::$VecN<T>, e1: super::$VecN<T>, t: super::$VecN<T>) -> super::$VecN<T> {
                super::$VecN {
                    $($field: super::Float::lerp(e0.$field, e1.$field, t.$field),)+
                }
            }

            /// performs linear interpolation between e0 and e1, t specifies the ratio to interpolate between the values
            pub fn lerp<T: super::Float>(e0: super::$VecN<T>, e1: super::$VecN<T>, t: T) -> super::$VecN<T> {
                super::$VecN {
                    $($field: super::Float::lerp(e0.$field, e1.$field, t),)+
                }
            }

            /// returns vector with component wise hermite interpolation between 0-1
            pub fn smoothstep<T: super::Float>(e0: super::$VecN<T>, e1: super::$VecN<T>, t: T) -> super::$VecN<T> {
                super::$VecN {
                    $($field: super::Float::smoothstep(e0.$field, e1.$field, t),)+
                }
            }

            /// returns vector with component wise hermite interpolation between 0-1
            pub fn smoothstepn<T: super::Float>(e0: super::$VecN<T>, e1: super::$VecN<T>, t: super::$VecN<T>) -> super::$VecN<T> {
                super::$VecN {
                    $($field: super::Float::smoothstep(e0.$field, e1.$field, t.$field),)+
                }
            }

            /// returns vector with values from a rounded component wise
            pub fn round<T: super::Float>(a: super::$VecN<T>) -> super::$VecN<T> {
                super::$VecN {
                    $($field: super::Float::round(a.$field),)+
                }
            }

            /// returns true if a is not a number
            pub fn is_nan<T: super::Float>(a: super::$VecN<T>) -> super::$VecN<bool> {
                super::$VecN {
                    $($field: super::Float::is_nan(a.$field),)+
                }
            }

            /// returns true if a is inf
            pub fn is_infinite<T: super::Float>(a: super::$VecN<T>) -> super::$VecN<bool> {
                super::$VecN {
                    $($field: super::Float::is_infinite(a.$field),)+
                }
            }

            /// returns true is a is finite
            pub fn is_finite<T: super::Float>(a: super::$VecN<T>) -> super::$VecN<bool> {
                super::$VecN {
                    $($field: super::Float::is_finite(a.$field),)+
                }
            }

            /// returns a vector containing component wise min of a and b
            pub fn min<T: super::Number>(a: super::$VecN<T>, b: super::$VecN<T>) -> super::$VecN<T> {
                super::$VecN {
                    $($field: super::Number::min(a.$field, b.$field),)+
                }
            }
            
            /// returns a vector containing component wise max of a and b
            pub fn max<T: super::Number>(a: super::$VecN<T>, b: super::$VecN<T>) -> super::$VecN<T> {
                super::$VecN {
                    $($field: super::Number::max(a.$field, b.$field),)+
                }
            }

            /// returns component wise sign value; -1 = negative, 1 = positive or 0 (integers only)
            pub fn sign<T: super::SignedNumber>(a: super::$VecN<T>) -> super::$VecN<T> {
                super::$VecN {
                    $($field: super::SignedNumber::signum(a.$field),)+
                }
            }

            /// returns component wise sign value; -1 = negative, 1 = positive or 0 (integers only)
            pub fn signum<T: super::SignedNumber>(a: super::$VecN<T>) -> super::$VecN<T> {
                super::$VecN {
                    $($field: super::SignedNumber::signum(a.$field),)+
                }
            }

            /// returns a omponent wise vector containing the absolute (postive) value of a
            pub fn abs<T: super::SignedNumber>(a: super::$VecN<T>) -> super::$VecN<T> {
                super::$VecN {
                    $($field: super::SignedNumber::abs(a.$field),)+
                }
            }

            /// returns a vector with elements of x clamped component wise to min and max
            pub fn clamp<T: super::Number>(x: super::$VecN<T>, min: super::$VecN<T>, max: super::$VecN<T>) -> super::$VecN<T> {
                super::$VecN {
                    $($field: super::Number::max(super::Number::min(x.$field, max.$field), min.$field),)+
                }
            }
            
            /// returns a vector with saturated elements clamped between 0-1. equivalent to clamp (x, 0, 1)
            pub fn saturate<T: super::Float>(x: super::$VecN<T>) -> super::$VecN<T> {
                clamp(x, super::$VecN::zero(), super::$VecN::one())
            }

            /// returns a vector stepped component wise; 1 if a is >= b, 0 otherwise
            pub fn step<T: super::Number>(a: super::$VecN<T>, b: super::$VecN<T>) -> super::$VecN<T> {
                super::$VecN {
                    $($field: super::Number::step(a.$field, b.$field),)+
                }
            }

            /// returns a reflection vector using an incident ray and a surface normal
            pub fn reflect<T: super::Float>(i: super::$VecN<T>, n: super::$VecN<T>) -> super::$VecN<T> {
                // https://docs.microsoft.com/en-us/windows/win32/direct3dhlsl/dx-graphics-hlsl-reflect
                (i - T::two()) * n * dot(i, n)
            }

            /// returns a refraction vector using an entering ray, a surface normal, and a refraction index
            pub fn refract<T: super::Float>(i: super::$VecN<T>, n: super::$VecN<T>, eta: T) -> super::$VecN<T> {
                // https://asawicki.info/news_1301_reflect_and_refract_functions.html
                let n_dot_i = dot(n, i);
                let k = T::one() - eta * eta * (T::one() - n_dot_i * n_dot_i);
                if k < T::zero() {
                    return super::$VecN::zero();
                }
                (i * eta) - n * ((n_dot_i + T::sqrt(k)) * eta)
            }

            pub fn fmod<T: super::Float>(x: super::$VecN<T>, y: super::$VecN<T>) -> super::$VecN<T> {
                x % y
            }

            pub fn frac<T: super::Float>(v: super::$VecN<T>) -> super::$VecN<T> {
                super::$VecN {
                    $($field: super::Float::frac(v.$field),)+
                }
            }

            pub fn trunc<T: super::Float>(v: super::$VecN<T>) -> super::$VecN<T> {
                super::$VecN {
                    $($field: super::Float::trunc(v.$field),)+
                }
            }

            pub fn modf<T: super::Float>(v: super::$VecN<T>) -> (super::$VecN<T>, super::$VecN<T>) {
                (
                    super::$VecN {
                        $($field: super::Float::frac(v.$field),)+
                    },
                    super::$VecN {
                        $($field: super::Float::trunc(v.$field),)+
                    }
                )
            }

            pub fn cos<T: super::Float>(v: super::$VecN<T>) -> super::$VecN<T> {
                super::$VecN {
                    $($field: super::Float::cos(v.$field),)+
                }
            }

            pub fn sin<T: super::Float>(v: super::$VecN<T>) -> super::$VecN<T> {
                super::$VecN {
                    $($field: super::Float::sin(v.$field),)+
                }
            }

            pub fn tan<T: super::Float>(v: super::$VecN<T>) -> super::$VecN<T> {
                super::$VecN {
                    $($field: super::Float::tan(v.$field),)+
                }
            }

            pub fn acos<T: super::Float>(v: super::$VecN<T>) -> super::$VecN<T> {
                super::$VecN {
                    $($field: super::Float::acos(v.$field),)+
                }
            }

            pub fn asin<T: super::Float>(v: super::$VecN<T>) -> super::$VecN<T> {
                super::$VecN {
                    $($field: super::Float::asin(v.$field),)+
                }
            }

            pub fn atan<T: super::Float>(v: super::$VecN<T>) -> super::$VecN<T> {
                super::$VecN {
                    $($field: super::Float::atan(v.$field),)+
                }
            }

            pub fn cosh<T: super::Float>(v: super::$VecN<T>) -> super::$VecN<T> {
                super::$VecN {
                    $($field: super::Float::cosh(v.$field),)+
                }
            }

            pub fn sinh<T: super::Float>(v: super::$VecN<T>) -> super::$VecN<T> {
                super::$VecN {
                    $($field: super::Float::sinh(v.$field),)+
                }
            }

            pub fn tanh<T: super::Float>(v: super::$VecN<T>) -> super::$VecN<T> {
                super::$VecN {
                    $($field: super::Float::tanh(v.$field),)+
                }
            }

            pub fn sin_cos<T: super::Float>(v: super::$VecN<T>) -> (super::$VecN<T>, super::$VecN<T>) {
                (
                    super::$VecN {
                        $($field: super::Float::sin(v.$field),)+
                    },
                    super::$VecN {
                        $($field: super::Float::cos(v.$field),)+
                    }
                )
            }

            pub fn atan2<T: super::Float>(y: super::$VecN<T>, x: super::$VecN<T>) -> super::$VecN<T> {
                super::$VecN {
                    $($field: super::Float::atan2(y.$field, x.$field),)+
                }
            }

            pub fn exp<T: super::Float>(v: super::$VecN<T>) -> super::$VecN<T> {
                super::$VecN {
                    $($field: super::Float::exp(v.$field),)+
                }
            }

            pub fn exp2<T: super::Float>(v: super::$VecN<T>) -> super::$VecN<T> {
                super::$VecN {
                    $($field: super::Float::exp2(v.$field),)+
                }
            }

            pub fn log2<T: super::Float>(v: super::$VecN<T>) -> super::$VecN<T> {
                super::$VecN {
                    $($field: super::Float::log2(v.$field),)+
                }
            }

            pub fn log10<T: super::Float>(v: super::$VecN<T>) -> super::$VecN<T> {
                super::$VecN {
                    $($field: super::Float::log10(v.$field),)+
                }
            }

            pub fn log<T: super::Float>(v: super::$VecN<T>, base: T) -> super::$VecN<T> {
                super::$VecN {
                    $($field: super::Float::log(v.$field, base),)+
                }
            }
        }
    }
}

/// macro to stamp out various typed c-style constructors. ie. let v = vec3f(0.0, 1.0, 0.0);
macro_rules! vec_ctor {
    ($VecN:ident { $($field:ident),+ }, $ctor:ident, $splat:ident, $t:ident) => {
        pub fn $ctor($($field: $t,)+) -> $VecN<$t> {
            $VecN {
                $($field: $field,)+
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

//
// Functions
//

pub fn cross<T: Number>(a: Vec3<T>, b: Vec3<T>) -> Vec3<T> {
    Vec3 {
        x: (a.y * b.z) - (a.z * b.y), 
        y: (a.z * b.x) - (a.x * b.z),
        z: (a.x * b.y) - (a.y * b.x),
    }
}

/// perpedicular vector anti-clockwise rotation by 90 degrees
pub fn perp<T: SignedNumber>(a: Vec2<T>) -> Vec2<T> {
    Vec2 {
        x: -a.y, 
        y: a.x
    }
}

pub fn dot<T: Number, V: VecN<T>>(a: V, b: V) -> T {
    V::dot(a, b)
}

pub fn sqrt<T: Float, Exp: VecN<i32>, Tuple: VecN<T>, V: FloatOps<T, Exp, (Tuple, Tuple)>>(a: V) {
    V::sqrt(a);
}

pub fn powi<T: Float, Exp: VecN<i32>, Tuple: VecN<T>, V: FloatOps<T, Exp, (Tuple, Tuple)>>(a: V, b: Exp) {
    V::powi(a, b);
}

//
// Macro Decl
//

vec_impl!(Vec2 { x, 0, y, 1 }, 2, v2);
vec_impl!(Vec3 { x, 0, y, 1, z, 2 }, 3, v3);
vec_impl!(Vec4 { x, 0, y, 1, z, 2, w, 3 }, 4, v4);

#[cfg(feature = "lhs_scalar_vec_ops")]
vec_scalar_lhs!(Vec2 { x, y }, f32);
vec_scalar_lhs!(Vec3 { x, y, z }, f32);
vec_scalar_lhs!(Vec4 { x, y, z, w }, f32);
vec_scalar_lhs!(Vec2 { x, y }, f64);
vec_scalar_lhs!(Vec3 { x, y, z }, f64);
vec_scalar_lhs!(Vec4 { x, y, z, w }, f64);
vec_scalar_lhs!(Vec2 { x, y }, i32);
vec_scalar_lhs!(Vec3 { x, y, z }, i32);
vec_scalar_lhs!(Vec4 { x, y, z, w }, i32);
vec_scalar_lhs!(Vec2 { x, y }, u32);
vec_scalar_lhs!(Vec3 { x, y, z }, u32);
vec_scalar_lhs!(Vec4 { x, y, z, w }, u32);

#[cfg(feature = "short_hand_constructors")]
vec_ctor!(Vec2 { x, y }, vec2b, splat2b, bool);
vec_ctor!(Vec3 { x, y, z }, vec3b, splat3b, bool);
vec_ctor!(Vec4 { x, y, z, w }, vec4b, splat4b, bool);
vec_ctor!(Vec2 { x, y }, vec2f, splat2f, f32);
vec_ctor!(Vec3 { x, y, z }, vec3f, splat3f, f32);
vec_ctor!(Vec4 { x, y, z, w }, vec4f, splat4f, f32);
vec_ctor!(Vec2 { x, y }, vec2d, splat2d, f64);
vec_ctor!(Vec3 { x, y, z }, vec3d, splat3d, f64);
vec_ctor!(Vec4 { x, y, z, w }, vec4d, splat4d, f64);
vec_ctor!(Vec2 { x, y }, vec2i, splat2i, i32);
vec_ctor!(Vec3 { x, y, z }, vec3i, splat3i, i32);
vec_ctor!(Vec4 { x, y, z, w }, vec4i, splat4i, i32);
vec_ctor!(Vec2 { x, y }, vec2u, splat2u, u32);
vec_ctor!(Vec3 { x, y, z }, vec3u, splat3u, u32);
vec_ctor!(Vec4 { x, y, z, w }, vec4u, splat4u, u32);

// TODO: swizzles
// TODO: refactor functions to follow dott

// experims
// v3 / v2 mod, with use... didnt correctly deduce the function by type

// generic... (code gen)
/*
fn dot<V: VecN<T>, T: Number>(v1: &V, v2: &V) -> T {
    let mut r = T::default();
    for i in 0..V::len() {
        r += v1[i] * v2[i];
    }
    r
}
*/

/*
fn dot_v2_index<T: Default + std::ops::Mul<Output = T> + std::ops::Add<Output = T> + std::ops::AddAssign + Copy>(v1: &Vec2<T>, v2: &Vec2<T>) -> T {
    let mut r = T::default();
    for i in 0..2 {
        r += v1[i] * v2[i];
    }
    r
}

fn dot_v2<T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Copy>(v1: &Vec2<T>, v2: &Vec2<T>) -> T {
    v1[0] * v2[0] + v1[1] * v2[1]
}

impl<T> Vec3<T> where T: Number {
    fn dot(x1: &Vec3<T>, x2: &Vec3<T>) -> T {
        x1.x * x2.x + x1.y * x2.y + x1.z * x2.z
    }
}

pub fn dot<T: super::Number>(a: super::$VecN<T>, b: super::$VecN<T>) -> T {
    T::default() // needs this!? ;_;
    $( 
        +(a.$field * b.$field)
    )+
}

// overloading through traits... template style specialisation is not possible
#[derive(Debug, Copy, Clone)]
pub struct Bec<T, X, const N: usize> {
    pub v: [T; N]
}

pub trait Bec2 {}
pub trait Bec3 {}

pub fn t1<T: Number, X: Bec2>(a: Bec<T, X, 2>, b: Bec<T, X, 2>) -> T {
    a.v[0] + b.v[1]
}

pub fn t1<T: Number, X: Bec3>(a: Bec<T, X, 2>, b: Bec<T, X, 2>) -> T {
    a.v[0] + b.v[2]
}

// ? repeater
pub fn xxx<T: super::Number>(_a: super::$VecN<T>, _b: super::$VecN<T>) -> T{
    $( 
        //let mut c = (a.$field * b.$field);
        println!("? {}", _a.$field);
    )?

    $(
        println!("* {}", _a.$field);

        //+ (a.$field * b.$field)
    )*

    T::zero()
}

// lhs
impl<T> Add<Vec2<T>> for T where T: Number {
    type Output = Vec2<T>;
    fn add(self, other: Vec2<T>) -> Vec2<T> {
        Vec2 {
            x: other.x + self,
            y: other.y + self,
        }
    }
}

/*
impl<T> Add<Vec2<T>> for T {
    type Output = T;
    fn add(self, other: Vec2<T>) -> Vec2<T> {
        Vec2::zero()
    }
}
*/

// impl<T> Add<Vec2<T>> for T {
//     ^ type parameter `T` must be covered by another type when it appears before the first local type (`Vec2<T>`)
*/