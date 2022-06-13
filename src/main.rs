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

//
// Vec2
//

#[derive(Debug, Copy, Clone)]
pub struct Vec2<T: Number> {
    x: T,
    y: T
}

impl<T> VecN<T> for Vec2<T> where T: Number {
    fn len() -> usize {
        2
    }
}

impl<T> Display for Vec2<T> where T: Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl<T> Index<usize> for Vec2<T> where T: Number {
    type Output = T;
    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            _ => &self.y
        }
    }
}

impl<T> Add for Vec2<T> where T: Number {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> AddAssign for Vec2<T> where T: Number {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl<T> Sub for Vec2<T> where T: Number {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> SubAssign for Vec2<T> where T: Number {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

impl<T> Mul for Vec2<T> where T: Number {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl<T> MulAssign for Vec2<T> where T: Number {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
        };
    }
}

impl<T> Div for Vec2<T> where T: Number {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl<T> DivAssign for Vec2<T> where T: Number {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
        };
    }
}

impl<T> Neg for Vec2<T> where T: SignedNumber {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T> PartialEq for Vec2<T> where T: Number  {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T> Eq for Vec2<T> where T: Number  {}

//
// Vec3
//

#[derive(Debug, Copy, Clone)]
pub struct Vec3<T: Number> {
    x: T,
    y: T,
    z: T
}

impl<T> Vec3<T> where T: Number {
    fn dot(x1: &Vec3<T>, x2: &Vec3<T>) -> T {
        x1.x * x2.x + x1.y * x2.y + x1.z * x2.z
    }
}

impl<T> VecN<T> for Vec3<T> where T: Number {
    fn len() -> usize {
        3
    }
}

impl<T> Index<usize> for Vec3<T> where T: Number {
    type Output = T;
    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => &self.z
        }
    }
}

//
// Functions
//

/*
fn dot<V: VecN<T>, T: Number>(v1: &V, v2: &V) -> T {
    let mut r = T::default();
    for i in 0..V::len() {
        r += v1[i] * v2[i];
    }
    r
}
*/

mod v3 {
    pub fn dot<T: super::Number>(x1: &super::Vec3<T>, x2: &super::Vec3<T>) -> T {
        x1.x * x2.x + x1.y * x2.y + x1.z * x2.z
    }
}

mod v2 {
    pub fn dot<T: super::Number>(x1: &super::Vec2<T>, x2: &super::Vec2<T>) -> T {
        x1.x * x2.x + x1.y * x2.y
    }
}

// Vec4
/*
struct Vec4<T> {
    x: T,
    y: T,
    z: T,
    w: T
}

impl<T> VecN<T> for Vec4<T> {
    fn len() -> usize {
        4
    }
}

impl<T> Index<usize> for Vec4<T> {
    type Output = T;
    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => &self.w
        }
    }
}
*/

// use Vec3::dot;

fn main() {
    let v2 : Vec2<f32> = Vec2 {
        x: 2.0,
        y: 3.0
    };

    let ve : Vec2<f32> = Vec2 {
        x: 2.0,
        y: 3.0
    };

    let vf : Vec3<f32> = Vec3 {
        x: 1.0,
        y: 0.0,
        z: 1.0
    };

    let dot3 = v3::dot(&vf, &vf);
    println!("dot3 = {}", dot3);

    let dot33 = dot(&vf, &vf);
    println!("dot33 = {}", dot3);

    let dot2 = v2::dot(&ve, &ve);
    println!("dot2 = {}", dot2);

    let vneg = -v2;
    println!("neg = {}", vneg);

    if v2 == ve {
        println!("equals!");
    }

    let dp = v2::dot(&v2, &v2);
    println!("dot = {}", dp);
}

// constructors
// - combos
// initialisers
// - unitx, etc
// add
// add assign
// sub
// sub assign
// mul
// mul assign
// div
// div assign
// equal to
// not equal to
// unary minus

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
// dst??
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
// rcp ? (recipricol)
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
// transpose
// trunc

// experims
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
*/