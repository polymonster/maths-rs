use maths_rs::vec::*;
use maths_rs::num::*;
use maths_rs::mat::*;
use maths_rs::generics::*;

pub type Vec2f = Vec2<f32>;
pub type Vec3f = Vec3<f32>;
pub type Vec4f = Vec4<f32>;
pub type Vec2i = Vec2<i32>;
pub type Vec3i = Vec3<i32>;
pub type Vec4i = Vec4<i32>;
pub type Vec3u = Vec3<u32>;

pub type Mat2f = Mat2<f32>;
pub type Mat3f = Mat3<f32>;
pub type Mat34f = Mat34<f32>;
pub type Mat4f = Mat4<f32>;

#[test]
fn v2_construct() {
    let v2 = Vec2f {
        x: 2.0,
        y: 3.0
    };
    let v2_short = vec2f(2.0, 3.0);
    let v2_new = Vec2f::new(2.0, 3.0);
    assert_eq!(v2, v2_short);
    assert_eq!(v2, v2_new);
}

#[test]
fn v2_len() {
    assert_eq!(Vec2f::len(), 2);
}

#[test]
fn v3_construct() {
    let v3 = Vec3f {
        x: 1.0,
        y: 2.0,
        z: 3.0
    };
    let v3_new = Vec3f::new(1.0, 2.0, 3.0);
    let v3_short = vec3f(1.0, 2.0, 3.0);
    assert_eq!(v3, v3_new);
    assert_eq!(v3, v3_short);
}

#[test]
fn v4_construct() {
    let v4 = Vec4f {
        x: 1.0,
        y: 2.0,
        z: 3.0,
        w: 4.0
    };
    let v4_short = vec4f(1.0, 2.0, 3.0, 4.0);
    assert_eq!(v4, v4_short);
}

#[test]
fn v4_len() {
    assert_eq!(Vec4f::len(), 4);
}

#[test]
fn vec_index() {
    let mut v = vec4f(7.0, 8.0, 9.0, 10.0);

    let x1 = v[0];
    let x2 = v[1];
    let x3 = v[2];
    let x4 = v[3];

    v[0] = 10.0;
    v[1] = 9.0;
    v[2] = 8.0;
    v[3] = 7.0;

    let s1 = v[0];
    let s2 = v[1];
    let s3 = v[2];
    let s4 = v[3];

    assert_eq!(x1, 7.0);
    assert_eq!(x2, 8.0);
    assert_eq!(x3, 9.0);
    assert_eq!(x4, 10.0);

    assert_eq!(s1, 10.0);
    assert_eq!(s2, 9.0);
    assert_eq!(s3, 8.0);
    assert_eq!(s4, 7.0);

    assert_eq!(v, vec4f(10.0, 9.0, 8.0, 7.0));
}

#[test]
fn equal() {
    let a = Vec2f {
        x: 2.0,
        y: 3.0
    };
    let b = Vec2f {
        x: 2.0,
        y: 3.0
    };
    let c = Vec2f {
        x: 3.0,
        y: 2.0
    };
    assert_eq!(a.x, b.x);
    assert_eq!(a.y, b.y);
    assert_eq!(a, b);
    assert_ne!(a.x, c.x);
    assert_ne!(a.y, c.y);
    assert_ne!(a, c);
}

#[test]
fn neg() {
    // neg
    let v1 = vec2f(10.0, 20.0);
    let expected = vec2f(-10.0, -20.0);
    let result = -v1;
    assert_eq!(result, expected);

    // neg with negative
    let v1 = vec2f(-10.0, -20.0);
    let expected = vec2f(10.0, 20.0);
    let result = -v1;
    assert_eq!(result, expected);
}

#[test]
fn add() {
    // add
    let v1 = vec2f(10.0, 20.0);
    let v2 = vec2f(20.0, 10.0);
    let expected = vec2f(30.0, 30.0);
    let result = v1 + v2;
    assert_eq!(expected, result);

    // add with negative
    let v1 = vec2f(0.0, 0.0);
    let v2 = vec2f(-20.0, -10.0);
    let expected = vec2f(-20.0, -10.0);
    let result = v1 + v2;
    assert_eq!(expected, result);
}

#[test]
fn add_assign() {
    // add
    let mut v1 = vec2f(10.0, 20.0);
    let v2 = vec2f(20.0, 10.0);
    let expected = vec2f(30.0, 30.0);
    v1 += v2;
    assert_eq!(expected, v1);

    // add with negative
    let mut v1 = vec2f(0.0, 0.0);
    let v2 = vec2f(-20.0, -10.0);
    let expected = vec2f(-20.0, -10.0);
    v1 += v2;
    assert_eq!(expected, v1);
}

#[test]
fn add_scalar() {
    // add
    let v1 = vec2f(5.0, 15.0);
    let s1 = 55.0;
    let expected = vec2f(60.0, 70.0);
    let result = v1 + s1;
    assert_eq!(expected, result);

    // add with negative
    let v1 = vec2f(0.0, 100.0);
    let v2 = -100.0;
    let expected = vec2f(-100.0, 0.0);
    let result = v1 + v2;
    assert_eq!(expected, result);
}

#[test]
fn add_assign_scalar() {
    // add
    let mut v1 = vec2f(5.0, 15.0);
    let s1 = 55.0;
    let expected = vec2f(60.0, 70.0);
    v1 += s1;
    assert_eq!(expected, v1);

    // add with negative
    let mut v1 = vec2f(0.0, 100.0);
    let v2 = -100.0;
    let expected = vec2f(-100.0, 0.0);
    v1 += v2;
    assert_eq!(expected, v1);
}

#[test]
fn add_vec_rhs_scalar_lhs() {
    // add
    let v1 = vec2f(5.0, 15.0);
    let s1 = 55.0;
    let expected = vec2f(60.0, 70.0);
    let result = s1 + v1;
    assert_eq!(expected, result);
}

#[test]
fn sub() {
    // sub
    let v1 = vec2f(40.0, 90.0);
    let v2 = vec2f(20.0, 22.0);
    let expected = vec2f(20.0, 68.0);
    let result = v1 - v2;
    assert_eq!(expected, result);

    // sub with negative
    let v1 = vec2f(31.0, 224.0);
    let v2 = vec2f(-67.0, -320.0);
    let expected = vec2f(98.0, 544.0);
    let result = v1 - v2;
    assert_eq!(expected, result);
}

#[test]
fn sub_assign() {
    // sub
    let mut v1 = vec2f(40.0, 90.0);
    let v2 = vec2f(20.0, 22.0);
    let expected = vec2f(20.0, 68.0);
    v1 -= v2;
    assert_eq!(expected, v1);

    // sub with negative
    let mut v1 = vec2f(31.0, 224.0);
    let v2 = vec2f(-67.0, -320.0);
    let expected = vec2f(98.0, 544.0);
    v1 -= v2;
    assert_eq!(expected, v1);
}

#[test]
fn sub_scalar() {
    // sub
    let v1 = vec2f(276.0, 17.0);
    let v2 = 112.0;
    let expected = vec2f(164.0, -95.0);
    let result = v1 - v2;
    assert_eq!(expected, result);

    // sub with negative
    let v1 = vec2f(28.0, 162.0);
    let v2 = -13.0;
    let expected = vec2f(41.0, 175.0);
    let result = v1 - v2;
    assert_eq!(expected, result);
}

#[test]
fn sub_scalar_lhs() {
    // sub
    let v1 = vec2f(276.0, 17.0);
    let v2 = 112.0;
    let expected = vec2f(-164.0, 95.0);
    let result = v2 - v1;
    assert_eq!(expected, result);
}

#[test]
fn sub_assign_scalar() {
    // sub
    let mut v1 = vec2f(276.0, 17.0);
    let v2 = 112.0;
    let expected = vec2f(164.0, -95.0);
    v1 -= v2;
    assert_eq!(expected, v1);

    // sub with negative
    let mut v1 = vec2f(28.0, 162.0);
    let v2 = -13.0;
    let expected = vec2f(41.0, 175.0);
    v1 -= v2;
    assert_eq!(expected, v1);
}

#[test]
fn mul() {
    // mul
    let v1 = vec2f(16.0, 543.0);
    let v2 = vec2f(2.0, 10.0);
    let expected = vec2f(32.0, 5430.0);
    let result = v1 * v2;
    assert_eq!(expected, result);

    // mul with negative
    let v1 = vec2f(18.0, 4.0);
    let v2 = vec2f(-56.0, -9.0);
    let expected = vec2f(-1008.0, -36.0);
    let result = v1 * v2;
    assert_eq!(expected, result);
}

#[test]
fn mul_assign() {
    // mul
    let mut v1 = vec2f(16.0, 543.0);
    let v2 = vec2f(2.0, 10.0);
    let expected = vec2f(32.0, 5430.0);
    v1 *= v2;
    assert_eq!(expected, v1);

    // mul with negative
    let mut v1 = vec2f(18.0, 4.0);
    let v2 = vec2f(-56.0, -9.0);
    let expected = vec2f(-1008.0, -36.0);
    v1 *= v2;
    assert_eq!(expected, v1);
}

#[test]
fn mul_scalar() {
    // mul
    let v1 = vec2f(16.5, 22.2);
    let v2 = 33.0;
    let expected = vec2f(544.5, 732.6);
    let result = v1 * v2;
    assert_eq!(Vec2f::approx(expected, result, 0.001), true);

    // mul with negative
    let v1 = vec2f(2.5, 4.8);
    let v2 = -11.1;
    let expected = vec2f(-27.75, -53.28);
    let result = v1 * v2;
    assert_eq!(Vec2f::approx(expected, result, 0.001), true);
}

#[test]
fn mul_scalar_lhs() {
    // mul
    let v1 = vec2f(16.5, 22.2);
    let v2 = 33.0;
    let expected = vec2f(544.5, 732.6);
    let result = v2 * v1;
    assert_eq!(Vec2f::approx(expected, result, 0.001), true);
}

#[test]
fn mul_assign_scalar() {
    // mul
    let mut v1 = vec2f(16.5, 22.2);
    let v2 = 33.0;
    let expected = vec2f(544.5, 732.6);
    v1 *= v2;
    assert_eq!(Vec2f::approx(expected, v1, 0.001), true);

    // mul with negative
    let mut v1 = vec2f(2.5, 4.8);
    let v2 = -11.1;
    let expected = vec2f(-27.75, -53.28);
    v1 *= v2;
    assert_eq!(Vec2f::approx(expected, v1, 0.001), true);
}

#[test]
fn div() {
    // div
    let v1 = vec2f(16.0, 543.0);
    let v2 = vec2f(2.0, 10.0);
    let expected = vec2f(8.0, 54.3);
    let result = v1 / v2;
    assert_eq!(Vec2f::approx(expected, result, 0.001), true);

    // div with negative
    let v1 = vec2f(18.0, 4.0);
    let v2 = vec2f(-56.0, -9.0);
    let expected = vec2f(-0.32142857142, -0.44444444444);
    let result = v1 / v2;
    assert_eq!(Vec2f::approx(expected, result, 0.001), true);
}

#[test]
fn div_assign() {
    // div
    let mut v1 = vec2f(16.0, 543.0);
    let v2 = vec2f(2.0, 10.0);
    let expected = vec2f(8.0, 54.3);
    v1 /= v2;
    assert_eq!(Vec2f::approx(expected, v1, 0.001), true);

    // div with negative
    let mut v1 = vec2f(18.0, 4.0);
    let v2 = vec2f(-56.0, -9.0);
    let expected = vec2f(-0.32142857142, -0.44444444444);
    v1 /= v2;
    assert_eq!(Vec2f::approx(expected, v1, 0.001), true);
}

#[test]
fn div_scalar() {
    // div
    let v1 = vec2f(16.5, 22.2);
    let v2 = 33.0;
    let expected = vec2f(0.5, 0.67272727272);
    let result = v1 / v2;
    assert_eq!(Vec2f::approx(expected, result, 0.001), true);

    // div with negative
    let v1 = vec2f(2.5, 4.8);
    let v2 = -11.1;
    let expected = vec2f(-0.22522522522, -0.43243243243);
    let result = v1 / v2;
    assert_eq!(Vec2f::approx(expected, result, 0.001), true);
}

#[test]
fn div_scalar_lhs() {
    // div
    let v1 = vec2f(16.5, 22.2);
    let v2 = 33.0;
    let expected = vec2f(2.0, 1.48648648649);
    let result = v2 / v1;
    assert_eq!(Vec2f::approx(expected, result, 0.001), true);
}

#[test]
fn div_assign_scalar() {
    // div
    let mut v1 = vec2f(16.5, 22.2);
    let v2 = 33.0;
    let expected = vec2f(0.5, 0.67272727272);
    v1 /= v2;
    assert_eq!(Vec2f::approx(expected, v1, 0.001), true);

    // div with negative
    let mut v1 = vec2f(2.5, 4.8);
    let v2 = -11.1;
    let expected = vec2f(-0.22522522522, -0.43243243243);
    v1 /= v2;
    assert_eq!(Vec2f::approx(expected, v1, 0.001), true);
}

#[test]
fn rem() {
    let v1 = vec2f(16.0, 10.0);
    let v2 = vec2f(18.0, 5.0);
    let expected = vec2f(2.0, 5.0);
    let result = v2 % v1;
    println!("{} {}", expected, result);
    assert_eq!(Vec2f::approx(expected, result, 0.001), true);
    assert_eq!(Vec2f::approx(expected, Vec2f::fmod(v2, v1), 0.001), true);
}

#[test]
fn rem_assign() {
    let mut v1 = vec2f(27.0, 32.0);
    let v2 = vec2f(8.0, 7.0);
    let expected = vec2f(3.0, 4.0);
    v1 %= v2;
    assert_eq!(Vec2f::approx(expected, v1, 0.001), true);
}

#[test]
fn rem_scalar() {
    let v1 = vec2f(27.0, 32.0);
    let v2 = 4.0;
    let expected = vec2f(3.0, 0.0);
    let result = v1 % v2;
    assert_eq!(Vec2f::approx(expected, result, 0.001), true);
}

#[test]
fn rem_scalar_lhs() {
    let v1 = vec2f(16.0, 9.0);
    let v2 = 33.2;
    let expected = vec2f(1.2, 6.2);
    let result = v2 % v1;
    assert_eq!(Vec2f::approx(expected, result, 0.001), true);
}

#[test]
fn rem_assign_scalar() {
    let mut v1 = vec2f(33.2, 11.0);
    let v2 = 16.5;
    let expected = vec2f(0.2, 11.0);
    v1 %= v2;
    assert_eq!(Vec2f::approx(expected, v1, 0.001), true);
}

#[test]
fn deref() {
    let v4 = vec4f(0.0, 1.0, 2.0, 3.0);
    let slice : &[f32] = &v4;
    for i in 0..slice.len() {
        assert_eq!(slice[i], i as f32);
    }

    let mut v5 = vec4f(0.0, 1.0, 2.0, 3.0);
    let mut_slice : &mut [f32] = &mut v5;
    for f in mut_slice {
        *f *= 2.0;
    }

    let modified_slice : &[f32] = &v5;
    for i in 0..modified_slice.len() {
        assert_eq!(modified_slice[i], (i as f32) * 2.0);
    }

    let slice_u8 = v4.as_u8_slice();
    assert_eq!(slice_u8.len(), 16);
}

#[test]
fn mad() {
    let x = vec3f(4.0, 4.0, 4.0);
    let y = vec3f(2.0, 4.0, 6.0);
    let z = vec3f(3.0, 4.0, 5.0);
    let result = Vec3f::mad(x, y, z);
    let expected = vec3f(11.0, 20.0, 29.0);
    assert_eq!(result, expected);
}

#[test]
fn dot_product() {
    let v1 = vec2f(2.0, 4.0);
    let dp = Vec2f::dot(v1, v1);
    let expected = 20.0;
    assert_eq!(dp, expected);
    assert_eq!(dot(v1, v1), 20.0);
    let v1 = vec3f(2.0, 4.0, 6.0);
    let dp = Vec3f::dot(v1, v1);
    let expected = 56.0;
    assert_eq!(dp, expected);
    assert_eq!(dot(v1, v1), 56.0);
}

#[test]
fn cross_product() {
    let v1 = vec3f(1.0, 0.0, 0.0);
    let v2 = vec3f(0.0, 0.0, 1.0);
    let cp = cross(v1, v2);
    let expected = vec3f(0.0, -1.0, 0.0);
    assert_eq!(cp, expected);

    let cp = cross(v2, v1);
    let expected = vec3f(0.0, 1.0, 0.0);
    assert_eq!(cp, expected);
}

#[test]
fn perp_product() {
    let pp = perp(vec2f(1.0, 0.0));
    assert_eq!(pp, vec2f(0.0, 1.0));
    let pp = perp(vec2f(0.5, 0.5));
    assert_eq!(pp, vec2f(-0.5, 0.5));
}

#[test]
fn zero() {
    let vz = Vec3f::zero();
    let expected = vec3f(0.0, 0.0, 0.0);
    assert_eq!(vz, expected);
}

#[test]
fn one() {
    let vo = Vec3f::one();
    let expected = vec3f(1.0, 1.0, 1.0);
    assert_eq!(vo, expected);
}

#[test]
fn units() {
    assert_eq!(Vec3f::unit_x(), vec3f(1.0, 0.0, 0.0));
    assert_eq!(Vec3f::unit_y(), vec3f(0.0, 1.0, 0.0));
    assert_eq!(Vec3f::unit_z(), vec3f(0.0, 0.0, 1.0));
    assert_eq!(Vec4f::unit_w(), vec4f(0.0, 0.0, 0.0, 1.0));
}

#[test]
fn colours() {
    assert_eq!(Vec4f::red(), vec4f(1.0, 0.0, 0.0, 1.0));
    assert_eq!(Vec4f::green(), vec4f(0.0, 1.0, 0.0, 1.0));
    assert_eq!(Vec4f::blue(), vec4f(0.0, 0.0, 1.0, 1.0));
    assert_eq!(Vec4f::cyan(), vec4f(0.0, 1.0, 1.0, 1.0));
    assert_eq!(Vec4f::magenta(), vec4f(1.0, 0.0, 1.0, 1.0));
    assert_eq!(Vec4f::yellow(), vec4f(1.0, 1.0, 0.0, 1.0));
    assert_eq!(Vec4f::black(), vec4f(0.0, 0.0, 0.0, 1.0));
    assert_eq!(Vec4f::white(), vec4f(1.0, 1.0, 1.0, 1.0));
}

#[test]
fn limits() {
    assert_eq!(Vec3f::min_value(), splat3f(f32::MIN));
    assert_eq!(Vec3f::max_value(), splat3f(f32::MAX));
    assert_eq!(Vec3i::min_value(), splat3i(i32::MIN));
    assert_eq!(Vec3i::max_value(), splat3i(i32::MAX));
    assert_eq!(Vec3u::min_value(), splat3u(u32::MIN));
    assert_eq!(Vec3u::max_value(), splat3u(u32::MAX));
}

#[test]
fn all_any() {
    // all
    assert_eq!(Vec4f::all(vec4f(1.0, 1.0, 1.0, 1.0)), true);
    assert_eq!(Vec4f::all(vec4f(0.1, 0.1, 0.1, 0.1)), true);
    assert_eq!(Vec4f::all(vec4f(0.0, 0.0, 0.0, 0.0)), false);

    // any
    assert_eq!(Vec4f::any(vec4f(1.0, 0.0, 0.0, 0.0)), true);
    assert_eq!(Vec4f::any(vec4f(0.0, 1.0, 0.0, 0.0)), true);
    assert_eq!(Vec4f::any(vec4f(0.0, 0.0, 1.0, 0.0)), true);
    assert_eq!(Vec4f::any(vec4f(0.0, 0.0, 0.0, 1.0)), true);
    assert_eq!(Vec4f::any(vec4f(0.0, 0.0, 0.0, 0.0)), false);
}

#[test]
fn float_funcs() {
    assert_eq!(Vec2f::round(vec2f(0.1, 0.6)), vec2f(0.0, 1.0));
    assert_eq!(Vec2f::floor(vec2f(0.1, 0.6)), vec2f(0.0, 0.0));
    assert_eq!(Vec2f::ceil(vec2f(0.1, 0.6)), vec2f(1.0, 1.0));
}

#[test]
fn from() {
    let v2 = vec2f(2.0, 3.0);
    let v3 = vec3f(3.0, 4.0, 5.0);
    let v4 = vec4f(4.0, 5.0, 6.0, 7.0);

    // from vector
    assert_eq!(Vec2f::from(v4), vec2f(4.0, 5.0));
    assert_eq!(Vec2f::from(v3), vec2f(3.0, 4.0));
    assert_eq!(Vec3f::from(v2), vec3f(2.0, 3.0, 0.0));
    assert_eq!(Vec3f::from(v4), vec3f(4.0, 5.0, 6.0));
    assert_eq!(Vec4f::from(v2), vec4f(2.0, 3.0, 0.0, 0.0));
    assert_eq!(Vec4f::from(v3), vec4f(3.0, 4.0, 5.0, 0.0));

    // from scalar
    assert_eq!(Vec2f::from(1.0), vec2f(1.0, 1.0));
    assert_eq!(Vec2i::from(2), vec2i(2, 2));
    assert_eq!(Vec3f::from(3.0), vec3f(3.0, 3.0, 3.0));
    assert_eq!(Vec3i::from(4), vec3i(4, 4, 4));
    assert_eq!(Vec4f::from(5.0), vec4f(5.0, 5.0, 5.0, 5.0));
    assert_eq!(Vec4i::from(6), vec4i(6, 6, 6, 6));

    // from tuples
    assert_eq!(Vec3f::from((vec2f(1.0, 0.0), 6.0)), vec3f(1.0, 0.0, 6.0));
    assert_eq!(Vec4f::from((vec2f(3.0, 4.0), 8.0, 9.0)), vec4f(3.0, 4.0, 8.0, 9.0));
    assert_eq!(Vec4f::from((vec2f(8.0, 9.0), vec2f(1.0, 2.0))), vec4f(8.0, 9.0, 1.0, 2.0));
    assert_eq!(Vec4f::from((vec3f(10.0, 11.0, 12.0), 13.0)), vec4f(10.0, 11.0, 12.0, 13.0));
}

#[test]
fn splat() {
    assert_eq!(splat2f(1.0), vec2f(1.0, 1.0));
    assert_eq!(splat2i(2), vec2i(2, 2));
    assert_eq!(splat3f(3.0), vec3f(3.0, 3.0, 3.0));
    assert_eq!(splat3i(4), vec3i(4, 4, 4));
    assert_eq!(splat4f(5.0), vec4f(5.0, 5.0, 5.0, 5.0));
    assert_eq!(splat4i(6), vec4i(6, 6, 6, 6));
}

#[test]
fn length() {
    let v3 = vec3f(1.0, 2.0, 3.0);
    let sq = (14.0_f32).sqrt();
    assert_eq!(Vec3f::length(v3), sq);
    assert_eq!(Vec3f::mag(v3), sq);
    assert_eq!(Vec3f::mag2(v3), 14.0);
}

#[test]
fn distance() {
    let x = vec3f(5.0, 8.0, 4.0);
    let y = vec3f(20.0, 30.0, -10.0);
    let d = y - x;
    assert_eq!(Vec3f::distance(x, y), Vec3f::mag(d));
    assert_eq!(Vec3f::dist(x, y), Vec3f::mag(d));
    assert_eq!(Vec3f::dist2(x, y), Vec3f::mag2(d));
}

#[test]
fn float_checks() {
    assert_eq!(Vec3f::is_infinite(vec3f(f32::INFINITY, 0.0, f32::INFINITY)), vec3f(1.0, 0.0, 1.0));
    assert_eq!(Vec3f::is_nan(vec3f(f32::NAN, 0.0, f32::NAN)), vec3f(1.0, 0.0, 1.0));
    assert_eq!(Vec3f::is_finite(vec3f(f32::INFINITY, 0.0, f32::INFINITY)), vec3f(0.0, 1.0, 0.0));
}

#[test]
fn pow() {
    assert_eq!(Vec2f::powi(vec2f(5.0, 8.0), 4), vec2f(625.0, 4096.0));
    assert_eq!(Vec2f::powf(vec2f(5.0, 8.0), 4.0), vec2f(625.0, 4096.0));
    assert_eq!(Vec2i::pow(vec2i(4, 8), 8), vec2i(65536, 16777216));
}

#[test]
fn clamp_min_max_saturate_step() {
    // min / max
    assert_eq!(Vec3f::min(vec3f(22.0, 7.0, 5.0), vec3f(27.0, 4.0, 1.0)), vec3f(22.0, 4.0, 1.0));
    assert_eq!(Vec3f::max(vec3f(22.0, 7.0, 5.0), vec3f(27.0, 4.0, 1.0)), vec3f(27.0, 7.0, 5.0));
    assert_eq!(Vec3f::min(vec3f(-22.0, -7.0, -5.0), vec3f(-27.0, -4.0, -1.0)), vec3f(-27.0, -7.0, -5.0));
    assert_eq!(Vec3f::max(vec3f(-22.0, -7.0, -5.0), vec3f(-27.0, -4.0, -1.0)), vec3f(-22.0, -4.0, -1.0));

    // clamp
    assert_eq!(Vec3f::clamp(vec3f(-22.0, 7.0, 5.0), vec3f(-5.0, -5.0, -5.0), vec3f(5.0, 5.0, 5.0)), vec3f(-5.0, 5.0, 5.0));

    // saturate
    assert_eq!(Vec3f::saturate(vec3f(22.0, -12.0, 55.0)), vec3f(1.0, 0.0, 1.0));

    // step
    assert_eq!(Vec3f::step(vec3f(5.0, 6.0, 1.0), vec3f(1.0, 2.0, 3.0)), vec3f(1.0, 1.0, 0.0));
}

#[test]
fn abs_sign() {
    assert_eq!(Vec3f::abs(vec3f(-22.0, -12.0, 66.0)), vec3f(22.0, 12.0, 66.0));
    assert_eq!(Vec3f::sign(vec3f(123.0, -123.0, 999.0)), vec3f(1.0, -1.0, 1.0));
    assert_eq!(Vec3f::signum(vec3f(123.0, -123.0, 999.0)), vec3f(1.0, -1.0, 1.0));
    assert_eq!(Vec3f::sign(vec3f(0.0, -0.0, 0.0)), vec3f(1.0, -1.0, 1.0));
    assert_eq!(Vec3i::sign(vec3i(-1, 0, 1)), vec3i(-1, 0, 1));
}

#[test]
fn interpolate() {
    assert_eq!(Vec3f::lerp(vec3f(10.0, 4.0, 60.0), vec3f(20.0, 0.0, -60.0), 0.5), vec3f(15.0, 2.0, 0.0));
    assert_eq!(Vec3f::lerpn(vec3f(10.0, 4.0, 60.0), vec3f(20.0, 0.0, -60.0), splat3f(0.5)), vec3f(15.0, 2.0, 0.0));

    // TODO: smoothstep
    //assert_eq!(Vec3f::smoothstep(vec3f(10.0, 4.0, 60.0), vec3f(20.0, 0.0, -60.0), 0.75), vec3f(15.0, 2.0, 0.0));
    //assert_eq!(Vec3f::smoothstepn(vec3f(10.0, 4.0, 60.0), vec3f(20.0, 0.0, -60.0), splat3f(0.75)), vec3f(15.0, 2.0, 0.0));
}

#[test]
fn fractional() {
    assert_eq!(Vec2f::approx(Vec2f::frac(vec2f(1.1, 2.2)), vec2f(0.1, 0.2), 0.001), true);
    assert_eq!(Vec2f::approx(Vec2f::trunc(vec2f(1.1, 2.2)), vec2f(1.0, 2.0), 0.001), true);

    // modf
    let (fpart, ipart) = Vec2f::modf(vec2f(1.1, 2.2));
    assert_eq!(Vec2f::approx(fpart, vec2f(0.1, 0.2), 0.001), true);
    assert_eq!(Vec2f::approx(ipart, vec2f(1.0, 2.0), 0.001), true);
}

#[test]
fn trig() {
    assert_eq!(Vec2f::sin(vec2f(0.0, 1.0)), vec2f(f32::sin(0.0), f32::sin(1.0)));
    assert_eq!(Vec2f::cos(vec2f(0.0, 1.0)), vec2f(f32::cos(0.0), f32::cos(1.0)));
    assert_eq!(Vec2f::tan(vec2f(0.0, 1.0)), vec2f(f32::tan(0.0), f32::tan(1.0)));

    assert_eq!(Vec2f::asin(vec2f(0.0, 1.0)), vec2f(f32::asin(0.0), f32::asin(1.0)));
    assert_eq!(Vec2f::acos(vec2f(0.0, 1.0)), vec2f(f32::acos(0.0), f32::acos(1.0)));
    assert_eq!(Vec2f::atan(vec2f(0.0, 1.0)), vec2f(f32::atan(0.0), f32::atan(1.0)));

    assert_eq!(Vec2f::sinh(vec2f(0.0, 1.0)), vec2f(f32::sinh(0.0), f32::sinh(1.0)));
    assert_eq!(Vec2f::cosh(vec2f(0.0, 1.0)), vec2f(f32::cosh(0.0), f32::cosh(1.0)));
    assert_eq!(Vec2f::tanh(vec2f(0.0, 1.0)), vec2f(f32::tanh(0.0), f32::tanh(1.0)));

    let (sin, cos) = Vec2f::sin_cos(vec2f(0.0, 1.0));
    assert_eq!(Vec2f::approx(sin, vec2f(f32::sin(0.0), f32::sin(1.0)), 0.001), true);
    assert_eq!(Vec2f::approx(cos, vec2f(f32::cos(0.0), f32::cos(1.0)), 0.001), true);
}

#[test]
fn exp_log() {
    let v3 = vec3f(1.0, 2.0, 3.0);
    assert_eq!(Vec3f::approx(Vec3f::exp(v3), vec3f(f32::exp(1.0), f32::exp(2.0), f32::exp(3.0)), 0.1), true);
    assert_eq!(Vec3f::approx(Vec3f::log10(v3), vec3f(f32::log10(1.0), f32::log10(2.0), f32::log10(3.0)), 0.1), true);
    assert_eq!(Vec3f::approx(Vec3f::log2(v3), vec3f(f32::log2(1.0), f32::log2(2.0), f32::log2(3.0)), 0.1), true);
    assert_eq!(Vec3f::approx(Vec3f::log(v3, 5.0), vec3f(f32::log(1.0, 5.0), f32::log(2.0, 5.0), f32::log(3.0, 5.0)), 0.1), true);
}

#[test]
fn reflect_refract() {
    // TODO:
    let v3 = vec3f(0.5, -0.5, 0.0);
    let normal = vec3f(0.0, 1.0, 0.0);
    let _refl = Vec3f::reflect(v3, normal);
    //assert_eq!(refl, vec3f(-0.5, 0.5, 0.0));
}

// TODO: atan2

#[test]
fn matrix_get_rows_columns() {
    let m4 = Mat4f {
        m: [
            0.0, 1.0, 2.0, 3.0,
            4.0, 5.0, 6.0, 7.0,
            8.0, 9.0, 10.0, 11.0,
            12.0, 13.0, 14.0, 15.0
        ]
    };
    assert_eq!(m4.get_row(2), vec4f(8.0, 9.0, 10.0, 11.0));
    assert_eq!(m4.get_column(3), vec4f(3.0, 7.0, 11.0, 15.0));

    let m34 = Mat34f {
        m: [
            0.0, 1.0, 2.0, 3.0,
            4.0, 5.0, 6.0, 7.0,
            8.0, 9.0, 10.0, 11.0
        ]
    };
    assert_eq!(m34.get_row(1), vec4f(4.0, 5.0, 6.0, 7.0));
    assert_eq!(m34.get_column(2), vec3f(2.0, 6.0, 10.0));
}

#[test]
fn matrix_zero() {
    let m4 = Mat4f::zero();
    for i in 0..4 {
        assert_eq!(m4.get_row(i), vec4f(0.0, 0.0, 0.0, 0.0));
        assert_eq!(m4.get_column(i), vec4f(0.0, 0.0, 0.0, 0.0));
    }
}

#[test]
fn matrix_identity() {
    let m4 = Mat4f::identity();
    assert_eq!(m4.get_row(0), vec4f(1.0, 0.0, 0.0, 0.0));
    assert_eq!(m4.get_row(1), vec4f(0.0, 1.0, 0.0, 0.0));
    assert_eq!(m4.get_row(2), vec4f(0.0, 0.0, 1.0, 0.0));
    assert_eq!(m4.get_row(3), vec4f(0.0, 0.0, 0.0, 1.0));

    let m34 = Mat34f::identity();
    assert_eq!(m34.get_row(0), vec4f(1.0, 0.0, 0.0, 0.0));
    assert_eq!(m34.get_row(1), vec4f(0.0, 1.0, 0.0, 0.0));
    assert_eq!(m34.get_row(2), vec4f(0.0, 0.0, 1.0, 0.0));
}

#[test]
fn matrix_mul_translate() {
    // 4x4
    let t1 = Mat4f::create_translation(vec3f(10.0, 10.0, 10.0));
    let t2 = Mat4f::create_translation(vec3f(50.0, 44.0, -10.0));
    let result = t1 * t2;
    let colt = result.get_column(3);
    assert_eq!(colt, vec4f(60.0, 54.0, 0.0, 1.0));
    // 3x4
    let t1 = Mat34f::create_translation(vec3f(22.0, 801.0, 554.0));
    let t2 = Mat34f::create_translation(vec3f(13.0, 14.0, 15.0));
    let result = t1 * t2;
    let colt = result.get_column(3);
    assert_eq!(colt, vec3f(35.0, 815.0, 569.0));
}

#[test]
fn matrix_index() {
    // index
    let m4 = Mat4f::identity();
    assert_eq!(m4[(0, 0)], 1.0);
    assert_eq!(m4[(1, 1)], 1.0);
    assert_eq!(m4[(2, 2)], 1.0);
    assert_eq!(m4[(3, 3)], 1.0);

    // at
    let m4 = Mat4f::create_translation(vec3f(500.0, 600.0, 700.0));
    assert_eq!(m4.at(0, 0), 1.0);
    assert_eq!(m4.at(1, 1), 1.0);
    assert_eq!(m4.at(2, 2), 1.0);
    assert_eq!(m4.at(3, 3), 1.0);
    assert_eq!(m4.at(0, 3), 500.0);
    assert_eq!(m4.at(1, 3), 600.0);
    assert_eq!(m4.at(2, 3), 700.0);

    // mut index
    let mut mm = Mat4f::identity();
    mm[(0, 0)] = 2.0;
    mm[(1, 1)] = 3.0;
    mm[(2, 2)] = 4.0;
    mm[(3, 3)] = 5.0;
    let expected = Mat4f::from((
        2.0, 0.0, 0.0, 0.0,
        0.0, 3.0, 0.0, 0.0,
        0.0, 0.0, 4.0, 0.0,
        0.0, 0.0, 0.0, 5.0
    ));
    assert_eq!(mm, expected);
}

#[test]
fn matrix_determinant() {
    let m2 = Mat2f {
        m: [
            1.0, 2.0,
            3.0, 4.0
        ]
    };
    let det = m2.determinant();
    assert_eq!(det, -2.0);

    let m3 = Mat3f {
        m: [
            4.0, 2.0, 0.0,
            4.0, 3.0, 0.0,
            8.0, 8.0, 1.0
        ]
    };
    let det = m3.determinant();
    assert_eq!(det, 4.0);

    let m4 = Mat4f::from((
        1.0, 3.0, 5.0, 9.0,
        1.0, 3.0, 1.0, 7.0,
        4.0, 3.0, 9.0, 7.0,
        5.0, 2.0, 0.0, 9.0
    ));
    let det = m4.determinant();
    assert_eq!(det, -376.0);
}

#[test]
fn matrix_transpose() {
    // 2x2
    let m2 = Mat2f::from((
        0.0, 1.0,
        2.0, 3.0
    ));
    let t2 = Mat2f::from((
        0.0, 2.0,
        1.0, 3.0
    ));
    assert_eq!(m2.transpose(),t2);

    // 3x3
    let m3 = Mat3f::from((
        0.0, 1.0, 2.0,
        3.0, 4.0, 5.0,
        6.0, 7.0, 8.0
    ));
    let t3 = Mat3f::from((
        0.0, 3.0, 6.0,
        1.0, 4.0, 7.0,
        2.0, 5.0, 8.0
    ));
    assert_eq!(m3.transpose(),t3);

    // 4x4
    let m4 = Mat4f::from((
        0.0, 1.0, 2.0, 3.0, 
        4.0, 5.0, 6.0, 7.0, 
        8.0, 9.0, 10.0, 11.0,
        12.0, 13.0, 14.0, 15.0
    ));
    let t4 = Mat4f::from((
        0.0, 4.0, 8.0, 12.0, 
        1.0, 5.0, 9.0, 13.0, 
        2.0, 6.0, 10.0, 14.0,
        3.0, 7.0, 11.0, 15.0
    ));
    assert_eq!(m4.transpose(),t4);
}

#[test]
fn matrix_inverse() {
    // 2x2
    let m2 = Mat2f::from((
        5.0, 2.0,
        -7.0, -3.0
    ));
    let inv = m2.inverse();
    let expected = Mat2f::from((
        3.0, 2.0,
        -7.0, -5.0
    ));
    assert_eq!(inv, expected);
    let m2_inv = m2 * inv;
    assert_eq!(m2_inv, Mat2f::identity());

    // 3x3
    let m3 = Mat3f::from((
        1.0, 2.0, 3.0,
        0.0, 1.0, 4.0,
        5.0, 6.0, 0.0
    ));
    let inv = m3.inverse();
    let expected = Mat3f::from((
        -24.0, 18.0, 5.0,
        20.0, -15.0, -4.0,
        -5.0, 4.0, 1.0
    ));
    assert_eq!(inv, expected);
    let m3_inv = m3 * inv;
    assert_eq!(m3_inv, Mat3f::identity());

    // 3x4
    let m34 = Mat34f::from((
        1.0, 2.0, 3.0, 10.0,
        0.0, 1.0, 4.0, 100.0,
        5.0, 6.0, 0.0, 20.0,
    ));
    let m34_inv = m34 * m34.inverse();
    assert_eq!(m34_inv, Mat34f::identity());

    // 4x4
    let m4 = Mat4f::from((
        1.0, 2.0, 3.0, 10.0,
        0.0, 1.0, 4.0, 100.0,
        5.0, 6.0, 0.0, 20.0,
        9.0, 0.0, 1.0, 1.0,
    ));
    let m4_inv = m4 * m4.inverse();
    assert_eq!(Mat4f::approx(m4_inv, Mat4f::identity(), 0.001), true);
}

#[test]
fn matrix_from_get_row_get_column() {
    // 2x2
    let m2 = Mat2f::from((
        1.0, 2.0,
        3.0, 4.0
    ));
    let m2v = Mat2f::from((
        vec2f(1.0, 2.0),
        vec2f(3.0, 4.0)
    ));
    let expected = Mat2f {
        m: [
            1.0, 2.0,
            3.0, 4.0
        ]
    };
    assert_eq!(m2, expected);
    assert_eq!(m2v, expected);
    assert_eq!(m2, m2v);

    assert_eq!(m2.get_row(0), vec2f(1.0, 2.0));
    assert_eq!(m2.get_row(1), vec2f(3.0, 4.0));
    
    assert_eq!(m2.get_column(0), vec2f(1.0, 3.0));
    assert_eq!(m2.get_column(1), vec2f(2.0, 4.0));

    // 3x3
    let m3 = Mat3f::from((
        1.0, 2.0, 3.0,
        4.0, 5.0, 6.0,
        7.0, 8.0, 9.0
    ));
    let m3v = Mat3f::from((
        vec3f(1.0, 2.0, 3.0),
        vec3f(4.0, 5.0, 6.0),
        vec3f(7.0, 8.0, 9.0)
    ));
    let expected = Mat3f {
        m: [
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
            7.0, 8.0, 9.0
        ]
    };
    assert_eq!(m3, expected);
    assert_eq!(m3v, expected);
    assert_eq!(m3, m3v);

    assert_eq!(m3.get_row(0), vec3f(1.0, 2.0, 3.0));
    assert_eq!(m3.get_row(1), vec3f(4.0, 5.0, 6.0));
    assert_eq!(m3.get_row(2), vec3f(7.0, 8.0, 9.0));

    assert_eq!(m3.get_column(0), vec3f(1.0, 4.0, 7.0));
    assert_eq!(m3.get_column(1), vec3f(2.0, 5.0, 8.0));
    assert_eq!(m3.get_column(2), vec3f(3.0, 6.0, 9.0));

    // 3x4
    let m34 = Mat34f::from((
        1.0, 2.0, 3.0, 4.0, 
        5.0, 6.0, 7.0, 8.0, 
        9.0, 10.0, 11.0, 12.0
    ));
    let m34v = Mat34f::from((
        vec4f(1.0, 2.0, 3.0, 4.0),
        vec4f(5.0, 6.0, 7.0, 8.0),
        vec4f(9.0, 10.0, 11.0, 12.0),
    ));
    let expected = Mat34f {
        m: [
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 10.0, 11.0, 12.0
        ]
    };
    assert_eq!(m34, expected);
    assert_eq!(m34v, expected);
    assert_eq!(m34, m34v);

    assert_eq!(m34.get_row(0), vec4f(1.0, 2.0, 3.0, 4.0));
    assert_eq!(m34.get_row(1), vec4f(5.0, 6.0, 7.0, 8.0));
    assert_eq!(m34.get_row(2), vec4f(9.0, 10.0, 11.0, 12.0));

    assert_eq!(m34.get_column(0), vec3f(1.0, 5.0, 9.0));
    assert_eq!(m34.get_column(1), vec3f(2.0, 6.0, 10.0));
    assert_eq!(m34.get_column(2), vec3f(3.0, 7.0, 11.0));
    assert_eq!(m34.get_column(3), vec3f(4.0, 8.0, 12.0));

    // 4x4
    let m4 = Mat4f::from((
        1.0, 2.0, 3.0, 4.0, 
        5.0, 6.0, 7.0, 8.0, 
        9.0, 10.0, 11.0, 12.0,
        13.0, 14.0, 15.0, 16.0
    ));
    let m4v = Mat4f::from((
        vec4f(1.0, 2.0, 3.0, 4.0),
        vec4f(5.0, 6.0, 7.0, 8.0),
        vec4f(9.0, 10.0, 11.0, 12.0),
        vec4f(13.0, 14.0, 15.0, 16.0),
    ));
    let expected = Mat4f {
        m: [
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 10.0, 11.0, 12.0,
            13.0, 14.0, 15.0, 16.0,
        ]
    };
    assert_eq!(m4, expected);
    assert_eq!(m4v, expected);
    assert_eq!(m4, m4v);

    assert_eq!(m4.get_row(0), vec4f(1.0, 2.0, 3.0, 4.0));
    assert_eq!(m4.get_row(1), vec4f(5.0, 6.0, 7.0, 8.0));
    assert_eq!(m4.get_row(2), vec4f(9.0, 10.0, 11.0, 12.0));
    assert_eq!(m4.get_row(3), vec4f(13.0, 14.0, 15.0, 16.0));

    assert_eq!(m4.get_column(0), vec4f(1.0, 5.0, 9.0, 13.0));
    assert_eq!(m4.get_column(1), vec4f(2.0, 6.0, 10.0, 14.0));
    assert_eq!(m4.get_column(2), vec4f(3.0, 7.0, 11.0, 15.0));
    assert_eq!(m4.get_column(3), vec4f(4.0, 8.0, 12.0, 16.0));

    // 2x2 from 3x3
    let m2_from_m3 = Mat2f::from(m3);
    let expected = Mat2f::from((
        1.0, 2.0,
        4.0, 5.0
    ));
    assert_eq!(m2_from_m3, expected);
    
    // 2x2 from 3x4
    let m2_from_m34 = Mat2f::from(m34);
    let expected = Mat2f::from((
        1.0, 2.0,
        5.0, 6.0
    ));
    assert_eq!(m2_from_m34, expected);

    // 2x2 form 4x4
    let m2_from_m3 = Mat2f::from(m4);
    let expected = Mat2f::from((
        1.0, 2.0,
        5.0, 6.0
    ));
    assert_eq!(m2_from_m3, expected);

    // 3x3 from 2x2
    let m3_from_m2 = Mat3f::from(m2);
    let expected = Mat3f::from((
        1.0, 2.0, 0.0,
        3.0, 4.0, 0.0,
        0.0, 0.0, 1.0,
    ));
    assert_eq!(m3_from_m2, expected);

    // 3x3 from 3x4
    let m3_from_m34 = Mat3f::from(m34);
    let expected = Mat3f::from((
        1.0, 2.0, 3.0,
        5.0, 6.0, 7.0,
        9.0, 10.0, 11.0,
    ));
    assert_eq!(m3_from_m34, expected);

    // 3x3 from 4x4
    let m3_from_m4 = Mat3f::from(m4);
    assert_eq!(m3_from_m4, expected);

    // 3x4 from 2x2
    let m34_from_m2 = Mat34f::from(m2);
    let expected = Mat34f::from((
        1.0, 2.0, 0.0, 0.0,
        3.0, 4.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
    ));
    assert_eq!(m34_from_m2, expected);

    // 3x4 from 3x3
    let m34_from_m3 = Mat34f::from(m3);
    let expected = Mat34f::from((
        1.0, 2.0, 3.0, 0.0,
        4.0, 5.0, 6.0, 0.0,
        7.0, 8.0, 9.0, 0.0,
    ));
    assert_eq!(m34_from_m3, expected);

    // 3x4 from 4x4
    let m34_from_m4 = Mat34f::from(m4);
    let expected = Mat34f::from((
        1.0, 2.0, 3.0, 4.0,
        5.0, 6.0, 7.0, 8.0,
        9.0, 10.0, 11.0, 12.0,
    ));
    assert_eq!(m34_from_m4, expected);

    // 4x4 from 2x2
    let m4_from_m2 = Mat4f::from(m2);
    let expected = Mat4f::from((
        1.0, 2.0, 0.0, 0.0,
        3.0, 4.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    ));
    assert_eq!(m4_from_m2, expected);

    // 4x4 from 3x3
    let m4_from_m3 = Mat4f::from(m3);
    let expected = Mat4f::from((
        1.0, 2.0, 3.0, 0.0,
        4.0, 5.0, 6.0, 0.0,
        7.0, 8.0, 9.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    ));
    assert_eq!(m4_from_m3, expected);

    // 4x4 from 3x4
    let m4_from_m34 = Mat4f::from(m34);
    let expected = Mat4f::from((
        1.0, 2.0, 3.0, 4.0,
        5.0, 6.0, 7.0, 8.0,
        9.0, 10.0, 11.0, 12.0,
        0.0, 0.0, 0.0, 1.0
    ));
    assert_eq!(m4_from_m34, expected);
}

#[test]
fn matrix_deref() {
    let mut m3 = Mat3f::from((
        1.0, 2.0, 3.0,
        4.0, 5.0, 6.0,
        7.0, 8.0, 9.0
    ));
    let m3_slice : &[f32] = &m3;
    assert_eq!(m3_slice[0], 1.0);
    assert_eq!(m3_slice[1], 2.0);
    assert_eq!(m3_slice[2], 3.0);
    assert_eq!(m3_slice[3], 4.0);
    assert_eq!(m3_slice[4], 5.0);
    assert_eq!(m3_slice[5], 6.0);
    assert_eq!(m3_slice[6], 7.0);
    assert_eq!(m3_slice[7], 8.0);
    assert_eq!(m3_slice[8], 9.0);

    let m3_mut_slice : &mut [f32] = &mut m3;
    m3_mut_slice[0] = 0.0;
    m3_mut_slice[1] = 0.0;
    m3_mut_slice[2] = 0.0;
    m3_mut_slice[3] = 0.0;
    m3_mut_slice[4] = 0.0;
    m3_mut_slice[5] = 0.0;
    m3_mut_slice[6] = 0.0;
    m3_mut_slice[7] = 0.0;
    m3_mut_slice[8] = 0.0;
    assert_eq!(m3[0], 0.0);
    assert_eq!(m3[1], 0.0);
    assert_eq!(m3[2], 0.0);
    assert_eq!(m3[3], 0.0);
    assert_eq!(m3[4], 0.0);
    assert_eq!(m3[5], 0.0);
    assert_eq!(m3[6], 0.0);
    assert_eq!(m3[7], 0.0);
    assert_eq!(m3[8], 0.0);
}

#[test]
fn matrix_rotate() {
    // 2x2 z rotation
    let m2 = Mat2f::create_z_rotation(f32::deg_to_rad(90.0));
    let mi = Mat2f::identity();
    let rotated = m2 * mi;
    let expected = Mat2f::from((
        0.0, -1.0,
        1.0, 0.0
    ));
    assert_eq!(Mat2f::approx(rotated, expected, 0.001), true);

    // 3x3 z rotation
    let m3 = Mat3f::create_z_rotation(f32::deg_to_rad(-90.0));
    let mi = Mat3f::identity();
    let rotated = m3 * mi;
    let expected = Mat3f::from((
        0.0, 1.0, 0.0,
        -1.0, 0.0, 0.0,
        0.0, 0.0, 1.0
    ));
    assert_eq!(Mat3f::approx(rotated, expected, 0.001), true);

    // 3x3 z-axis
    let axis_z = Mat3f::create_rotation(Vec3f::unit_z(), f32::deg_to_rad(-90.0));
    let rotated = axis_z * mi;
    assert_eq!(Mat3f::approx(rotated, expected, 0.001), true);

    // 3x4 z rotation
    let m34 = Mat34f::create_z_rotation(f32::deg_to_rad(270.0));
    let mi = Mat34f::identity();
    let rotated = m34 * mi;
    let expected = Mat34f::from((
        0.0, 1.0, 0.0, 0.0,
        -1.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
    ));
    assert_eq!(Mat34f::approx(rotated, expected, 0.001), true);
    
    // 3x4 z-axis
    let axis_z = Mat34f::create_rotation(Vec3f::unit_z(), f32::deg_to_rad(270.0));
    let rotated = axis_z * mi;
    assert_eq!(Mat34f::approx(rotated, expected, 0.001), true);

    // 4x4 z rotation
    let m4 = Mat4f::create_z_rotation(f32::deg_to_rad(180.0));
    let mi = Mat4f::identity();
    let rotated = m4 * mi;
    let expected = Mat4f::from((
        -1.0, 0.0, 0.0, 0.0,
        0.0, -1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    ));
    assert_eq!(Mat4f::approx(rotated, expected, 0.001), true);

    // 4x4 z-axis
    let axis_z = Mat4f::create_rotation(Vec3f::unit_z(), f32::deg_to_rad(180.0));
    let rotated = axis_z * mi;
    assert_eq!(Mat4f::approx(rotated, expected, 0.001), true);

    // 3x3 x-rotation
    let m3 = Mat3f::create_x_rotation(f32::deg_to_rad(90.0));
    let mi = Mat3f::identity();
    let rotated = m3 * mi;
    let expected = Mat3f::from((
        1.0, 0.0, 0.0,
        0.0, 0.0, -1.0,
        0.0, 1.0, 0.0
    ));
    assert_eq!(Mat3f::approx(rotated, expected, 0.001), true);

    // 3x3 x-axis
    let axis_x = Mat3f::create_rotation(Vec3f::unit_x(), f32::deg_to_rad(90.0));
    let rotated = axis_x * mi;
    assert_eq!(Mat3f::approx(rotated, expected, 0.001), true);

    // 3x4 x-rotation
    let m34 = Mat34f::create_x_rotation(f32::deg_to_rad(180.0));
    let mi = Mat34f::identity();
    let rotated = m34 * mi;
    let expected = Mat34f::from((
        1.0, 0.0, 0.0, 0.0,
        0.0, -1.0, 0.0, 0.0,
        0.0, 0.0, -1.0, 0.0,
    ));
    assert_eq!(Mat34f::approx(rotated, expected, 0.001), true);

    // 3x4 x-axis
    let axis_x = Mat34f::create_rotation(Vec3f::unit_x(), f32::deg_to_rad(180.0));
    let rotated = axis_x * mi;
    assert_eq!(Mat34f::approx(rotated, expected, 0.001), true);

    // 4x4 x-rotation
    let m4 = Mat4f::create_x_rotation(f32::deg_to_rad(-90.0));
    let mi = Mat4f::identity();
    let rotated = m4 * mi;
    let expected = Mat4f::from((
        1.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, -1.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    ));
    assert_eq!(Mat4f::approx(rotated, expected, 0.001), true);

    // 4x4 x-axis
    let axis_x = Mat4f::create_rotation(Vec3f::unit_x(), f32::deg_to_rad(-90.0));
    let rotated = axis_x * mi;
    assert_eq!(Mat4f::approx(rotated, expected, 0.001), true);

    // 3x3 y-rotation
    let m3 = Mat3f::create_y_rotation(f32::deg_to_rad(90.0));
    let mi = Mat3f::identity();
    let rotated = m3 * mi;
    let expected = Mat3f::from((
        0.0, 0.0, 1.0,
        0.0, 1.0, 0.0,
        -1.0, 0.0, 0.0
    ));
    assert_eq!(Mat3f::approx(rotated, expected, 0.001), true);

    // 3x3 y-axis
    let axis_y = Mat3f::create_rotation(Vec3f::unit_y(), f32::deg_to_rad(90.0));
    let rotated = axis_y * mi;
    assert_eq!(Mat3f::approx(rotated, expected, 0.001), true);

    // 3x4 y-rotation
    let m34 = Mat34f::create_y_rotation(f32::deg_to_rad(180.0));
    let mi = Mat34f::identity();
    let rotated = m34 * mi;
    let expected = Mat34f::from((
        -1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, -1.0, 0.0,
    ));
    assert_eq!(Mat34f::approx(rotated, expected, 0.001), true);

    // 3x4 y-axis
    let axis_y = Mat34f::create_rotation(Vec3f::unit_y(), f32::deg_to_rad(180.0));
    let rotated = axis_y * mi;
    assert_eq!(Mat34f::approx(rotated, expected, 0.001), true);

    // 4x4 y-rotation
    let m4 = Mat4f::create_y_rotation(f32::deg_to_rad(-90.0));
    let mi = Mat4f::identity();
    let rotated = m4 * mi;
    let expected = Mat4f::from((
        0.0, 0.0, -1.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        1.0, -0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    ));
    assert_eq!(Mat4f::approx(rotated, expected, 0.001), true);

    // 4x4 y-axis
    let axis_y = Mat4f::create_rotation(Vec3f::unit_y(), f32::deg_to_rad(-90.0));
    let rotated = axis_y * mi;
    assert_eq!(Mat4f::approx(rotated, expected, 0.001), true);
}

#[test]
fn matrix_mul_vec() {
    // 2x2 * v2 rot
    let m2 = Mat2f::create_scale(vec2f(10.0, 20.0));
    let v2 = vec2f(60.0, 70.0);
    let transformed = m2 * v2;
    let expected = vec2f(600.0, 1400.0);
    assert_eq!(transformed, expected);

    // 2x2 * v2 scale
    let m2 = Mat2f::create_z_rotation(f32::deg_to_rad(90.0));
    let v2 = Vec2f::unit_x();
    let transformed = m2 * v2;
    let expected = vec2f(0.0, 1.0);
    assert_eq!(Vec2f::approx(transformed, expected, 0.001), true);

    // 3x3 * v3 rot
    let m3 = Mat3f::create_z_rotation(f32::deg_to_rad(-90.0));
    let v3 = Vec3f::unit_x();
    let transformed = m3 * v3;
    let expected = vec3f(0.0, -1.0, 0.0);
    assert_eq!(Vec3f::approx(transformed, expected, 0.001), true);

    // 3x3 * v3 scale
    let m3 = Mat3f::create_scale(vec3f(10.0, 11.0, 12.0));
    let v3 = Vec3f::one();
    let transformed = m3 * v3;
    let expected = vec3f(10.0, 11.0, 12.0);
    assert_eq!(Vec3f::approx(transformed, expected, 0.001), true);

    // 3x4 * v3 rot
    let m3 = Mat34f::create_x_rotation(f32::deg_to_rad(-90.0));
    let v3 = vec3f(0.0, 0.0, 5.0);
    let transformed = m3 * v3;
    let expected = vec3f(0.0, 5.0, 0.0);
    assert_eq!(Vec3f::approx(transformed, expected, 0.001), true);

    // 3x4 * v3 scale
    let m3 = Mat34f::create_scale(vec3f(10.0, 2.0, 30.0));
    let v3 = vec3f(1.0, 2.0, 3.0);
    let transformed = m3 * v3;
    let expected = vec3f(10.0, 4.0, 90.0);
    assert_eq!(Vec3f::approx(transformed, expected, 0.001), true);

    // 3x4 * v3 translate
    let m34 = Mat34f::create_translation(vec3f(50.0, -10.0, 20.0));
    let v3 = vec3f(3.0, 4.0, 5.0);
    let transformed = m34 * v3;
    let expected = vec3f(53.0, -6.0, 25.0);
    assert_eq!(Vec3f::approx(transformed, expected, 0.001), true);

    // 4x4 rot
    let m4 = Mat4f::create_z_rotation(f32::deg_to_rad(90.0));
    let v3 = vec3f(10.0, 0.0, 0.0);
    let (transformed, _w) = m4 * v3;
    let expected = vec3f(0.0, 10.0, 0.0);
    assert_eq!(Vec3f::approx(transformed, expected, 0.001), true);
    
    // 4x4 scale
    let m4 = Mat4f::create_scale(vec3f(50.0, -10.0, 20.0));
    let v3 = vec3f(3.0, 4.0, 5.0);
    let (transformed, _w) = m4 * v3;
    let expected = vec3f(150.0, -40.0, 100.0);
    assert_eq!(transformed, expected);

    // 4x4 translate
    let m4 = Mat4f::create_translation(vec3f(50.0, 90.0, -20.0));
    let v3 = vec3f(3.0, 4.0, 5.0);
    let (transformed, _w) = m4 * v3;
    let expected = vec3f(53.0, 94.0, -15.0);
    assert_eq!(transformed, expected);
}

#[test]
fn generics() {
    let v1 : f32 = 1.0;
    let v2 = vec2f(1.0, 1.0);
    let v3 = vec3f(1.0, 1.0, 1.0);
    
    let _s1 = sqrt(v1);
    let _s2 = sqrt(v2);
    let _s3 = sqrt(v3);

    let _p1 = powi(v1, 10);
    let _p2 = powi(v2, 10);
    let _p3 = powi(v3, 10);

    let _min1 = min(v1, v1);
    let _min2 = min(v2, v2);
    let _min3 = min(v3, v3);

    let _m2 = mag(v2);
    let _m3 = mag(v3);
    let _n2 = normalize(v2);
    let _n3 = normalize(v3);
    let _d2 = dot(v2, v2);
    let _d3 = dot(v3, v3);
}