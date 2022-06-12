use std::ops::Index;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::ops::Div;
use std::ops::DivAssign;

use std::fmt::Display;

trait Number: Copy + Default + Display + Add<Output=Self> + AddAssign + Mul<Output=Self> + MulAssign {
}

impl Number for f32 {}
impl Number for f64 {}
impl Number for i32 {}
impl Number for i64 {}

trait VecN<T: Number> {
    fn len() -> usize;
}

// Vec2
struct Vec2<T: Number> {
    x: T,
    y: T
}

impl<T> VecN<T> for Vec2<T> where T: Number {
    fn len() -> usize {
        2
    }
}

impl<T> Index<usize> for Vec2<T> where T: Number {
    type Output = T;
    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            _ => &self.y // how to handle out of bounds access?
        }
    }
}

// Vec3
/*
struct Vec3<T> {
    x: T,
    y: T,
    z: T
}

impl<T> VecN<T> for Vec3<T> {
    fn len() -> usize {
        3
    }
}

impl<T> Index<usize> for Vec3<T> {
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
*/

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

fn dot<V: Index<usize, Output = T> + VecN<T>, T: Number>(v1: &V, v2: &V) -> T {
    let mut r = T::default();
    for i in 0..V::len() {
        r += v1[i] * v2[i];
    }
    r
}

fn main() {
    let v2 : Vec2<f32> = Vec2 {
        x: 2.0,
        y: 3.0
    };

    let result = dot(&v2, &v2);

    println!("result = {}", result);
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