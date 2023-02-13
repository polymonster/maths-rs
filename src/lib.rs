/// base traits and operations for scalar numbers, signed numbers, integers and floats
pub mod num;

/// multi dimensional column vector with generic vec2, vec3 and vec4 implementations
pub mod vec;

/// multi dimensional row-major matrix with generic mat2, mat3, mat34 and mat4 implementations
pub mod mat;

/// generic quaternion for varying floating point precision
pub mod quat;

/// module containing vector swizzling traits
pub mod swizz;

use num::*;
use vec::*;
use mat::*;
use quat::*;

#[cfg(feature = "short_types")]
pub type Vec2f = Vec2<f32>;
pub type Vec3f = Vec3<f32>;
pub type Vec4f = Vec4<f32>;
pub type Vec2d = Vec2<f64>;
pub type Vec3d = Vec3<f64>;
pub type Vec4d = Vec4<f64>;
pub type Vec2i = Vec2<i32>;
pub type Vec3i = Vec3<i32>;
pub type Vec4i = Vec4<i32>;
pub type Vec2u = Vec2<u32>;
pub type Vec3u = Vec3<u32>;
pub type Vec4u = Vec4<u32>;
pub type Mat2f = Mat2<f32>;
pub type Mat3f = Mat3<f32>;
pub type Mat34f = Mat34<f32>;
pub type Mat4f = Mat4<f32>;
pub type Mat2d = Mat2<f64>;
pub type Mat3d = Mat3<f64>;
pub type Mat34d = Mat34<f64>;
pub type Mat4d = Mat4<f64>;
pub type Quatf = Quat<f32>;
pub type Quatd = Quat<f64>;

/// classification for tests vs planes (behind, infront or intersects)
#[derive(PartialEq, Debug)]
pub enum Classification {
    /// behind the plane in the opposite direction of the planes normal
    Behind,
    /// infront of the plane in the direction of the planes normal
    Infront,
    /// intersecting the plane
    Intersects,
}

/// returns the minimum of `a` and `b`
pub fn min<T: Number, V: NumberOps<T>>(a: V, b: V) -> V {
    V::min(a, b)
}

/// returns the maximum of `a` and `b`
pub fn max<T: Number, V: NumberOps<T>>(a: V, b: V) -> V {
    V::max(a, b)
}

/// returns the value `x` clamped to the range of `min` and `max`
pub fn clamp<T: Number, V: NumberOps<T>>(x: V, min: V, max: V) -> V {
    V::clamp(x, min, max)
}

/// returns 1 if `a > b` or 0 otherwise
pub fn step<T: Number, V: NumberOps<T>>(a: V, b: V) -> V {
    V::step(a, b)
}

/// returns -1 if value `a` is negative, 1 if positive and 0 if zero (integers)
pub fn signum<T: SignedNumber, V: SignedNumberOps<T>>(a: V) -> V {
    V::signum(a)
}

/// returns the value `a` with the same sign as second paremeter `sign`
pub fn copysign<T: Float, V: FloatOps<T>>(a: V, sign: T) -> V {
    V::copysign(a, sign)
}

/// returns the absolute (positive) value of `a` 
pub fn abs<T: SignedNumber, V: SignedNumberOps<T>>(a: V) -> V {
    V::abs(a)
}

/// returns the radian value converted from value `a` which is specificied in degrees
pub fn deg_to_rad<T: Float, V: FloatOps<T>>(a: V) -> V {
    V::deg_to_rad(a)
}

/// returns the degree value converted from value `a` which is specificied in radians
pub fn rad_to_deg<T: Float, V: FloatOps<T>>(a: V) -> V {
    V::deg_to_rad(a)
}

/// returns the floored value of `a` (round down to nearest integer)
pub fn floor<T: Float, V: FloatOps<T>>(a: V) -> V {
    V::floor(a)
}

/// returns the ceil'd value of `a` (round up to nearest integer)
pub fn ceil<T: Float, V: FloatOps<T>>(a: V) -> V {
    V::ceil(a)
}

/// returns the value `a` rounded to closest integer floor if `a < 0.5` or ceil if `a >= 0.5`
pub fn round<T: Float, V: FloatOps<T>>(a: V) -> V {
    V::round(a)
}

/// returns true if value `a` is approximately equal to value `b` within the specified epsilon `eps`
pub fn approx<T: Float, V: FloatOps<T>>(a: V, b: V, eps: T) -> bool {
    V::approx(a, b, eps)
}

/// returns the square root of value `a`
pub fn sqrt<T: Float, V: FloatOps<T>>(a: V) -> V {
    V::sqrt(a)
}

/// returns value `a` raised to the integer power of value `b`
pub fn powi<T: Float, V: FloatOps<T>>(a: V, b: i32) -> V {
    V::powi(a, b)
}

/// returns value `a` squared (raised to the power 2)
pub fn sqr<T: Float, V: FloatOps<T> + Base<T>>(a: V) -> V {
    a * a
}

/// returns value `a` cubed (raised to the power 3)
pub fn cube<T: Float, V: FloatOps<T> + Base<T>>(a: V) -> V {
    a * a * a
}

/// returns value `a` raised to the floating point power of value `b`
pub fn powf<T: Float, V: FloatOps<T>>(a: V, b: T) -> V {
    V::powf(a, b)
}

/// returns fused multiply add `m * a + b`
pub fn mad<T: Float, V: FloatOps<T>>(m: V, a: V, b: V) -> V {
    V::mad(m, a, b)
}

/// returns the floating point remainder of `a / b`
pub fn fmod<T: Float, V: FloatOps<T>>(a: V, b: V) -> V {
    V::fmod(a, b)
}

/// returns the fractional part of value `a`
pub fn frac<T: Float, V: FloatOps<T>>(a: V) -> V {
    V::frac(a)
}

/// truncates value `a` - removing the fractional part, truncating to an integer
pub fn trunc<T: Float, V: FloatOps<T>>(a: V) -> V {
    V::trunc(a)
}

/// returns the reciprocal square root of value `a`
pub fn rsqrt<T: Float, V: FloatOps<T>>(a: V) -> V {
    V::rsqrt(a)
}

/// returns the reciprocal of value `a`
pub fn recip<T: Float, V: FloatOps<T>>(a: V) -> V {
    V::recip(a)
}

/// returns the value `v` broken down into a tuple (fractional, integer) parts
pub fn modf<T: Float, V: FloatOps<T>>(v: V) -> (V, V) {
    V::modf(v)
}

/// returns a value interpolated between edges `e0` and `e1` by percentage `t`
pub fn lerp<T: Float, V: FloatOps<T>>(e0: V, e1: V, t: T) -> V {
    V::lerp(e0, e1, t)
}

/// returns a value interpolated between edges `e0` and `e1` by percentage `t` with the result being normalised
pub fn nlerp<T: Float, V: VecFloatOps<T> + Nlerp<T>>(e0: V, e1: V, t: T) -> V {
    V::nlerp(e0, e1, t)
}

/// returns a value spherically interpolated between edges `e0` and `e1` by percentage `t`
pub fn slerp<T: Float + NumberOps<T> + FloatOps<T>, V: Slerp<T>>(e0: V, e1: V, t: T) -> V {
    V::slerp(e0, e1, t)
}

/// returns the hermite interpolated value between edge `e0` and `e1` by percentage `t`
pub fn smoothstep<T: Float, V: FloatOps<T>>(e0: V, e1: V, t: T) -> V {
    V::smoothstep(e0, e1, t)
}

// comment sweep here!

/// returns the saturated value of x into to the 0-1 range, this is the same as clamp(x, 0.0, 1.0)
pub fn saturate<T: Float, V: FloatOps<T>>(x: V) -> V {
    V::saturate(x)
}

/// returns the vector cross product of a x b, makes sense only for 3 or 7 dimensional vectors 
pub fn cross<T: Number, V: Cross<T>>(a: V, b: V) -> V {
    V::cross(a, b)
}

/// returns the scalar triple product of a x b x c, makes sense only for 3 dimensional vectors 
pub fn scalar_triple<T: Number + SignedNumber, V: Triple<T>>(a: V, b: V, c: V) -> T {
    V::scalar_triple(a, b, c)
}

/// returns the vector triple product of a x b x c, mainly used for 3D vectors, but with a 2D specialisation leveraging z-up
pub fn vector_triple<T: Number + SignedNumber, V: Triple<T>>(a: V, b: V, c: V) -> V {
    V::vector_triple(a, b, c)
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
pub fn barycentric<T: Float + NumberOps<T>, V: VecN<T> + VecFloatOps<T> + NumberOps<T>>(p: V, t1: V, t2: V, t3: V) -> (T, T, T) {
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

/// returns an xyz directional unit vector converted from azimuth altitude
pub fn azimuth_altitude_to_xyz<T: Float + FloatOps<T>>(azimuth: T, altitude: T) -> Vec3<T> {
    let z = T::sin(altitude);
    let hyp = T::cos(altitude);
    let y = hyp * T::cos(azimuth);
    let x = hyp * T::sin(azimuth);
    Vec3::<T>::new(x, z, y)
}

/// returns (azimuth, altitude) converted from xyz directional unit vector
pub fn xyz_to_azimuth_altitude<T: Float + FloatOps<T>>(xyz: Vec3<T>) -> (T, T) {
    (T::atan2(xyz.y, xyz.x), T::atan2(xyz.z, sqrt(xyz.x * xyz.x + xyz.y * xyz.y)))
}
   
/// returns a convex hull wound clockwise from point cloud "points"
pub fn convex_hull_from_points<T: Float + SignedNumberOps<T> + NumberOps<T> + FloatOps<T>>(points: &Vec<Vec2<T>>) -> Vec<Vec2<T>> {    
    //find right most point
    let mut cur = points[0];
    let mut curi = 0;
    for (i, item) in points.iter().enumerate().skip(1) {
        if item.x > cur.x || (item.x == cur.x && item.y > cur.y) {
            // pritorites by xmost then y most if == x
            cur = *item;
            curi = i;
        }
    }
    
    // wind the hull clockwise by using cross product to test which side of an edge a point lies on
    // discarding points that do not form the perimeter
    let mut hull = vec![cur];
    loop {
        let mut rm = (curi+1)%points.len();
        let mut x1 = points[rm];

        for (i, item) in points.iter().enumerate() {
            if i == curi {
                continue;
            }
            let x2 = *item;
            let v1 = x1 - cur;
            let v2 = x2 - cur;
            let z = v2.x * v1.y - v2.y * v1.x;
            if z > T::zero() {
                x1 = *item;
                rm = i;
            }
        }

        if approx(x1, hull[0], T::small_epsilon()) {
            break;
        }
            
        cur = x1;
        curi = rm;
        hull.push(x1);
    }
    hull
}

/// returns a plane placked into Vec4 in the form .xyz = plane normal, .w = plane distance / constant from x (point on plane) and n (planes normal)
pub fn plane_from_normal_and_point<T: SignedNumber>(x: Vec3<T>, n: Vec3<T>) -> Vec4<T> {
    Vec4 {
        x: n.x,
        y: n.y,
        z: n.z,
        w: plane_distance(x, n)
    }
}

/// returns the normalized unit vector normal of triangle t1-t2-t3
pub fn get_triangle_normal<T: Float, V: VecFloatOps<T> + VecN<T> + Cross<T>>(t1: V, t2: V, t3: V) -> V {
    normalize(cross(t2 - t1, t3 - t1))
}

/// returns the 3D normalized device coordinate of point p projected by view_projection matrix, perfroming homogenous divide
pub fn project_to_ndc<T: Float>(p: Vec3<T>, view_projection: Mat4<T>) -> Vec3<T> {
    let ndc = view_projection * Vec4::from((p, T::one()));
    Vec3::from(ndc) / ndc.w
}

/// returns the 2D screen coordinate of 3D point p projected with view_projection, performing homogenous divide and viewport correction
/// assumes screen coordinates are vup in the y-axis y.0 = bottom y.height = top
pub fn project_to_sc<T: Float + FloatOps<T>>(p: Vec3<T>, view_projection: Mat4<T>, viewport: Vec2<T>) -> Vec3<T> {
    let ndc = project_to_ndc(p, view_projection);
    let sc  = ndc * T::point_five() + T::point_five();
    Vec3::from((Vec2::<T>::from(sc) * viewport, sc.z))
}

/// returns the 2D screen coordinate of 3D point p projected with view_projection, performing homogenous divide and viewport correction
/// coordinates are vdown in the y-axis vdown = y.0 = top y.height = bottom
pub fn project_to_sc_vdown<T: Float + FloatOps<T>>(p: Vec3<T>, view_projection: Mat4<T>, viewport: Vec2<T>) -> Vec2<T> {
    let ndc = project_to_ndc(p, view_projection);
    let sc  = ndc * Vec3::new(T::point_five(), -T::point_five(), T::point_five()) + T::point_five();
    Vec2::<T>::from(sc) * viewport
}

/// returns the unprojected 3D world position of point p which is specified in normalized device coordinates
pub fn unproject_ndc<T: Float>(p: Vec3<T>, view_projection: Mat4<T>) -> Vec3<T> {
    let inv = view_projection.inverse();
    inv * p
}

/// returns the unprojected 3D world position of screen coordinate p
/// assumes screen coordinates are vup in the y-axis y.0 = bottom y.height = top
pub fn unproject_sc<T: Float>(p: Vec3<T>, view_projection: Mat4<T>, viewport: Vec2<T>) -> Vec3<T> {
    let ndc_xy = (Vec2::from(p) / viewport) * Vec2::from(T::two()) - Vec2::from(T::one());
    let ndc = Vec3::from((ndc_xy, p.z));
    unproject_ndc(ndc, view_projection)
}

/// returns the unprojected 3D world position of screen coordinate p
/// coordinates are vdown in the y-axis vdown = y.0 = top y.height = bottom
pub fn unproject_sc_vdown<T: Float>(p: Vec3<T>, view_projection: Mat4<T>, viewport: Vec2<T>) -> Vec3<T> {
    let ndc_xy = (Vec2::from(p) / viewport) * Vec2::new(T::two(), -T::two()) - Vec2::from(T::one());
    let ndc = Vec3::from((ndc_xy, p.z));
    unproject_ndc(ndc, view_projection)
}

/// returns the distance to the plane define by a point on the plane x and normal of the plane n
pub fn plane_distance<T: SignedNumber, V: VecN<T>>(x: V, n: V) -> T {
    -V::dot(x, n)
}

/// returns the distance parameter t of point p projected along the line l1-l2, the value is not clamped to the line segment extents
pub fn distance_on_line<T: Float, V: VecFloatOps<T> + VecN<T>>(p: V, l1: V, l2: V) -> T {
    let v1 = p - l1;
    let v2 = V::normalize(l2 - l1);
    dot(v2, v1)
}

/// returns the distance parameter t of point p projected along the ray r0 with direction rv, the value is not clamped to 0 or the start of the ray
pub fn distance_on_ray<T: Float, V: VecFloatOps<T> + VecN<T>>(p: V, r0: V, rv: V) -> T {
    dot(p - r0, rv)
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
    dist(p, l1 * s12 + l2 * (T::one() - s12))
}

/// returns the distance that point p is from an aabb defined by aabb_min to aabb_max
pub fn point_aabb_distance<T: Float, V: VecN<T> + NumberOps<T> + VecFloatOps<T>>(p: V, aabb_min: V, aabb_max: V) -> T {
    dist(closest_point_on_aabb(p, aabb_min, aabb_max), p)
}

// returns the unsigned distance from point p0 to the sphere (or 2d circle) centred at s0 with radius r
pub fn point_sphere_distance<T: Float, V: VecN<T> + NumberOps<T> + VecFloatOps<T>>(p0: V, s0: V, r: T) -> T {
    dist(p0, closest_point_on_sphere(p0, s0, r))
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

/// returns the distance from point p to the cone defined by position cp, with height h and radius at the base of r
pub fn point_cone_distance<T: Float, V: VecN<T> + SignedVecN<T> + VecFloatOps<T>>(p: V, cp: V, cv: V, h: T, r: T) -> T {
    dist(p, closest_point_on_cone(p, cp, cv, h, r))
}

/// returns the distance from point p to the edge of the convex hull defined by point list 'hull' with clockwise winding
pub fn point_convex_hull_distance<T: Float + FloatOps<T>>(p: Vec2<T>, hull: &Vec<Vec2<T>>) -> T {
    dist(p, closest_point_on_polygon(p, hull))
}

/// returns the distance from point p to the edge of the polygon defined by point list 'poly'
pub fn point_polygon_distance<T: Float + FloatOps<T>>(p: Vec2<T>, hull: &Vec<Vec2<T>>) -> T {
    dist(p, closest_point_on_polygon(p, hull))
}

/// returns the unsigned distance from point p to the obb defined by matrix obb, where the matrix transforms a unit cube
/// from -1 to 1 into an obb
pub fn point_obb_distance<T: Float, V: VecFloatOps<T> + NumberOps<T> + SignedNumberOps<T> + VecN<T> + SignedVecN<T>, M: MatTranslate<V> + MatInverse<T> + MatN<T, V>>(p: V, mat: M) -> T {
    dist(p, closest_point_on_obb(p, mat))
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
pub fn closest_point_on_plane<T: SignedNumber, V: VecN<T> + SignedVecN<T>>(p: V, x: V, n: V) -> V {
    p + n * -(V::dot(p, n) - V::dot(x, n))
}

/// returns the closest point on the aabb defined by aabb_min and aabb_max to point p, if the point is inside the aabb it will return p
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
pub fn closest_point_on_obb<T: Float, V: VecFloatOps<T> + NumberOps<T> + SignedNumberOps<T> + VecN<T> + SignedVecN<T>, M: MatTranslate<V> + MatInverse<T> + MatN<T, V>>(p: V, mat: M) -> V {
    let invm = mat.inverse();
    let tp = invm * p;
    let cp = closest_point_on_aabb(tp, V::minus_one(), V::one());
    mat * cp
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

/// returns the closest point to p on the cone defined by cp position, with direction cv height h an radius r
pub fn closest_point_on_cone<T: Float, V: VecN<T> + VecFloatOps<T>>(p: V, cp: V, cv: V, h: T, r: T) -> V {
    // centre point of the cones base (where radius is largest)
    let l2 = cp + cv * h;

    // find point onbase plane and clamp to the extent
    let cplane = closest_point_on_plane(p, l2, cv);
    let extent = l2 + normalize(cplane - l2) * r;
    
    // test closest point on line with the axis along the side and bottom of the cone
    let e1 = closest_point_on_line_segment(p, cp, extent);
    let e2 = closest_point_on_line_segment(p, l2, extent);
    
    if dist2(p, e1) < dist2(p, e2) {
        e1
    }
    else {
        e2
    }
}

/// returns the clostest point to p on the edge of the convex hull defined by point list 'hull' with clockwise winding
pub fn closest_point_on_convex_hull<T: Float + FloatOps<T>>(p: Vec2<T>, hull: &Vec<Vec2<T>>) -> Vec2<T> {
    closest_point_on_polygon(p, hull)
}

/// returns the clostest point to p on the edge of the polygon defined by point list polygon.
pub fn closest_point_on_polygon<T: Float + FloatOps<T>>(p: Vec2<T>, poly: &Vec<Vec2<T>>) -> Vec2<T> {
    let ncp = poly.len();
    let mut cp = Vec2::max_value();
    let mut cd2 = T::max_value();
    for i in 0..ncp {
        let i2 = (i+1)%ncp;
        let cpp = closest_point_on_line_segment(p, poly[i], poly[i2]);
        let cppd2 = dist2(p, cpp);
        if dist2(p, cpp) < cd2 { 
            cp = cpp;
            cd2 = cppd2;
        }
    }
    cp
}

/// returns true if point p is inside the aabb defined by aabb_min and aabb_max
pub fn point_inside_aabb<T: Float, V: VecN<T> + VecFloatOps<T>>(p: V, aabb_min: V, aabb_max: V) -> bool {
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
pub fn point_inside_obb<T: Float, V: VecFloatOps<T> + NumberOps<T> + SignedNumberOps<T> + VecN<T> + SignedVecN<T>, M: MatTranslate<V> + MatInverse<T> + MatN<T, V>>(p: V, mat: M) -> bool {
    let invm = mat.inverse();
    let tp = invm * p;
    point_inside_aabb(tp, V::minus_one(), V::one())
}

/// returns true if the point p is inside the triangle defined by t1-t2-t3
pub fn point_inside_triangle<T: Float + FloatOps<T> + NumberOps<T>, V: VecN<T> + NumberOps<T> + VecFloatOps<T>>(p: V, t1: V, t2: V, t3: V) -> bool {
    let (w23, w31, w12) = barycentric(p, t1, t2, t3);
    w23 >= T::zero() && w31 >= T::zero() && w12 >= T::zero()
}

/// returns true if point p is inside cone defined by position cp facing direction cv with height h and radius r
pub fn point_inside_cone<T: Float + FloatOps<T> + NumberOps<T>, V: VecN<T> + VecFloatOps<T>>(p: V, cp: V, cv: V, h: T, r: T) -> bool {
    let l2 = cp + cv * h;
    let dh = distance_on_line(p, cp, l2) / h;
    let x0 = closest_point_on_line_segment(p, cp, l2);
    let d = dist(x0, p);
    d < dh * r && dh < T::one()
}

/// returns true if the point p is inside the 2D convex hull defined by point list 'hull' with clockwise winding
pub fn point_inside_convex_hull<T: Float>(p: Vec2<T>, hull: &Vec<Vec2<T>>) -> bool {
    let ncp = hull.len();
    for i in 0..ncp {
        let i2 = (i+1)%ncp;
        let p1 = hull[i];
        let p2 = hull[i2];
        let v1 = p2 - p1;
        let v2 = p - p1;
        let z = v1.x * v2.y - v2.x - v1.y;
        if z > T::zero() {
            return false;
        }
    }
    true
}

/// returns true if point p is inside the polygon defined by point list 'poly'
pub fn point_inside_polygon<T: Float>(p: Vec2<T>, poly: &Vec<Vec2<T>>) -> bool {
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

/// returns true if the point p is inside the frustum defined by 6 planes packed as vec4's .xyz = normal, .w = plane distance
pub fn point_inside_frustum<T: Number>(p: Vec3<T>, planes: &[Vec4<T>; 6]) -> bool {
    for plane in planes.iter().take(6) {
        let d = dot(p, Vec3::from(*plane)) + plane.w;
        if d > T::zero() {
            return false;
        }
    }
    true
}
    
/// returns the classification of point p vs the plane defined by point on plane x and normal n
pub fn point_vs_plane<T: SignedNumber + SignedNumberOps<T>>(p: Vec3<T>,  x: Vec3<T>, n: Vec3<T>) -> Classification {
    let pd = plane_distance(x, n);
    let d = dot(n, p) + pd;
    if d < T::zero() {
        Classification::Behind
    }
    else if d > T::zero() {
        Classification::Infront
    }
    else {
        Classification::Intersects
    }
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

/// returns the classification of a capsule defined by line c1-c2 with radius r vs a plane defined by point on plane x and normal n
pub fn capsule_vs_plane<T: Float + FloatOps<T> + SignedNumber + SignedNumberOps<T>>(c1: Vec3<T>, c2: Vec3<T>, r: T, x: Vec3<T>, n: Vec3<T>) -> Classification {
    let pd = plane_distance(x, n);
    // classify both spheres at the ends of the capsule
    // sphere 1
    let d1 = dot(n, c1) + pd;
    let r1 = if d1 > r {
        Classification::Infront
    }
    else if d1 < -r {
        Classification::Behind
    }
    else {
        Classification::Intersects
    };
    // sphere 2
    let d2 = dot(n, c2) + pd;
    let r2 = if d2 > r {
        Classification::Infront
    }
    else if d2 < -r {
        Classification::Behind
    }
    else {
        Classification::Intersects
    };
    if r1 == r2 {
        // if both speheres are the same, we return their classification this could give us infront, behind or intersects
        r1
    }
    else {
        // the else case means r1 != r2 and this means we are on opposite side of the plane or one of them intersects
        Classification::Intersects
    }
}

/// return the classification of cone defined by position cp, direction cv with height h and radius at the base of r. vs the plane defined by point x and normal n
pub fn cone_vs_plane<T: Float + SignedNumberOps<T>, V: VecN<T> + Cross<T> + Dot<T> + SignedVecN<T> + VecFloatOps<T>>(cp: V, cv: V, h: T, r: T, x: V, n: V) -> Classification {
    let l2 = cp + cv * h;
    let pd = plane_distance(x, n);
    // check if the tip and cones extent are on different sides of the plane
    let d1 = dot(n, cp) + pd;
    // extent from the tip is at the base centre point perp of cv at the radius edge... we need to choose the side toward the plane
    let perp = normalize(cross(cross(n, cv), cv));
    let extent = l2 + perp * r;
    let extent2 = l2 + perp * -r;
    let d2 = dot(n, extent) + pd;
    let d3 = dot(n, extent2) + pd;
    if d1 < T::zero() && d2 < T::zero() && d3 < T::zero() {
        Classification::Behind
    }
    else if d1 > T::zero() && d2 > T::zero() && d3 > T::zero() {
        Classification::Infront
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

/// returns the intersection point of the infinite line defined as point on line l0 and direction lv with the plane defined by point on plane x and normal n
pub fn line_vs_plane<T: Float + SignedNumberOps<T>>(l0: Vec3<T>, lv: Vec3<T>, x: Vec3<T>, n: Vec3<T>) -> Option<Vec3<T>> {
    let r = dot(lv, n);
    if r == T::zero() {
        None
    }
    else {
        let t = -(dot(l0, n) - dot(x, n))  / dot(lv, n);
        Some(l0 + lv * t)
    }
}

/// returns the intersection point of the line segment l1-l2 and plane defined by point on plane x and normal n, returns None if no intersection exists
pub fn line_segment_vs_plane<T: Float + FloatOps<T> + SignedNumber + SignedNumberOps<T>>(l1: Vec3<T>, l2: Vec3<T>, x: Vec3<T>, n: Vec3<T>) -> Option<Vec3<T>> {
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

/// returns true if the sphere or circle at centre s1 with radius r1 intsercts capsule c0-c1 with radius cr
pub fn sphere_vs_capsule<T: Float + FloatOps<T>, V: VecN<T> + VecFloatOps<T> + FloatOps<T>>(s1: V, r1: T, c0: V, c1: V, cr: T) -> bool {
    let cp = closest_point_on_line_segment(s1, c0, c1);
    let r2 = sqr(r1 + cr);
    dist2(s1, cp) < r2
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

/// returns true if the aabb defined by aabb_min to aabb_max overlaps obb defined by matrix, where the matrix transforms an aabb with -1 to 1 extents into an obb
pub fn aabb_vs_obb<T: Number + Float + SignedNumber + SignedNumberOps<T> + NumberOps<T> + FloatOps<T>, V: VecN<T> + NumberOps<T> + FloatOps<T> + Triple<T> + Cross<T>, M: MatTranslate<V> + MatInverse<T> + MatRotate3D<T, V> + MatN<T, V> + std::ops::Mul<Vec3<T>, Output=Vec3<T>>>(aabb_min: Vec3<T>, aabb_max: Vec3<T>, obb: M) -> bool {
    // this function is for convenience, you can extract vertices and pass to gjk_3d yourself
    let corners = [
        Vec3::<T>::new(-T::one(), -T::one(), -T::one()),
        Vec3::<T>::new( T::one(), -T::one(), -T::one()),
        Vec3::<T>::new( T::one(),  T::one(), -T::one()),
        Vec3::<T>::new(-T::one(),  T::one(),  T::one()),
        Vec3::<T>::new(-T::one(), -T::one(),  T::one()),
        Vec3::<T>::new( T::one(), -T::one(),  T::one()),
        Vec3::<T>::new( T::one(),  T::one(),  T::one()),
        Vec3::<T>::new(-T::one(),  T::one(),  T::one()),
    ];
    
    // aabb
    let verts0 = vec![
        aabb_min,
        Vec3::<T>::new(aabb_min.x, aabb_min.y, aabb_max.z),
        Vec3::<T>::new(aabb_max.x, aabb_min.y, aabb_min.z),
        Vec3::<T>::new(aabb_max.x, aabb_min.y, aabb_max.z),
        Vec3::<T>::new(aabb_min.x, aabb_max.y, aabb_min.z),
        Vec3::<T>::new(aabb_min.x, aabb_max.y, aabb_max.z),
        Vec3::<T>::new(aabb_max.x, aabb_max.y, aabb_min.z),
        aabb_max
    ];
    
    // obb from corners
    let mut verts1 = Vec::new();
    for corner in corners {
        verts1.push(obb * corner);
    }
    
    gjk_3d(verts0, verts1)
}

// returns true if the sphere with centre s0 and radius r0 overlaps obb defined by matrix obb, where the matrix
// transforms a unit cube with extents -1 to 1 into an obb
pub fn sphere_vs_obb<T: Float, V: VecN<T> + VecFloatOps<T> + NumberOps<T> + SignedNumberOps<T>, M: MatTranslate<V> + MatInverse<T> + MatRotate3D<T, V> + MatN<T, V>>(s: V, r: T, obb: M) -> bool {
    // test the distance to the closest point on the obb
    let cp = closest_point_on_obb(s, obb);
    dist2(s, cp) < r * r
}

/// returns true if the aabb defined by obb0 overlaps obb1 where the obb's are defined by a matrix and the matrix transforms an aabb with -1 to 1 extents into an obb
pub fn obb_vs_obb<T: Number + Float + SignedNumber + SignedNumberOps<T> + NumberOps<T> + FloatOps<T>, V: VecN<T> + NumberOps<T> + FloatOps<T> + Triple<T> + Cross<T>, M: MatTranslate<V> + MatInverse<T> + MatRotate3D<T, V> + MatN<T, V> + std::ops::Mul<Vec3<T>, Output=Vec3<T>>>(obb0: M, obb1: M) -> bool {
    // this function is for convenience, you can extract vertices and pass to gjk_3d yourself
    let corners = [
        Vec3::<T>::new(-T::one(), -T::one(), -T::one()),
        Vec3::<T>::new( T::one(), -T::one(), -T::one()),
        Vec3::<T>::new( T::one(),  T::one(), -T::one()),
        Vec3::<T>::new(-T::one(),  T::one(),  T::one()),
        Vec3::<T>::new(-T::one(), -T::one(),  T::one()),
        Vec3::<T>::new( T::one(), -T::one(),  T::one()),
        Vec3::<T>::new( T::one(),  T::one(),  T::one()),
        Vec3::<T>::new(-T::one(),  T::one(),  T::one()),
    ];
    
    // obb from corners
    let mut verts0 = Vec::new();
    for corner in corners {
        verts0.push(obb0 * corner);
    }

    let mut verts1 = Vec::new();
    for corner in corners {
        verts1.push(obb1 * corner);
    }
    
    gjk_3d(verts0, verts1)
}

/// returns true if the capsule cp0-cp1 with radius cr0 overlaps the capsule cp2-cp3 with radius cr1
pub fn capsule_vs_capsule<T: Float + FloatOps<T> + SignedNumberOps<T>, V: VecN<T> + VecFloatOps<T> + FloatOps<T> + SignedNumberOps<T>>(cp0: V, cp1: V, cr0: T, cp2: V, cp3: V, cr1: T) -> bool {
    // min sqr distance between capsule to save on sqrts
    let r2 = (cr0 + cr1) * (cr0 + cr1);

    // check shortest distance between the 2 capsule line segments, if less than the sq radius we overlap
    if let Some((start, end)) = shortest_line_segment_between_line_segments(cp0, cp1, cp2, cp3) {
        let d = dist2(start, end);
        d < r2
    }
    else {
        // we must be orthogonal, which means there is no one single closest point between the 2 lines
        // find the distance between the 2 axes
        let l0 = normalize(cp1 - cp0);
        let t0 = dot(cp2 - cp0, l0);
        let ip0 = cp0 + l0 * t0;
        let d = dist2(cp2, ip0);

        // check axes are within distance
        if d < r2 {
            let l1 = normalize(cp3 - cp2);
            let t1 = dot(cp0 - cp2, l1);
            
            // now check if the capsule axes overlap
            if t0 >= T::zero() && t0*t0 < dist2(cp1, cp0) {
                true
            }
            else {
                t1 > T::zero() && t1*t1 < dist2(cp2, cp3)
            }
        }
        else {
            false
        }
    }
}

/// returns the intersection point of ray wih origin r0 and direction rv against the sphere (or circle) centred at s0 with radius r
pub fn ray_vs_sphere<T: Float + FloatOps<T> + NumberOps<T>, V: VecN<T> + VecFloatOps<T>>(r0: V, rv: V, s0: V, r: T) -> Option<V> {
    let oc = r0 - s0;
    let a = dot(rv, rv);
    let b = T::two() * dot(oc, rv);
    let c = dot(oc,oc) - r*r;
    let discriminant = b*b - T::four()*a*c;
    let hit = discriminant > T::zero();
    if !hit {
        None
    }
    else {
        let t1 = (-b - sqrt(discriminant)) / (T::two()*a);
        let t2 = (-b + sqrt(discriminant)) / (T::two()*a);
        let t = if t1 > T::zero() && t2 > T::zero() {
            min(t1, t2)
        }
        else if t1 > T::zero() {
            t1
        }
        else {
            t2
        };
        Some(r0 + rv * t)
    }
}

/// returns the intersection point of the ray with origin r0 and direction rv with the aabb defined by aabb_min and aabb_max
pub fn ray_vs_aabb<T: Number + NumberOps<T>, V: VecN<T>>(r0: V, rv: V, aabb_min: V, aabb_max: V) -> Option<V> {
    // http://gamedev.stackexchange.com/a/18459
    // min / max's from aabb axes
    let dirfrac = V::one() / rv;
    let tx = (aabb_min[0] - r0[0]) * dirfrac[0];
    let tm = (aabb_max[0] - r0[0]) * dirfrac[0];
    let mut tmin = min(tx, tm);
    let mut tmax = max(tx, tm);
    for i in 0..V::len() {
        let t1 = (aabb_min[i] - r0[i]) * dirfrac[i];
        let t2 = (aabb_max[i] - r0[i]) * dirfrac[i];
        tmin = max(min(t1, t2), tmin);
        tmax = min(max(t1, t2), tmax);
    }

    if tmax < T::zero() || tmin > tmax {
        // if tmin > tmax, ray doesn't intersect AABB
        // if tmax < 0, ray (line) is intersecting AABB, but the whole AABB is behind us
        None
    }
    else {
        // otherwise tmin is length along the ray we intersect at
        Some(r0 + rv * tmin)
    }
}

/// returns the intersection of the 3D ray with origin r0 and direction rv with the obb defined by mat
pub fn ray_vs_obb<T: Float + NumberOps<T>, 
    V: VecFloatOps<T> + NumberOps<T> + SignedNumberOps<T> + VecN<T> + SignedVecN<T>, 
    M: MatTranslate<V> + MatInverse<T> + MatRotate3D<T, V> + MatN<T, V>
    + Into<Mat3<T>> + Copy>
    (r0: V, rv: V, mat: M) -> Option<V> where Mat3<T> : MatN<T, V> {
    let invm = mat.inverse();
    let tr1 = invm * r0;
    let rotm : Mat3<T> = invm.into();
    let trv = rotm * rv;
    let ip = ray_vs_aabb(tr1, normalize(trv), -V::one(), V::one());
    ip.map(|ip| mat * ip)
}

/// returns the intersection  point of ray r0 and normalized direction rv with triangle t0-t1-t2
pub fn ray_vs_triangle<T: Float>(r0: Vec3<T>, rv: Vec3<T>, t0: Vec3<T>, t1: Vec3<T>, t2: Vec3<T>) -> Option<Vec3<T>> {
    // möller–trumbore intersection algorithm
    // ported verbatim https://en.wikipedia.org/wiki/Möller–Trumbore_intersection_algorithm
    let edge1 = t1 - t0;
    let edge2 = t2 - t0;
    let h = cross(rv, edge2);
    let a = dot(edge1, h);
    if a > T::small_epsilon() && a < T::small_epsilon() {
        // ray is parallel to the triangle
        None
    }
    else {
        let f = T::one() / a;
        let s = r0 - t0;
        let u = f * dot(s, h);
        if u < T::zero() || u > T::one() {
            None
        }
        else {
            let q = cross(s, edge1);
            let v = f * dot(rv, q);
            if v < T::zero() || u + v > T::one() {
                None
            }
            else {
                // now we can compute t to find out where the intersection point is on the line
                let t = f * dot(edge2, q);
                if t > T::zero() {
                    Some(r0 + rv * t)
                }
                else {
                    // line intersects but ray does not
                    None
                }
            }
        }
    }
}

/// returns the intersection point of ray wih origin r0 and direction rv against the capsule with line c0 - c1 and radius cr
pub fn ray_vs_capsule<T: Float + FloatOps<T> + NumberOps<T> + SignedNumberOps<T>, V: VecN<T> + Cross<T> + VecFloatOps<T> + SignedNumberOps<T> + FloatOps<T>>(r0: V, rv: V, c0: V, c1: V, cr: T) -> Option<V> {
    // shortest line seg within radius will indicate we intersect an infinite cylinder about an axis
    let seg = shortest_line_segment_between_line_and_line_segment(r0, r0 + rv, c0, c1);
    let mut ipc = V::max_value();
    let mut bc = false;
    if let Some((l0, l1)) = seg {
        // check we intesect the cylinder
        if dist2(l0, l1) < sqr(cr) {
            // intesection of ray and infinite cylinder about axis
            // https://stackoverflow.com/questions/4078401/trying-to-optimize-line-vs-cylinder-intersection
            let a = c0;
            let b = c1;
            let v = rv;
            let r = cr;
            
            let ab = b - a;
            let ao = r0 - a;
            let aoxab = cross(ao, ab);
            let vxab = cross(v, ab);
            let ab2 = dot(ab, ab);
            
            let aa = dot(vxab, vxab);
            let bb = T::two() * dot(vxab, aoxab);
            let cc = dot(aoxab, aoxab) - (r*r * ab2);
            let dd = bb * bb - T::four() * aa * cc;
            
            if dd >= T::zero() {
                let t = (-bb - sqrt(dd)) / (T::two() * aa);
                if t >= T::zero() {
                    // intersection point
                    ipc = r0 + rv * t;
                    // clamps to finite cylinder extents
                    let ipd = distance_on_line(ipc, a, b);
                    if ipd >= T::zero() && ipd <= dist(a, b) {
                        bc = true;
                    }
                }
            }
        }
    }

    // if our line doesnt intersect the cylinder, we might still intersect the top / bottom sphere
    // test intersections with the end spheres
    let bs1 = ray_vs_sphere(r0, rv, c0, cr);
    let bs2 = ray_vs_sphere(r0, rv, c1, cr);

    // get optional intersection points
    let ips1 = if let Some(ip) = bs1 {
        ip
    }
    else {
        V::max_value()
    };

    let ips2 = if let Some(ip) = bs2 {
        ip
    }
    else {
        V::max_value()
    };

    // we need to choose the closes intersection if we have multiple
    let ips : [V; 3] = [ips1, ips2, ipc];
    let bips : [bool; 3] = [bs1.is_some(), bs2.is_some(), bc];

    let mut iclosest = -1;
    let mut dclosest = T::max_value();
    for i in 0..3 {
        if bips[i] {
            let dd = distance_on_line(ips[i], r0, r0 + rv);
            if dd < dclosest {
                iclosest = i as i32;
                dclosest = dd;
            }
        }
    }

    // if we have a valid closest point
    if iclosest != -1 {
        Some(ips[iclosest as usize])
    }
    else {
        None
    }
}

// returns true if there is an intersection between ray wih origin r0 and direction rv against the cylinder with line c0 - c1 and radius cr
// the intersection point is stored in ip if one exists
pub fn ray_vs_cylinder<T: Float + FloatOps<T> + NumberOps<T> + SignedNumberOps<T>>(r0: Vec3<T>, rv: Vec3<T>, c0: Vec3<T>, c1: Vec3<T>, cr: T) -> Option<Vec3<T>> {
    // intesection of ray and infinite cylinder about axis
    // https://stackoverflow.com/questions/4078401/trying-to-optimize-line-vs-cylinder-intersection
    let a = c0;
    let b = c1;
    let v = rv;
    let r = cr;
    
    let ab = b - a;
    let ao = r0 - a;
    let aoxab = cross(ao, ab);
    let vxab = cross(v, ab);
    let ab2 = dot(ab, ab);
    
    let aa = dot(vxab, vxab);
    let bb = T::two() * dot(vxab, aoxab);
    let cc = dot(aoxab, aoxab) - (r*r * ab2);
    let dd = bb * bb - T::four() * aa * cc;
    
    if dd >= T::zero() {
        let t = (-bb - sqrt(dd)) / (T::two() * aa);
        if t >= T::zero() {
            // intersection point
            let ipc = r0 + rv * t;
            // clamps to finite cylinder extents
            let ipd = distance_on_line(ipc, a, b);
            if ipd >= T::zero() && ipd <= dist(a, b) {
                return Some(ipc);
            }
        }
    }
    
    // intersect with the top and bottom circles
    let ip_top = ray_vs_plane(r0, rv, c0, normalize(c0 - c1));
    let ip_bottom = ray_vs_plane(r0, rv, c1, normalize(c1 - c0));
    let r2 = r*r;

    let mut btop = false;
    if let Some(ip_top) = ip_top {
        if dist2(ip_top, c0) < r2 {
            btop = true;
        }
    }

    if let Some(ip_bottom) = ip_bottom {
        if dist2(ip_bottom, c1) < r2 {
            if btop {
                if let Some(ip_top) = ip_top {
                    let d1 = distance_on_line(ip_top, r0, r0 + rv);
                    let d2 = distance_on_line(ip_bottom, r0, r0 + rv);
                    if d2 < d1 {
                        return Some(ip_bottom);
                    }
                }
            }
            else {
                return Some(ip_bottom);
            }
        }
    }
    
    if btop {
        if let Some(ip_top) = ip_top {
            return Some(ip_top);
        }
    }


    None
}

/// returns true if the sphere with centre s and radius r is inside the frustum defined by 6 planes packed as vec4's .xyz = normal, .w = plane distance
pub fn sphere_vs_frustum<T: Number>(s: Vec3<T>, r: T, planes: &[Vec4<T>; 6]) -> bool {
    for p in planes.iter().take(6) {
        let d = dot(s, Vec3::from(*p)) + p.w;
        if d > r {
            return false;
        }
    }
    true
}

/// returns true if the aabb defined by aabb_pos (centre) and aabb_extent is inside the frustum defined by 6 planes packed as vec4's .xyz = normal, .w = plane distance
pub fn aabb_vs_frustum<T: SignedNumber + SignedNumberOps<T>>(aabb_pos: Vec3<T>, aabb_extent: Vec3<T>, planes: &[Vec4<T>; 6]) -> bool {
    let mut inside = true;
    for p in planes.iter().take(6) {
        let pn = Vec3::from(*p);
        let sign_flip = Vec3::signum(pn) * T::minus_one();
        let pd = p.w;
        let d2 = dot(aabb_pos + aabb_extent * sign_flip, pn);
        if d2 > -pd {
            inside = false;
        }
    }
    inside
}

/// returns the intersection point if the line segment l1-l2 intersects with s1-s2
pub fn line_segment_vs_line_segment<T: Float + SignedNumberOps<T> + FloatOps<T>>(l1: Vec3<T>, l2: Vec3<T>,  s1: Vec3<T>, s2: Vec3<T>) -> Option<Vec3<T>> {
    let da = l2 - l1;
    let db = s2 - s1;
    let dc = s1 - l1;
    if dot(dc, cross(da, db)) != T::zero() {
        // lines are not coplanar
        None
    }
    else {
        let s = dot(cross(dc, db), cross(da, db)) / mag2(cross(da, db));
        if s >= T::zero() && s <= T::one() {
            let ip = l1 + da * s;
            let t = distance_on_line(ip, s1, s2) / dist(s1, s2);
            if t >= T::zero() && t <= T::one() {
                Some(ip)
            }
            else {
                None
            }
        }
        else {
            None
        }
    }
}

/// returns the intersection point if the infinite line l1-l2 intersects with s1-s2
pub fn line_vs_line<T: Float + SignedNumberOps<T> + FloatOps<T>>(l1: Vec3<T>, l2: Vec3<T>, s1: Vec3<T>, s2: Vec3<T>) -> Option<Vec3<T>> {
    let da = l2 - l1;
    let db = s2 - s1;
    let dc = s1 - l1;
    if dot(dc, cross(da, db)) != T::zero() {
        // lines are not coplanar
        None
    }
    else {
        let s = dot(cross(dc, db), cross(da, db)) / mag2(cross(da, db));
        let ip = l1 + da * s;
        let t = distance_on_line(ip, s1, s2) / dist(s1, s2);
        if t >= T::zero() && t <= T::one() {
            Some(ip)
        }
        else {
            None
        }
    }
}

/// returns the intersection point if the ray intersects the line segment l1-l2 
pub fn ray_vs_line_segment<T: Float + SignedNumberOps<T> + FloatOps<T>>(r0: Vec3<T>, rv: Vec3<T>, l1: Vec3<T>, l2: Vec3<T>) -> Option<Vec3<T>> {
    let da = l2 - l1;
    let db = rv;
    let dc = r0 - l1;
    if dot(dc, cross(da, db)) != T::zero() {
        // lines are not coplanar
        None
    }
    else {
        let s = dot(cross(dc, db), cross(da, db)) / mag2(cross(da, db));
        let ip = l1 + da * s;
        let t = distance_on_ray(ip, r0, rv);
        if s >= T::zero() && s <= T::one() && t >= T::zero() {
            Some(ip)
        }
        else {
            None
        }
    }
}

/// returns the shortest line segment between 2 line segments (p1-p2) and (p3-p4) as an option tuple where .0 is the point on line segment 1 and .1 is the point on line segment 2
pub fn shortest_line_segment_between_line_segments<T: Float + SignedNumberOps<T> + FloatOps<T>, V: VecN<T> + SignedNumberOps<T> + FloatOps<T> + Dot<T>>(p1: V, p2: V, p3: V, p4: V) -> Option<(V, V)> {
    // https://web.archive.org/web/20120404121511/http://local.wasp.uwa.edu.au/~pbourke/geometry/lineline3d/lineline.c
    
    let p13 = p1 - p3;
    let p43 = p4 - p3;

    if approx(abs(p43), V::zero(), T::small_epsilon()) {
        return None;
    }

    let p21 = p2 - p1;
    if approx(abs(p21), V::zero(), T::small_epsilon()) {
        return None;
    }

    let d1343 = dot(p13, p43);
    let d4321 = dot(p43, p21);
    let d1321 = dot(p13, p21);
    let d4343 = dot(p43, p43);
    let d2121 = dot(p21, p21);

    let denom = d2121 * d4343 - d4321 * d4321;
    if abs(denom) < T::small_epsilon() {
        return None;
    }

    let numer = d1343 * d4321 - d1321 * d4343;
    let mua = saturate(numer / denom);
    let mub = saturate((d1343 + d4321 * mua) / d4343);

    Some((
        p1 + (p21 * mua),
        p3 + (p43 * mub)
    ))
}

/// returns the shortest line segment between 2 lines (p1-p2) and (p3-p4) as an option tuple where .0 is the point on line segment 1 and .1 is the point on line segment 2
pub fn shortest_line_segment_between_lines<T: Float + SignedNumberOps<T> + FloatOps<T>, V: VecN<T> + SignedNumberOps<T> + FloatOps<T> + Dot<T>>(p1: V, p2: V, p3: V, p4: V) -> Option<(V, V)> {
    // https://web.archive.org/web/20120404121511/http://local.wasp.uwa.edu.au/~pbourke/geometry/lineline3d/lineline.c
    
    let p13 = p1 - p3;
    let p43 = p4 - p3;

    if approx(abs(p43), V::zero(), T::small_epsilon()) {
        return None;
    }

    let p21 = p2 - p1;
    if approx(abs(p21), V::zero(), T::small_epsilon()) {
        return None;
    }

    let d1343 = dot(p13, p43);
    let d4321 = dot(p43, p21);
    let d1321 = dot(p13, p21);
    let d4343 = dot(p43, p43);
    let d2121 = dot(p21, p21);

    let denom = d2121 * d4343 - d4321 * d4321;
    if abs(denom) < T::small_epsilon() {
        return None;
    }

    let numer = d1343 * d4321 - d1321 * d4343;
    let mua = numer / denom;
    let mub = (d1343 + d4321 * mua) / d4343;

    Some((
        p1 + (p21 * mua),
        p3 + (p43 * mub)
    ))
}

/// returns the shortest line segment between 2 line segments (p1-p2) and (p3-p4) as an option tuple where .0 is the point on line segment 1 and .1 is the point on line segment 2
pub fn shortest_line_segment_between_line_and_line_segment<T: Float + SignedNumberOps<T> + FloatOps<T>, V: VecN<T> + SignedNumberOps<T> + FloatOps<T> + Dot<T>>(p1: V, p2: V, p3: V, p4: V) -> Option<(V, V)> {
    // https://web.archive.org/web/20120404121511/http://local.wasp.uwa.edu.au/~pbourke/geometry/lineline3d/lineline.c
    
    let p13 = p1 - p3;
    let p43 = p4 - p3;

    if approx(abs(p43), V::zero(), T::small_epsilon()) {
        return None;
    }

    let p21 = p2 - p1;
    if approx(abs(p21), V::zero(), T::small_epsilon()) {
        return None;
    }

    let d1343 = dot(p13, p43);
    let d4321 = dot(p43, p21);
    let d1321 = dot(p13, p21);
    let d4343 = dot(p43, p43);
    let d2121 = dot(p21, p21);

    let denom = d2121 * d4343 - d4321 * d4321;
    if abs(denom) < T::small_epsilon() {
        return None;
    }

    let numer = d1343 * d4321 - d1321 * d4343;
    let mua = numer / denom;
    let mub = saturate((d1343 + d4321 * mua) / d4343);

    Some((
        p1 + (p21 * mua),
        p3 + (p43 * mub)
    ))
}

/// returns soft clipping (in a cubic fashion) of x; let m be the threshold (anything above m stays unchanged), and n the value things will take when the signal is zero
pub fn almost_identity<T: Number + Float>(x: T, m: T, n: T) -> T {
    // <https://iquilezles.org/articles/functions/>
    let a = T::two()*n - m;
    let b = T::two()*m - T::three()*n;
    let t = x/m;
    if x > m {
        x
    }
    else {
        (a*t + b)*t*t + n
    }
}

/// returns the integral smoothstep of x it's derivative is never larger than 1
pub fn integral_smoothstep<T: Number + Float + FloatOps<T>>(x: T, t: T) -> T {
    // inigo quilez: https://iquilezles.org/articles/functions/
    if x > t {
        x - t/T::two()
    }
    else {
        x*x*x*(T::one()-x*T::point_five()/t)/t/t
    }
}

/// returns an exponential impulse (y position on a graph for x); k controls the stretching of the function
pub fn exp_impulse<T: Number + Float, X: Base<T> + FloatOps<T>>(k: X, x: X) -> X {
    // inigo quilez: https://iquilezles.org/articles/functions/
    let h = k * x;
    h * X::exp(X::one() - h)
}

/// returns an quadratic impulse (y position on a graph for x); k controls the stretching of the function
pub fn quad_impulse<T: Number + Float + Base<T> + FloatOps<T>>(k: T, x: T) -> T {
    // inigo quilez: https://iquilezles.org/articles/functions/
    T::two() * T::sqrt(k) * x / (T::one()+k*x*x)
}

/// returns a quadratic impulse (y position on a graph for x); n is the degree of the polynomial and k controls the stretching of the function
pub fn poly_impulse<T: Number + Float + Base<T> + FloatOps<T>>(k: T, x: T, n: T) -> T {
    // inigo quilez: https://iquilezles.org/articles/functions/
    let one = T::one();
    (n/(n-one))* T::powf((n-one)*k,one/n)*x/(one+k*T::powf(x,n))
}

/// returns an exponential sustained impulse (y position on a graph for x); control on the width of attack with k and release with f
pub fn exp_sustained_impulse<T: SignedNumber + Float, X: Base<T> + FloatOps<T> + SignedNumberOps<T> + Ord>(x: X, f: X, k: X) -> X {
    // inigo quilez: https://iquilezles.org/articles/functions/
    let s = X::max(x-f, X::zero());
    X::min(x*x/(f*f), X::one() + (X::two()/f)*s*X::exp(-k*s))
}

/// returns a cubic pulse (y position on a graph for x); equivalent to: smoothstep(c-w,c,x)-smoothstep(c,c+w,x)
pub fn cubic_pulse<X: Float + SignedNumberOps<X>>(c: X, w: X, x: X) -> X {
    // inigo quilez: https://iquilezles.org/articles/functions/
    let mut x = abs(x - c);
    if x > w {
        X::zero()
    }
    else {
        x /= w;
        X::one() - x * x*(X::three() - X::two()*x)
    }
}

/// returns an exponential step (y position on a graph for x); k is control parameter, n is power which gives sharper curves.
pub fn exp_step<T: SignedNumber + Float, X: Base<T> + FloatOps<T> + SignedNumberOps<T>>(x: X, k: X, n: T) -> X {
    // inigo quilez: https://iquilezles.org/articles/functions/
    X::exp(-k * X::powf(x, n))
}

/// returns gain (y position on a graph for x); remapping the unit interval into the unit interval by expanding the sides and compressing the center
pub fn gain<T: SignedNumber + Float + FloatOps<T>>(x: T, k: T) -> T {
    // inigo quilez: https://iquilezles.org/articles/functions/
    let y = if x < T::point_five() { 
        x
    }
    else {
        T::one()-x
    };
    let a = T::point_five()*T::powf(T::two()*(y), k);
    if x < T::point_five() {
        a
    }
    else {
        T::one() - a
    }
}

/// returns a parabola (y position on a graph for x); use k to control its shape
pub fn parabola<T: SignedNumber + Float, X: Base<T> + FloatOps<T> + SignedNumberOps<T>>(x: X, k: T) -> X {
    // inigo quilez: https://iquilezles.org/articles/functions/
    powf(X::four() * x * (X::one() - x), k)
}

/// returns a power curve (y position on a graph for x); this is a generalziation of the parabola
pub fn pcurve<T: SignedNumber + Float + Base<T> + FloatOps<T>>(x: T, a: T, b: T) -> T {
    // inigo quilez: https://iquilezles.org/articles/functions/
    let k = powf(a + b, a + b) / (powf(a, a) * powf(b, b));
    k * powf(x, a) * powf(T::one() - x, b)
}

/// returns a sin curve (y position on a graph for x); can be used for some bouncing behaviors. give k different integer values to tweak the amount of bounces
pub fn sinc<T: SignedNumber + Float, X: Base<T> + FloatOps<T> + SignedNumberOps<T>>(x: X, k: X) -> X {
    // inigo quilez: https://iquilezles.org/articles/functions/
    let a = X::zero() * (k*x-X::one());
    X::sin(a)/a
}

 /// returns a hsv value in 0-1 range converted from rgb in 0-1 range
 pub fn rgb_to_hsv<T: Float + SignedNumberOps<T> + From<f64>>(rgb: Vec3<T>) -> Vec3<T> {
    // from Foley & van Dam p592
    // optimized: http://lolengine.net/blog/2013/01/13/fast-rgb-to-hsv 
    let mut r = rgb.x;
    let mut g = rgb.y;
    let mut b = rgb.z;
    
    let mut k = T::zero();
    if g < b {
        std::mem::swap(&mut g, &mut b);
        k = T::minus_one();
    }

    if r < g {
        std::mem::swap(&mut r, &mut g);
        k = -T::two() / T::from(6.0) - k;
    }
    
    let chroma = r - if g < b { g } else { b };

    Vec3 {
    x: abs(k + (g - b) / (T::from(6.0)  * chroma + T::small_epsilon())),
    y: chroma / (r + T::small_epsilon()),
    z: r
    }
 }
 
 /// returns an rgb value in 0-1 range converted from hsv in 0-1 range
 pub fn hsv_to_rgb<T: Float + FloatOps<T> + From<f64>>(hsv: Vec3<T>) -> Vec3<T> where i32: From<T> {
    // from Foley & van Dam p593: http://en.wikipedia.org/wiki/HSL_and_HSV
    let h = hsv.x;
    let s = hsv.y;
    let v = hsv.z;
        
    if s == T::zero() {
        // gray
        return Vec3 {
            x: v,
            y: v,
            z: v
        };
    }

    let h = T::fmod(h, T::one()) / T::from(0.16666666666);
    let i = i32::from(h);
    let f = h - floor(h);
    let p = v * (T::one() - s);
    let q = v * (T::one() - s * f);
    let t = v * (T::one() - s * (T::one() - f));

    match i {
        0 => {
            Vec3::new(v, t, p)
        }
        1 => {
            Vec3::new(q, v, p)
        }
        2 => {
            Vec3::new(p, v, t)
        }
        3 => {
            Vec3::new(p, q, v)
        }
        4 => {
            Vec3::new(t, p, v)
        }
        _ => {
            Vec3::new(v, p, q)
        }
    }
}

/// returns a vec4 of rgba in 0-1 range from a packed rgba8 inside u32 (4 bytes, R8G8B8A8)
pub fn rgba8_to_vec4<T: Float + FloatOps<T> + From<u32> + From<f64>>(rgba: u32) -> Vec4<T> {
    let one_over_255 = T::from(1.0 / 255.0);
    Vec4 {
        x: T::from(rgba & 0xff) * one_over_255,
        y: T::from((rgba >>  8) & 0xff) * one_over_255,
        z: T::from((rgba >> 16) & 0xff) * one_over_255,
        w: T::from((rgba >> 24) & 0xff) * one_over_255
    }
}

/// returns a packed u32 containing rgba8 (4 bytes, R8G8B8A8) converted from a Vec4 of rgba in 0-1 range
pub fn vec4f_to_rgba8<T: Float + From<f64>>(v: Vec4<T>) -> u32 where u32: From<T> {
    let mut rgba : u32 = 0;
    let x = T::from(255.0);
    rgba |= u32::from(v[0] * x);
    rgba |= u32::from(v[1] * x) << 8;
    rgba |= u32::from(v[2] * x) << 16;
    rgba |= u32::from(v[3] * x) << 24;
    rgba
}

/// returns value t between the range c and d with offset b creating smooth easing at the start (t^2)
pub fn smooth_start2<T: Float, X: Base<T>>(t: X, b: X, c: X, d: X) -> X {
    let t = t/d;
    c * t*t + b
}

/// returns value t between the range c and d with offset b creating smooth easing at the start (t^3)
pub fn smooth_start3<T: Float, X: Base<T>>(t: X, b: X, c: X, d: X) -> X {
    let t = t/d;
    c * t*t*t + b
}

/// returns value t between the range c and d with offset b creating smooth easing at the start (t^4)
pub fn smooth_start4<T: Float, X: Base<T>>(t: X, b: X, c: X, d: X) -> X {
    let t = t/d;
    c * t*t*t*t + b
}

/// returns value t between the range c and d with offset b creating smooth easing at the start (t^5)
pub fn smooth_start5<T: Float, X: Base<T>>(t: X, b: X, c: X, d: X) -> X {
    let t = t/d;
    c * t*t*t*t*t + b
}

/// returns value t between the range c and d with offset b creating smooth easing at the end of t (t^2)
pub fn smooth_stop2<T: Float, X: Base<T> + SignedNumberOps<T>>(t: X, b: X, c: X, d: X) -> X {
    let t = t/d;
    -c * t * (t - X::two()) + b
}

/// returns value t between the range c and d with offset b creating smooth easing at the end of t (t^3)
pub fn smooth_stop3<T: Float, X: Base<T> + SignedNumberOps<T>>(t: X, b: X, c: X, d: X) -> X {
    let t = t/d;
    c * (t*t*t + X::one()) + b
}

/// returns value t between the range c and d with offset b creating smooth easing at the end of t (t^4)
pub fn smooth_stop4<T: Float, X: Base<T> + SignedNumberOps<T>>(t: X, b: X, c: X, d: X) -> X {
    let t = t/d;
    c * (t*t*t*t + X::one()) + b
}

/// returns value t between the range c and d with offset b creating smooth easing at the end of t (t^5)
pub fn smooth_stop5<T: Float, X: Base<T> + SignedNumberOps<T>>(t: X, b: X, c: X, d: X) -> X {
    let t = t/d;
    c * (t*t*t*t*t + X::one()) + b
}

/// returns the morten order index from x,y position
pub fn morton_xy(x: u64, y: u64) -> u64 {
    let mut x = x;
    x = (x | (x << 16)) & 0x0000FFFF0000FFFF;
    x = (x | (x << 8)) & 0x00FF00FF00FF00FF;
    x = (x | (x << 4)) & 0x0F0F0F0F0F0F0F0F;
    x = (x | (x << 2)) & 0x3333333333333333;
    x = (x | (x << 1)) & 0x5555555555555555;

    let mut y = y;
    y = (y | (y << 16)) & 0x0000FFFF0000FFFF;
    y = (y | (y << 8)) & 0x00FF00FF00FF00FF;
    y = (y | (y << 4)) & 0x0F0F0F0F0F0F0F0F;
    y = (y | (y << 2)) & 0x3333333333333333;
    y = (y | (y << 1)) & 0x5555555555555555;

    x | (y << 1)
}

/// returns the morten order index from x,y,z position
pub fn morton_xyz(x: u64, y: u64, z: u64) -> u64 {
    let mut x = x;
    x = (x | (x << 16)) & 0xFFFF00000000FFFF;
    x = (x | (x <<  8)) & 0xF00F00F00F00F;
    x = (x | (x <<  4)) & 0x30C30C30C30C30C3;
    x = (x | (x <<  2)) & 0x9249249249249249;

    let mut y = y;
    y = (y | (y << 16)) & 0xFFFF00000000FFFF;
    y = (y | (y <<  8)) & 0xF00F00F00F00F;
    y = (y | (y <<  4)) & 0x30C30C30C30C30C3;
    y = (y | (y <<  2)) & 0x9249249249249249;

    let mut z = z;
    z = (z | (z << 16)) & 0xFFFF00000000FFFF;
    z = (z | (z <<  8)) & 0xF00F00F00F00F;
    z = (z | (z <<  4)) & 0x30C30C30C30C30C3;
    z = (z | (z <<  2)) & 0x9249249249249249;

    x | (y << 1) | (z << 2)
}

/// returns the number even bits extracted from x as set bits in the return; value 0b010101 returns 0b111
pub fn morton_1(x: u64) -> u64 {
    let mut x = x & 0x5555555555555555;
    x = (x | (x >> 1)) & 0x3333333333333333;
    x = (x | (x >> 2)) & 0x0F0F0F0F0F0F0F0F;
    x = (x | (x >> 4)) & 0x00FF00FF00FF00FF;
    x = (x | (x >> 8)) & 0x0000FFFF0000FFFF;
    x = (x | (x >> 16)) & 0x00000000FFFFFFFF;
    x
}

/// returns the number of bits divisible by 3. value 0b001001001 returns 0b111
pub fn morton_2(x: u64) -> u64 {
    let mut x = x & 0x9249249249249249;
    x = (x | (x >> 2)) & 0x30C30C30C30C30C3;
    x = (x | (x >> 4)) & 0xF00F00F00F00F;
    x = (x | (x >> 8)) & 0xFF0000FF0000FF;
    x = (x | (x >> 16)) &  0xFFFF00000000FFFF;
    x = (x | (x >> 32)) & 0x00000000FFFFFFFF;
    x
}

/// returns the x,y grid position for morten order index d
pub fn morton_to_xy(d: u64) -> (u64, u64) {
    (morton_1(d), morton_1(d >> 1))
}

/// returns the x,y,z grid position for morten order index d
pub fn morton_to_xyz(d: u64) -> (u64, u64, u64) {
    (morton_2(d), morton_2(d >> 1), morton_2(d >> 2))
}

/// remap v within in_start -> in_end range to the new range out_start -> out_end
pub fn map_to_range<T: Float, X: Base<T>>(v: X, in_start: X, in_end: X, out_start: X, out_end: X) -> X {
    let slope = X::one() * (out_end - out_start) / (in_end - in_start);
    out_start + slope * (v - in_start)
}

/// finds support vertices for gjk based on convex meshses where convex0 and convex1 are an array of vertices that form a convex hull
pub fn gjk_mesh_support_function<T: Float + FloatOps<T> + NumberOps<T> + SignedNumberOps<T>, V: VecN<T> + VecFloatOps<T> + SignedNumberOps<T> + FloatOps<T>> (convex0: &Vec<V>, convex1: &Vec<V>, dir: V) -> V {
    let furthest_point = |dir: V, vertices: &Vec<V>| -> V {
        let mut fd = -T::max_value();
        let mut fv = vertices[0];
        for v in vertices {
            let d = dot(dir, *v);
            if d > fd {
                fv = *v;
                fd = d;
            }
        }
        fv
    };
    
    // selects the furthest points on the 2 meshes in opposite directions
    let fp0 = furthest_point(dir, convex0);
    let fp1 = furthest_point(-dir, convex1);
    fp0 - fp1
}

/// simplex evolution for 2d mesh overlaps using gjk
fn handle_simplex_2d<T: Float + FloatOps<T> + NumberOps<T> + SignedNumber + SignedNumberOps<T>, V: VecN<T> + VecFloatOps<T> + Triple<T>> (simplex: &mut Vec<V>, dir: &mut V) -> bool {
    match simplex.len() {
        2 => {
            let a = simplex[1];
            let b = simplex[0];
            let ab = b - a;
            let ao = -a;
            
            *dir = vector_triple(ab, ao, ab);
            
            false
        },
        3 => {
            let a = simplex[2];
            let b = simplex[1];
            let c = simplex[0];
            
            let ab = b - a;
            let ac = c - a;
            let ao = -a;
            
            let abperp = vector_triple(ac, ab, ab);
            let acperp = vector_triple(ab, ac, ac);
            
            if dot(abperp, ao) > T::zero() {
                simplex.remove(0);
                *dir = abperp;
                false
            }
            else if dot(acperp, ao) > T::zero() {
                simplex.remove(1);
                *dir = acperp;
                false
            }
            else {
                true
            }
        }
        _ => {
            panic!("we should always have 2 or 3 points in the simplex!");
        }
    }
}

/// returns true if the 2d convex hull convex0 overlaps with convex1 using the gjk algorithm
pub fn gjk_2d<T: Float + FloatOps<T> + NumberOps<T> + SignedNumber + SignedNumberOps<T>, V: VecN<T> + FloatOps<T> + SignedNumberOps<T> + VecFloatOps<T> + Triple<T>>(convex0: Vec<V>, convex1: Vec<V>) -> bool {
    // implemented following details in this insightful video: https://www.youtube.com/watch?v=ajv46BSqcK4
    
    // start with arbitrary direction
    let mut dir = V::unit_x();
    let support = gjk_mesh_support_function(&convex0, &convex1, dir);
    dir = normalize(-support);
    
    // iterative build and test simplex
    let mut simplex = vec![support];
    
    let max_iters = 32;
    for _i in 0..max_iters {
        let a = gjk_mesh_support_function(&convex0, &convex1, dir);
        
        if dot(a, dir) < T::zero() {
            return false;
        }
        simplex.push(a);
        
        if handle_simplex_2d(&mut simplex, &mut dir) {
            return true;
        }
    }
    
    // if we reach here we likely have got stuck in a simplex building loop, we assume the shapes are touching but not intersecting
    false
}

/// returns true if the convex hull convex0 overlaps convex1 where convex hull is an array of vertices forming a 2D convex polygon
pub fn convex_hull_vs_convex_hull<T: Float + FloatOps<T> + NumberOps<T> + SignedNumber + SignedNumberOps<T>, V: VecN<T> + FloatOps<T> + SignedNumberOps<T> + VecFloatOps<T> + Triple<T>>(convex0: Vec<V>, convex1: Vec<V>) -> bool {
    gjk_2d(convex0, convex1)
}

// simplex evolution for 3d mesh overlaps
fn handle_simplex_3d<T: Float + FloatOps<T> + NumberOps<T> + SignedNumber + SignedNumberOps<T>, V: VecN<T> + VecFloatOps<T> + Triple<T> + Cross<T>> (simplex: &mut Vec<V>, dir: &mut V) -> bool {
    match simplex.len() {
        2 => {
            let a = simplex[1];
            let b = simplex[0];
            
            let ab = b - a;
            let ao = -a;
            
            *dir = vector_triple(ab, ao, ab);
            
            false
        },
        3 => {
            let a = simplex[2];
            let b = simplex[1];
            let c = simplex[0];
            
            let ab = b - a;
            let ac = c - a;
            let ao = -a;
            
            *dir = cross(ac, ab);
    
            // flip normal so it points toward the origin
            if dot(*dir, ao) < T::zero() {
                *dir = -*dir;
            }
    
            false
        },
        4 => {
            let a = simplex[3];
            let b = simplex[2];
            let c = simplex[1];
            let d = simplex[0];
            
            let centre = (a+b+c+d) / T::four();
            
            let ab = b - a;
            let ac = c - a;
            let ad = d - a;
            let ao = -a;
            
            let mut abac = cross(ab, ac);
            let mut acad = cross(ac, ad);
            let mut adab = cross(ad, ab);
            
            // flip the normals so they always face outward
            let centre_abc = (a + b + c) / T::three();
            let centre_acd = (a + c + d) / T::three();
            let centre_adb = (a + d + b) / T::three();
            
            if dot(centre - centre_abc, abac) > T::zero() {
                abac = -abac;
            }
            
            if dot(centre - centre_acd, acad) > T::zero() {
                acad = -acad;
            }
            
            if dot(centre - centre_adb, adab) > T::zero() {
                adab = -adab;
            }
            
            if dot(abac, ao) > T::zero() {
                // erase c
                simplex.remove(0);
                *dir = abac;
                false
            }
            else if dot(acad, ao) > T::zero() {
                // erase a
                simplex.remove(1);
                *dir = acad;
                false
            }
            else if dot(adab, ao) > T::zero() {
                // erase b
                simplex.remove(2);
                *dir = adab;
                false
            }
            else {
                true
            }
        },
        _ => {
            panic!("we should always have 2, 3 or 4 points in the simplex!");
        }
    }
}

/// returns true if the 3D convex hull convex0 overlaps with convex1 using the gjk algorithm
pub fn gjk_3d<T: Float + FloatOps<T> + NumberOps<T> + SignedNumber + SignedNumberOps<T>, V: VecN<T> + FloatOps<T> + SignedNumberOps<T> + VecFloatOps<T> + Triple<T> + Cross<T>>(convex0: Vec<V>, convex1: Vec<V>) -> bool {
    // implemented following details in this insightful video: https://www.youtube.com/watch?v=ajv46BSqcK4
    
    // start with arbitrary direction
    let mut dir = V::unit_x();
    let support = gjk_mesh_support_function(&convex0, &convex1, dir);
    dir = normalize(-support);
    
    // iterative build and test simplex
    let mut simplex = vec![support];
    
    let max_iters = 32;
    for _i in 0..max_iters {
        let a = gjk_mesh_support_function(&convex0, &convex1, dir);
        
        if dot(a, dir) < T::zero() {
            return false;
        }
        simplex.push(a);
        
        if handle_simplex_3d(&mut simplex, &mut dir) {
            return true;
        }
    }
    
    // if we reach here we likely have got stuck in a simplex building loop, we assume the shapes are touching but not intersecting
    false
}

// tests;;
// quilez functions
// quat tests