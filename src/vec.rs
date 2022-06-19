use std::ops::Index;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Neg;
use std::cmp::PartialEq;
use std::cmp::PartialOrd;

use std::fmt::Display;
use std::fmt::Formatter;

//
// Vec Traits 
//

pub trait VecN<T: Number>: Index<usize, Output=T> {
    fn len() -> usize;
}

macro_rules! number_trait_impl {
    ($($func:ident),*) => {
        pub trait Number: 
            Copy + Default + Display + 
            Add<Output=Self> + AddAssign + 
            Mul<Output=Self> + MulAssign + 
            Div<Output=Self> + DivAssign +
            Sub<Output=Self> + SubAssign +
            PartialEq + PartialOrd {
                fn zero() -> Self;
                fn one() -> Self;
                fn two() -> Self;
                fn min(a: Self, b: Self) -> Self;
                fn max(a: Self, b: Self) -> Self;
                fn step(a: Self, b: Self) -> Self;
        }
        number_impl!(f64 { $($func),* }, 0.0, 1.0);
        number_impl!(f32 { $($func),* }, 0.0, 1.0);
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
        pub trait Float: SignedNumber {
            $(fn $func(v: Self) -> Self;)*
            fn isnan(v: Self) -> bool;
            fn isinf(v: Self) -> bool;
            fn isfinite(v: Self) -> bool;
            fn smoothstep(e0: Self, e1: Self, t: Self) -> Self;
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

            fn isnan(v: Self) -> bool {
                v.is_nan()
            }

            fn isinf(v: Self) -> bool {
                v.is_infinite()
            }

            fn isfinite(v: Self) -> bool {
                v.is_finite()
            }

            fn smoothstep(e0: Self, e1: Self, t: Self) -> Self {
                if t < e0 { return Self::zero(); }
                if (t >= e1) { return Self::one(); }
                let x = (t - e0) / (e1 - e0);
                x * x * (3 as Self - 2 as Self * x)
            }
        }
    }
}

//
// Abbreviations
//

/// 2-dimensional f32 vector
pub type Vec2f = Vec2<f32>;

/// 3-dimensional f32 vector
pub type Vec3f = Vec3<f32>;

/// 4-dimensional f32 vector
pub type Vec4f = Vec4<f32>;

/// 2-dimensional f64 vector
pub type Vec2d = Vec2<f64>;

/// 3-dimensional f64 vector
pub type Vec3d = Vec3<f64>;

/// 4-dimensional f64 vector
pub type Vec4d = Vec4<f64>;

/// 2-dimensional i32 vector
pub type Vec2i = Vec2<i32>;

/// 3-dimensional i32 vector
pub type Vec3i = Vec3<i32>;

/// 4-dimensional i32 vector
pub type Vec4i = Vec4<i32>;

/// 2-dimensional u32 vector
pub type Vec2u = Vec2<u32>;

/// 3-dimensional u32 vector
pub type Vec3u = Vec3<u32>;

/// 4-dimensional u32 vector
pub type Vec4u = Vec4<u32>;

/// 2-dimensional bool vector
pub type Vec2b = Vec2<bool>;

/// 3-dimensional bool vector
pub type Vec3b = Vec3<bool>;

/// 4-dimensional bool vector
pub type Vec4b = Vec4<bool>;

// 
// Macro Implementation
//

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

            pub fn zero() -> $VecN<T> {
                $VecN {
                    $($field: T::zero(),)+
                }
            }

            pub fn one() -> $VecN<T> {
                $VecN {
                    $($field: T::one(),)+
                }
            }

            pub fn unit_x() -> $VecN<T> {
                let v = [T::one(), T::zero(), T::zero(), T::zero()];
                Self {
                    $($field: v[$field_index],)+
                }
            }

            pub fn unit_y() -> $VecN<T> {
                let v = [T::zero(), T::one(), T::zero(), T::zero()];
                Self {
                    $($field: v[$field_index],)+
                }
            }

            pub fn unit_z() -> $VecN<T> {
                let v = [T::zero(), T::zero(), T::one(), T::zero()];
                Self {
                    $($field: v[$field_index],)+
                }
            }

            pub fn red() -> $VecN<T> {
                let v = [T::one(), T::zero(), T::zero(), T::one()];
                Self {
                    $($field: v[$field_index],)+
                }
            }

            pub fn green() -> $VecN<T> {
                let v = [T::zero(), T::one(), T::zero(), T::one()];
                Self {
                    $($field: v[$field_index],)+
                }
            }

            pub fn blue() -> $VecN<T> {
                let v = [T::zero(), T::zero(), T::one(), T::one()];
                Self {
                    $($field: v[$field_index],)+
                }
            }

            pub fn cyan() -> $VecN<T> {
                let v = [T::zero(), T::one(), T::one(), T::one()];
                Self {
                    $($field: v[$field_index],)+
                }
            }

            pub fn magenta() -> $VecN<T> {
                let v = [T::one(), T::zero(), T::one(), T::one()];
                Self {
                    $($field: v[$field_index],)+
                }
            }

            pub fn yellow() -> $VecN<T> {
                let v = [T::one(), T::one(), T::zero(), T::one()];
                Self {
                    $($field: v[$field_index],)+
                }
            }

            pub fn black() -> $VecN<T> {
                let v = [T::zero(), T::zero(), T::zero(), T::one()];
                Self {
                    $($field: v[$field_index],)+
                }
            }

            pub fn white() -> $VecN<T> {
                Self::one()
            }
        }

        /// for n-dimensional functionality Self::len()
        impl<T> VecN<T> for $VecN<T> where T: Number {
            fn len() -> usize {
                $len
            }
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

            // returns vector with component-wise reciprocal square root (1/sqrt(a))
            pub fn rsqrt<T: super::Float>(a: super::$VecN<T>) -> super::$VecN<T> {
                super::$VecN {
                    $($field: T::recip(T::sqrt(a.$field)),)+
                }
            }

            // returns vector with component-wise reciprocal
            pub fn rcp<T: super::Float>(a: super::$VecN<T>) -> super::$VecN<T> {
                super::$VecN {
                    $($field: T::recip(a.$field),)+
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

            /// returns a normalized unit vector
            pub fn normalize<T: super::Float>(a: super::$VecN<T>) -> super::$VecN<T> {
                let m = mag(a);
                a / m
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
                $(a.$field - b.$field < eps &&)+
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

            /// returns vector with component wise hermite interpolation between 0-1
            pub fn smoothstep<T: super::Float>(e0: super::$VecN<T>, e1: super::$VecN<T>, t: super::$VecN<T>) -> super::$VecN<T> {
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
            pub fn isnan<T: super::Float>(a: super::$VecN<T>) -> super::$VecN<bool> {
                super::$VecN {
                    $($field: super::Float::isnan(a.$field),)+
                }
            }

            /// returns true if a is inf
            pub fn isinf<T: super::Float>(a: super::$VecN<T>) -> super::$VecN<bool> {
                super::$VecN {
                    $($field: super::Float::isinf(a.$field),)+
                }
            }

            /// returns true is a is finite
            pub fn isfinite<T: super::Float>(a: super::$VecN<T>) -> super::$VecN<bool> {
                super::$VecN {
                    $($field: super::Float::isfinite(a.$field),)+
                }
            }

            // returns a vector containing component wise min of a and b
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

            // returns a reflection vector using an incident ray and a surface normal
            pub fn reflect<T: super::Float>(i: super::$VecN<T>, n: super::$VecN<T>) -> super::$VecN<T> {
                // https://docs.microsoft.com/en-us/windows/win32/direct3dhlsl/dx-graphics-hlsl-reflect
                (i - T::two()) * n * dot(i, n)
            }

            // returns a refraction vector using an entering ray, a surface normal, and a refraction index
            pub fn refract<T: super::Float>(i: super::$VecN<T>, n: super::$VecN<T>, eta: T) -> super::$VecN<T> {
                // https://asawicki.info/news_1301_reflect_and_refract_functions.html
                let n_dot_i = dot(n, i);
                let k = T::one() - eta * eta * (T::one() - n_dot_i * n_dot_i);
                if k < T::zero() {
                    return super::$VecN::zero();
                }
                (i * eta) - n * ((n_dot_i + T::sqrt(k)) * eta)
            }
        }
    }
}

macro_rules! vec_ctor {
    ($VecN:ident { $($field:ident),+ }, $ctor:ident, $t:ident) => {
        pub fn $ctor($($field: $t,)+) -> $VecN<$t> {
            $VecN {
                $($field: $field,)+
            }
        }
    }
}

//
// rhs.. TODO:
//

impl Add<Vec2<f32>> for f32 {
    type Output = Vec2<f32>;
    fn add(self, other: Vec2<f32>) -> Vec2<f32> {
        Vec2 {
            x: other.x + self,
            y: other.y + self,
        }
    }
}

//
// From
//

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

//
// Macro Decl
//

float_trait_impl!(floor, ceil, round, sqrt, recip);
signed_number_trait_impl!(signum, abs);
number_trait_impl!();

vec_impl!(Vec2 { x, 0, y, 1 }, 2, v2);
vec_impl!(Vec3 { x, 0, y, 1, z, 2 }, 3, v3);
vec_impl!(Vec4 { x, 0, y, 1, z, 2, w, 3 }, 4, v4);

vec_ctor!(Vec2 { x, y }, vec2b, bool);
vec_ctor!(Vec3 { x, y, z }, vec3b, bool);
vec_ctor!(Vec4 { x, y, z, w }, vec4b, bool);

vec_ctor!(Vec2 { x, y }, vec2f, f32);
vec_ctor!(Vec3 { x, y, z }, vec3f, f32);
vec_ctor!(Vec4 { x, y, z, w }, vec4f, f32);

vec_ctor!(Vec2 { x, y }, vec2d, f64);
vec_ctor!(Vec3 { x, y, z }, vec3d, f64);
vec_ctor!(Vec4 { x, y, z, w }, vec4d, f64);

vec_ctor!(Vec2 { x, y }, vec2i, i32);
vec_ctor!(Vec3 { x, y, z }, vec3i, i32);
vec_ctor!(Vec4 { x, y, z, w }, vec4i, i32);

vec_ctor!(Vec2 { x, y }, vec2u, u32);
vec_ctor!(Vec3 { x, y, z }, vec3u, u32);
vec_ctor!(Vec4 { x, y, z, w }, vec4u, u32);

// acos
// atan
// atan2
// cos
// cosh
// exp
// exp2
// fmod
// frac
// frexp
// ldexp
// lerp
// log
// log10
// log2
// modf
// pow
// sin
// sincos
// sinh
// tan
// tanh

// experims
// v3 / v2 mod, with use... didnt correctly deduce the function by type

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
*/