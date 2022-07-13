pub mod num;
pub mod vec;
pub mod mat;
pub mod quat;
pub mod generics;

use vec::*;
use num::*;

pub fn closest_point_on_line<T: Float, V: VecFloatOps<T> + VecN<T>>(l1: V, l2: V, p: V) -> V {
    let v1 = p - l1;
    let v2 = V::normalize(l2 - l1);
    let t = V::dot(v2, v1);
    let d = V::dist(l1, l2);
    if t < T::zero() {
        l1
    }
    else if t > d {
        l2
    }
    else {
        l1 + (v2 * t)
    }
}