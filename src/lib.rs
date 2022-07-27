/// numerical traits and operations for scalar numbers, signed numbers, integers and floats
pub mod num;

/// multi dimensional column vector with vec2, vec3 and vec4 implementations
pub mod vec;

/// multi dimensional row-major matrix with mat2, mat3, mat34 and mat4 implementations
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

/// returns a convex hull wound clockwise from point cloud "points"
pub fn convex_hull_from_points<T: Float + SignedNumberOps<T> + NumberOps<T> + FloatOps<T>>(points: &Vec<Vec2<T>>) -> Vec<Vec2<T>> {    
    //find right most point
    let mut cur = points[0];
    let mut curi = 0;
    for (i, item) in points.iter().enumerate().skip(1) {
        if item.x > cur.x && item.y > cur.y {
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
pub fn get_triangle_normal<T: Float, V: VecFloatOps<T> + VecN<T> + VecCross<T>>(t1: V, t2: V, t3: V) -> V {
    normalize(cross(t2 - t1, t3 - t1))
}

/// returns the 3D normalized device coordinate of point p projected by view_projection matrix, perfroming homogenous divide
pub fn project_to_ndc<T: Float>(p: Vec3<T>, view_projection: Mat4<T>) -> Vec3<T> {
    let ndc = view_projection * Vec4::from((p, T::one()));
    Vec3::from(ndc) / ndc.w
}

/// returns the 2D screen coordinate of 3D point p projected with view_projection, performing homogenous divide and viewport correction
/// assumes screen coordinates are vup in the y-axis y.0 = bottom y.height = top
pub fn project_to_sc<T: Float>(p: Vec3<T>, view_projection: Mat4<T>, viewport: Vec2<T>) -> Vec2<T> {
    let ndc = project_to_ndc(p, view_projection);
    let sc  = ndc * T::point_five() + T::point_five();
    Vec2::<T>::from(sc) * viewport
}

/// returns the 2D screen coordinate of 3D point p projected with view_projection, performing homogenous divide and viewport correction
/// coordinates are vdown in the y-axis vdown = y.0 = top y.height = bottom
pub fn project_to_sc_vdown<T: Float>(p: Vec3<T>, view_projection: Mat4<T>, viewport: Vec2<T>) -> Vec2<T> {
    let ndc = project_to_ndc(p, view_projection);
    let sc  = ndc * Vec3::new(T::point_five(), -T::point_five(), T::point_five()) + T::point_five();
    Vec2::<T>::from(sc) * viewport
}

/// returns the unprojected 3D world position of point p which is specified in normalized device coordinates
pub fn unproject_ndc<T: Float>(p: Vec3<T>, view_projection: Mat4<T>) -> Vec3<T> {
    let inv = view_projection.inverse();
    let (usc, w) = inv * p;
    usc / w
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

/// returns the distance from point p to the cone defined by position cp, with height h and radius at the base of r
pub fn point_cone_distance<T: Float, V: VecN<T> + SingedVecN<T> + VecFloatOps<T>>(p: V, cp: V, cv: V, h: T, r: T) -> T {
    dist(p, closest_point_on_cone(p, cp, cv, h, r))
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
pub fn point_inside_convex_hull<T: Float>(p: Vec2<T>, hull: Vec<Vec2<T>>) -> bool {
    let p0 = Vec3::from((p, T::zero()));
    let ncp = hull.len();
    for i in 0..ncp {
        let i2 = (i+1)%ncp;
        let p1 = Vec3::from((hull[i], T::zero()));
        let p2 = Vec3::from((hull[i2], T::zero()));
        let v1 = p2 - p1;
        let v2 = p0 - p1;
        if cross(v1, v2).z > T::zero() {
            return false;
        }
    }
    true
}

/// returns true if point p is inside the polygon defined by point list 'poly'
pub fn point_inside_polygon<T: Float>(p: Vec2<T>, poly: Vec<Vec2<T>>) -> bool {
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

pub fn ray_vs_obb<T: Float + NumberOps<T>, 
    V: VecFloatOps<T> + NumberOps<T> + SignedNumberOps<T> + VecN<T> + SingedVecN<T>, 
    M: MatTranslate<V> + MatInverse<T> + MatRotate3D<T, V> + std::ops::Mul<V, Output=V>
    + Into<Mat3<T>> + Copy>
    (r0: V, rv: V, mat: M) -> Option<V> where Mat3<T> : std::ops::Mul<V, Output=V> {
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

/// returns true if the sphere with centre s and radius r is inside the furstum defined by 6 planes packed as vec4's .xyz = normal, .w = plane distance
pub fn sphere_vs_frustum<T: Number>(s: Vec3<T>, r: T, planes: &[Vec4<T>; 6]) -> bool {
    for p in 0..6 {
        let d = dot(s, Vec3::from(planes[p])) + planes[p].w;
        if d > r {
            return false;
        }
    }
    true
}

/// returns true if the aabb defined by aabb_pos (centre) and aabb_extent is inside the furstum defined by 6 planes packed as vec4's .xyz = normal, .w = plane distance
pub fn aabb_vs_frustum<T: SignedNumber + SignedNumberOps<T>>(aabb_pos: Vec3<T>, aabb_extent: Vec3<T>, planes: &[Vec4<T>; 6]) -> bool {
    let mut inside = true;
    for p in 0..6 {
        let pn = Vec3::from(planes[p]);
        let sign_flip = Vec3::signum(pn) * T::minus_one();
        let pd = planes[p].w;
        let d2 = dot(aabb_pos + aabb_extent * sign_flip, pn);
        if d2 > -pd {
            inside = false;
        }
    }
    inside
}

// closest point on hull
// closest point on poly
// point hull distance
// point poly distance

// mat
// ortho basis frivs + huges

// utils
// hsv

// TODO: finalise
// think about obb's and mul with vec3 returning tuple

// TODO: tests
// missing fail cases
// point inside hull (test)
// point inside poly (test)
// ray sphere (test)
// ray triangle (test)
// projection, ndc
// projection, sc
// unprojection, ndc,
// unprojection sc

// TODO c++
// point inside cone test is whack
// point plane distance
// point sphere distance
// fix point inside triangle, closest point on triangle + tests
// closest point on cone
// point cone distance
// convex hull from points test
// point inside hull (test)
// point inside poly (test)
// ray sphere (test)
// ray triangle (test)

// TODO: new?
// line_vs_cone
// ray_vs_cone
// cone_vs_sphere
// cone_vs_plane
// cone_vs_aabb
// capsules?
// obb_vs_aabb
// obb_vs_sphere
// line_vs_sphere
// line_vs_aabb
// obb_vs_aabb
// obb_vs_sphere