/// numerical traits for numbers, signed numbers, integers and floats
pub mod num;

/// multi dimensional vector with vec2, vec3 and vec4
pub mod vec;

/// multi dimensional matrix with mat2, mat3, mat34 and mat4
pub mod mat;

/// quaternion
pub mod quat;

use mat::*;
use vec::*;
use num::*;

/// returns minimum of a and b
pub fn min<T: Number, V: NumberOps<T>>(a: V, b: V) -> V {
    V::min(a, b)
}

/// returns maximum of a and b
pub fn max<T: Number, V: NumberOps<T>>(a: V, b: V) -> V {
    V::max(a, b)
}

/// returns value x clamped to the range of min and max
pub fn clamp<T: Number, V: NumberOps<T>>(x: V, min: V, max: V) -> V {
    V::clamp(x, min, max)
}

/// returns 1 if a > b or 0 otherwise
pub fn step<T: Number, V: NumberOps<T>>(a: V, b: V) -> V {
    V::step(a, b)
}

/// returns -1 if number is negative, 1 if positive and 0 if zero (integers)
pub fn sign<T: SignedNumber, V: SignedNumberOps<T>>(a: V) -> V {
    V::sign(a)
}

/// return the absolute (positive) value of a 
pub fn abs<T: SignedNumber, V: SignedNumberOps<T>>(a: V) -> V {
    V::abs(a)
}

/// convert degrees to radians
pub fn deg_to_rad<T: Float, V: FloatOps<T>>(a: V) -> V {
    V::deg_to_rad(a)
}

/// convert radians to degress
pub fn rad_to_deg<T: Float, V: FloatOps<T>>(a: V) -> V {
    V::deg_to_rad(a)
}

/// floors value (round down to nearest integer)
pub fn floor<T: Float, V: FloatOps<T>>(a: V) -> V {
    V::floor(a)
}

/// ceils value (round up to nearest integer)
pub fn ceil<T: Float, V: FloatOps<T>>(a: V) -> V {
    V::ceil(a)
}

/// round to closest integer floor if a < 0.5 or ceil if a >= 0.5
pub fn round<T: Float, V: FloatOps<T>>(a: V) -> V {
    V::round(a)
}

/// return square root of a
pub fn sqrt<T: Float, V: FloatOps<T>>(a: V) -> V {
    V::sqrt(a)
}

/// return a raised to the integer power b
pub fn powi<T: Float, V: FloatOps<T>>(a: V, b: i32) -> V {
    V::powi(a, b)
}

/// return a raised to the floating point power b
pub fn powf<T: Float, V: FloatOps<T>>(a: V, b: T) -> V {
    V::powf(a, b)
}

/// return value t linearly interpolated between edge e0 and e1
pub fn lerp<T: Float, V: FloatOps<T>>(e0: V, e1: V, t: T) -> V {
    V::lerp(e0, e1, t)
}

/// return hermite interpolated value t between edge e0 and e1
pub fn smoothstep<T: Float, V: FloatOps<T>>(e0: V, e1: V, t: T) -> V {
    V::smoothstep(e0, e1, t)
}

/// saturates value to 0-1 range, this is the same as clamp(x, 0.0, 1.0)
pub fn saturate<T: Float, V: FloatOps<T>>(x: V) -> V {
    V::saturate(x)
}

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

/// returns the closest point to p on the line the ray r0 with diection rv
pub fn closest_point_on_ray<T: Float, V: VecFloatOps<T> + VecN<T>>(r0: V, rv: V, p: V) -> V {
    let v1 = p - r0;
    let t = dot(v1, rv);
    if t < T::zero() {
        r0
    }
    else {
        r0 + rv * t
    }
}

/// returns the closest point to point p on the obb defined by mat. mat will transform an aabb centred at 0 with extents -1 to 1 into an obb
pub fn closest_point_on_obb<T: Float, V: VecFloatOps<T> + NumberOps<T> + VecN<T>, M: MatTranslate<T> + MatInverse<T> + std::ops::Mul<V, Output=V>>(mat: M, p: V) -> V {
    let invm = M::inverse(&mat);
    let tp = invm * p;
    let cp = closest_point_on_aabb(tp, V::one(), V::one());
    mat * cp
}