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

/// classification for tests vs planes
#[derive(PartialEq, Debug)]
pub enum Classification {
    /// behind the plane in the opposite direction of the planes normal
    Behind,
    /// infront of the plane in the direction of the planes normal
    Infront,
    /// intersecting the plane
    Intersects,
}

/// returns the minimum of a and b
pub fn min<T: Number, V: NumberOps<T>>(a: V, b: V) -> V {
    V::min(a, b)
}

/// returns the maximum of a and b
pub fn max<T: Number, V: NumberOps<T>>(a: V, b: V) -> V {
    V::max(a, b)
}

/// returns the value x clamped to the range of min and max
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

/// returns the absolute (positive) value of a 
pub fn abs<T: SignedNumber, V: SignedNumberOps<T>>(a: V) -> V {
    V::abs(a)
}

/// returns the radian value converted from a which is specificied in degrees
pub fn deg_to_rad<T: Float, V: FloatOps<T>>(a: V) -> V {
    V::deg_to_rad(a)
}

/// returns the degree value converted from a which is specificied in radians
pub fn rad_to_deg<T: Float, V: FloatOps<T>>(a: V) -> V {
    V::deg_to_rad(a)
}

/// returns the floored value of a (round down to nearest integer)
pub fn floor<T: Float, V: FloatOps<T>>(a: V) -> V {
    V::floor(a)
}

/// returns the ceil'd value of a(round up to nearest integer)
pub fn ceil<T: Float, V: FloatOps<T>>(a: V) -> V {
    V::ceil(a)
}

/// returns the value a rounded to closest integer floor if a < 0.5 or ceil if a >= 0.5
pub fn round<T: Float, V: FloatOps<T>>(a: V) -> V {
    V::round(a)
}

/// returns true if a is approximately equal to b within the specified epsilon
pub fn approx<T: Float, V: FloatOps<T>>(a: V, b: V, eps: T) -> bool {
    V::approx(a, b, eps)
}

/// returns the square root of a
pub fn sqrt<T: Float, V: FloatOps<T>>(a: V) -> V {
    V::sqrt(a)
}

/// returns a raised to the integer power b
pub fn powi<T: Float, V: FloatOps<T>>(a: V, b: i32) -> V {
    V::powi(a, b)
}

/// returns a raised to the floating point power b
pub fn powf<T: Float, V: FloatOps<T>>(a: V, b: T) -> V {
    V::powf(a, b)
}

/// returns value t linearly interpolated between edge e0 and e1
pub fn lerp<T: Float, V: FloatOps<T>>(e0: V, e1: V, t: T) -> V {
    V::lerp(e0, e1, t)
}

/// returns the hermite interpolated value t between edge e0 and e1
pub fn smoothstep<T: Float, V: FloatOps<T>>(e0: V, e1: V, t: T) -> V {
    V::smoothstep(e0, e1, t)
}

/// returns the saturated value of x into to the 0-1 range, this is the same as clamp(x, 0.0, 1.0)
pub fn saturate<T: Float, V: FloatOps<T>>(x: V) -> V {
    V::saturate(x)
}

/// returns the vector cross product of a x b, makes sense only for 3 or 7 dimensional vectors 
pub fn cross<T: Number, V: VecCross<T>>(a: V, b: V) -> V {
    V::cross(a, b)
}

/// returns the perpedicular vector of a performing anti-clockwise rotation by 90 degrees
pub fn perp<T: SignedNumber>(a: Vec2<T>) -> Vec2<T> {
    Vec2 {
        x: -a.y, 
        y: a.x
    }
}

/// returns the vector dot product between a . b
pub fn dot<T: Number, V: VecN<T>>(a: V, b: V) -> T {
    V::dot(a, b)
}

/// returns the scalar magnitude or length of vector
pub fn length<T: Float, V: VecFloatOps<T>>(a: V) -> T {
    V::length(a)
}

/// returns the scalar magnitude or length of vector
pub fn mag<T: Float, V: VecFloatOps<T>>(a: V) -> T {
    V::mag(a)
}

/// returns the scalar magnitude or length of vector squared to avoid using sqrt
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

/// returns the barycentric coordinate (u, v, w) of point p inside triangle t1-t2-t3
pub fn barycentric<T: Float + NumberOps<T>, V: VecFloatOps<T> + VecN<T> + NumberOps<T>>(p: V, t1: V, t2: V, t3: V) -> (T, T, T) {
    let x13 = t1 - t3;
    let x23 = t2 - t3;
    let x03 = p - t3;
    let m13 = mag2(x13);
    let m23 = mag2(x23);
    let d = dot(x13, x23);
    let invdet = T::one() / T::max(m13 * m23 - d * d, T::small_epsilon());
    let a = dot(x13, x03);
    let b = dot(x23, x03);
    let u = invdet * (m23 * a - d * b);
    let v = invdet * (m13 * b - d * a);
    let w = T::one() - u - v;
    (u, v, w)
}

/// returns the normalized unit vector normal of triangle t1-t2-t3
pub fn get_triangle_normal<T: Float, V: VecFloatOps<T> + VecN<T> + VecCross<T>>(t1: V, t2: V, t3: V) -> V {
    normalize(cross(t2 - t1, t3 - t1))
}

/// returns the distance to the plane define by a point on the plane x and normal of the plane n
pub fn plane_distance<T: SignedNumber, V: VecN<T>>(x: V, n: V) -> T {
    -V::dot(x, n)
}

/// returns the distance to the plane from point p where the plane is defined by point on plane x and normal n
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

/// returns the distance parameter t of point p projected along the line l1-l2, the value is not clamped to the line segment extents
pub fn distance_on_line<T: Float, V: VecFloatOps<T> + VecN<T>>(p: V, l1: V, l2: V) -> T {
    let v1 = p - l1;
    let v2 = V::normalize(l2 - l1);
    dot(v2, v1)
}

/// returns the distance that point p is from an aabb defined by aabb_min to aabb_max
pub fn point_aabb_distance<T: Float, V: VecN<T> + NumberOps<T> + VecFloatOps<T>>(p: V, aabb_min: V, aabb_max: V) -> T {
    dist(closest_point_on_aabb(p, aabb_min, aabb_max), p)
}

/// returns the distance point p is from a triangle defined by t1-t2-t3
pub fn point_triangle_distance<T: Float + FloatOps<T> + NumberOps<T>, V: VecN<T> + NumberOps<T> + VecFloatOps<T>>(p: V, t1: V, t2: V, t3: V) -> T {
    let (w23, w31, w12) = barycentric(p, t1, t2, t3);
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

/// returns the closest point on the plane to point p wher the plane is defined by point on plane x and normal n
pub fn closest_point_on_plane<T: SignedNumber, V: VecN<T> + SingedVecN<T>>(p: V, x: V, n: V) -> V {
    p - n * (V::dot(p, n) - V::dot(x, n))
}

/// returns the closest point on the aabb defined by aabb_min and aabb_max to point p
pub fn closest_point_on_aabb<T: Float, V: NumberOps<T> + VecN<T>>(p: V, aabb_min: V, aabb_max: V) -> V {
    V::min(V::max(p, aabb_min), aabb_max)
}

/// returns the closest point from p on sphere or circle centred at s with radius r
pub fn closest_point_on_sphere<T: Float, V: VecFloatOps<T> + VecN<T>>(p: V, s: V, r: T) -> V {
    s + V::normalize(p - s) * r
}

/// returns the closest point to p on the the ray starting at r0 with diection rv
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

/// returns the closest point to p on the triangle defined by t1-t2-t3
pub fn closest_point_on_triangle<T: Float + FloatOps<T> + NumberOps<T>, V: VecN<T> + NumberOps<T> + VecFloatOps<T>>(p: V, t1: V, t2: V, t3: V) -> V {
    let (w23, w31, w12) = barycentric(p, t1, t2, t3);
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

/// find the closest point to p on the cone defined by cp position, with direction cv height h an radius r
pub fn closest_point_on_cone<T: Float, V: VecN<T> + SingedVecN<T> + VecFloatOps<T>>(p: V, cp: V, cv: V, h: T, r: T) -> V {
    let l2 = cp + cv * h;
    let dh = distance_on_line(p, cp, l2) / h;
    let x0 = closest_point_on_line_segment(p, cp, l2);
    let d = dist(x0, p);
    if dh >= T::one() {
        // clamp to the tip
        l2
    }
    else if dh <= T::zero() {
        // clamp to the base
        // base plane
        let pp = closest_point_on_plane(p, cp, cv);
        let vv = pp - x0;
        let m = mag(pp - x0);
        if m < r {
            pp
        }
        else {
            let v = vv / m;
            x0 + v * r
        }
    }
    else if d < dh * r {
        // inside the code
        p
    }
    else {
        let v = normalize(p - x0);
        // clamp to the radius
        x0 + (v * dh * r)
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

/// returns true if sphere (or cirlcle) with centre s and radius r contains point p
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
    let (w23, w31, w12) = barycentric(p, t1, t2, t3);
    if w23 >= T::zero() && w31 >= T::zero() && w12 >= T::zero() {
        true
    }
    else {
        false
    }
}

/// returns true if point p is inside cone defined by position cp facing direction cv with height h and radius r
pub fn point_inside_cone<T: Float + FloatOps<T> + NumberOps<T>, V: VecN<T> + VecFloatOps<T>>(p: V, cp: V, cv: V, h: T, r: T) -> bool {
    let l2 = cp + cv * h;
    let dh = distance_on_line(p, cp, l2) / h;
    let x0 = closest_point_on_line_segment(p, cp, l2);
    let d = dist(x0, p);
    if d < dh * r && dh < T::one() {
        true
    }
    else {
        false
    }
}

/// returns true if the point p is inside the 2D convex hull defined by point list 'hull' with clockwise winding
pub fn point_inside_convex_hull<T: Float>(p: Vec2<T>, hull: Vec<Vec2<T>>) -> bool
{
    let p0 = Vec3::from((p, T::zero()));
    let ncp = hull.len();
    for i in 0..ncp {
        let i2 = (i+1)%ncp;
        let p1 = Vec3::from((hull[i], T::zero()));
        let p2 = Vec3::from((hull[i2], T::zero()));
        let v1 = p2 - p1;
        let v2 = p0 - p1;
        if cross(v1, v2).z > T::zero() {
            return true;
        }
    }
    true
}

/// returns true if point p is inside the polygon defined by point list 'poly'
pub fn point_inside_poly<T: Float>(p: Vec2<T>, poly: Vec<Vec2<T>>) -> bool
{
    // copyright (c) 1970-2003, Wm. Randolph Franklin
    // https://wrf.ecse.rpi.edu/Research/Short_Notes/pnpoly.html
    let npol = poly.len();
    let mut c = false;
    let mut j = npol-1;
    for i in 0..npol {
        if (((poly[i].y <= p.y) && (p.y < poly[j].y)) || ((poly[j].y <= p.y) && (p.y < poly[i].y))) &&
            (p.x < (poly[j].x - poly[i].x) * (p.y - poly[i].y) / (poly[j].y - poly[i].y) + poly[i].x) {
            c = !c;
        }
        j = i
    }
    c
}

/// returns the classification of the 3D aabb defined as aabb_min to aabb_max vs the plane defined by point on plane x and normal n
pub fn aabb_vs_plane<T: SignedNumber + SignedNumberOps<T>>(aabb_min: Vec3<T>, aabb_max: Vec3<T>, x: Vec3<T>, n: Vec3<T>) -> Classification {
    let e = (aabb_max - aabb_min) / T::two();
    let centre = aabb_min + e;
    let radius = abs(n.x * e.x) + abs(n.y * e.y) + abs(n.z * e.z);
    let pd = plane_distance(x, n);
    let d = dot(n, centre) + pd;
    if d > radius {
        Classification::Infront
    }
    else if d < -radius {
        Classification::Behind
    }
    else {
        Classification::Intersects
    }
}

/// returns the classification of sphere defined by centre s and radius r vs the plane defined by point on plane x and normal n
pub fn sphere_vs_plane<T: SignedNumber + SignedNumberOps<T>>(s: Vec3<T>, r: T,  x: Vec3<T>, n: Vec3<T>) -> Classification {
    let pd = plane_distance(x, n);
    let d = dot(n, s) + pd;
    if d > r {
        Classification::Infront
    }
    else if d < -r {
        Classification::Behind
    }
    else {
        Classification::Intersects
    }
}

/// returns the intersection point of the ray defined as origin of ray r0 and direction rv with the plane defined by point on plane x and normal n
pub fn ray_vs_plane<T: Float + SignedNumberOps<T>>(r0: Vec3<T>, rv: Vec3<T>, x: Vec3<T>, n: Vec3<T>) -> Option<Vec3<T>> {
    let t = -(dot(r0, n) - dot(x, n))  / dot(rv, n);
    if t < T::zero() {
        None
    }
    else {
        Some(r0 + rv * t)
    }
}

/// returns the intersection point of the bidirectional ray defined as point on ray r0 and direction rv with the plane defined by point on plane x and normal n
pub fn bidirectional_ray_vs_plane<T: Float + SignedNumberOps<T>>(r0: Vec3<T>, rv: Vec3<T>, x: Vec3<T>, n: Vec3<T>) -> Option<Vec3<T>> {
    let r = dot(rv, n);
    if r == T::zero() {
        None
    }
    else {
        let t = -(dot(r0, n) - dot(x, n))  / dot(rv, n);
        Some(r0 + rv * t)
    }
}

/// returns the intersection point of the line l1-l2 and plane defined by point on plane x and normal n, returns None if no intersection exists
pub fn line_vs_plane<T: Float + FloatOps<T> + SignedNumber + SignedNumberOps<T>>(l1: Vec3<T>, l2: Vec3<T>, x: Vec3<T>, n: Vec3<T>) -> Option<Vec3<T>> {
    let ll = l2-l1;
    let m = mag(ll);
    let lv = ll / m;
    let t = -(dot(l1, n) - dot(x, n))  / dot(lv, n);
    if t < T::zero() || t > m {
        None
    }
    else {
        Some(l1 + lv * t)
    }
}

/// returns true if the sphere or circle at centre s1 with radius r1 intsercts s2-r2
pub fn sphere_vs_sphere<T: Float, V: VecN<T> + VecFloatOps<T>>(s1: V, r1: T, s2: V, r2: T) -> bool {
    let d2 = dist2(s1, s2);
    let r22 = r1 + r2;
    d2 < r22 * r22
}

/// returns ture if the aabb defined by aabb_min to aabb_max intersects the sphere (or circle) centred at s with radius r
pub fn aabb_vs_sphere<T: Float, V: VecN<T> + VecFloatOps<T> + NumberOps<T>>(aabb_min: V, aabb_max: V, s: V, r: T) -> bool {
    let cp = closest_point_on_aabb(s, aabb_min, aabb_max);
    dist2(cp, s) < r * r
}

/// returns true if the aabb defined by aabb_min1 to aabb_max1 intersects aabb_min2-aabb_max2
pub fn aabb_vs_aabb<T: Number, V: VecN<T> + NumberOps<T>>(aabb_min1: V, aabb_max1: V, aabb_min2: V, aabb_max2: V) -> bool {
    for i in 0..V::len() {
        if aabb_max1[i] < aabb_min2[i] || aabb_min1[i] > aabb_max2[i] {
            return false;
        } 
    }
    true
}

// line_vs_sphere
// ray_vs_sphere
// line_vs_aabb
// ray_vs_aabb
// ray_vs_triangle

// convex_hull_from_points
// closest point on hull
// closest point on poly
// point hull distance
// point poly distance
// point cone distance

// aabb_vs_aabb

// TODO: new
// line_vs_cone
// ray_vs_cone
// cone_vs_sphere
// cone_vs_plane
// cone_vs_aabb
// capsules?

// mat
// ortho basis frivs + huges

// utils
// hsv etc

// TODO:
// point inside hull (test)
// point inside poly (test)

// TODO c++
// point inside cone test is whack
// point plane distance
// point sphere distance
// fix point inside triangle, closest point on triangle + tests
// closest point on cone