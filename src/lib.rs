pub mod num;
pub mod vec;
pub mod mat;
pub mod quat;
pub mod generics;

use vec::*;
use num::*;

/// returns the closest point on the line l1-l2 to point p
pub fn closest_point_on_line<T: Float, V: VecFloatOps<T> + VecN<T>>(l1: V, l2: V, p: V) -> V {
    let v1 = p - l1;
    let v2 = V::normalize(l2 - l1);
    let t = V::dot(v2, v1);
    if t < T::zero() {
        l1
    }
    else if t > V::dist(l1, l2) {
        l2
    }
    else {
        l1 + (v2 * t)
    }
}

/// returns the closest point on the AABB defined by aabb_min->aabb_max to point p
pub fn closest_point_on_aabb<T: Float, V: NumberOps<T> + VecN<T>>(aabb_min: V, aabb_max: V, p: V) -> V {
    V::min(V::max(p, aabb_min), aabb_max)
}

/// returns the closest point from p on sphere or circle s with radius r
pub fn closest_point_on_sphere<T: Float, V: VecFloatOps<T> + VecN<T>>(s: V, r: T, p: V) -> V {
    s + V::normalize(p - s) * r
}