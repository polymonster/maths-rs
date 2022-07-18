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

/// return true if a is approximately equal to b within the specified epsilon
pub fn approx<T: Float, V: FloatOps<T>>(a: V, b: V, eps: T) -> bool {
    V::approx(a, b, eps)
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

/// return 3 dimensional vector cross product of a x b
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

/// vector dot product between a . b returing a scalar value
pub fn dot<T: Number, V: VecN<T>>(a: V, b: V) -> T {
    V::dot(a, b)
}

/// returns scalar magnitude or length of vector
pub fn length<T: Float, V: VecFloatOps<T>>(a: V) -> T {
    V::length(a)
}

/// returns scalar magnitude or length of vector
pub fn mag<T: Float, V: VecFloatOps<T>>(a: V) -> T {
    V::mag(a)
}

/// returns scalar magnitude or length of vector squared to avoid using sqrt
pub fn mag2<T: Float, V: VecFloatOps<T>>(a: V) -> T {
    V::mag2(a)
}

/// returns a normalized unit vector of a
pub fn normalize<T: Float, V: VecFloatOps<T>>(a: V) -> V {
    V::normalize(a)
}

/// returns scalar distance between 2 points (magnitude of the vector between the 2 points)
pub fn distance<T: Float, V: VecFloatOps<T>>(a: V, b: V) -> T {
    V::distance(a, b)
}

/// returns scalar distance between 2 points (magnitude of the vector between the 2 points)
pub fn dist<T: Float, V: VecFloatOps<T>>(a: V, b: V) -> T {
    V::dist(a, b)
}

/// returns scalar squared distance between 2 points to avoid using sqrt
pub fn dist2<T: Float, V: VecFloatOps<T>>(a: V, b: V) -> T {
    V::dist2(a, b)
}

/*
// get distance to plane x defined by point on plane x0 and normal of plane xN
maths_inline f32 plane_distance(const vec3f& x0, const vec3f& xN)
{
    return dot(xN, x0) * -1.0f;
}

// get distance from point p0 to plane defined by point x0 and normal xN
maths_inline f32 point_plane_distance(const vec3f& p0, const vec3f& x0, const vec3f& xN)
{
    f32 d = plane_distance(x0, xN);
    return dot(p0, xN) + d;
}

// returns the intersection point of ray defined by origin r0 and direction rV,
// with plane defined by point on plane x0 normal of plane xN
inline vec3f ray_plane_intersect(const vec3f& r0, const vec3f& rV, const vec3f& x0, const vec3f& xN)
{
    f32 d = plane_distance(x0, xN);
    f32 t = -(dot(r0, xN) + d) / dot(rV, xN);
    
    return r0 + (rV * t);
}
*/

/// returns the distance to the plane define by a point on the plane x and normal of the plane n
pub fn plane_distance<T: SignedNumber, V: VecN<T>>(x: V, n: V) -> T {
    -V::dot(x, n)
}

/// return the distance to the plane from point p where the plane is defined by point on plane x and normal n
pub fn point_plane_distance<T: SignedNumber, V: VecN<T>>(x: V, n: V, p: V) -> T {
    V::dot(p, n) - V::dot(x, n)
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

/// returns the closest point on the plane defined by point on plane x and normal n to point p
pub fn closest_point_on_plane<T: SignedNumber, V: VecN<T> + SingedVecN<T>>(x: V, n: V, p: V) -> V {
    p - n * (V::dot(p, n) - V::dot(x, n))
}

/// returns the closest point on the AABB defined by aabb_min and aabb_max to point p
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

/// returns the closest point to point p on the obb defined by mat which will transform an aabb centred at 0 with extents -1 to 1 into an obb
pub fn closest_point_on_obb<T: Float, V: VecFloatOps<T> + NumberOps<T> + SignedNumberOps<T> + VecN<T> + SingedVecN<T>, M: MatTranslate<V> + MatInverse<T> + std::ops::Mul<V, Output=V>>(mat: M, p: V) -> V {
    let invm = M::inverse(&mat);
    let tp = invm * p;
    let cp = closest_point_on_aabb(V::minus_one(), V::one(), tp);
    let tcp = mat * cp;
    tcp
}

/*
// TODO: requires point inside
/// returns the cloest point on triangle v1-v2-v3 to point p
/// side is 1 or -1 depending on whether the point is infront or behind the triangle
pub fn closest_point_on_triangle<T: Float>(t0: Vec3<T>, t1: Vec3<T>, t2: Vec3<T>, p: Vec3<T>) {

}
*/

/// returns true if point p is inside the aabb defined by min and max
pub fn point_inside_aabb<T: Float, V: VecFloatOps<T> + VecN<T>>(min: V, max: V, p: V) -> bool {
    for i in 0..V::len() {
        if p[i] < min[i] || p[i] > max[i] {
            return false;
        }
    }
    true
}

/// returns true if sphere with centre s and radius r contains point p
pub fn point_inside_sphere<T: Float, V: VecFloatOps<T> + VecN<T>>(s: V, r: T, p: V) -> bool {
    dist2(p, s) < r * r
}

/// returns true if the point p is inside the obb defined by mat which will transform an aabb centred at 0 with extents -1 to 1 into an obb
pub fn point_inside_obb<T: Float, V: VecFloatOps<T> + NumberOps<T> + SignedNumberOps<T> + VecN<T> + SingedVecN<T>, M: MatTranslate<V> + MatInverse<T> + std::ops::Mul<V, Output=V>>(mat: M, p: V) -> bool
{
    let invm = mat.inverse();
    let tp = invm * p;
    point_inside_aabb(V::minus_one(), V::one(), tp)
}