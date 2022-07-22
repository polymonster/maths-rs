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

/// returns the distance to the plane define by a point on the plane x and normal of the plane n
pub fn plane_distance<T: SignedNumber, V: VecN<T>>(x: V, n: V) -> T {
    -V::dot(x, n)
}

/// return the distance to the plane from point p where the plane is defined by point on plane x and normal n
pub fn point_plane_distance<T: SignedNumber, V: VecN<T>>( p: V, x: V, n: V) -> T {
    V::dot(p, n) - V::dot(x, n)
}

/// returns the distance that point p is from the line segment defined by l1-l2
pub fn point_line_segment_distance<T: Float + FloatOps<T>, V: VecFloatOps<T> + VecN<T>>(p: V, l1: V, l2: V) -> T {
    
    let dx = l2 - l1;
    let m2 = mag2(dx);

    // find parameter value of closest point on segment
    let s12 = saturate(dot(l2 - p, dx) / m2);

    // and find the distance
    return dist(p, l1 * s12 + l2 * (T::one() - s12));
}

/// projects point p along line l1-l2 to return distance t along the line, the value is not clamped to the line segment extents
pub fn distance_on_line<T: Float + FloatOps<T>, V: VecFloatOps<T> + VecN<T>>(p: V, l1: V, l2: V) -> T {
    let v1 = p - l1;
    let v2 = V::normalize(l2 - l1);
    dot(v2, v1)
}

// returns the distance that point p is from an aabb defined by aabb_min -> aabb_max
pub fn point_aabb_distance<T: Float, V: VecN<T> + NumberOps<T> + VecFloatOps<T>>(p: V, aabb_min: V, aabb_max: V) -> T {
    dist(closest_point_on_aabb(p, aabb_min, aabb_max), p)
}

/// find the distance point p is from a triangle defined by t1-t2-t3
pub fn point_triangle_distance<T: Float + FloatOps<T> + NumberOps<T>, V: VecN<T> + NumberOps<T> + VecFloatOps<T>>(p: V, t1: V, t2: V, t3: V) -> T {
    // first find barycentric coordinates of closest point on infinite plane
    let x13 = t1 - t3;
    let x23 = t2 - t3;
    let x03 = p - t3;
    let m13 = mag2(x13);
    let m23 = mag2(x23);
    let d = dot(x13, x23);
    let invdet = T::one() / T::max(m13 * m23 - d * d, T::small_epsilon());
    let a = dot(x13, x03);
    let b = dot(x23, x03);
    // the barycentric coordinates themselves
    let w23 = invdet * (m23 * a - d * b);
    let w31 = invdet * (m13 * b - d * a);
    let w12 = T::one() - w23 - w31;
    if w23 >= T::zero() && w31 >= T::zero() && w12 >= T::zero() {
        // if we're inside the triangle
        dist(p, t1 * w23 + t2 * w31 + t3 * w12)
    }
    else {
        // clamp to one of the edges
        if w23 > T::zero() {
            // this rules out edge 2-3 for us
            T::min(point_line_segment_distance(p, t1, t2), point_line_segment_distance(p, t1, t3))
        }
        else if w31 > T::zero() {
            // this rules out edge 1-3
            T::min(point_line_segment_distance(p, t1, t2), point_line_segment_distance(p, t2, t3))
        }
        else {
            // w12 must be >0, ruling out edge 1-2
            T::min(point_line_segment_distance(p, t1, t3), point_line_segment_distance(p, t2, t3))
        }
    }
}

/// returns the closest point on the line l1-l2 to point p
pub fn closest_point_on_line_segment<T: Float, V: VecFloatOps<T> + VecN<T>>(p: V, l1: V, l2: V) -> V {
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
pub fn closest_point_on_plane<T: SignedNumber, V: VecN<T> + SingedVecN<T>>(p: V, x: V, n: V) -> V {
    p - n * (V::dot(p, n) - V::dot(x, n))
}

/// returns the closest point on the AABB defined by aabb_min and aabb_max to point p
pub fn closest_point_on_aabb<T: Float, V: NumberOps<T> + VecN<T>>(p: V, aabb_min: V, aabb_max: V) -> V {
    V::min(V::max(p, aabb_min), aabb_max)
}

/// returns the closest point from p on sphere or circle s with radius r
pub fn closest_point_on_sphere<T: Float, V: VecFloatOps<T> + VecN<T>>(p: V, s: V, r: T) -> V {
    s + V::normalize(p - s) * r
}

/// returns the closest point to p on the line the ray r0 with diection rv
pub fn closest_point_on_ray<T: Float, V: VecFloatOps<T> + VecN<T>>(p: V, r0: V, rv: V) -> V {
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
pub fn closest_point_on_obb<T: Float, V: VecFloatOps<T> + NumberOps<T> + SignedNumberOps<T> + VecN<T> + SingedVecN<T>, M: MatTranslate<V> + MatInverse<T> + std::ops::Mul<V, Output=V>>(p: V, mat: M) -> V {
    let invm = M::inverse(&mat);
    let tp = invm * p;
    let cp = closest_point_on_aabb(tp, V::minus_one(), V::one());
    let tcp = mat * cp;
    tcp
}

/// find the closest point to p on the triangle defined by t1-t2-t3
pub fn closest_point_on_triangle<T: Float + FloatOps<T> + NumberOps<T>, V: VecN<T> + NumberOps<T> + VecFloatOps<T>>(p: V, t1: V, t2: V, t3: V) -> V {
    // first find barycentric coordinates of closest point on infinite plane
    let x13 = t1 - t3;
    let x23 = t2 - t3;
    let x03 = p - t3;
    let m13 = mag2(x13);
    let m23 = mag2(x23);
    let d = dot(x13, x23);
    let invdet = T::one() / T::max(m13 * m23 - d * d, T::small_epsilon());
    let a = dot(x13, x03);
    let b = dot(x23, x03);
    // the barycentric coordinates themselves
    let w23 = invdet * (m23 * a - d * b);
    let w31 = invdet * (m13 * b - d * a);
    let w12 = T::one() - w23 - w31;
    if w23 >= T::zero() && w31 >= T::zero() && w12 >= T::zero() {
        p
    }
    else {
        // clamp to one of the edges
        if w23 > T::zero() {
            // this rules out edge 2-3 for us
            let p1 = closest_point_on_line_segment(p, t1, t2);
            let p2 = closest_point_on_line_segment(p, t1, t3);
            if dist2(p, p1) < dist2(p, p2) {
                p1
            }
            else {
                p2
            }
        }
        else if w31 > T::zero() {
            // this rules out edge 1-3
            let p1 = closest_point_on_line_segment(p, t1, t2);
            let p2 = closest_point_on_line_segment(p, t2, t3);
            if dist2(p, p1) < dist2(p, p2) {
                p1
            }
            else {
                p2
            }
        }
        else {
            // w12 must be >0, ruling out edge 1-2
            let p1 = closest_point_on_line_segment(p, t1, t3);
            let p2 = closest_point_on_line_segment(p, t2, t3);
            if dist2(p, p1) < dist2(p, p2) {
                p1
            }
            else {
                p2
            }
        }
    }
}

/// returns true if point p is inside the aabb defined by aabb_min and aabb_max
pub fn point_inside_aabb<T: Float, V: VecFloatOps<T> + VecN<T>>(p: V, aabb_min: V, aabb_max: V) -> bool {
    for i in 0..V::len() {
        if p[i] < aabb_min[i] || p[i] > aabb_max[i] {
            return false;
        }
    }
    true
}

/// returns true if sphere with centre s and radius r contains point p
pub fn point_inside_sphere<T: Float, V: VecFloatOps<T> + VecN<T>>(p: V, s: V, r: T) -> bool {
    dist2(p, s) < r * r
}

/// returns true if the point p is inside the obb defined by mat which will transform an aabb centred at 0 with extents -1 to 1 into an obb
pub fn point_inside_obb<T: Float, V: VecFloatOps<T> + NumberOps<T> + SignedNumberOps<T> + VecN<T> + SingedVecN<T>, M: MatTranslate<V> + MatInverse<T> + std::ops::Mul<V, Output=V>>(p: V, mat: M) -> bool {
    let invm = mat.inverse();
    let tp = invm * p;
    point_inside_aabb(tp, V::minus_one(), V::one())
}

/// returns true if the point p is inside the triangle defined by t1-t2-t3
pub fn point_inside_triangle<T: Float + FloatOps<T> + NumberOps<T>, V: VecN<T> + NumberOps<T> + VecFloatOps<T>>(p: V, t1: V, t2: V, t3: V) -> bool {
    // first find barycentric coordinates of closest point on infinite plane
    let x13 = t1 - t3;
    let x23 = t2 - t3;
    let x03 = p - t3;
    let m13 = mag2(x13);
    let m23 = mag2(x23);
    let d = dot(x13, x23);
    let invdet = T::one() / T::max(m13 * m23 - d * d, T::small_epsilon());
    let a = dot(x13, x03);
    let b = dot(x23, x03);
    // the barycentric coordinates themselves
    let w23 = invdet * (m23 * a - d * b);
    let w31 = invdet * (m13 * b - d * a);
    let w12 = T::one() - w23 - w31;
    if w23 >= T::zero() && w31 >= T::zero() && w12 >= T::zero() {
        true
    }
    else {
        false
    }
}

// return true if point p is inside cone defined by position cp facing direction cv with height h and radius r
pub fn point_inside_cone<T: Float + FloatOps<T> + NumberOps<T>, V: VecN<T> + VecFloatOps<T>>(p: V, cp: V, cv: V, h: T, r: T) -> bool {
    let l2 = cp + cv * h;
    let dh = distance_on_line(cp, l2, p) / h;
    let x0 = closest_point_on_line_segment(cp, l2, p);
    let d = dist(x0, p);
    if d < dh * r && dh < T::one() {
        true
    }
    else {
        false
    }
}

// TODO:
// distance on line test
// point inside cone test

// TODO c++
// point plane distance
// point sphere distance
// fix point inside triangle, closest point on triangle + tests