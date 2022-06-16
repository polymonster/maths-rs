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
use std::fmt::Display;
use std::fmt::Formatter;

//
// Vec Traits 
//

pub trait Number: 
    Copy + Default + Display + 
    Add<Output=Self> + AddAssign + 
    Mul<Output=Self> + MulAssign + 
    Div<Output=Self> + DivAssign +
    Sub<Output=Self> + SubAssign +
    PartialEq {
}

pub trait SignedNumber:
    Number + Neg<Output=Self> {
}

pub trait VecN<T: Number>: Index<usize, Output=T> {
    fn len() -> usize;
}

impl SignedNumber for f64 {}
impl SignedNumber for i64 {}
impl SignedNumber for f32 {}
impl SignedNumber for i32 {}
impl SignedNumber for i16 {}
impl SignedNumber for i8 {}

impl Number for f64 {}
impl Number for i64 {}
impl Number for u64 {}
impl Number for f32 {}
impl Number for i32 {}
impl Number for u32 {}
impl Number for i16 {}
impl Number for u16 {}
impl Number for i8 {}
impl Number for u8 {}

// Abbreviations

pub type Vec2f = Vec2<f32>;
pub type Vec3f = Vec3<f32>;
pub type Vec4f = Vec4<f32>;

// 
// Macro Implementation
//

macro_rules! vec_impl {
    ($VecN:ident { $($field:ident, $field_index:expr),+ }, $len:expr, $module:ident, $ctorf:ident) => {
        // concrete type
        #[derive(Debug, Copy, Clone)]
        pub struct $VecN<T: Number> {
            $(pub $field: T,)+
        }
        
        /// ie let v = vec3f(x, y, z);
        pub fn $ctorf($($field: f32,)+) -> $VecN<f32> {
            $VecN {
                $($field: $field,)+
            }
        }

        /// Vec4::<f32>::new(1.0, 2.0, 3.0, 4.0) or with abbreviated type Vec4f::new(1.0, 2.0, 3.0, 4.0)
        impl<T> $VecN<T> where T: Number {
            pub fn new($($field: T,)+) -> $VecN<T> {
                $VecN {
                    $($field: $field,)+
                }
            }
        }

        /// for n-dimensional functionality Self::len()
        impl<T> VecN<T> for $VecN<T> where T: Number {
            fn len() -> usize {
                $len
            }
        }

        /// for n-dimensional functionality v[n]
        impl<T> Index<usize> for $VecN<T> where T: Number {
            type Output = T;
            fn index(&self, i: usize) -> &Self::Output {
                match i {
                    $($field_index => &self.$field, )+
                    _ => &self.x
                }
            }
        }

        /// displays like [10.0, 12.0, 13.0]
        impl<T> Display for $VecN<T> where T: Number {
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
        
        impl<T> PartialEq for $VecN<T> where T: Number  {
            fn eq(&self, other: &Self) -> bool {
                $(self.$field == other.$field &&)+
                true
            }
        }
        
        impl<T> Eq for $VecN<T> where T: Number  {}

        pub mod $module {
            pub fn approxf(a: super::$VecN<f32>, b: super::$VecN<f32>, eps: f32) -> bool {
                $(a.$field - b.$field < eps &&)+
                true
            }
            pub fn approxd(a: super::$VecN<f64>, b: super::$VecN<f64>, eps: f64) -> bool {
                $(a.$field - b.$field < eps &&)+
                true
            }
        }
    }
}

vec_impl!(Vec2 { x, 0, y, 1 }, 2, v2, vec2f);
vec_impl!(Vec3 { x, 0, y, 1, z, 2 }, 3, v3, vec3f);
vec_impl!(Vec4 { x, 0, y, 1, z, 2, w, 3 }, 4, v4, vec4f);

//
// Functions
//

/*
pub mod v3 {
    pub fn dot<T: super::Number>(x1: &super::Vec3<T>, x2: &super::Vec3<T>) -> T {
        x1.x * x2.x + x1.y * x2.y + x1.z * x2.z
    }
}

pub mod v2 {
    pub fn dot<T: super::Number>(x1: &super::Vec2<T>, x2: &super::Vec2<T>) -> T {
        x1.x * x2.x + x1.y * x2.y
    }
}
*/

// constructors
// - combos
// initialisers
// - unitx, etc

// funcs
// abs
// all
// acos
// any
// atan
// atan2
// ceil
// clamp
// cos
// cosh
// cross
// distance
// dot
// dst ??
// exp
// exp2
// floor
// fmod
// frac
// frexp
// fwidth
// isfinite
// isinf
// isnan
// ldexp ?
// length
// lerp
// lit ?
// log
// log10
// log2
// max
// min
// modf
// mul
// noise
// normalize
// pow
// rcp (recipricol)
// reflect
// refract
// round
// rsqrt
// saturate
// sign
// sin
// sincos
// sinh
// smoothstep
// sqrt
// step
// tan
// tanh
// trunc

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
*/