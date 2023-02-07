use maths_rs::*;
use maths_rs::vec::*;
use maths_rs::num::*;
use maths_rs::mat::*;
use maths_rs::swizz::*;

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

    // type conversions
    let vf = vec2f(6.6, 7.7);
    let vi = vec2i(8, 9);
    let vu = vec2u(u32::max_value(), u32::max_value());
    let vs = vec2i(-1, -1);

    assert_eq!(Vec2i::from(vf), vec2i(6, 7));
    assert_eq!(Vec2f::from(vi), vec2f(8.0, 9.0));
    assert_eq!(Vec2u::from(vs), vec2u(u32::max_value(), u32::max_value()));
    assert_eq!(Vec2f::from(vs), vec2f(-1.0, -1.0));
    assert_eq!(Vec2i::from(vu), vec2i(-1, -1));
    assert_eq!(Vec2f::from(vu), vec2f(4294967295.0, 4294967295.0));
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
    assert_eq!(Vec3f::signum(vec3f(123.0, -123.0, 999.0)), vec3f(1.0, -1.0, 1.0));
    assert_eq!(Vec3f::signum(vec3f(123.0, -123.0, 999.0)), vec3f(1.0, -1.0, 1.0));
    assert_eq!(Vec3f::signum(vec3f(0.0, -0.0, 0.0)), vec3f(1.0, -1.0, 1.0));
    assert_eq!(Vec3i::signum(vec3i(-1, 0, 1)), vec3i(-1, 0, 1));

    assert_eq!(f32::copysign(10.0, -1.0), -10.0);
    assert_eq!(f32::copysign(-10.0, -1.0), -10.0);
    assert_eq!(f32::copysign(-10.0, 1.0), 10.0);

    assert_eq!(copysign(vec3f(-10.0, 10.0, -10.0), -1.0), vec3f(-10.0, -10.0, -10.0));
    assert_eq!(copysign(vec3f(10.0, 10.0, -10.0), 1.0), vec3f(10.0, 10.0, 10.0));
    assert_eq!(copysign(vec3f(-10.0, -10.0, -10.0), 1.0), vec3f(10.0, 10.0, 10.0));
}

#[test]
fn interpolate() {
    assert_eq!(Vec3f::lerp(vec3f(10.0, 4.0, 60.0), vec3f(20.0, 0.0, -60.0), 0.5), vec3f(15.0, 2.0, 0.0));
    assert_eq!(Vec3f::vlerp(vec3f(10.0, 4.0, 60.0), vec3f(20.0, 0.0, -60.0), splat3f(0.5)), vec3f(15.0, 2.0, 0.0));
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
    let t1 = Mat4f::from_translation(vec3f(10.0, 10.0, 10.0));
    let t2 = Mat4f::from_translation(vec3f(50.0, 44.0, -10.0));
    let result = t1 * t2;
    let colt = result.get_column(3);
    assert_eq!(colt, vec4f(60.0, 54.0, 0.0, 1.0));
    // 3x4
    let t1 = Mat34f::from_translation(vec3f(22.0, 801.0, 554.0));
    let t2 = Mat34f::from_translation(vec3f(13.0, 14.0, 15.0));
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
    let m4 = Mat4f::from_translation(vec3f(500.0, 600.0, 700.0));
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
    let expected = Mat4f::new(
        2.0, 0.0, 0.0, 0.0,
        0.0, 3.0, 0.0, 0.0,
        0.0, 0.0, 4.0, 0.0,
        0.0, 0.0, 0.0, 5.0
    );
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

    let m4 = Mat4f::new(
        1.0, 3.0, 5.0, 9.0,
        1.0, 3.0, 1.0, 7.0,
        4.0, 3.0, 9.0, 7.0,
        5.0, 2.0, 0.0, 9.0
    );
    let det = m4.determinant();
    assert_eq!(det, -376.0);
}

#[test]
fn matrix_transpose() {
    // 2x2
    let m2 = Mat2f::new(
        0.0, 1.0,
        2.0, 3.0
    );
    let t2 = Mat2f::new(
        0.0, 2.0,
        1.0, 3.0
    );
    assert_eq!(m2.transpose(),t2);

    // 3x3
    let m3 = Mat3f::new(
        0.0, 1.0, 2.0,
        3.0, 4.0, 5.0,
        6.0, 7.0, 8.0
    );
    let t3 = Mat3f::new(
        0.0, 3.0, 6.0,
        1.0, 4.0, 7.0,
        2.0, 5.0, 8.0
    );
    assert_eq!(m3.transpose(),t3);

    // 4x4
    let m4 = Mat4f::new(
        0.0, 1.0, 2.0, 3.0, 
        4.0, 5.0, 6.0, 7.0, 
        8.0, 9.0, 10.0, 11.0,
        12.0, 13.0, 14.0, 15.0
    );
    let t4 = Mat4f::new(
        0.0, 4.0, 8.0, 12.0, 
        1.0, 5.0, 9.0, 13.0, 
        2.0, 6.0, 10.0, 14.0,
        3.0, 7.0, 11.0, 15.0
    );
    assert_eq!(m4.transpose(),t4);
}

#[test]
fn matrix_inverse() {
    // 2x2
    let m2 = Mat2f::new(
        5.0, 2.0,
        -7.0, -3.0
    );
    let inv = m2.inverse();
    let expected = Mat2f::new(
        3.0, 2.0,
        -7.0, -5.0
    );
    assert_eq!(inv, expected);
    let m2_inv = m2 * inv;
    assert_eq!(m2_inv, Mat2f::identity());

    // 3x3
    let m3 = Mat3f::new(
        1.0, 2.0, 3.0,
        0.0, 1.0, 4.0,
        5.0, 6.0, 0.0
    );
    let inv = m3.inverse();
    let expected = Mat3f::new(
        -24.0, 18.0, 5.0,
        20.0, -15.0, -4.0,
        -5.0, 4.0, 1.0
    );
    assert_eq!(inv, expected);
    let m3_inv = m3 * inv;
    assert_eq!(m3_inv, Mat3f::identity());

    // 3x4
    let m34 = Mat34f::new(
        1.0, 2.0, 3.0, 10.0,
        0.0, 1.0, 4.0, 100.0,
        5.0, 6.0, 0.0, 20.0,
    );
    let m34_inv = m34 * m34.inverse();
    assert_eq!(m34_inv, Mat34f::identity());

    // 4x4
    let m4 = Mat4f::new(
        1.0, 2.0, 3.0, 10.0,
        0.0, 1.0, 4.0, 100.0,
        5.0, 6.0, 0.0, 20.0,
        9.0, 0.0, 1.0, 1.0,
    );
    let m4_inv = m4 * m4.inverse();
    assert_eq!(Mat4f::approx(m4_inv, Mat4f::identity(), 0.001), true);
}

#[test]
fn matrix_from_get_row_get_column() {
    // 2x2
    let m2 = Mat2f::new(
        1.0, 2.0,
        3.0, 4.0
    );
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
    let m3 = Mat3f::new(
        1.0, 2.0, 3.0,
        4.0, 5.0, 6.0,
        7.0, 8.0, 9.0
    );
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
    let m34 = Mat34f::new(
        1.0, 2.0, 3.0, 4.0, 
        5.0, 6.0, 7.0, 8.0, 
        9.0, 10.0, 11.0, 12.0
    );
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
    let m4 = Mat4f::new(
        1.0, 2.0, 3.0, 4.0, 
        5.0, 6.0, 7.0, 8.0, 
        9.0, 10.0, 11.0, 12.0,
        13.0, 14.0, 15.0, 16.0
    );
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

    // casts
    let m2f = Mat2f::new(
        1.0, 2.0,
        3.0, 4.0,
    );
    let m2d = Mat2d::new(
        1.0, 2.0,
        3.0, 4.0,
    );
    assert_eq!(Mat2d::from(m2f), m2d);
    assert_eq!(Mat2f::from(m2d), m2f);

    let m3f = Mat3f::new(
        1.0, 2.0, 3.0,
        4.0, 5.0, 6.0,
        7.0, 8.0, 9.0
    );
    let m3d = Mat3d::new(
        1.0, 2.0, 3.0,
        4.0, 5.0, 6.0,
        7.0, 8.0, 9.0
    );
    assert_eq!(Mat3d::from(m3f), m3d);
    assert_eq!(Mat3f::from(m3d), m3f);

    let m34f = Mat34f::new(
        1.0, 2.0, 3.0, 4.0,
        5.0, 6.0, 7.0, 8.0,
        9.0, 10.0, 11.0, 12.0
    );
    let m34d = Mat34d::new(
        1.0, 2.0, 3.0, 4.0,
        5.0, 6.0, 7.0, 8.0,
        9.0, 10.0, 11.0, 12.0
    );
    assert_eq!(Mat34d::from(m34f), m34d);
    assert_eq!(Mat34f::from(m34d), m34f);

    let m4f = Mat4f::new(
        1.0, 2.0, 3.0, 4.0,
        5.0, 6.0, 7.0, 8.0,
        9.0, 10.0, 11.0, 12.0,
        13.0, 14.0, 15.0, 16.0
    );
    let m4d = Mat4d::new(
        1.0, 2.0, 3.0, 4.0,
        5.0, 6.0, 7.0, 8.0,
        9.0, 10.0, 11.0, 12.0,
        13.0, 14.0, 15.0, 16.0
    );
    assert_eq!(Mat4d::from(m4f), m4d);
    assert_eq!(Mat4f::from(m4d), m4f);
}

#[test]
fn matrix_deref() {
    let mut m3 = Mat3f::new(
        1.0, 2.0, 3.0,
        4.0, 5.0, 6.0,
        7.0, 8.0, 9.0
    );
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
    let m2 = Mat2f::from_z_rotation(f32::deg_to_rad(90.0));
    let mi = Mat2f::identity();
    let rotated = m2 * mi;
    let expected = Mat2f::new(
        0.0, -1.0,
        1.0, 0.0
    );
    assert_eq!(Mat2f::approx(rotated, expected, 0.001), true);

    // 3x3 z rotation
    let m3 = Mat3f::from_z_rotation(f32::deg_to_rad(-90.0));
    let mi = Mat3f::identity();
    let rotated = m3 * mi;
    let expected = Mat3f::new(
        0.0, 1.0, 0.0,
        -1.0, 0.0, 0.0,
        0.0, 0.0, 1.0
    );
    assert_eq!(Mat3f::approx(rotated, expected, 0.001), true);

    // 3x3 z-axis
    let axis_z = Mat3f::from_rotation(Vec3f::unit_z(), f32::deg_to_rad(-90.0));
    let rotated = axis_z * mi;
    assert_eq!(Mat3f::approx(rotated, expected, 0.001), true);

    // 3x4 z rotation
    let m34 = Mat34f::from_z_rotation(f32::deg_to_rad(270.0));
    let mi = Mat34f::identity();
    let rotated = m34 * mi;
    let expected = Mat34f::new(
        0.0, 1.0, 0.0, 0.0,
        -1.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
    );
    assert_eq!(Mat34f::approx(rotated, expected, 0.001), true);
    
    // 3x4 z-axis
    let axis_z = Mat34f::from_rotation(Vec3f::unit_z(), f32::deg_to_rad(270.0));
    let rotated = axis_z * mi;
    assert_eq!(Mat34f::approx(rotated, expected, 0.001), true);

    // 4x4 z rotation
    let m4 = Mat4f::from_z_rotation(f32::deg_to_rad(180.0));
    let mi = Mat4f::identity();
    let rotated = m4 * mi;
    let expected = Mat4f::new(
        -1.0, 0.0, 0.0, 0.0,
        0.0, -1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    );
    assert_eq!(Mat4f::approx(rotated, expected, 0.001), true);

    // 4x4 z-axis
    let axis_z = Mat4f::from_rotation(Vec3f::unit_z(), f32::deg_to_rad(180.0));
    let rotated = axis_z * mi;
    assert_eq!(Mat4f::approx(rotated, expected, 0.001), true);

    // 3x3 x-rotation
    let m3 = Mat3f::from_x_rotation(f32::deg_to_rad(90.0));
    let mi = Mat3f::identity();
    let rotated = m3 * mi;
    let expected = Mat3f::new(
        1.0, 0.0, 0.0,
        0.0, 0.0, -1.0,
        0.0, 1.0, 0.0
    );
    assert_eq!(Mat3f::approx(rotated, expected, 0.001), true);

    // 3x3 x-axis
    let axis_x = Mat3f::from_rotation(Vec3f::unit_x(), f32::deg_to_rad(90.0));
    let rotated = axis_x * mi;
    assert_eq!(Mat3f::approx(rotated, expected, 0.001), true);

    // 3x4 x-rotation
    let m34 = Mat34f::from_x_rotation(f32::deg_to_rad(180.0));
    let mi = Mat34f::identity();
    let rotated = m34 * mi;
    let expected = Mat34f::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, -1.0, 0.0, 0.0,
        0.0, 0.0, -1.0, 0.0,
    );
    assert_eq!(Mat34f::approx(rotated, expected, 0.001), true);

    // 3x4 x-axis
    let axis_x = Mat34f::from_rotation(Vec3f::unit_x(), f32::deg_to_rad(180.0));
    let rotated = axis_x * mi;
    assert_eq!(Mat34f::approx(rotated, expected, 0.001), true);

    // 4x4 x-rotation
    let m4 = Mat4f::from_x_rotation(f32::deg_to_rad(-90.0));
    let mi = Mat4f::identity();
    let rotated = m4 * mi;
    let expected = Mat4f::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, -1.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    );
    assert_eq!(Mat4f::approx(rotated, expected, 0.001), true);

    // 4x4 x-axis
    let axis_x = Mat4f::from_rotation(Vec3f::unit_x(), f32::deg_to_rad(-90.0));
    let rotated = axis_x * mi;
    assert_eq!(Mat4f::approx(rotated, expected, 0.001), true);

    // 3x3 y-rotation
    let m3 = Mat3f::from_y_rotation(f32::deg_to_rad(90.0));
    let mi = Mat3f::identity();
    let rotated = m3 * mi;
    let expected = Mat3f::new(
        0.0, 0.0, 1.0,
        0.0, 1.0, 0.0,
        -1.0, 0.0, 0.0
    );
    assert_eq!(Mat3f::approx(rotated, expected, 0.001), true);

    // 3x3 y-axis
    let axis_y = Mat3f::from_rotation(Vec3f::unit_y(), f32::deg_to_rad(90.0));
    let rotated = axis_y * mi;
    assert_eq!(Mat3f::approx(rotated, expected, 0.001), true);

    // 3x4 y-rotation
    let m34 = Mat34f::from_y_rotation(f32::deg_to_rad(180.0));
    let mi = Mat34f::identity();
    let rotated = m34 * mi;
    let expected = Mat34f::new(
        -1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, -1.0, 0.0,
    );
    assert_eq!(Mat34f::approx(rotated, expected, 0.001), true);

    // 3x4 y-axis
    let axis_y = Mat34f::from_rotation(Vec3f::unit_y(), f32::deg_to_rad(180.0));
    let rotated = axis_y * mi;
    assert_eq!(Mat34f::approx(rotated, expected, 0.001), true);

    // 4x4 y-rotation
    let m4 = Mat4f::from_y_rotation(f32::deg_to_rad(-90.0));
    let mi = Mat4f::identity();
    let rotated = m4 * mi;
    let expected = Mat4f::new(
        0.0, 0.0, -1.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        1.0, -0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    );
    assert_eq!(Mat4f::approx(rotated, expected, 0.001), true);

    // 4x4 y-axis
    let axis_y = Mat4f::from_rotation(Vec3f::unit_y(), f32::deg_to_rad(-90.0));
    let rotated = axis_y * mi;
    assert_eq!(Mat4f::approx(rotated, expected, 0.001), true);
}

#[test]
fn matrix_mul_vec() {
    // 2x2 * v2 rot
    let m2 = Mat2f::from_scale(vec2f(10.0, 20.0));
    let v2 = vec2f(60.0, 70.0);
    let transformed = m2 * v2;
    let expected = vec2f(600.0, 1400.0);
    assert_eq!(transformed, expected);

    // 2x2 * v2 scale
    let m2 = Mat2f::from_z_rotation(f32::deg_to_rad(90.0));
    let v2 = Vec2f::unit_x();
    let transformed = m2 * v2;
    let expected = vec2f(0.0, 1.0);
    assert_eq!(Vec2f::approx(transformed, expected, 0.001), true);

    // 3x3 * v3 rot
    let m3 = Mat3f::from_z_rotation(f32::deg_to_rad(-90.0));
    let v3 = Vec3f::unit_x();
    let transformed = m3 * v3;
    let expected = vec3f(0.0, -1.0, 0.0);
    assert_eq!(Vec3f::approx(transformed, expected, 0.001), true);

    // 3x3 * v3 scale
    let m3 = Mat3f::from_scale(vec3f(10.0, 11.0, 12.0));
    let v3 = Vec3f::one();
    let transformed = m3 * v3;
    let expected = vec3f(10.0, 11.0, 12.0);
    assert_eq!(Vec3f::approx(transformed, expected, 0.001), true);

    // 3x4 * v3 rot
    let m3 = Mat34f::from_x_rotation(f32::deg_to_rad(-90.0));
    let v3 = vec3f(0.0, 0.0, 5.0);
    let transformed = m3 * v3;
    let expected = vec3f(0.0, 5.0, 0.0);
    assert_eq!(Vec3f::approx(transformed, expected, 0.001), true);

    // 3x4 * v3 scale
    let m3 = Mat34f::from_scale(vec3f(10.0, 2.0, 30.0));
    let v3 = vec3f(1.0, 2.0, 3.0);
    let transformed = m3 * v3;
    let expected = vec3f(10.0, 4.0, 90.0);
    assert_eq!(Vec3f::approx(transformed, expected, 0.001), true);

    // 3x4 * v3 translate
    let m34 = Mat34f::from_translation(vec3f(50.0, -10.0, 20.0));
    let v3 = vec3f(3.0, 4.0, 5.0);
    let transformed = m34 * v3;
    let expected = vec3f(53.0, -6.0, 25.0);
    assert_eq!(Vec3f::approx(transformed, expected, 0.001), true);

    // 4x4 rot
    let m4 = Mat4f::from_z_rotation(f32::deg_to_rad(90.0));
    let v3 = vec3f(10.0, 0.0, 0.0);
    let transformed = m4 * v3;
    let expected = vec3f(0.0, 10.0, 0.0);
    assert_eq!(Vec3f::approx(transformed, expected, 0.001), true);
    
    // 4x4 scale
    let m4 = Mat4f::from_scale(vec3f(50.0, -10.0, 20.0));
    let v3 = vec3f(3.0, 4.0, 5.0);
    let transformed = m4 * v3;
    let expected = vec3f(150.0, -40.0, 100.0);
    assert_eq!(transformed, expected);

    // 4x4 translate
    let m4 = Mat4f::from_translation(vec3f(50.0, 90.0, -20.0));
    let v3 = vec3f(3.0, 4.0, 5.0);
    let transformed = m4 * v3;
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

#[test]
fn closest_point_on_line_test() {
    // 2d
    let l1 = vec2f(0.0, 0.0);
    let l2 = vec2f(0.0, 10.0);
    let p = vec2f(1.0, 5.0);
    let cp = closest_point_on_line_segment(p, l1, l2);
    assert_eq!(cp, vec2f(0.0, 5.0));
    // 3d
    let l1 = vec3f(0.0, 0.0, 20.0);
    let l2 = vec3f(0.0, 10.0, 20.0);
    let p = vec3f(1.0, 5.0, 0.0);
    let cp = closest_point_on_line_segment(p, l1, l2);
    assert_eq!(cp, vec3f(0.0, 5.0, 20.0));
    // 2d clamp to start
    let l1 = vec2f(10.0, 10.0);
    let l2 = vec2f(20.0, 50.0);
    let p = vec2f(-10.0, 0.0);
    let cp = closest_point_on_line_segment(p, l1, l2);
    assert_eq!(cp, vec2f(10.0, 10.0));
    // 2d clamp to end
    let l1 = vec2f(10.0, 10.0);
    let l2 = vec2f(20.0, 50.0);
    let p = vec2f(25.0, 60.0);
    let cp = closest_point_on_line_segment(l1, l2, p);
    assert_eq!(cp, vec2f(20.0, 50.0));
}

#[test]
fn closest_point_on_aabb_test() {
    let aabb_min = vec2f(-10.0, -10.0);
    let aabb_max = vec2f(10.0, 10.0);
    // bottom edge
    let p = vec2f(5.0, -15.0);
    let cp = closest_point_on_aabb(p, aabb_min, aabb_max);
    assert_eq!(cp, vec2f(5.0, -10.0));
    // corner
    let p = vec2f(-15.0, -15.0);
    let cp = closest_point_on_aabb(p, aabb_min, aabb_max);
    assert_eq!(cp, vec2f(-10.0, -10.0));
    // right edge
    let p = vec2f(20.0, -7.0);
    let cp = closest_point_on_aabb(p, aabb_min, aabb_max);
    assert_eq!(cp, vec2f(10.0, -7.0));
    // inside 
    let p = vec2f(1.0, 1.0);
    let cp = closest_point_on_aabb(p, aabb_min, aabb_max);
    assert_eq!(cp, vec2f(1.0, 1.0));
}

#[test]
fn closest_point_on_sphere_test() {
    // circle
    let s = splat2f(0.0);
    let r = 10.0;
    let p = vec2f(20.0, 0.0);
    let cp = closest_point_on_sphere(p, s, r);
    assert_eq!(cp, vec2f(r, 0.0));
    // sphere
    let s = splat3f(0.0);
    let r = 10.0;
    let p = vec3f(20.0, 20.0, 20.0);
    let cp = closest_point_on_sphere(p, s, r);
    assert_eq!(cp, normalize(splat3f(1.0)) * r);
}

#[test]
fn closest_point_on_ray_test() {
    // 2d
    let r0 = vec2f(0.0, 0.0);
    let rv = vec2f(0.0, 1.0);
    let p = vec2f(1.0, 500.0);
    let cp = closest_point_on_ray(p, r0, rv);
    assert_eq!(cp, vec2f(0.0, 500.0));
}

#[test]
fn closest_point_on_obb_test() {
    // 3x4
    let mat = Mat34f::new(
        1.44084, 4.81496, -0.0373597, -0.11, 
        1.65605, -2.58742, -0.655296, -7.14, 
        -1.41194, 1.87878, -0.806716, -1.55, 
    );
    let p = vec3f(-7.98, 0.31, -7.8);
    let result = closest_point_on_obb(p, mat);
    assert_eq!(approx(result, vec3f(-3.49981, -3.17162, -5.17936), 0.01), true);

    // 4x4
    let mat = Mat4f::new(
        1.44084, 4.81496, -0.0373597, -0.11, 
        1.65605, -2.58742, -0.655296, -7.14, 
        -1.41194, 1.87878, -0.806716, -1.55,
        0.0, 0.0, 0.0, 1.0
    );
    let p = vec3f(-7.98, 0.31, -7.8);
    let result = closest_point_on_obb(p, mat);
    assert_eq!(approx(result, vec3f(-3.49981, -3.17162, -5.17936), 0.01), true);
}

#[test]
fn point_inisde_aabb_test() {
    let aabb_min = vec3f(-10.0, -10.0, -10.0);
    let aabb_max = vec3f(10.0, 10.0, 10.0);
    let p = vec3f(-5.0, -5.0, -5.0);
    assert_eq!(point_inside_aabb(p, aabb_min, aabb_max), true);
    let p = vec3f(9.0, 0.0, 9.0);
    assert_eq!(point_inside_aabb(p, aabb_min, aabb_max), true);
    let p = vec3f(-50.0, -50.0, -50.0);
    assert_eq!(point_inside_aabb(p, aabb_min, aabb_max), false);
    let p = vec3f(50.0, 50.0, 50.0);
    assert_eq!(point_inside_aabb(p, aabb_min, aabb_max), false);
}

#[test]
fn point_inside_sphere_test() {
    let s = vec3f(10.0, 5.0, 10.0);
    let r = 10.0;
    let p = vec3f(11.0, 6.0, 11.0);
    assert_eq!(point_inside_sphere(p, s, r), true);
    let p = vec3f(0.0, 0.0, 0.0);
    assert_eq!(point_inside_sphere(p, s, r), false);
}

#[test]
fn point_inside_obb_test() {
    // inside
    let mat = Mat34f::new(
        -2.10233, -0.747065, 0.887925, -3.2, 
        0.964173, -2.97305, -0.208202, 3.48, 
        7.7732, 0.166721, 0.265972, -0.21,
    );
    let p = vec3f(-4.73, 6.14, 7.15);
    let result = point_inside_obb(p, mat);
    assert_eq!(result, true);
    // outside
    let mat = Mat34f::new(
        1.44084, 4.81496, -0.0373597, -0.11, 
        1.65605, -2.58742, -0.655296, -7.14, 
        -1.41194, 1.87878, -0.806716, -1.55
    );
    let p = vec3f(-7.98, 0.31, -7.8);
    let result = point_inside_obb(p, mat);
    assert_eq!(result, false);

    // 4x4
    // inside
    let mat = Mat4f::new(
        -2.10233, -0.747065, 0.887925, -3.2, 
        0.964173, -2.97305, -0.208202, 3.48, 
        7.7732, 0.166721, 0.265972, -0.21,
        0.0, 0.0, 0.0, 1.0
    );
    let p = vec3f(-4.73, 6.14, 7.15);
    let result = point_inside_obb(p, mat);
    assert_eq!(result, true);
    // outside
    let mat = Mat4f::new(
        1.44084, 4.81496, -0.0373597, -0.11, 
        1.65605, -2.58742, -0.655296, -7.14, 
        -1.41194, 1.87878, -0.806716, -1.55,
        0.0, 0.0, 0.0, 1.0
    );
    let p = vec3f(-7.98, 0.31, -7.8);
    let result = point_inside_obb(p, mat);
    assert_eq!(result, false);
}

#[test]
fn point_inside_frustum_test() {
    let planes = Mat4f::new(
		0.85501, 1.45179e-08, 0.467094, 0.0, 
		0.39811, 1.52002, -0.728735, 0.0, 
		0.420904, -0.479617, -0.770459, 60.004, 
		0.420736, -0.479426, -0.770151, 60.0
    ).get_frustum_planes();

    // inside
    let pos = vec3f(-5.0, 5.0, 50.0);
    let result = point_inside_frustum(pos, &planes);
    assert_eq!(result, false);

    let pos = vec3f(-8.76, -8.04, 5.3);
    let result = point_inside_frustum(pos, &planes);
    assert_eq!(result, false);

    // outside
    let pos = vec3f(4.85, 7.45, 2.28);
    let result = point_inside_frustum(pos, &planes);
    assert_eq!(result, false);

    let pos = vec3f(0.0100002, 1.53, -2.92);
    let result = point_inside_frustum(pos, &planes);
    assert_eq!(result, false);
}

#[test]
fn plane_distance_test() {
    let x = vec3f(-1.22, 9.23, 7.09);
    let n = vec3f(-0.675523, 0.731817, -0.0900697);
    let p = vec3f(-0.7, 2.72, 5.44);
    assert_eq!(approx(point_plane_distance(p, x, n), -4.96678, 0.0001), true);

    let x = vec3f(-6.73, 7.29, -1.6);
    let n = vec3f(-0.786656, 0.0268178, 0.61681);
    let p = vec3f(0.42, 9.87, -4.97);
    assert_eq!(approx(point_plane_distance(p, x, n), -7.63405, 0.0001), true);

    let x = vec3f(-0.67, 0.99, -7.22);
    let n = vec3f(-0.922576, 0.384407, -0.0329491);
    let p = vec3f(7.09, 1.57, 5.6);
    assert_eq!(approx(point_plane_distance(p, x, n), -7.35864, 0.0001), true);
}

#[test]
fn closest_point_on_plane_test() {
    let x = vec3f(0.0, 0.0, 0.0);
    let n = vec3f(0.0, 1.0, 0.0);
    let p = vec3f(10.0, 10.0, 10.0);
    assert_eq!(closest_point_on_plane(p, x, n), vec3f(10.0, 0.0, 10.0));
    let x = vec3f(0.0, 0.0, -5.0);
    let n = vec3f(0.0, 0.0, -1.0);
    let p = vec3f(10.0, 10.0, 10.0);
    assert_eq!(closest_point_on_plane(p, x, n), vec3f(10.0, 10.0, -5.0));
}

#[test]
fn point_line_segment_distance_test() {
    let x0 = vec3f(0.83, -9.52, -1.35);
    let x1 = vec3f(-2.73, 2.4, 7.54);
    let x2 = vec3f(-4.6, -6.04, -0.65);
    let result = point_line_segment_distance(x0, x1, x2);
    assert_eq!(approx(result, 6.48732, 0.001), true);

    let x0 = vec3f(-8.08, -2.9, -2.53);
    let x1 = vec3f(8.76, 2.37, -3.25);
    let x2 = vec3f(-7.55, -2.9, -5.85);
    let result = point_line_segment_distance(x0, x1, x2);
    assert_eq!(approx(result, 3.36204, 0.001), true);

    let x0 = vec3f(5.01, 5.25, -7.11);
    let x1 = vec3f(7.15, 6.93, 5.79);
    let x2 = vec3f(1.29, 1.64, 9.79);
    let result = point_line_segment_distance(x0, x1, x2);
    assert_eq!(approx(result, 13.1838, 0.001), true);
}

#[test]
fn point_aabb_distance_test() {
    let p = vec2f(233.960938, 277.550781);
    let aabb_min = vec2f(172.299042, 266.398956);
    let aabb_max = vec2f(304.234772, 287.898956);
    let result = point_aabb_distance(p, aabb_min, aabb_max);
    assert_eq!(approx(result, 0.0, 0.001), true);

    let p = vec2f(233.960938, 277.550781);
    let aabb_min = vec2f(193.332703, 505.797485);
    let aabb_max = vec2f(291.221558, 532.797485);
    let result = point_aabb_distance(p, aabb_min, aabb_max);
    assert_eq!(approx(result, 228.246704, 0.001), true);

    let p = vec2f(274.113281, 513.644531);
    let aabb_min = vec2f(172.299042, 266.398956);
    let aabb_max = vec2f(304.234772, 287.898956);
    let result = point_aabb_distance(p, aabb_min, aabb_max);
    assert_eq!(approx(result, 225.745575, 0.001), true);
}

#[test]
fn point_triangle_distance_test() {
    let x0 = vec3f(-9.43, 0.0, 5.1);
    let x1 = vec3f(2.95, 0.0, 8.21);
    let x2 = vec3f(7.31, 0.0, -4.99);
    let x3 = vec3f(-7.55, 0.0, -0.39);
    let result = point_triangle_distance(x0, x1, x2, x3);
    assert_eq!(approx(result, 5.43846, 0.001), true);

    let x0 = vec3f(-3.25, 0.0, -4.82);
    let x1 = vec3f(-8.27, 0.0, -8.63);
    let x2 = vec3f(6.69, 0.0, 0.96);
    let x3 = vec3f(-4.97, 0.0, 4.21);
    let result = point_triangle_distance(x0, x1, x2, x3);
    assert_eq!(approx(result, 4.76837e-07, 0.001), true);

    let x0 = vec3f(-2.65, 0.0, 4.15);
    let x1 = vec3f(-7.38, 0.0, -9.46);
    let x2 = vec3f(-7.37, 0.0, -3.57);
    let x3 = vec3f(-1.11, 0.0, 2.5);
    let result = point_triangle_distance(x0, x1, x2, x3);
    assert_eq!(approx(result, 2.25701, 0.001), true);
}

#[test]
fn point_inside_triangle_test() {
    let p = vec3f(-3.25, 0.0, -4.82);
    let v1 = vec3f(-8.27, 0.0, -8.63);
    let v2 = vec3f(6.69, 0.0, 0.96);
    let v3 = vec3f(-4.97, 0.0, 4.21);
    assert_eq!(point_inside_triangle(p, v1, v2, v3), true);

    let p = vec3f(-4.57, 0.0, -2.67);
    let v1 = vec3f(-0.2, 0.0, 0.0);
    let v2 = vec3f(5.96, 0.0, 9.47);
    let v3 = vec3f(3.7, 0.0, -9.13);
    assert_eq!(point_inside_triangle(p, v1, v2, v3), false);

    let p = vec3f(-2.65, 0.0, 4.15);
    let v1 = vec3f(-7.38, 0.0, -9.46);
    let v2 = vec3f(-7.37, 0.0, -3.57);
    let v3 = vec3f(-1.11, 0.0, 2.5);
    assert_eq!(point_inside_triangle(p, v1, v2, v3), false);
}

#[test]
fn closest_point_on_triangle_test() {
    let p = vec3f(-2.65, 0.0, 4.15);
    let v1 = vec3f(-7.38, 0.0, -9.46);
    let v2 = vec3f(-7.37, 0.0, -3.57);
    let v3 = vec3f(-1.11, 0.0, 2.5);
    let result = closest_point_on_triangle(p, v1, v2, v3);
    assert_eq!(approx(result, vec3f(-1.11, 0.0, 2.5), 0.001), true);

    let p = vec3f(4.2, 0.0, -9.4);
    let v1 = vec3f(-0.37, 0.0, 7.58);
    let v2 = vec3f(6.52, 0.0, 0.92);
    let v3 = vec3f(3.16, 0.0, -2.86);
    let result = closest_point_on_triangle(p, v1, v2, v3);
    assert_eq!(approx(result, vec3f(3.16, 0.0, -2.86), 0.001), true);

    let p = vec3f(1.28, 0.0, -4.29);
    let v1 = vec3f(-2.59, 0.0, 8.47);
    let v2 = vec3f(1.96, 0.0, -2.13);
    let v3 = vec3f(-4.99, 0.0, -7.66);
    let result = closest_point_on_triangle(p, v1, v2, v3);
    assert_eq!(approx(result, vec3f(0.491223, 0.0, -3.29868), 0.001), true);

    let p = vec3f(-1.8, 0.0, 7.64);
    let v1 = vec3f(2.53, 0.0, 8.71);
    let v2 = vec3f(3.29, 0.0, -7.07);
    let v3 = vec3f(7.19, 0.0, 4.71);
    let result = closest_point_on_triangle(p, v1, v2, v3);
    assert_eq!(approx(result, vec3f(2.57139, 0.0, 7.85054), 0.001), true);

    let p = vec3f(-9.56, 0.0, -1.72);
    let v1 = vec3f(-5.9, 0.0, -9.2);
    let v2 = vec3f(-8.87, 0.0, 3.42);
    let v3 = vec3f(-8.62, 0.0, -6.18);
    let result = closest_point_on_triangle(p, v1, v2, v3);
    assert_eq!(approx(result, vec3f(-8.7367, 0.0, -1.69856), 0.001), true);

    let p = vec3f(-4.57, 0.0, -2.67);
    let v1 = vec3f(-0.2, 0.0, 0.0);
    let v2 = vec3f(5.96, 0.0, 9.47);
    let v3 = vec3f(3.7, 0.0, -9.13);
    let result = closest_point_on_triangle(p, v1, v2, v3);
    assert_eq!(approx(result, vec3f(0.0901885, 0.0, -0.67933846), 0.001), true);
}

#[test]
fn distance_on_line_test() {
    let p = vec2f(5.0, 2.0);
    let l1 = vec2f(0.0, 0.0);
    let l2 = vec2f(0.0, 10.0);
    assert_eq!(distance_on_line(p, l1, l2), 2.0);

    let p = vec2f(5.0, -10.0);
    let l1 = vec2f(0.0, 0.0);
    let l2 = vec2f(0.0, 10.0);
    assert_eq!(distance_on_line(p, l1, l2), -10.0);
}

#[test]
fn point_cone_tests() {
    {
        let cp = vec3f(13.270000, 2.290000, 3.400000);
        let cv = vec3f(0.526860, -0.833470, 0.166570);
        let h = 3.030000;
        let r = 8.880000;
        let p = vec3f(-4.580000, 9.870000, -9.970000);
        let overlap = point_inside_cone(p, cp, cv, h, r);
        let cpp = closest_point_on_cone(p, cp, cv, h, r);
        let dd = point_cone_distance(p, cp, cv, h, r);
        // point_cone_distance
        assert_eq!(approx(dd, 21.913309, 0.001), true);
        // point_inside_cone
        assert_eq!(overlap, false);
        // closest_point_on_cone
        assert_eq!(approx(cpp, vec3f(10.240630, -3.991611, -1.699705), 0.001), true);
    }
    {
        let cp = vec3f(-0.670000, 0.990000, -7.220000);
        let cv = vec3f(0.825527, 0.544982, -0.146629);
        let h = 8.350000;
        let r = 8.160000;
        let p = vec3f(-7.910000, 11.570000, 10.600000);
        let overlap = point_inside_cone(p, cp, cv, h, r);
        let cpp = closest_point_on_cone(p, cp, cv, h, r);
        let dd = point_cone_distance(p, cp, cv, h, r);
        // point_cone_distance
        assert_eq!(approx(dd, 17.609268, 0.001), true);
        // point_inside_cone
        assert_eq!(overlap, false);
        // closest_point_on_cone
        assert_eq!(approx(cpp, vec3f(4.383177, 10.083116, -1.920115), 0.001), true);
    }
    {
        let cp = vec3f(3.100000, 6.330000, -0.210000);
        let cv = vec3f(-0.236535, 0.969756, 0.060208);
        let h = 5.790000;
        let r = 6.490000;
        let p = vec3f(-6.740000, 5.120000, 7.670000);
        let overlap = point_inside_cone(p, cp, cv, h, r);
        let cpp = closest_point_on_cone(p, cp, cv, h, r);
        let dd = point_cone_distance(p, cp, cv, h, r);
        // point_cone_distance
        assert_eq!(approx(dd, 7.358819, 0.001), true);
        // point_inside_cone
        assert_eq!(overlap, false);
        // closest_point_on_cone
        assert_eq!(approx(cpp, vec3f(-3.155349, 10.503510, 4.159968), 0.001), true);
    }
    {
        let cp = vec3f(8.360001, 4.850000, 7.450001);
        let cv = vec3f(0.305963, -0.738557, 0.600767);
        let h = 0.910000;
        let r = 7.720000;
        let p = vec3f(9.670000, 11.720000, 3.930000);
        let overlap = point_inside_cone(p, cp, cv, h, r);
        let cpp = closest_point_on_cone(p, cp, cv, h, r);
        let dd = point_cone_distance(p, cp, cv, h, r);
        // point_cone_distance
        assert_eq!(approx(dd, 7.197949, 0.001), true);
        // point_inside_cone
        assert_eq!(overlap, false);
        // closest_point_on_cone
        assert_eq!(approx(cpp, vec3f(11.125882, 6.039529, 8.104102), 0.001), true);
    }
    {
        let cp = vec3f(7.080000, 14.440000, -3.320000);
        let cv = vec3f(-0.811891, -0.129089, 0.569358);
        let h = 6.240000;
        let r = 4.900000;
        let p = vec3f(3.570000, 10.010000, 1.530000);
        let overlap = point_inside_cone(p, cp, cv, h, r);
        let cpp = closest_point_on_cone(p, cp, cv, h, r);
        let dd = point_cone_distance(p, cp, cv, h, r);
        // point_cone_distance
        assert_eq!(approx(dd, 0.057009, 0.001), true);
        // point_inside_cone
        assert_eq!(overlap, true);
        // closest_point_on_cone
        assert_eq!(approx(cpp, vec3f(3.523714, 10.002641, 1.562458), 0.001), true);
    }
    {
        let cp = vec3f(11.660000, 5.490000, -4.760000);
        let cv = vec3f(-0.497079, -0.855900, -0.142643);
        let h = 8.530000;
        let r = 6.990000;
        let p = vec3f(10.299999, 4.030000, -7.780000);
        let overlap = point_inside_cone(p, cp, cv, h, r);
        let cpp = closest_point_on_cone(p, cp, cv, h, r);
        let dd = point_cone_distance(p, cp, cv, h, r);
        // point_cone_distance
        assert_eq!(approx(dd, 0.631556, 0.001), true);
        // point_inside_cone
        assert_eq!(overlap, false);
        // closest_point_on_cone
        assert_eq!(approx(cpp, vec3f(10.134563, 3.588377, -7.359925), 0.001), true);
    }
    {
        let cp = vec3f(-7.020000, 4.810000, 1.350000);
        let cv = vec3f(0.754257, -0.605441, -0.254044);
        let h = 13.650000;
        let r = 4.870000;
        let p = vec3f(14.080000, 2.280000, 14.330000);
        let overlap = point_inside_cone(p, cp, cv, h, r);
        let cpp = closest_point_on_cone(p, cp, cv, h, r);
        let dd = point_cone_distance(p, cp, cv, h, r);
        // point_cone_distance
        assert_eq!(approx(dd, 15.629295, 0.001), true);
        // point_inside_cone
        assert_eq!(overlap, false);
        // closest_point_on_cone
        assert_eq!(approx(cpp, vec3f(5.753925, -2.019639, 1.821424), 0.001), true);
    }
    {
        let cp = vec3f(1.160000, 5.800000, 3.670000);
        let cv = vec3f(-0.850492, 0.438373, 0.290675);
        let h = 4.390000;
        let r = 8.520000;
        let p = vec3f(-1.020000, 5.920000, 7.200001);
        let overlap = point_inside_cone(p, cp, cv, h, r);
        let cpp = closest_point_on_cone(p, cp, cv, h, r);
        let dd = point_cone_distance(p, cp, cv, h, r);
        // point_cone_distance
        assert_eq!(approx(dd, 1.261747, 0.001), true);
        // point_inside_cone
        assert_eq!(overlap, true);
        // closest_point_on_cone
        assert_eq!(approx(cpp, vec3f(-0.004237, 5.198958, 7.400818), 0.001), true);
    }
    {
        let cp = vec3f(11.510000, 10.490000, 7.660000);
        let cv = vec3f(0.001249, -0.209566, 0.977794);
        let h = 6.100000;
        let r = 14.670000;
        let p = vec3f(8.340000, 14.390000, 11.840000);
        let overlap = point_inside_cone(p, cp, cv, h, r);
        let cpp = closest_point_on_cone(p, cp, cv, h, r);
        let dd = point_cone_distance(p, cp, cv, h, r);
        // point_cone_distance
        assert_eq!(approx(dd, 0.841472, 0.001), true);
        // point_inside_cone
        assert_eq!(overlap, true);
        // closest_point_on_cone
        assert_eq!(approx(cpp, vec3f(8.157934, 14.814392, 11.136566), 0.001), true);
    }
    {
        let cp = vec3f(7.950001, 4.360000, -6.260000);
        let cv = vec3f(-0.625592, 0.727616, 0.281442);
        let h = 12.310000;
        let r = 5.650000;
        let p = vec3f(3.270000, 12.730000, -4.190000);
        let overlap = point_inside_cone(p, cp, cv, h, r);
        let cpp = closest_point_on_cone(p, cp, cv, h, r);
        let dd = point_cone_distance(p, cp, cv, h, r);
        // point_cone_distance
        assert_eq!(approx(dd, 2.170182, 0.001), true);
        // point_inside_cone
        assert_eq!(overlap, true);
        // closest_point_on_cone
        assert_eq!(approx(cpp, vec3f(5.131972, 13.424145, -5.062304), 0.001), true);
    }
    {
        let cp = vec3f(1.840000, 0.550000, 6.559999);
        let cv = vec3f(0.081153, 0.994310, -0.069000);
        let h = 1.640000;
        let r = 13.990000;
        let p = vec3f(10.340000, 0.830000, 1.750000);
        let overlap = point_inside_cone(p, cp, cv, h, r);
        let cpp = closest_point_on_cone(p, cp, cv, h, r);
        let dd = point_cone_distance(p, cp, cv, h, r);
        // point_cone_distance
        assert_eq!(approx(dd, 0.163782, 0.001), true);
        // point_inside_cone
        assert_eq!(overlap, true);
        // closest_point_on_cone
        assert_eq!(approx(cpp, vec3f(10.343329, 0.666263, 1.751929), 0.001), true);
    }
}

#[test]
fn point_vs_plane_test() {
    let x = vec3f(6.580000, -0.700000, 2.720000);
    let n = vec3f(0.810243, -0.405122, 0.423536);
    let p = vec3f(-1.930000, 2.490000, -9.270000);
    let c = point_vs_plane(p, x, n);
    assert_eq!(c, Classification::Behind);

    let x = vec3f(-5.080000, 0.420000, 9.870001);
    let n = vec3f(0.075497, 0.679474, 0.729805);
    let p = vec3f(7.090000, -5.600000, -8.350000);
    let c = point_vs_plane(p, x, n);
    assert_eq!(c, Classification::Behind);

    let x = vec3f(1.690000, 7.090000, 1.570000);
    let n = vec3f(0.876123, 0.481867, -0.014602);
    let p = vec3f(-1.600000, -3.880000, -6.970000);
    let c = point_vs_plane(p, x, n);
    assert_eq!(c, Classification::Behind);

    let x = vec3f(0.970000, 8.260000, 5.120000);
    let n = vec3f(-0.282165, -0.769540, -0.572880);
    let p = vec3f(-7.220000, 8.160000, 3.350000);
    let c = point_vs_plane(p, x, n);
    assert_eq!(c, Classification::Infront);

    let x = vec3f(-8.059999, -6.430000, 0.010000);
    let n = vec3f(0.764273, 0.115362, 0.634491);
    let p = vec3f(7.450001, 2.280000, -9.090000);
    let c = point_vs_plane(p, x, n);
    assert_eq!(c, Classification::Infront);

    let x = vec3f(-0.230000, 4.080000, -7.720000);
    let n = vec3f(0.377198, -0.022860, 0.925850);
    let p = vec3f(-9.760000, 8.010000, -1.470000);
    let c = point_vs_plane(p, x, n);
    assert_eq!(c, Classification::Infront);

    let x = vec3f(-1.180000, -2.480000, 5.660000);
    let n = vec3f(0.216354, 0.500320, -0.838374);
    let p = vec3f(-8.120000, 1.230000, 5.050000);
    let c = point_vs_plane(p, x, n);
    assert_eq!(c, Classification::Infront);

    let x = vec3f(8.709999, 8.280001, 1.370000);
    let n = vec3f(0.424115, 0.563050, 0.709296);
    let p = vec3f(1.440000, 5.010000, -1.030000);
    let c = point_vs_plane(p, x, n);
    assert_eq!(c, Classification::Behind);

    let x = vec3f(-7.690000, 7.450001, -8.250000);
    let n = vec3f(-0.839636, -0.025835, 0.542534);
    let p = vec3f(-7.060000, 9.040001, 6.090000);
    let c = point_vs_plane(p, x, n);
    assert_eq!(c, Classification::Infront);

    let x = vec3f(6.580000, -0.700000, 2.720000);
    let n = vec3f(0.810243, -0.405122, 0.423536);
    let p = vec3f(6.580000, -0.700000, 2.720000);
    let c = point_vs_plane(p, x, n);
    assert_eq!(c, Classification::Intersects);
}

#[test]
fn aabb_vs_plane_test() {
    let aabb_min = vec3f(1.11, 6.35, 5.56);
    let aabb_max = vec3f(5.59, 11.01, 14.34);
    let x0 = vec3f(6.62, -7.89, 8.08);
    let n = vec3f(0.623017, 0.493559, -0.606835);
    let result = aabb_vs_plane(aabb_min, aabb_max, x0, n);
    assert_eq!(result, Classification::Intersects);

    let aabb_min = vec3f(-16.95, -4.23, -2.3);
    let aabb_max = vec3f(-1.17, 3.91, 17.4);
    let x0 = vec3f(8.74, 3.31, -4.44);
    let n = vec3f(0.0151013, -0.422837, 0.90608);
    let result = aabb_vs_plane(aabb_min, aabb_max, x0, n);
    assert_eq!(result, Classification::Infront);

    let aabb_min = vec3f(-1.45, 2.53, -9.93);
    let aabb_max = vec3f(0.71, 7.51, 3.45);
    let x0 = vec3f(-8.07, -2.31, 6.8);
    let n = vec3f(-0.703985, -0.703985, 0.0938646);
    let result = aabb_vs_plane(aabb_min, aabb_max, x0, n);
    assert_eq!(result, Classification::Behind);
}

#[test]
fn sphere_vs_plane_test() {
    let s = vec3f(-4.54, 1.07, 2.38);
    let r = 4.8;
    let x0 = vec3f(3.58, 2.0, 2.54);
    let n = vec3f(0.772411, 0.193103, 0.605056);
    let result = sphere_vs_plane(s, r, x0, n);
    assert_eq!(result, Classification::Behind);

    let s = vec3f(-5.82, 3.57, -4.15);
    let r = 5.93;
    let x0 = vec3f(-10.0, -0.74, 0.35);
    let n = vec3f(-0.91386, 0.276283, 0.297536);
    let result = sphere_vs_plane(s, r, x0, n);
    assert_eq!(result, Classification::Intersects);

    let s = vec3f(1.32, -0.27, 6.03);
    let r = 4.06;
    let x0 = vec3f(-5.27, 1.03, 7.48);
    let n = vec3f(0.82288, -0.4066, -0.396919);
    let result = sphere_vs_plane(s, r, x0, n);
    assert_eq!(result, Classification::Infront);
}

#[test]
fn line_vs_plane_test() {
    let r0 = vec3f(-2.48, 5.66, -2.84);
    let rv = vec3f(0.437602, -0.733279, 0.520391);
    let x = vec3f(5.01, -1.03, 8.71);
    let n = vec3f(-0.723007, 0.371545, 0.582422);
    let result = line_vs_plane(r0, rv, x, n);
    assert_eq!(result.is_some(), true);
    if let Some(result) = result {
        assert_eq!(approx(result, vec3f(-0.682132, 2.64736, -0.701995), 0.001), true);
    }

    let r0 = vec3f(1.77, -6.03, -7.06);
    let rv = vec3f(0.0350043, -0.796348, -0.603825);
    let x = vec3f(7.45, -8.25, 6.35);
    let n = vec3f(-0.0185944, 0.390482, 0.920423);
    let result = line_vs_plane(r0, rv, x, n);
    assert_eq!(result.is_some(), true);
    if let Some(result) = result {
        assert_eq!(approx(result, vec3f(1.31114, 4.40918, 0.855423), 0.001), true);
    }

    let r0 = vec3f(9.68, -5.88, -7.4);
    let rv = vec3f(0.39763, 0.655741, -0.641789);
    let x = vec3f(-6.05, 9.68, 1.13);
    let n = vec3f(0.257437, -0.806637, 0.532037);
    let result = line_vs_plane(r0, rv, x, n);
    assert_eq!(result.is_some(), true);
    if let Some(result) = result {
        assert_eq!(approx(result, vec3f(15.925, 4.41882, -17.4797), 0.001), true);
    }
    
    // this will not intersect
    let x = vec3f(0.0, 0.0, 0.0);
    let n = vec3f(1.0, 0.0, 0.0);
    let r0 = vec3f(10.0, 5.0, -10.0);
    let rv = vec3f(0.0, 0.0, 1.0);
    let result = line_vs_plane(r0, rv, x, n);
    assert_eq!(result.is_none(), true);
}

#[test]
fn ray_vs_plane_test() {
    // intersect
    let r0 = vec3f(-2.48, 5.66, -2.84);
    let rv = vec3f(0.437602, -0.733279, 0.520391);
    let x = vec3f(5.01, -1.03, 8.71);
    let n = vec3f(-0.723007, 0.371545, 0.582422);
    let result = ray_vs_plane(r0, rv, x, n);
    assert_eq!(result.is_some(), true);
    if let Some(result) = result {
        assert_eq!(approx(result, vec3f(-0.682132, 2.64736, -0.701995), 0.001), true);
    }

    let r0 = vec3f(9.68, -5.88, -7.4);
    let rv = vec3f(0.39763, 0.655741, -0.641789);
    let x = vec3f(-6.05, 9.68, 1.13);
    let n = vec3f(0.257437, -0.806637, 0.532037);
    let result = ray_vs_plane(r0, rv, x, n);
    assert_eq!(result.is_some(), true);
    if let Some(result) = result {
        assert_eq!(approx(result, vec3f(15.925, 4.41882, -17.4797), 0.001), true);
    }
    
    // no intersect
    let x = vec3f(0.0, 0.0, 0.0);
    let n = vec3f(1.0, 0.0, 0.0);
    let r0 = vec3f(10.0, 5.0, -10.0);
    let rv = vec3f(0.0, 0.0, 1.0);
    let result = ray_vs_plane(r0, rv, x, n);
    assert_eq!(result.is_none(), true);

    let r0 = vec3f(1.77, -6.03, -7.06);
    let rv = vec3f(0.0350043, -0.796348, -0.603825);
    let x = vec3f(7.45, -8.25, 6.35);
    let n = vec3f(-0.0185944, 0.390482, 0.920423);
    let result = ray_vs_plane(r0, rv, x, n);
    assert_eq!(result.is_none(), true);
}

#[test]
pub fn line_segment_vs_plane_test() {
    let x = vec3f(0.0, 0.0, 0.0);
    let n = vec3f(1.0, 0.0, 0.0);
    // intersects
    let l1 = vec3f(10.0, 5.0, 10.0);
    let l2 = vec3f(-10.0, 5.0, 10.0);
    let ip = line_segment_vs_plane(l1, l2, x, n);
    assert_eq!(ip.is_some(), true);
    if let Some(ip) = ip {
        assert_eq!(ip, vec3f(0.0, 5.0, 10.0));
    }
    // does not intersect
    let l1 = vec3f(10.0, 5.0, -10.0);
    let l2 = vec3f(10.0, 5.0, 10.0);
    let ip = line_segment_vs_plane(l1, l2, x, n);
    assert_eq!(ip.is_none(), true);
}

#[test]
pub fn sphere_vs_sphere_test() {
    // intersect
    let s0 = vec3f(-7.15, 8.78, -7.16);
    let r0 = 6.98;
    let s1 = vec3f(0.67, 1.4, -2.93);
    let r1 = 6.58;
    let result = sphere_vs_sphere(s0, r0, s1, r1);
    assert_eq!(result, true);
    // intersect
    let s0 = vec3f(5.49, 2.66, 4.67);
    let r0 = 9.39;
    let s1 = vec3f(7.73, 5.81, 2.95);
    let r1 = 8.9;
    let result = sphere_vs_sphere(s0, r0, s1, r1);
    assert_eq!(result, true);
    // no intersect
    let s0 = vec3f(3.21, 1.51, 2.26);
    let r0 = 6.15;
    let s1 = vec3f(-2.86, -5.63, 8.79);
    let r1 = 2.51;
    let result = sphere_vs_sphere(s0, r0, s1, r1);
    assert_eq!(result, false);
}

#[test]
pub fn aabb_vs_sphere_test() {
    let s0 = vec3f(5.95, -5.31, -2.9);
    let r0 = 7.49;
    let aabb_min = vec3f(-5.71, -11.76, -14.12);
    let aabb_max = vec3f(13.33, 7.16, -5.32);
    let result = aabb_vs_sphere(aabb_min, aabb_max, s0, r0);
    assert_eq!(result, true);

    let s0 = vec3f(-3.3, 3.94, -3.9);
    let r0 = 2.43;
    let aabb_min = vec3f(-16.09, -10.73, -4.44);
    let aabb_max = vec3f(0.47, 7.73, 13.06);
    let result = aabb_vs_sphere(aabb_min, aabb_max, s0, r0);
    assert_eq!(result,true);

    let s0 = vec3f(-3.95, 6.07, -3.67);
    let r0 = 8.11;
    let aabb_min = vec3f(2.58, -7.65, -6.75);
    let aabb_max = vec3f(16.34, 4.67, 0.19);
    let result = aabb_vs_sphere(aabb_min, aabb_max, s0, r0);
    assert_eq!(result,true);

    let s0 = vec3f(7.28, 1.2, -0.98);
    let r0 = 3.55;
    let aabb_min = vec3f(7.49, -5.93, -8.37);
    let aabb_max = vec3f(11.63, 1.35, -4.81);
    let result = aabb_vs_sphere(aabb_min, aabb_max, s0, r0);
    assert_eq!(result,false);
}

#[test]
fn aabb_vs_aabb_test() {
    let min0 = vec3f(6.23, 0.35, -5.4);
    let max0 = vec3f(12.35, 17.15, -2.64);
    let min1 = vec3f(-4.32, -3.97, -7.92);
    let max1 = vec3f(9.84, 8.69, 0.84);
    let result = aabb_vs_aabb(min0, max0, min1, max1);
    assert_eq!(result,true);

    let min0 = vec3f(-5.54, 2.4, -4.68);
    let max0 = vec3f(-3.94, 7.04, 9.06);
    let min1 = vec3f(-15.49, -13.51, 3.21);
    let max1 = vec3f(-2.67, 2.41, 4.19);
    let result = aabb_vs_aabb(min0, max0, min1, max1);
    assert_eq!(result,true);

    let min0 = vec3f(0.59, -15.13, -2.59);
    let max0 = vec3f(17.91, -4.07, 5.65);
    let min1 = vec3f(-16.04, -14.82, -4.47);
    let max1 = vec3f(-1.18, -0.16, 9.63);
    let result = aabb_vs_aabb(min0, max0, min1, max1);
    assert_eq!(result,false);
}

#[test]
fn convex_hull() {
    //  0   1   2   3   4   5   6   7
    //  -   -   -   x   -   -   -   -          
    //  a   x   0       0   x       y
    //          0   0   0   0
    //  x       0   0       0       x
    //              0
    //              0
    //  a           x       a
    //

    // x = edge, 0 = dscarded, a = outside

    let points = vec![
        vec2f(3.0, 0.0), // x
        vec2f(1.0, 1.0), // x
        vec2f(2.0, 1.0), // 0
        vec2f(4.0, 1.0), // 0
        vec2f(5.0, 1.0), // x
        vec2f(2.0, 2.0), // 0
        vec2f(3.0, 2.0), // 0
        vec2f(4.0, 2.0), // 0
        vec2f(5.0, 2.0), // 0
        vec2f(0.0, 3.0), // x
        vec2f(2.0, 3.0), // 0
        vec2f(3.0, 3.0), // 0
        vec2f(5.0, 3.0), // 0
        vec2f(7.0, 3.0), // x
        vec2f(3.0, 4.0), // 0
        vec2f(3.0, 5.0), // 0
        vec2f(3.0, 6.0), // x
    ];

    let hull = convex_hull_from_points(&points);

    let expected = vec![
        vec2f(7.0, 3.0), // x
        vec2f(3.0, 6.0), // x
        vec2f(0.0, 3.0), // x
        vec2f(1.0, 1.0), // x
        vec2f(3.0, 0.0), // x
        vec2f(5.0, 1.0), // x
    ];

    assert_eq!(hull, expected);

    // outside
    assert_eq!(point_inside_convex_hull(vec2f(0.0, 0.0), &hull), false);
    assert_eq!(point_inside_convex_hull(vec2f(0.0, 5.0), &hull), false);
    assert_eq!(point_inside_convex_hull(vec2f(5.0, 5.0), &hull), false);

    // inside
    assert_eq!(point_inside_convex_hull(vec2f(1.0, 2.0), &hull), false);
    assert_eq!(point_inside_convex_hull(vec2f(1.0, 3.0), &hull), false);
    assert_eq!(point_inside_convex_hull(vec2f(1.0, 4.0), &hull), false);
    assert_eq!(point_inside_convex_hull(vec2f(1.0, 5.0), &hull), false);
    assert_eq!(point_inside_convex_hull(vec2f(2.0, 2.0), &hull), false);
    assert_eq!(point_inside_convex_hull(vec2f(2.0, 3.0), &hull), false);
    assert_eq!(point_inside_convex_hull(vec2f(2.0, 5.0), &hull), false);
    assert_eq!(point_inside_convex_hull(vec2f(3.0, 3.0), &hull), false);
    assert_eq!(point_inside_convex_hull(vec2f(4.0, 3.0), &hull), false);

    // closest point to y
    let p = vec2f(7.0, 1.0);
    let seg = closest_point_on_line_segment(p, vec2f(5.0, 1.0), vec2f(7.0, 3.0));
    assert_eq!(closest_point_on_convex_hull(p, &hull), closest_point_on_line_segment(p, vec2f(5.0, 1.0), vec2f(7.0, 3.0)));
    
    let p = vec2f(7.0, 1.0);
    assert_eq!(point_convex_hull_distance(p, &hull), dist(p, seg));

}

#[test]
fn polygon() {
    //   0   1   2   3   4   5   6   7
    //0  -   -   -   x   -   -   -   -          
    //1  a   x   0       0   x       y
    //2          0   0   0   0
    //3  x       0   0       0       x
    //4              x
    //5              a
    //6  x           y        a      x
    //--------------------------------

    // x = edge, 0 = inside, a = outside

    let square = vec![
        vec2f(0.0, 0.0),
        vec2f(0.0, 1.0),
        vec2f(1.0, 1.0),
        vec2f(1.0, 0.0),
    ];
    assert_eq!(point_inside_polygon(vec2f(0.5, 0.5), &square), true);

    let poly = vec![
        vec2f(3.0, 0.0), // x
        vec2f(5.0, 1.0), // x
        vec2f(7.0, 3.0), // x
        vec2f(7.0, 6.0), // x
        vec2f(3.0, 4.0), // x
        vec2f(0.0, 6.0), // x
        vec2f(0.0, 3.0), // x
        vec2f(1.0, 1.0), // x
    ];

    // outside
    assert_eq!(point_inside_polygon(vec2f(0.0, 0.0), &poly), false);
    assert_eq!(point_inside_polygon(vec2f(3.0, 5.0), &poly), false);
    assert_eq!(point_inside_polygon(vec2f(5.0, 6.0), &poly), false);

    // inside
    assert_eq!(point_inside_polygon(vec2f(4.0, 2.0), &poly), true);
    assert_eq!(point_inside_polygon(vec2f(3.0, 2.0), &poly), true);
    assert_eq!(point_inside_polygon(vec2f(3.0, 3.0), &poly), true);
    assert_eq!(point_inside_polygon(vec2f(2.0, 1.0), &poly), true);
    assert_eq!(point_inside_polygon(vec2f(4.0, 1.0), &poly), true);

    // closest point to y
    let p = vec2f(7.0, 1.0);
    let seg = closest_point_on_line_segment(p, vec2f(5.0, 1.0), vec2f(7.0, 3.0));
    assert_eq!(closest_point_on_polygon(p, &poly), seg);
    assert_eq!(point_polygon_distance(p, &poly), dist(p, seg));

    let p = vec2f(2.5, 6.0);
    let seg = closest_point_on_line_segment(p, vec2f(3.0, 4.0), vec2f(0.0, 6.0));
    assert_eq!(closest_point_on_polygon(p, &poly), seg);
    assert_eq!(point_polygon_distance(p, &poly), dist(p, seg));

    let p = vec2f(3.0, 4.0);
    let seg = vec2f(3.0, 4.0);
    assert_eq!(closest_point_on_polygon(p, &poly), seg);
    assert_eq!(point_polygon_distance(p, &poly), dist(p, seg));
}

#[test]
fn ray_vs_aabb_test() {
    let emin = vec3f(-8.94, -8.37, -5.3);
    let emax = vec3f(7.18, -3.57, 13.9);
    let r1 = vec3f(-1.95, -6.86, 6.52);
    let rv = vec3f(-0.428178, 0.616866, 0.660409);
    let result = ray_vs_aabb(r1, rv, emin, emax);
    assert_eq!(approx(result.unwrap(), vec3f(-0.901882, -8.37, 4.90341), 0.0001), true);

    let emin = vec3f(2.87, -2.17, -11.75);
    let emax = vec3f(12.21, -1.41, -2.53);
    let r1 = vec3f(3.97, -10.0, -6.86);
    let rv = vec3f(-0.0352975, 0.873613, -0.48534);
    let result = ray_vs_aabb(r1, rv, emin, emax);
    assert_eq!(approx(result.unwrap() ,vec3f(3.65364, -2.17, -11.21), 0.0001), true);

    let emin = vec3f(1.49, -13.86, -0.16);
    let emax = vec3f(2.57, 2.28, 9.1);
    let r1 = vec3f(-1.68, -8.66, -0.72);
    let rv = vec3f(0.891587, -0.435426, 0.124407);
    let result = ray_vs_aabb(r1, rv, emin, emax);
    assert_eq!(approx(result.unwrap(), vec3f(2.33334, -10.62, -0.16), 0.0001), true);

    let emin = vec3f(3.04, -2.38, -9.95);
    let emax = vec3f(14.78, 9.32, -2.67);
    let r1 = vec3f(-4.4, 9.57, -9.08);
    let rv = vec3f(0.390941, 0.681875, 0.618233);
    let result = ray_vs_aabb(r1, rv, emin, emax);
    assert_eq!(result.is_none(), true);

    // 2D
    let emin = vec2f(-10.0, -10.0);
    let emax = vec2f(10.0, 10.0);
    let r1 = vec2f(-20.0, 5.0);
    let rv = vec2f(1.0, 0.0);
    let result = ray_vs_aabb(r1, rv, emin, emax);
    assert_eq!(approx(result.unwrap(), vec2f(-10.0, 5.0), 0.0001), true);
}

#[test]
fn ray_vs_triangle_test() {
    let t0 = vec3f(-5.0, 0.0, -5.0);
    let t1 = vec3f(0.0, 0.0, 0.0);
    let t2 = vec3f(5.0, 0.0, -5.0);
    let r0 = vec3f(0.0, 10.0, 0.0);
    let rv = -Vec3f::unit_y();
    let result = ray_vs_triangle(r0, rv, t0, t1, t2);
    assert_eq!(result.unwrap(), Vec3f::zero());

    let r0 = vec3f(-2.0, -10.0, -2.0);
    let rv = Vec3f::unit_y();
    let result = ray_vs_triangle(r0, rv, t0, t1, t2);
    assert_eq!(result.unwrap(), vec3f(-2.0, 0.0, -2.0));

    let r0 = vec3f(-4.0, 10.0, -4.5);
    let rv = -Vec3f::unit_y();
    let result = ray_vs_triangle(r0, rv, t0, t1, t2);
    assert_eq!(result.unwrap(), vec3f(-4.0, 0.0, -4.5));
}

#[test]
fn ray_vs_obb_test() {
    // 3x4 hit
    let mat = Mat34f::new(
        -4.29814, -0.134092, 4.26139, 4.39, 
        2.24637, -0.347541, 8.13622, -1.33, 
        0.0485004, 4.21357, 0.806702, -8.81
    );
    let r1 = vec3f(6.85, -0.81, -6.19);
    let rv = vec3f(-0.70182, 0.062384, -0.709618);
    let result = ray_vs_obb(r1, rv, mat);
    assert_eq!(approx(result.unwrap(), vec3f(8.62138, -0.967456, -4.39894), 0.0001), true);

    // 3x4 hit
    let mat = Mat34f::new(
        -1.62582, -3.23365, 1.34286, 7.29, 
        -0.884833, 5.89037, 0.796295, -0.45, 
        -0.613404, 0.0739455, -4.70789, 0.82, 
    );
    let r1 = vec3f(0.13, -5.59, -2.19);
    let rv = vec3f(0.783562, 0.178533, 0.595111);
    let result = ray_vs_obb(r1, rv, mat);
    assert_eq!(approx(result.unwrap(), vec3f(6.33623, -4.17592, 2.52359), 0.0001), true);
}

#[test]
fn ray_vs_sphere_test() {
    let sp = Vec3f::zero();
    let r = 100.0;

    // hit w/ intersection point
    let rp = Vec3f::new(0.0, 0.0, -1000.0);
    let rv = Vec3f::unit_z();
    let result = ray_vs_sphere(rp, rv, sp, r);
    assert_eq!(result.is_some(), true);
    if let Some(ip) = result {
        assert_eq!(ip, Vec3f::new(0.0, 0.0, -100.0));
    }

    let rp = Vec3f::new(0.0, 0.0, 1000.0);
    let rv = -Vec3f::unit_z();
    let result = ray_vs_sphere(rp, rv, sp, r);
    assert_eq!(result.is_some(), true);
    if let Some(ip) = result {
        assert_eq!(ip, Vec3f::new(0.0, 0.0, 100.0));
    }

    let rp = Vec3f::new(-500.0, 0.0, 0.0);
    let rv = Vec3f::unit_x();
    let result = ray_vs_sphere(rp, rv, sp, r);
    assert_eq!(result.is_some(), true);
    if let Some(ip) = result {
        assert_eq!(ip, Vec3f::new(-100.0, 0.0, 0.0));
    }

    let rp = Vec3f::new(500.0, 0.0, 0.0);
    let rv = -Vec3f::unit_x();
    let result = ray_vs_sphere(rp, rv, sp, r);
    assert_eq!(result.is_some(), true);
    if let Some(ip) = result {
        assert_eq!(ip, Vec3f::new(100.0, 0.0, 0.0));
    }

    // misses around axis
    //   +
    // - 0 +
    //   -
    let rp = Vec3f::new(101.0, 0.0, -1000.0);
    let rv = Vec3f::unit_z();
    let result = ray_vs_sphere(rp, rv, sp, r);
    assert_eq!(result.is_none(), true);

    let rp = Vec3f::new(-101.0, 0.0, -1000.0);
    let rv = Vec3f::unit_z();
    let result = ray_vs_sphere(rp, rv, sp, r);
    assert_eq!(result.is_none(), true);

    let rp = Vec3f::new(0.0, -101.0, -1000.0);
    let rv = Vec3f::unit_z();
    let result = ray_vs_sphere(rp, rv, sp, r);
    assert_eq!(result.is_none(), true);

    let rp = Vec3f::new(0.0, 101.0, -1000.0);
    let rv = Vec3f::unit_z();
    let result = ray_vs_sphere(rp, rv, sp, r);
    assert_eq!(result.is_none(), true);

    // hits, not testing intersection points
    let rp = Vec3f::new(10.0, 1000.0, -1000.0);
    let rv = normalize(Vec3f::unit_z() + (-Vec3f::unit_y()));
    let result = ray_vs_sphere(rp, rv, sp, r);
    assert_eq!(result.is_some(), true);
}

#[test]
fn sphere_vs_frustum_test() {
    let planes = Mat4f::new(
		0.85501, 1.45179e-08, 0.467094, 0.0, 
		0.39811, 1.52002, -0.728735, 0.0, 
		0.420904, -0.479617, -0.770459, 60.004, 
		0.420736, -0.479426, -0.770151, 60.0
    ).get_frustum_planes();

    // intersect
    let pos = vec3f(-4.21, -1.79, 9.67);
    let radius = 6.33;
    let result = sphere_vs_frustum(pos, radius, &planes);
    assert_eq!(result, true);

    let pos = vec3f(-8.76, -8.04, 5.3);
    let radius = 9.44;
    let result = sphere_vs_frustum(pos, radius, &planes);
    assert_eq!(result, true);

    // outside
    let pos = vec3f(4.85, 7.45, 2.28);
    let radius = 3.28;
    let result = sphere_vs_frustum(pos, radius, &planes);
    assert_eq!(result, false);

    let pos = vec3f(0.0100002, 1.53, -2.92);
    let radius = 9.09;
    let result = sphere_vs_frustum(pos, radius, &planes);
    assert_eq!(result, false);
}

#[test]
fn aabb_vs_frustum_test() {
    let planes = Mat4f::new(
		0.85501, 1.45179e-08, 0.467094, 0.0, 
		0.39811, 1.52002, -0.728735, 0.0, 
		0.420904, -0.479617, -0.770459, 60.004, 
		0.420736, -0.479426, -0.770151, 60.0
    ).get_frustum_planes();

    // fail / outside
    let epos = vec3f(-9.09, -8.06, -6.43);
    let eext = vec3f(4.85, 7.45, 2.28);
    let result = aabb_vs_frustum(epos, eext, &planes);
    assert_eq!(result, false);

    let epos = vec3f(-6.03, -7.06, 9.04);
    let eext = vec3f(1.37, 3.58, 1.77);
    let result = aabb_vs_frustum(epos, eext, &planes);
    assert_eq!(result, false);

	// intersect inside
    let epos = vec3f(-1.03, 8.71, 8.28);
    let eext = vec3f(5.62, 1.44, 5.01);
    let result = aabb_vs_frustum(epos, eext, &planes);
    assert_eq!(result, true);

    let epos = vec3f(-8.25, 6.35, -7.02);
    let eext = vec3f(6.09, 7.69, 7.45);
    let result = aabb_vs_frustum(epos, eext, &planes);
    assert_eq!(result, true);
}

#[test]
fn live_vs_line_test() {
    let l1 = vec3f(4.76, 0.0, 4.61);
    let l2 = vec3f(5.63, 0.0, -7.86);
    let s1 = vec3f(7.22, 0.0, 5.11);
    let s2 = vec3f(-2.09, 0.0, -1.29);
    let result = line_segment_vs_line_segment(l1, l2, s1, s2);
    assert_eq!(approx(result.unwrap(), vec3f(4.8393, 0.0, 3.47343), 0.0001), true);

    let l1 = vec3f(-0.93, 0.0, 9.43);
    let l2 = vec3f(-7.42, 0.0, -4.61);
    let s1 = vec3f(-7.19, 0.0, -0.35);
    let s2 = vec3f(4.57, 0.0, 6.05);
    let result = line_segment_vs_line_segment(l1, l2, s1, s2);
    assert_eq!(approx(result.unwrap(), vec3f(-4.86623, 0.0, 0.914634), 0.0001), true);

    let l1 = vec3f(8.79, 0.0, 1.98);
    let l2 = vec3f(-1.1, 0.0, -3.97);
    let s1 = vec3f(-1.73, 0.0, 5.85);
    let s2 = vec3f(-8.02, 0.0, -4.13);
    let result = line_segment_vs_line_segment(l1, l2, s1, s2);
    assert_eq!(result.is_none(), true);

    // infitine line
    let l1 = vec3f(4.76, 0.0, 4.61);
    let l2 = vec3f(5.63, 0.0, -7.86);
    let s1 = vec3f(7.22, 0.0, 5.11);
    let s2 = vec3f(-2.09, 0.0, -1.29);
    let result = line_vs_line(l1, l2, s1, s2);
    assert_eq!(approx(result.unwrap(), vec3f(4.8393, 0.0, 3.47343), 0.0001), true);

    let l1 = vec3f(8.79, 0.0, 1.98);
    let l2 = vec3f(-1.1, 0.0, -3.97);
    let s1 = vec3f(-1.73, 0.0, 5.85);
    let s2 = vec3f(-8.02, 0.0, -4.13);
    let result = line_vs_line(l1, l2, s1, s2);
    assert_eq!(result.is_none(), true);
}

#[test]
fn ray_vs_line_segment_test() {
    // intersect
    let r0 = vec3f(5.0, 0.0, -10.0);
    let rv = vec3f(0.0, 0.0, 1.0);
    let l1 = vec3f(-10.0, 0.0, 0.0);
    let l2 = vec3f(10.0, 0.0, -0.0);
    let result = ray_vs_line_segment(r0, rv, l1, l2);
    assert_eq!(approx(result.unwrap(), vec3f(5.0, 0.0, 0.0), 0.0001), true);

    // line is behind ray
    let r0 = vec3f(5.0, 0.0, 10.0);
    let rv = vec3f(0.0, 0.0, 1.0);
    let l1 = vec3f(-10.0, 0.0, 0.0);
    let l2 = vec3f(10.0, 0.0, 0.0);
    let result = ray_vs_line_segment(r0, rv, l1, l2);
    assert_eq!(result.is_none(), true);

    // distance on line
    let r0 = vec3f(0.0, 0.0, 0.0);
    let rv = vec3f(0.0, 0.0, 1.0);
    let ip = vec3f(0.0, 0.0, -1.0);
    let t = distance_on_line(ip, r0, rv);
    assert_eq!(t, -1.0);

    // distance on ray
    let r0 = vec3f(5.0, 0.0, 10.0);
    let rv = vec3f(0.0, 0.0, 1.0);
    let ip = vec3f(5.0, 0.0, 0.0);
    let t = distance_on_ray(ip, r0, rv);
    assert_eq!(t, -10.0);

    {
        let r0 = vec3f(0.000000, 0.000000, 0.000000);
        let rv = vec3f(-0.316227, 0.000000, -0.948683);
        let l00 = vec3f(9.870001, 0.000000, -4.970000);
        let l01 = vec3f(-6.730000, 0.000000, 7.290001);
        let i = ray_vs_line_segment(r0, rv, l00, l01);
        assert_eq!(i.is_none(), true);
    }
    {
        let r0 = vec3f(0.000000, 0.000000, 0.000000);
        let rv = vec3f(0.913302, 0.000000, 0.407283);
        let l00 = vec3f(-0.670000, 0.000000, 0.990000);
        let l01 = vec3f(-7.220000, 0.000000, 8.160000);
        let i = ray_vs_line_segment(r0, rv, l00, l01);
        assert_eq!(i.is_none(), true);
    }
    {
        let r0 = vec3f(0.000000, 0.000000, 0.000000);
        let rv = vec3f(-0.987241, 0.000000, 0.159232);
        let l00 = vec3f(-0.210000, 0.000000, 1.490000);
        let l01 = vec3f(-4.210000, 0.000000, -1.790000);
        let i = ray_vs_line_segment(r0, rv, l00, l01);
        assert_eq!(i.is_some(), true);
        if let Some(ip) = i {
            assert_eq!(approx(ip, vec3f(-1.693892, 0.000000, 0.273208), 0.001), true);
        }
    }
    {
        let r0 = vec3f(0.000000, 0.000000, 0.000000);
        let rv = vec3f(-0.975133, 0.000000, 0.221621);
        let l00 = vec3f(-9.090000, 0.000000, -8.059999);
        let l01 = vec3f(-6.430000, 0.000000, 0.010000);
        let i = ray_vs_line_segment(r0, rv, l00, l01);
        assert_eq!(i.is_none(), true);
    }
    {
        let r0 = vec3f(0.000000, 0.000000, 0.000000);
        let rv = vec3f(-0.643600, 0.000000, -0.765362);
        let l00 = vec3f(1.230000, 0.000000, 5.050000);
        let l01 = vec3f(-1.180000, 0.000000, -2.480000);
        let i = ray_vs_line_segment(r0, rv, l00, l01);
        assert_eq!(i.is_some(), true);
        if let Some(ip) = i {
            assert_eq!(approx(ip, vec3f(-0.623621, 0.000000, -0.741603), 0.001), true);
        }
    }
    {
        let r0 = vec3f(0.000000, 0.000000, 0.000000);
        let rv = vec3f(-0.919145, 0.000000, 0.393919);
        let l00 = vec3f(-6.050000, 0.000000, 9.680000);
        let l01 = vec3f(1.130000, 0.000000, -4.700000);
        let i = ray_vs_line_segment(r0, rv, l00, l01);
        assert_eq!(i.is_some(), true);
        if let Some(ip) = i {
            assert_eq!(approx(ip, vec3f(-1.547980, 0.000000, 0.663420), 0.001), true);
        }
    }
    {
        let r0 = vec3f(0.000000, 0.000000, 0.000000);
        let rv = vec3f(0.848689, 0.000000, 0.528893);
        let l00 = vec3f(8.209999, 0.000000, 6.950001);
        let l01 = vec3f(-2.880000, 0.000000, 6.709999);
        let i = ray_vs_line_segment(r0, rv, l00, l01);
        assert_eq!(i.is_none(), true);
    }
    {
        let r0 = vec3f(0.000000, 0.000000, 0.000000);
        let rv = vec3f(-0.126873, 0.000000, 0.991919);
        let l00 = vec3f(1.330000, 0.000000, 5.570000);
        let l01 = vec3f(-4.240000, 0.000000, 1.780000);
        let i = ray_vs_line_segment(r0, rv, l00, l01);
        assert_eq!(i.is_some(), true);
        if let Some(ip) = i {
            assert_eq!(approx(ip, vec3f(-0.548916, 0.000000, 4.291527), 0.001), true);
        }
    }
    {
        let r0 = vec3f(0.000000, 0.000000, 0.000000);
        let rv = vec3f(-0.842922, 0.000000, -0.538035);
        let l00 = vec3f(3.040000, 0.000000, 3.580000);
        let l01 = vec3f(2.000000, 0.000000, 2.540000);
        let i = ray_vs_line_segment(r0, rv, l00, l01);
        assert_eq!(i.is_none(), true);
    }
    {
        let r0 = vec3f(0.000000, 0.000000, 0.000000);
        let rv = vec3f(0.677476, 0.000000, -0.735545);
        let l00 = vec3f(6.500000, 0.000000, -0.290000);
        let l01 = vec3f(2.290000, 0.000000, -6.820000);
        let i = ray_vs_line_segment(r0, rv, l00, l01);
        assert_eq!(i.is_some(), true);
        if let Some(ip) = i {
            assert_eq!(approx(ip, vec3f(3.933561, 0.000000, -4.270723), 0.001), true);
        }
    }
    {
        let r0 = vec3f(0.000000, 0.000000, 0.000000);
        let rv = vec3f(-0.221621, 0.000000, 0.975133);
        let l00 = vec3f(2.050000, 0.000000, 0.800000);
        let l01 = vec3f(-5.820000, 0.000000, 2.740000);
        let i = ray_vs_line_segment(r0, rv, l00, l01);
        assert_eq!(i.is_some(), true);
        if let Some(ip) = i {
            assert_eq!(approx(ip, vec3f(-0.314275, 0.000000, 1.382807), 0.001), true);
        }
    }
}

#[test]
fn line_segment_between_line_segment_test() {
    let l1 = Vec2f::new(-10.0, 0.0);
    let l2 = Vec2f::new(10.0, 0.0);
    let l3 = Vec2f::new(-10.0, 10.0);
    let l4 = Vec2f::new(10.0, 11.0);
    let result = shortest_line_segment_between_line_segments(l1, l2, l3, l4);
    assert_eq!(result.is_some(), true);
    if let Some((p1, p2)) = result {
        assert_eq!(p1, Vec2f::new(-10.0, 0.0));
        assert_eq!(p2, Vec2f::new(-10.0, 10.0));
    }
}

#[test]
fn utils() {
    // TODO: quilez... these were to just test compilation
    let f : f32 = 0.0;
    let _ii = exp_impulse(f, f);
    let v = vec3f(-8.25, 6.35, -7.02);
    let _iv = exp_impulse(v, v);

    // lerp
    assert_eq!(lerp(10.0, 50.0, 0.5), 30.0);
    assert_eq!(lerp(0.0, 100.0, 0.75), 75.0);

    // map to range
    assert_eq!(map_to_range(5.0, 0.0, 10.0, 0.0, 1.0), 0.5);
    assert_eq!(map_to_range(0.9, 0.0, 2.0, 0.0, 100.0), 45.0);
}

#[test]
fn morton() {
    assert_eq!(morton_1(3), 1);
    assert_eq!(morton_1(9), 1);

    let odds = 0b10101010;
    assert_eq!(morton_1(odds), 0);

    let evens = 0b01010101;
    assert_eq!(morton_1(evens), 0b1111);

    let all = 0b11111111;
    assert_eq!(morton_1(all), 0b1111);
    
    //     0    1  |  2    3  |  4    5  |  6    7
    // --------------------------------------------
    // 0|  0    1  |  4    5  |  16   17 |  20   21
    // 1|  2    3  |  6    7  |  18   19 |  22   23
    // ------------|----------|----------|---------
    // 2|  8    9  |  12   13 |  24   25 |  28   29
    // 3|  10   11 |  14   15 |  26   27 |  30   31
    // --------------------------------------------
    // 4|  32   33 |  36   37 |  48   49 |  52   53
    // 5|  34   35 |  38   39 |  50   51 |  54   55
    // ------------|----------|----------|---------
    // 6|  40   41 |  44   45 |  56   57 |  60   61
    // 7|  42   43 |  46   47 |  58   59 |  62   63

    assert_eq!(morton_xy(0, 0), 0);
    assert_eq!(morton_xy(1, 1), 3);
    assert_eq!(morton_xy(5, 5), 51);
    assert_eq!(morton_xy(4, 7), 58);
    assert_eq!(morton_xy(7, 7), 63);

    assert_eq!(morton_to_xy(0), (0, 0));
    assert_eq!(morton_to_xy(62), (6, 7));
    assert_eq!(morton_to_xy(39), (3, 5));
    assert_eq!(morton_to_xy(26), (4, 3));
    assert_eq!(morton_to_xy(40), (0, 6));

    //     0    1  |  2    3 
    // -----------------------------
    //      4    5  |  12   13
    //      6    7  |  14   15
    //     \         \
    //   z  \          \
    //       |  0    1  |  8    9  |  
    //       |  2    3  |  10   11 | 

    assert_eq!(morton_xyz(0, 0, 0), 0);
    assert_eq!(morton_xyz(1, 0, 0), 1);
    assert_eq!(morton_xyz(0, 1, 0), 2);
    assert_eq!(morton_xyz(1, 1, 0), 3);
    assert_eq!(morton_xyz(0, 0, 1), 4);
    assert_eq!(morton_xyz(1, 0, 1), 5);
    assert_eq!(morton_xyz(0, 1, 1), 6);
    assert_eq!(morton_xyz(1, 1, 1), 7);
    assert_eq!(morton_xyz(2, 0, 0), 8);
    assert_eq!(morton_xyz(3, 0, 0), 9);
    assert_eq!(morton_xyz(2, 1, 0), 10); 
    assert_eq!(morton_xyz(3, 1, 0), 11);
    assert_eq!(morton_xyz(2, 0, 1), 12);
    assert_eq!(morton_xyz(3, 0, 1), 13);
    assert_eq!(morton_xyz(2, 1, 1), 14); 
    assert_eq!(morton_xyz(3, 1, 1), 15);
    assert_eq!(morton_xyz(0, 2, 0), 16);
    assert_eq!(morton_xyz(1, 2, 0), 17);
    assert_eq!(morton_xyz(0, 3, 0), 18);
    assert_eq!(morton_xyz(1, 3, 0), 19);
    assert_eq!(morton_xyz(4, 0, 0), 64);

    assert_eq!(morton_to_xyz(16), (0, 2, 0));
    assert_eq!(morton_to_xyz(8), (2, 0, 0));
    assert_eq!(morton_to_xyz(64), (4, 0, 0));
}

#[test]
fn swizzle() {
    // v2-v2
    let v2 = vec2f(1.0, 0.0);
    assert_eq!(v2.yx(), vec2f(0.0, 1.0));
    assert_eq!(v2.xx(), vec2f(1.0, 1.0));
    assert_eq!(v2.xy(), vec2f(1.0, 0.0));
    assert_eq!(v2.yy(), vec2f(0.0, 0.0));

    // v3-v2
    let v3 = vec3f(1.0, 2.0, 3.0);
    assert_eq!(v3.xy(), vec2f(1.0, 2.0));
    assert_eq!(v3.xz(), vec2f(1.0, 3.0));
    assert_eq!(v3.zz(), vec2f(3.0, 3.0));
    assert_eq!(v3.zx(), vec2f(3.0, 1.0));

    // v4-v2
    let v4 = vec4f(5.0, 4.0, 3.0, 2.0);
    assert_eq!(v4.xy(), vec2f(5.0, 4.0));
    assert_eq!(v4.xw(), vec2f(5.0, 2.0));
    assert_eq!(v4.ww(), vec2f(2.0, 2.0));
    assert_eq!(v4.yz(), vec2f(4.0, 3.0));

    // v3-v3
    let v3 = vec3f(7.0, 8.0, 9.0);
    assert_eq!(v3.zyx(), vec3f(9.0, 8.0, 7.0));
    assert_eq!(v3.xxx(), vec3f(7.0, 7.0, 7.0));
    assert_eq!(v3.yyx(), vec3f(8.0, 8.0, 7.0));
    assert_eq!(v3.zyz(), vec3f(9.0, 8.0, 9.0));

    // v4-v3
    let v4 = vec4f(7.0, 8.0, 9.0, 10.0);
    assert_eq!(v4.wyy(), vec3f(10.0, 8.0, 8.0));
    assert_eq!(v4.www(), vec3f(10.0, 10.0, 10.0));
    assert_eq!(v4.wxy(), vec3f(10.0, 7.0, 8.0));
    assert_eq!(v4.zwx(), vec3f(9.0, 10.0, 7.0));

    // v4-v4
    let v4 = vec4f(1.0, 2.0, 3.0, 4.0);
    assert_eq!(v4.wzyx(), vec4f(4.0, 3.0, 2.0, 1.0));
    assert_eq!(v4.xxxx(), vec4f(1.0, 1.0, 1.0, 1.0));
    assert_eq!(v4.zzyy(), vec4f(3.0, 3.0, 2.0, 2.0));
    assert_eq!(v4.xyxy(), vec4f(1.0, 2.0, 1.0, 2.0));

    // set v2-v2
    let v2 = vec2f(6.0, 4.0);
    let mut v2x = vec2f(7.0, 7.0);
    v2x.set_xy(v2);
    assert_eq!(v2x, vec2f(6.0, 4.0));
    v2x.set_yx(v2);
    assert_eq!(v2x, vec2f(4.0, 6.0));

    // set v3-v2
    let v2 = vec2f(8.0, 7.0);
    let mut v3x = vec3f(4.0, 4.0, 4.0);
    v3x.set_xz(v2);
    assert_eq!(v3x, vec3f(8.0, 4.0, 7.0));
    let mut v3x = vec3f(4.0, 4.0, 4.0);
    v3x.set_zy(v2);
    assert_eq!(v3x, vec3f(4.0, 7.0, 8.0));
    let mut v3x = vec3f(4.0, 4.0, 4.0);
    v3x.set_xz(v2);
    assert_eq!(v3x, vec3f(8.0, 4.0, 7.0));

    // set v4-v2
    let v2 = vec2f(9.0, 1.0);
    let mut v4x = vec4f(3.0, 3.0, 3.0, 3.0);
    v4x.set_xw(v2);
    assert_eq!(v4x, vec4f(9.0, 3.0, 3.0, 1.0));
    let mut v4x = vec4f(3.0, 3.0, 3.0, 3.0);
    v4x.set_wy(v2);
    assert_eq!(v4x, vec4f(3.0, 1.0, 3.0, 9.0));
    let mut v4x = vec4f(3.0, 3.0, 3.0, 3.0);
    v4x.set_yw(v2);
    assert_eq!(v4x, vec4f(3.0, 9.0, 3.0, 1.0));
    let mut v4x = vec4f(3.0, 3.0, 3.0, 3.0);
    v4x.set_zw(v2);
    assert_eq!(v4x, vec4f(3.0, 3.0, 9.0, 1.0));

    // set v3-v3
    let v3 = vec3f(8.0, 7.0, 5.0);
    let mut v3x = vec3f(4.0, 4.0, 4.0);
    v3x.set_xyz(v3);
    assert_eq!(v3x, vec3f(8.0, 7.0, 5.0));
    let mut v3x = vec3f(4.0, 4.0, 4.0);
    v3x.set_yxz(v3);
    assert_eq!(v3x, vec3f(7.0, 8.0, 5.0));
    let mut v3x = vec3f(4.0, 4.0, 4.0);
    v3x.set_zyx(v3);
    assert_eq!(v3x, vec3f(5.0, 7.0, 8.0));
    let mut v3x = vec3f(4.0, 4.0, 4.0);
    v3x.set_xzy(v3);
    assert_eq!(v3x, vec3f(8.0, 5.0, 7.0));

    // set v4-v3
    let v3 = vec3f(8.0, 7.0, 5.0);
    let mut v4x = vec3f(1.0, 1.0, 1.0);
    v4x.set_yxz(v3);
    assert_eq!(v4x, vec3f(7.0, 8.0, 5.0));
    let mut v4x = vec3f(1.0, 1.0, 1.0);
    v4x.set_zyx(v3);
    assert_eq!(v4x, vec3f(5.0, 7.0, 8.0));
    let mut v4x = vec3f(1.0, 1.0, 1.0);
    v4x.set_xzy(v3);
    assert_eq!(v4x, vec3f(8.0, 5.0, 7.0));
    let mut v4x = vec3f(1.0, 1.0, 1.0);
    v4x.set_zxy(v3);
    assert_eq!(v4x, vec3f(5.0, 8.0, 7.0));

    // set v4-v4
    let v4 = vec4f(8.0, 7.0, 5.0, 3.0);
    let mut v4x = vec4f(2.0, 2.0, 2.0, 2.0);
    v4x.set_xyzw(v4);
    assert_eq!(v4x, vec4f(8.0, 7.0, 5.0, 3.0));
    let mut v4x = vec4f(2.0, 2.0, 2.0, 2.0);
    v4x.set_wzyx(v4);
    assert_eq!(v4x, vec4f(3.0, 5.0, 7.0, 8.0));
    let mut v4x = vec4f(2.0, 2.0, 2.0, 2.0);
    v4x.set_wxyz(v4);
    assert_eq!(v4x, vec4f(3.0, 8.0, 7.0, 5.0));
    let mut v4x = vec4f(2.0, 2.0, 2.0, 2.0);
    v4x.set_ywxz(v4);
    assert_eq!(v4x, vec4f(7.0, 3.0, 8.0, 5.0));
    let mut v4x = vec4f(2.0, 2.0, 2.0, 2.0);
    v4x.set_xywz(v4);
    assert_eq!(v4x, vec4f(8.0, 7.0, 3.0, 5.0));
    let mut v4x = vec4f(2.0, 2.0, 2.0, 2.0);
    v4x.set_yxzw(v4);
    assert_eq!(v4x, vec4f(7.0, 8.0, 5.0, 3.0));
}

#[test]
fn cone_vs_plane_test() {
    {
        let x = vec3f(7.620001, -5.160000, -8.520000);
        let n = vec3f(0.447068, -0.894135, -0.025547);
        let cp = vec3f(-4.060000, 6.260000, 0.500000);
        let cv = vec3f(-0.104409, 0.977473, 0.183428);
        let h = 8.150000;
        let r = 8.660000;
        let c = cone_vs_plane(cp, cv, h, r, x, n);
        assert_eq!(c, Classification::Behind);
    }
    {
        let x = vec3f(-4.090000, -4.010000, -6.290000);
        let n = vec3f(0.447467, -0.642018, 0.622563);
        let cp = vec3f(-5.900000, -1.270000, 5.920000);
        let cv = vec3f(0.860584, -0.448540, 0.241263);
        let h = 1.280000;
        let r = 0.110000;
        let c = cone_vs_plane(cp, cv, h, r, x, n);
        assert_eq!(c, Classification::Infront);
    }
    {
        let x = vec3f(5.120000, -6.210000, -4.610000);
        let n = vec3f(-0.298879, -0.472422, -0.829149);
        let cp = vec3f(8.639999, -1.160000, 6.110001);
        let cv = vec3f(0.041942, 0.305601, -0.951235);
        let h = 0.090000;
        let r = 3.560000;
        let c = cone_vs_plane(cp, cv, h, r, x, n);
        assert_eq!(c, Classification::Behind);
    }
    {
        let x = vec3f(-5.590000, 3.990000, 3.970000);
        let n = vec3f(-0.052875, -0.994055, -0.095175);
        let cp = vec3f(-0.890000, 5.380000, 8.820000);
        let cv = vec3f(-0.416117, -0.413371, -0.809920);
        let h = 6.820000;
        let r = 4.530000;
        let c = cone_vs_plane(cp, cv, h, r, x, n);
        assert_eq!(c, Classification::Intersects);
    }
    {
        let x = vec3f(0.750000, 7.750000, -7.190000);
        let n = vec3f(0.855013, 0.337505, 0.393756);
        let cp = vec3f(-8.720000, 4.480000, -8.030000);
        let cv = vec3f(0.777299, -0.220305, -0.589298);
        let h = 6.610001;
        let r = 8.380000;
        let c = cone_vs_plane(cp, cv, h, r, x, n);
        assert_eq!(c, Classification::Intersects);
    }
    {
        let x = vec3f(-6.630000, -1.860000, 1.950000);
        let n = vec3f(-0.562526, -0.747356, -0.353588);
        let cp = vec3f(3.250000, -6.570000, 6.660000);
        let cv = vec3f(-0.836382, 0.543668, -0.069934);
        let h = 1.500000;
        let r = 9.850000;
        let c = cone_vs_plane(cp, cv, h, r, x, n);
        assert_eq!(c, Classification::Intersects);
    }
    {
        let x = vec3f(0.750000, -4.770000, 8.650000);
        let n = vec3f(-0.635334, -0.512599, 0.577576);
        let cp = vec3f(-0.270000, -8.430000, -4.440000);
        let cv = vec3f(-0.802155, -0.385470, 0.456026);
        let h = 3.530000;
        let r = 2.230000;
        let c = cone_vs_plane(cp, cv, h, r, x, n);
        assert_eq!(c, Classification::Behind);
    }
    {
        let x = vec3f(-0.480000, 3.760000, 3.620000);
        let n = vec3f(0.370472, -0.867684, -0.331475);
        let cp = vec3f(-7.060000, -6.910000, 6.820000);
        let cv = vec3f(0.775616, -0.610655, 0.159750);
        let h = 9.910000;
        let r = 3.960000;
        let c = cone_vs_plane(cp, cv, h, r, x, n);
        assert_eq!(c, Classification::Infront);
    }
    {
        let x = vec3f(6.440001, -2.650000, -2.350000);
        let n = vec3f(-0.397182, -0.917301, 0.028370);
        let cp = vec3f(4.040000, 8.459999, -0.450000);
        let cv = vec3f(-0.063015, 0.998000, -0.005086);
        let h = 8.250000;
        let r = 9.500000;
        let c = cone_vs_plane(cp, cv, h, r, x, n);
        assert_eq!(c, Classification::Behind);
    }
    {
        let x = vec3f(9.629999, -8.480000, 7.110001);
        let n = vec3f(0.688594, 0.713187, 0.131161);
        let cp = vec3f(2.770000, 2.510000, 2.900000);
        let cv = vec3f(-0.456845, -0.831829, -0.315203);
        let h = 2.200000;
        let r = 3.170000;
        let c = cone_vs_plane(cp, cv, h, r, x, n);
        assert_eq!(c, Classification::Intersects);
    }
    {
        let x = vec3f(-1.500000, -9.810000, 2.980000);
        let n = vec3f(-0.487893, 0.582048, 0.650524);
        let cp = vec3f(-4.990000, -9.540000, 4.880000);
        let cv = vec3f(-0.958712, 0.056250, -0.278762);
        let h = 5.190000;
        let r = 7.200001;
        let c = cone_vs_plane(cp, cv, h, r, x, n);
        assert_eq!(c, Classification::Intersects);
    }
    {
        let x = vec3f(7.110001, -6.530000, -1.560000);
        let n = vec3f(-0.236755, 0.947020, -0.217025);
        let cp = vec3f(-7.440000, -7.640000, 6.620001);
        let cv = vec3f(-0.343565, -0.516901, 0.784077);
        let h = 3.610000;
        let r = 6.389999;
        let c = cone_vs_plane(cp, cv, h, r, x, n);
        assert_eq!(c, Classification::Intersects);
    }
    {
        let x = vec3f(-5.130000, -7.340000, 1.000000);
        let n = vec3f(-0.856210, -0.081978, 0.510082);
        let cp = vec3f(-1.020000, 3.460000, -1.300000);
        let cv = vec3f(0.047138, -0.926287, 0.373858);
        let h = 0.010000;
        let r = 6.330000;
        let c = cone_vs_plane(cp, cv, h, r, x, n);
        assert_eq!(c, Classification::Intersects);
    }
}

#[test]
fn capsule_vs_plane_test() {
    {
        let x = vec3f(-5.600000, -8.350000, -5.080000);
        let n = vec3f(-0.554472, 0.831708, 0.028680);
        let cp0 = vec3f(-3.880000, -14.260001, 1.690000);
        let cp1 = vec3f(-3.880000, 0.320001, 1.690000);
        let cr = 6.730000;
        let c = capsule_vs_plane(cp0, cp1, cr, x, n);
        assert_eq!(c, Classification::Intersects);
    }
    {
        let x = vec3f(7.090000, 1.570000, 5.600000);
        let n = vec3f(0.831786, -0.025206, -0.554524);
        let cp0 = vec3f(8.260000, -3.040000, 2.670000);
        let cp1 = vec3f(8.260000, 13.280000, 2.670000);
        let cr = 8.160000;
        let c = capsule_vs_plane(cp0, cp1, cr, x, n);
        assert_eq!(c, Classification::Intersects);
    }
    {
        let x = vec3f(-7.020000, -0.190000, -3.650000);
        let n = vec3f(-0.683748, -0.275071, -0.675888);
        let cp0 = vec3f(6.690001, -13.490000, -9.060000);
        let cp1 = vec3f(6.690001, -4.210001, -9.060000);
        let cr = 0.630000;
        let c = capsule_vs_plane(cp0, cp1, cr, x, n);
        assert_eq!(c, Classification::Behind);
    }
    {
        let x = vec3f(6.290001, -4.990000, -4.830000);
        let n = vec3f(0.702914, 0.036995, -0.710313);
        let cp0 = vec3f(1.230000, -1.970000, -1.180000);
        let cp1 = vec3f(1.230000, 12.070000, -1.180000);
        let cr = 4.510000;
        let c = capsule_vs_plane(cp0, cp1, cr, x, n);
        assert_eq!(c, Classification::Behind);
    }
    {
        let x = vec3f(-2.480000, 5.660000, -2.840000);
        let n = vec3f(0.437602, -0.733279, 0.520391);
        let cp0 = vec3f(8.280001, -3.640000, 3.580000);
        let cp1 = vec3f(8.280001, 6.380000, 3.580000);
        let cr = 5.010000;
        let c = capsule_vs_plane(cp0, cp1, cr, x, n);
        assert_eq!(c, Classification::Infront);
    }
    {
        let x = vec3f(1.770000, -6.030000, -7.060000);
        let n = vec3f(0.035004, -0.796348, -0.603825);
        let cp0 = vec3f(-7.020000, -16.830000, -6.010000);
        let cp1 = vec3f(-7.020000, -0.330000, -6.010000);
        let cr = 7.450001;
        let c = capsule_vs_plane(cp0, cp1, cr, x, n);
        assert_eq!(c, Classification::Intersects);
    }
    {
        let x = vec3f(-7.800000, 8.440001, -6.340000);
        let n = vec3f(0.273377, -0.594997, 0.755807);
        let cp0 = vec3f(5.900000, -8.130000, 1.590000);
        let cp1 = vec3f(5.900000, -0.870000, 1.590000);
        let cr = 3.630000;
        let c = capsule_vs_plane(cp0, cp1, cr, x, n);
        assert_eq!(c, Classification::Infront);
    }
    {
        let x = vec3f(-3.770000, -0.530000, 3.850000);
        let n = vec3f(-0.865617, -0.292015, 0.406736);
        let cp0 = vec3f(-1.150000, -9.920000, -5.800000);
        let cp1 = vec3f(-1.150000, 2.380000, -5.800000);
        let cr = 2.470000;
        let c = capsule_vs_plane(cp0, cp1, cr, x, n);
        assert_eq!(c, Classification::Behind);
    }
    {
        let x = vec3f(-8.559999, 9.680000, 7.350000);
        let n = vec3f(0.179207, -0.896038, 0.406204);
        let cp0 = vec3f(-8.480000, -7.450001, 2.950000);
        let cp1 = vec3f(-8.480000, 12.330000, 2.950000);
        let cr = 9.580000;
        let c = capsule_vs_plane(cp0, cp1, cr, x, n);
        assert_eq!(c, Classification::Intersects);
    }
}

#[test]
fn ray_vs_capsule_test() {
    {
        let r0 = vec3f(5.768664, -15.179691, 6.019536);
        let rv = vec3f(0.076287, 0.553084, -0.829626);
        let cp0 = vec3f(-1.600000, -10.610001, -6.970000);
        let cp1 = vec3f(-1.600000, 2.850000, -6.970000);
        let cr = 4.970000;
        let i = ray_vs_capsule(r0, rv, cp0, cp1, cr);
        assert_eq!(i.is_some(), false);
    }
    {
        let r0 = vec3f(16.864887, 15.436190, 1.317085);
        let rv = vec3f(-0.876123, -0.481867, 0.014602);
        let cp0 = vec3f(0.970000, 0.100000, 5.120000);
        let cp1 = vec3f(0.970000, 16.420000, 5.120000);
        let cr = 7.220000;
        let i = ray_vs_capsule(r0, rv, cp0, cp1, cr);
        assert_eq!(i.is_some(), true);
        assert_eq!(approx(i.unwrap(), vec3f(7.204176, 10.122798, 1.478097), 0.001), true);
    }
    {
        let r0 = vec3f(18.455387, 8.953480, -20.680779);
        let rv = vec3f(-0.635396, -0.385294, 0.669194);
        let cp0 = vec3f(6.680000, 1.980000, -8.760000);
        let cp1 = vec3f(6.680000, 7.820000, -8.760000);
        let cr = 1.530000;
        let i = ray_vs_capsule(r0, rv, cp0, cp1, cr);
        assert_eq!(i.is_some(), true);
        assert_eq!(approx(i.unwrap(), vec3f(7.948673, 2.582387, -9.615197), 0.001), true);
    }
    {
        let r0 = vec3f(-0.230000, -15.680000, -7.720000);
        let rv = vec3f(0.000000, 1.000000, 0.000000);
        let cp0 = vec3f(-0.230000, -5.680000, -7.720000);
        let cp1 = vec3f(-0.230000, 13.840000, -7.720000);
        let cr = 9.760000;
        let i = ray_vs_capsule(r0, rv, cp0, cp1, cr);
        assert_eq!(i.is_some(), true);
        assert_eq!(approx(i.unwrap(), vec3f(-0.230000, -15.440001, -7.720000), 0.001), true);
    }
    {
        let r0 = vec3f(-10.537001, -20.226603, -5.503001);
        let rv = vec3f(0.569672, 0.762483, 0.306746);
        let cp0 = vec3f(4.250000, -1.449999, -8.850000);
        let cp1 = vec3f(4.250000, 14.830000, -8.850000);
        let cr = 8.139999;
        let i = ray_vs_capsule(r0, rv, cp0, cp1, cr);
        assert_eq!(i.is_some(), false);
    }
    {
        let r0 = vec3f(2.441557, 2.528962, -23.296383);
        let rv = vec3f(0.065728, -0.043819, 0.996875);
        let cp0 = vec3f(6.350000, -14.710000, -8.580000);
        let cp1 = vec3f(6.350000, 0.670000, -8.580000);
        let cr = 7.690000;
        let i = ray_vs_capsule(r0, rv, cp0, cp1, cr);
        assert_eq!(i.is_some(), true);
        assert_eq!(approx(i.unwrap(), vec3f(2.967510, 2.178326, -15.319421), 0.001), true);
    }
    {
        let r0 = vec3f(-11.932406, 18.119431, 8.037656);
        let rv = vec3f(0.341930, -0.487251, -0.803536);
        let cp0 = vec3f(1.130000, -14.620000, 0.060000);
        let cp1 = vec3f(1.130000, 5.220000, 0.060000);
        let cr = 9.920000;
        let i = ray_vs_capsule(r0, rv, cp0, cp1, cr);
        assert_eq!(i.is_some(), false);
    }
    {
        let r0 = vec3f(-4.075598, -0.434928, 17.405834);
        let rv = vec3f(0.213365, 0.568974, -0.794193);
        let cp0 = vec3f(6.709999, 0.800001, -4.100000);
        let cp1 = vec3f(6.709999, 17.219999, -4.100000);
        let cr = 8.209999;
        let i = ray_vs_capsule(r0, rv, cp0, cp1, cr);
        assert_eq!(i.is_some(), true);
        assert_eq!(approx(i.unwrap(), vec3f(0.317913, 11.281100, 1.052214), 0.001), true);
    }
    {
        let r0 = vec3f(0.077069, 21.519623, 3.644967);
        let rv = vec3f(0.179725, -0.980319, -0.081693);
        let cp0 = vec3f(8.129999, -11.070001, 2.380000);
        let cp1 = vec3f(8.129999, 6.470000, 2.380000);
        let cr = 8.740000;
        let i = ray_vs_capsule(r0, rv, cp0, cp1, cr);
        assert_eq!(i.is_some(), true);
        assert_eq!(approx(i.unwrap(), vec3f(1.745788, 12.417515, 2.886458), 0.001), true);
    }
    {
        let r0 = vec3f(11.845491, -2.182019, -3.787370);
        let rv = vec3f(-0.629629, -0.324354, 0.705948);
        let cp0 = vec3f(8.440001, 1.960000, -4.500000);
        let cp1 = vec3f(8.440001, 9.839999, -4.500000);
        let cr = 3.940000;
        let i = ray_vs_capsule(r0, rv, cp0, cp1, cr);
        assert_eq!(i.is_some(), false);
    }
    {
        let r0 = vec3f(13.651896, -15.548086, -4.503330);
        let rv = vec3f(-0.696394, 0.680008, 0.229400);
        let cp0 = vec3f(4.960000, -6.540000, -3.770000);
        let cp1 = vec3f(4.960000, 4.240001, -3.770000);
        let cr = 5.390000;
        let i = ray_vs_capsule(r0, rv, cp0, cp1, cr);
        assert_eq!(i.is_some(), true);
        assert_eq!(approx(i.unwrap(), vec3f(8.483912, -10.501701, -2.800935), 0.001), true);
    }
    {
        let r0 = vec3f(1.407182, -5.471208, -5.763961);
        let rv = vec3f(-0.416107, -0.178331, 0.891657);
        let cp0 = vec3f(-3.210000, -18.059999, 2.440000);
        let cp1 = vec3f(-3.210000, 1.100000, 2.440000);
        let cr = 5.340000;
        let i = ray_vs_capsule(r0, rv, cp0, cp1, cr);
        assert_eq!(i.is_some(), true);
        assert_eq!(approx(i.unwrap(), vec3f(-0.324479, -6.213347, -2.053260), 0.001), true);
    }
}

#[test]
fn ray_vs_cylinder_test() {
    {
        let r0 = vec3f(-15.203737, 6.055606, -4.583255);
        let rv = vec3f(0.554472, -0.831708, -0.028680);
        let cy0 = vec3f(-3.880000, -14.260001, 1.690000);
        let cy1 = vec3f(-3.880000, 0.320001, 1.690000);
        let cyr = 6.730000;
        let i = ray_vs_cylinder(r0, rv, cy0, cy1, cyr);
        assert_eq!(i.is_some(), false);
    }
    {
        let r0 = vec3f(21.496956, 1.133426, -4.004637);
        let rv = vec3f(-0.831786, 0.025206, 0.554524);
        let cy0 = vec3f(8.260000, -3.040000, 2.670000);
        let cy1 = vec3f(8.260000, 13.280000, 2.670000);
        let cyr = 8.160000;
        let i = ray_vs_cylinder(r0, rv, cy0, cy1, cyr);
        assert_eq!(i.is_some(), true);
        assert_eq!(approx(i.unwrap(), vec3f(15.876670, 1.303737, -0.257780), 0.001), true);
    }
    {
        let r0 = vec3f(15.056831, 17.546116, -11.426116);
        let rv = vec3f(-0.401653, -0.647563, 0.647563);
        let cy0 = vec3f(3.360000, -4.820000, 7.450001);
        let cy1 = vec3f(3.360000, 14.520000, 7.450001);
        let cyr = 9.670000;
        let i = ray_vs_cylinder(r0, rv, cy0, cy1, cyr);
        assert_eq!(i.is_some(), true);
        assert_eq!(approx(i.unwrap(), vec3f(8.448961, 6.892612, -0.772611), 0.001), true);
    }
    {
        let r0 = vec3f(10.119667, -22.706263, -0.770485);
        let rv = vec3f(-0.452623, 0.786135, -0.420860);
        let cy0 = vec3f(4.900000, -18.200001, -8.040000);
        let cy1 = vec3f(4.900000, 0.680000, -8.040000);
        let cyr = 2.920000;
        let i = ray_vs_cylinder(r0, rv, cy0, cy1, cyr);
        assert_eq!(i.is_some(), true);
        assert_eq!(approx(i.unwrap(), vec3f(5.396209, -14.502363, -5.162472), 0.001), true);
    }
    {
        let r0 = vec3f(-0.795774, 7.815088, -6.405851);
        let rv = vec3f(0.351940, -0.507207, 0.786689);
        let cy0 = vec3f(4.080000, -15.730000, -0.670000);
        let cy1 = vec3f(4.080000, 0.290000, -0.670000);
        let cyr = 8.010000;
        let i = ray_vs_cylinder(r0, rv, cy0, cy1, cyr);
        assert_eq!(i.is_some(), true);
        assert_eq!(approx(i.unwrap(), vec3f(4.425715, 0.290000, 5.265714), 0.001), true);
    }
    {
        let r0 = vec3f(6.690001, -23.490000, -9.060000);
        let rv = vec3f(0.000000, 1.000000, 0.000000);
        let cy0 = vec3f(6.690001, -13.490000, -9.060000);
        let cy1 = vec3f(6.690001, -4.210001, -9.060000);
        let cyr = 0.630000;
        let i = ray_vs_cylinder(r0, rv, cy0, cy1, cyr);
        assert_eq!(i.is_some(), true);
        assert_eq!(approx(i.unwrap(), vec3f(6.690001, -13.490000, -9.060000), 0.001), true);
    }
    {
        let r0 = vec3f(18.464821, -4.349220, -17.132975);
        let rv = vec3f(-0.702914, -0.036995, 0.710313);
        let cy0 = vec3f(1.230000, -1.970000, -1.180000);
        let cy1 = vec3f(1.230000, 12.070000, -1.180000);
        let cyr = 4.510000;
        let i = ray_vs_cylinder(r0, rv, cy0, cy1, cyr);
        assert_eq!(i.is_some(), false);
    }
    {
        let r0 = vec3f(5.099485, -7.040759, 6.173443);
        let rv = vec3f(-0.437602, 0.733279, -0.520391);
        let cy0 = vec3f(8.280001, -3.640000, 3.580000);
        let cy1 = vec3f(8.280001, 6.380000, 3.580000);
        let cyr = 5.010000;
        let i = ray_vs_cylinder(r0, rv, cy0, cy1, cyr);
        assert_eq!(i.is_some(), false);
    }
    {
        let r0 = vec3f(2.376292, -19.823158, -17.518549);
        let rv = vec3f(-0.035004, 0.796348, 0.603825);
        let cy0 = vec3f(-7.020000, -16.830000, -6.010000);
        let cy1 = vec3f(-7.020000, -0.330000, -6.010000);
        let cyr = 7.450001;
        let i = ray_vs_cylinder(r0, rv, cy0, cy1, cyr);
        assert_eq!(i.is_some(), false);
    }
    {
        let r0 = vec3f(2.137793, 13.814747, -21.165298);
        let rv = vec3f(0.420438, -0.586862, 0.691972);
        let cy0 = vec3f(9.010000, -11.050001, -1.690000);
        let cy1 = vec3f(9.010000, 2.850001, -1.690000);
        let cyr = 6.950001;
        let i = ray_vs_cylinder(r0, rv, cy0, cy1, cyr);
        assert_eq!(i.is_some(), true);
        assert_eq!(approx(i.unwrap(), vec3f(9.993134, 2.850000, -8.236716), 0.001), true);
    }
    {
        let r0 = vec3f(-2.701339, -15.851871, 9.038837);
        let rv = vec3f(0.010470, 0.952736, -0.303619);
        let cy0 = vec3f(-3.930000, -7.100000, 3.220000);
        let cy1 = vec3f(-3.930000, 2.560000, 3.220000);
        let cyr = 4.470000;
        let i = ray_vs_cylinder(r0, rv, cy0, cy1, cyr);
        assert_eq!(i.is_some(), true);
        assert_eq!(approx(i.unwrap(), vec3f(-2.605165, -7.100000, 6.249780), 0.001), true);
    }
}

#[test]
fn point_sphere_distance_test() {
    {
        let sp = vec3f(-1.600000, -3.880000, -6.970000);
        let sr = 4.970000;
        let p = vec3f(-5.080000, 0.420000, 9.870001);
        let dd = point_sphere_distance(p, sp, sr);
        assert_eq!(approx(dd, 12.755294, 0.001), true);
    }
    {
        let sp = vec3f(-7.220000, 8.160000, 3.350000);
        let sr = 5.600000;
        let p = vec3f(1.690000, 7.090000, 1.570000);
        let dd = point_sphere_distance(p, sp, sr);
        assert_eq!(approx(dd, 3.548846, 0.001), true);
    }
    {
        let sp = vec3f(-0.210000, 1.490000, -4.210000);
        let sr = 2.670000;
        let p = vec3f(0.970000, 8.260000, 5.120000);
        let dd = point_sphere_distance(p, sp, sr);
        assert_eq!(approx(dd, 8.917675, 0.001), true);
    }
    {
        let sp = vec3f(7.450001, 2.280000, -9.090000);
        let sr = 3.930000;
        let p = vec3f(-1.790000, 9.670000, -3.280000);
        let dd = point_sphere_distance(p, sp, sr);
        assert_eq!(approx(dd, 9.251267, 0.001), true);
    }
    {
        let sp = vec3f(6.680000, 4.900000, -8.760000);
        let sr = 1.530000;
        let p = vec3f(-8.059999, -6.430000, 0.010000);
        let dd = point_sphere_distance(p, sp, sr);
        assert_eq!(approx(dd, 19.026007, 0.001), true);
    }
    {
        let sp = vec3f(-9.760000, 8.010000, -1.470000);
        let sr = 7.219999;
        let p = vec3f(-8.040000, 5.300000, -0.970000);
        let dd = point_sphere_distance(p, sp, sr);
        assert_eq!(approx(dd, 3.971538, 0.001), true);
    }
    {
        let sp = vec3f(-6.010000, 9.680000, -5.880000);
        let sr = 6.350000;
        let p = vec3f(-7.690000, 7.450001, -8.250000);
        let dd = point_sphere_distance(p, sp, sr);
        assert_eq!(approx(dd, 2.687733, 0.001), true);
    }
    {
        let sp = vec3f(3.190000, 4.540000, 2.230000);
        let sr = 9.719999;
        let p = vec3f(-3.210000, 3.350000, 0.060000);
        let dd = point_sphere_distance(p, sp, sr);
        assert_eq!(approx(dd, 2.858149, 0.001), true);
    }
    {
        let sp = vec3f(3.170000, 5.250000, -8.000000);
        let sr = 7.400000;
        let p = vec3f(9.290001, 2.230000, -9.460000);
        let dd = point_sphere_distance(p, sp, sr);
        assert_eq!(approx(dd, 0.421002, 0.001), true);
    }
    {
        let sp = vec3f(3.220000, -6.950000, 1.760000);
        let sr = 5.530000;
        let p = vec3f(1.290000, -4.470000, 4.830000);
        let dd = point_sphere_distance(p, sp, sr);
        assert_eq!(approx(dd, 1.136801, 0.001), true);
    }
}

#[test]
fn sphere_vs_capsule_test() {
    {
        let sp = vec3f(-5.080000, 0.420000, 9.870001);
        let sr = 7.090000;
        let cp0 = vec3f(-1.600000, -10.610001, -6.970000);
        let cp1 = vec3f(-1.600000, 2.850000, -6.970000);
        let cr = 4.970000;
        let overlap = sphere_vs_capsule(sp, sr, cp0, cp1, cr);
        assert_eq!(overlap, false);
    }
    {
        let sp = vec3f(5.600000, -0.670000, 0.990000);
        let sr = 1.690000;
        let cp0 = vec3f(0.970000, 0.100000, 5.120000);
        let cp1 = vec3f(0.970000, 16.420000, 5.120000);
        let cr = 7.220000;
        let overlap = sphere_vs_capsule(sp, sr, cp0, cp1, cr);
        assert_eq!(overlap, true);
    }
    {
        let sp = vec3f(-0.210000, 1.490000, -4.210000);
        let sr = 2.670000;
        let cp0 = vec3f(3.930000, -6.310000, 4.850000);
        let cp1 = vec3f(3.930000, 13.030000, 4.850000);
        let cr = 1.790000;
        let overlap = sphere_vs_capsule(sp, sr, cp0, cp1, cr);
        assert_eq!(overlap, false);
    }
    {
        let sp = vec3f(7.219999, -3.340000, -4.510000);
        let sr = 8.040000;
        let cp0 = vec3f(-0.230000, -5.680000, -7.720000);
        let cp1 = vec3f(-0.230000, 13.840000, -7.720000);
        let cr = 9.760000;
        let overlap = sphere_vs_capsule(sp, sr, cp0, cp1, cr);
        assert_eq!(overlap, true);
    }
    {
        let sp = vec3f(-3.650000, -9.870000, 8.650000);
        let sr = 0.670000;
        let cp0 = vec3f(4.250000, -1.449999, -8.850000);
        let cp1 = vec3f(4.250000, 14.830000, -8.850000);
        let cr = 8.139999;
        let overlap = sphere_vs_capsule(sp, sr, cp0, cp1, cr);
        assert_eq!(overlap, false);
    }
    {
        let sp = vec3f(-4.830000, -8.050000, -8.950000);
        let sr = 9.060000;
        let cp0 = vec3f(-8.120000, -4.730000, 5.050000);
        let cp1 = vec3f(-8.120000, 7.190000, 5.050000);
        let cr = 5.960000;
        let overlap = sphere_vs_capsule(sp, sr, cp0, cp1, cr);
        assert_eq!(overlap, true);
    }
    {
        let sp = vec3f(-7.400000, 5.570000, 5.940000);
        let sr = 6.010000;
        let cp0 = vec3f(1.130000, -14.620000, 0.060000);
        let cp1 = vec3f(1.130000, 5.220000, 0.060000);
        let cr = 9.920000;
        let overlap = sphere_vs_capsule(sp, sr, cp0, cp1, cr);
        assert_eq!(overlap, true);
    }
    {
        let sp = vec3f(-6.340000, -4.660000, 2.260000);
        let sr = 0.940000;
        let cp0 = vec3f(8.440001, 1.960000, -4.500000);
        let cp1 = vec3f(8.440001, 9.839999, -4.500000);
        let cr = 3.940000;
        let overlap = sphere_vs_capsule(sp, sr, cp0, cp1, cr);
        assert_eq!(overlap, false);
    }
    {
        let sp = vec3f(3.850000, 2.170000, 2.720000);
        let sr = 1.590000;
        let cp0 = vec3f(4.960000, -6.540000, -3.770000);
        let cp1 = vec3f(4.960000, 4.240001, -3.770000);
        let cr = 5.390000;
        let overlap = sphere_vs_capsule(sp, sr, cp0, cp1, cr);
        assert_eq!(overlap, true);
    }
    {
        let sp = vec3f(7.350000, 9.150000, 6.250000);
        let sr = 5.800000;
        let cp0 = vec3f(-3.210000, -18.059999, 2.440000);
        let cp1 = vec3f(-3.210000, 1.100000, 2.440000);
        let cr = 5.340000;
        let overlap = sphere_vs_capsule(sp, sr, cp0, cp1, cr);
        assert_eq!(overlap, false);
    }
    {
        let sp = vec3f(6.920000, 8.150000, -0.090000);
        let sr = 2.950000;
        let cp0 = vec3f(5.530000, -14.200001, 8.250000);
        let cp1 = vec3f(5.530000, 5.140000, 8.250000);
        let cr = 9.670000;
        let overlap = sphere_vs_capsule(sp, sr, cp0, cp1, cr);
        assert_eq!(overlap, true);
    }
    {
        let sp = vec3f(2.390000, 0.540000, 3.050000);
        let sr = 3.730000;
        let cp0 = vec3f(1.660000, -13.910000, 3.040000);
        let cp1 = vec3f(1.660000, 1.030000, 3.040000);
        let cr = 0.710000;
        let overlap = sphere_vs_capsule(sp, sr, cp0, cp1, cr);
        assert_eq!(overlap, true);
    }
}

#[test]
fn caspule_vs_capsule_test() {
    {
        let cp0 = vec3f(-8.959845, -8.782539, 4.907854);
        let cp1 = vec3f(-0.980155, -4.677461, 9.672148);
        let cr = 5.080000;
        let cp2 = vec3f(12.017217, 2.311968, 0.548064);
        let cp3 = vec3f(-0.817216, -3.651968, 1.431936);
        let cr1 = 1.690000;
        let overlap = capsule_vs_capsule(cp0, cp1, cr, cp2, cp3, cr1);
        assert_eq!(overlap, false);
    }
    {
        let cp0 = vec3f(-5.426985, 1.042825, 10.367354);
        let cp1 = vec3f(8.486984, -6.882825, 8.512647);
        let cr = 8.059999;
        let cp2 = vec3f(0.023970, 0.023601, -5.752966);
        let cp3 = vec3f(14.416029, -6.703602, -3.267035);
        let cr1 = 8.040000;
        let overlap = capsule_vs_capsule(cp0, cp1, cr, cp2, cp3, cr1);
        assert_eq!(overlap, true);
    }
    {
        let cp0 = vec3f(-1.964608, -4.765986, -7.579928);
        let cp1 = vec3f(9.124608, 8.305986, -4.480071);
        let cr = 8.709999;
        let cp2 = vec3f(8.991868, -13.625567, -5.660562);
        let cp3 = vec3f(3.708133, -0.414433, -11.499437);
        let cr1 = 7.690000;
        let overlap = capsule_vs_capsule(cp0, cp1, cr, cp2, cp3, cr1);
        assert_eq!(overlap, true);
    }
    {
        let cp0 = vec3f(-3.830595, -11.371892, 0.618306);
        let cp1 = vec3f(0.450596, 6.131892, 0.521693);
        let cr = 6.709999;
        let cp2 = vec3f(7.213480, -11.195667, 1.404247);
        let cp3 = vec3f(12.226519, -6.844334, 0.495753);
        let cr1 = 3.210000;
        let overlap = capsule_vs_capsule(cp0, cp1, cr, cp2, cp3, cr1);
        assert_eq!(overlap, false);
    }
    {
        let cp0 = vec3f(-14.688107, -6.652096, 8.329040);
        let cp1 = vec3f(-2.791893, -10.887905, 1.790961);
        let cr = 7.110000;
        let cp2 = vec3f(-0.223576, -8.073273, -1.172043);
        let cp3 = vec3f(-12.456425, -1.246727, 5.692044);
        let cr1 = 0.940000;
        let overlap = capsule_vs_capsule(cp0, cp1, cr, cp2, cp3, cr1);
        assert_eq!(overlap, true);
    }
    {
        let cp0 = vec3f(8.766312, -7.762488, 1.417873);
        let cp1 = vec3f(-5.586311, 0.222488, -2.477873);
        let cr = 8.440001;
        let cp2 = vec3f(1.619980, 2.987260, -4.652875);
        let cp3 = vec3f(8.300020, -5.287259, -2.887125);
        let cr1 = 5.390000;
        let overlap = capsule_vs_capsule(cp0, cp1, cr, cp2, cp3, cr1);
        assert_eq!(overlap, true);
    }
    {
        let cp0 = vec3f(5.340000, -18.730000, -9.890000);
        let cp1 = vec3f(5.340000, -0.430000, -9.890000);
        let cr = 7.350000;
        let cp2 = vec3f(6.920000, 5.200000, -0.090000);
        let cp3 = vec3f(6.920000, 11.099999, -0.090000);
        let cr1 = 2.950000;
        let overlap = capsule_vs_capsule(cp0, cp1, cr, cp2, cp3, cr1);
        assert_eq!(overlap, false);
    }
    {
        let cp0 = vec3f(-4.400000, -16.049999, -0.160000);
        let cp1 = vec3f(-4.400000, -2.070001, -0.160000);
        let cr = 4.440000;
        let cp2 = vec3f(6.799999, -5.300000, -1.450000);
        let cp3 = vec3f(6.799999, 14.400001, -1.450000);
        let cr1 = 9.850000;
        let overlap = capsule_vs_capsule(cp0, cp1, cr, cp2, cp3, cr1);
        assert_eq!(overlap, true);
    }
    {
        let cp0 = vec3f(-6.690000, 5.200001, 6.379999);
        let cp1 = vec3f(-6.690000, 11.680000, 6.379999);
        let cr = 3.240000;
        let cp2 = vec3f(-4.690000, -11.889999, 4.190000);
        let cp3 = vec3f(-4.690000, -7.830000, 4.190000);
        let cr1 = 1.510000;
        let overlap = capsule_vs_capsule(cp0, cp1, cr, cp2, cp3, cr1);
        assert_eq!(overlap, false);
    }
    {
        let cp0 = vec3f(-7.400000, -11.920001, 5.450000);
        let cp1 = vec3f(-7.400000, 6.660001, 5.450000);
        let cr = 9.290001;
        let cp2 = vec3f(-3.520000, -0.399999, 5.500000);
        let cp3 = vec3f(-3.520000, 14.480001, 5.500000);
        let cr1 = 7.440000;
        let overlap = capsule_vs_capsule(cp0, cp1, cr, cp2, cp3, cr1);
        assert_eq!(overlap, true);
    }
}

#[test]
fn shortest_line_segment_between_line_segments_test() {
    {
        let l00 = vec3f(-4.970000, 0.000000, -6.730000);
        let l01 = vec3f(7.290001, 0.000000, -1.600000);
        let l10 = vec3f(1.690000, 0.000000, 7.090000);
        let l11 = vec3f(1.570000, 0.000000, 5.600000);
        
        let has = shortest_line_segment_between_line_segments(l00, l01, l10, l11);
        assert_eq!(has.is_some(), true);
        if let Some((r0, r1)) = has {
            assert_eq!(approx(r0, vec3f(0.770429, 0.000000, -4.328010), 0.001), true);
            assert_eq!(approx(r1, vec3f(1.570000, 0.000000, 5.600000), 0.001), true);
        }
    }
    {
        let l00 = vec3f(-7.220000, 0.000000, 8.160000);
        let l01 = vec3f(3.350000, 0.000000, 0.970000);
        let l10 = vec3f(2.670000, 0.000000, 8.100000);
        let l11 = vec3f(6.330000, 0.000000, -0.210000);

        let has = shortest_line_segment_between_line_segments(l00, l01, l10, l11);
        assert_eq!(has.is_some(), true);
        if let Some((r0, r1)) = has {
            assert_eq!(approx(r0, vec3f(3.350000, 0.000000, 0.970000), 0.001), true);
            assert_eq!(approx(r1, vec3f(5.410576, 0.000000, 1.877545), 0.001), true);
        }
    }
    {
        let l00 = vec3f(-1.790000, 0.000000, 9.670000);
        let l01 = vec3f(-3.280000, 0.000000, 3.930000);
        let l10 = vec3f(7.450001, 0.000000, 2.280000);
        let l11 = vec3f(-9.090000, 0.000000, -8.059999);

        let has = shortest_line_segment_between_line_segments(l00, l01, l10, l11);
        assert_eq!(has.is_some(), true);
        if let Some((r0, r1)) = has {
            assert_eq!(approx(r0, vec3f(-3.280000, 0.000000, 3.930000), 0.001), true);
            assert_eq!(approx(r1, vec3f(0.476745, 0.000000, -2.079339), 0.001), true);
        }
    }
    {
        let l00 = vec3f(1.530000, 0.000000, -2.920000);
        let l01 = vec3f(9.440001, 0.000000, 6.680000);
        let l10 = vec3f(-8.040000, 0.000000, 5.300000);
        let l11 = vec3f(-0.970000, 0.000000, 7.219999);

        let has = shortest_line_segment_between_line_segments(l00, l01, l10, l11);
        assert_eq!(has.is_some(), true);
        if let Some((r0, r1)) = has {
            assert_eq!(approx(r0, vec3f(9.440001, 0.000000, 6.680000), 0.001), true);
            assert_eq!(approx(r1, vec3f(-0.970000, 0.000000, 7.219999), 0.001), true);
        }
    }
    {
        let l00 = vec3f(-9.760000, 0.000000, 8.010000);
        let l01 = vec3f(-1.470000, 0.000000, -0.230000);
        let l10 = vec3f(-0.670000, 0.000000, -7.020000);
        let l11 = vec3f(-0.190000, 0.000000, -3.650000);

        let has = shortest_line_segment_between_line_segments(l00, l01, l10, l11);
        assert_eq!(has.is_some(), true);
        if let Some((r0, r1)) = has {
            assert_eq!(approx(r0, vec3f(-1.470000, 0.000000, -0.230000), 0.001), true);
            assert_eq!(approx(r1, vec3f(-0.190000, 0.000000, -3.650000), 0.001), true);
        }
    }
    {
        let l00 = vec3f(8.139999, 0.000000, 0.630000);
        let l01 = vec3f(-4.640000, 0.000000, 4.250000);
        let l10 = vec3f(-9.060000, 0.000000, 6.290001);
        let l11 = vec3f(-4.990000, 0.000000, -4.830000);

        let has = shortest_line_segment_between_line_segments(l00, l01, l10, l11);
        assert_eq!(has.is_some(), true);
        if let Some((r0, r1)) = has {
            assert_eq!(approx(r0, vec3f(-4.639999, 0.000000, 4.250000), 0.001), true);
            assert_eq!(approx(r1, vec3f(-7.879392, 0.000000, 3.064359), 0.001), true);
        }
    }
    {
        let l00 = vec3f(-5.960000, 0.000000, 4.510000);
        let l01 = vec3f(-7.020000, 0.000000, -8.120000);
        let l10 = vec3f(-1.180000, 0.000000, -2.480000);
        let l11 = vec3f(5.660000, 0.000000, -2.840000);

        let has = shortest_line_segment_between_line_segments(l00, l01, l10, l11);
        assert_eq!(has.is_some(), true);
        if let Some((r0, r1)) = has {
            assert_eq!(approx(r0, vec3f(-6.523049, 0.000000, -2.198787), 0.001), true);
            assert_eq!(approx(r1, vec3f(-1.180000, 0.000000, -2.480000), 0.001), true);
        }
    }
    {
        let l00 = vec3f(1.440000, 0.000000, 5.010000);
        let l01 = vec3f(-1.030000, 0.000000, 8.709999);
        let l10 = vec3f(3.580000, 0.000000, 1.770000);
        let l11 = vec3f(-6.030000, 0.000000, -7.060000);

        let has = shortest_line_segment_between_line_segments(l00, l01, l10, l11);
        assert_eq!(has.is_some(), true);
        if let Some((r0, r1)) = has {
            assert_eq!(approx(r0, vec3f(1.440000, 0.000000, 5.010000), 0.001), true);
            assert_eq!(approx(r1, vec3f(3.580000, 0.000000, 1.770000), 0.001), true);
        }
    }
    {
        let l00 = vec3f(-7.690000, 0.000000, 7.450001);
        let l01 = vec3f(-8.250000, 0.000000, 6.350000);
        let l10 = vec3f(-6.010000, 0.000000, 9.680000);
        let l11 = vec3f(-5.880000, 0.000000, -7.400000);

        let has = shortest_line_segment_between_line_segments(l00, l01, l10, l11);
        assert_eq!(has.is_some(), true);
        if let Some((r0, r1)) = has {
            assert_eq!(approx(r0, vec3f(-7.690000, 0.000000, 7.450001), 0.001), true);
            assert_eq!(approx(r1, vec3f(-5.993125, 0.000000, 7.462916), 0.001), true);
        }
    }
    {
        let l00 = vec3f(-9.920000, 0.000000, -6.050000);
        let l01 = vec3f(9.680000, 0.000000, 1.130000);
        let l10 = vec3f(-0.380000, 0.000000, 9.420000);
        let l11 = vec3f(3.650000, 0.000000, -9.180000);

        let has = shortest_line_segment_between_line_segments(l00, l01, l10, l11);
        assert_eq!(has.is_some(), true);
        if let Some((r0, r1)) = has {
            assert_eq!(approx(r0, vec3f(2.023844, 0.000000, -1.674653), 0.001), true);
            assert_eq!(approx(r1, vec3f(2.023841, 0.000000, -1.674652), 0.001), true);
        }
    }
    {
        let l00 = vec3f(8.209999, 0.000000, 6.950001);
        let l01 = vec3f(-2.880000, 0.000000, 6.709999);
        let l10 = vec3f(-1.690000, 0.000000, -2.620000);
        let l11 = vec3f(0.570000, 0.000000, 6.160000);

        let has = shortest_line_segment_between_line_segments(l00, l01, l10, l11);
        assert_eq!(has.is_some(), true);
        if let Some((r0, r1)) = has {
            assert_eq!(approx(r0, vec3f(0.731690, 0.000000, 6.788161), 0.001), true);
            assert_eq!(approx(r1, vec3f(0.570000, 0.000000, 6.160000), 0.001), true);
        }
    }
}

#[test]
fn shortest_line_segment_between_lines_test() {
    {
        let l00 = vec3f(-4.970000, -1.730000, 7.290001);
        let l01 = vec3f(-1.600000, 1.120000, -6.970000);
        let l10 = vec3f(5.600000, 4.330000, 0.990000);
        let l11 = vec3f(-7.220000, 3.160000, 3.350000);

        let has = shortest_line_segment_between_lines(l00, l01, l10, l11);
        assert_eq!(has.is_some(), true);
        if let Some((r0, r1)) = has {
            assert_eq!(approx(r0, vec3f(-3.714814, -0.668493, 1.978739), 0.01), true);
            assert_eq!(approx(r1, vec3f(-3.949841, 3.458447, 2.748005), 0.01), true);
        }
    }
    {
        let l00 = vec3f(2.670000, 3.100000, 6.330000);
        let l01 = vec3f(-0.210000, -3.510000, -4.210000);
        let l10 = vec3f(3.930000, -1.640000, 4.850000);
        let l11 = vec3f(7.450001, -2.720000, -9.090000);

        let has = shortest_line_segment_between_lines(l00, l01, l10, l11);
        assert_eq!(has.is_some(), true);
        if let Some((r0, r1)) = has {
            assert_eq!(approx(r0, vec3f(1.652786, 0.765352, 2.607278), 0.01), true);
            assert_eq!(approx(r1, vec3f(4.279663, -1.747283, 3.465259), 0.01), true);
        }
    }
    {
        let l00 = vec3f(1.530000, 2.080000, 9.440001);
        let l01 = vec3f(6.680000, -0.100000, -8.760000);
        let l10 = vec3f(7.219999, 1.660000, -4.510000);
        let l11 = vec3f(-9.760000, 3.010000, -1.470000);

        let has = shortest_line_segment_between_lines(l00, l01, l10, l11);
        assert_eq!(has.is_some(), true);
        if let Some((r0, r1)) = has {
            assert_eq!(approx(r0, vec3f(5.347884, 0.463886, -4.052328), 0.01), true);
            assert_eq!(approx(r1, vec3f(5.429743, 1.802335, -4.189483), 0.01), true);
        }
    }
    {
        let l00 = vec3f(-0.670000, -2.020000, -0.190000);
        let l01 = vec3f(-3.650000, -4.870000, 8.650000);
        let l10 = vec3f(4.250000, 1.690000, -8.850000);
        let l11 = vec3f(-9.060000, 1.290000, -4.990000);

        let has = shortest_line_segment_between_lines(l00, l01, l10, l11);
        assert_eq!(has.is_some(), true);
        if let Some((r0, r1)) = has {
            assert_eq!(approx(r0, vec3f(2.161434, 0.687915, -8.589286), 0.01), true);
            assert_eq!(approx(r1, vec3f(2.227627, 1.629223, -8.263496), 0.01), true);
        }
    }
    {
        let l00 = vec3f(-5.960000, -0.490000, -7.020000);
        let l01 = vec3f(-8.120000, -3.770000, 5.050000);
        let l10 = vec3f(-2.840000, -1.630000, -5.620000);
        let l11 = vec3f(1.440000, 0.010000, -1.030000);

        let has = shortest_line_segment_between_lines(l00, l01, l10, l11);
        assert_eq!(has.is_some(), true);
        if let Some((r0, r1)) = has {
            assert_eq!(approx(r0, vec3f(-5.898550, -0.396686, -7.363382), 0.01), true);
            assert_eq!(approx(r1, vec3f(-4.779817, -2.373295, -7.700318), 0.01), true);
        }
    }
    {
        let l00 = vec3f(3.580000, -3.230000, -6.030000);
        let l01 = vec3f(-7.060000, 4.040000, 6.090000);
        let l10 = vec3f(6.350000, -2.020000, -8.580000);
        let l11 = vec3f(-6.010000, 4.680000, -5.880000);

        let has = shortest_line_segment_between_lines(l00, l01, l10, l11);
        assert_eq!(has.is_some(), true);
        if let Some((r0, r1)) = has {
            assert_eq!(approx(r0, vec3f(5.584741, -4.599781, -8.313595), 0.01), true);
            assert_eq!(approx(r1, vec3f(6.778583, -2.252322, -8.673622), 0.01), true);
        }
    }
    {
        let l00 = vec3f(-9.920000, -1.050000, 9.680000);
        let l01 = vec3f(1.130000, 0.300000, 0.060000);
        let l10 = vec3f(-9.180000, 3.520000, -2.330000);
        let l11 = vec3f(8.209999, 1.950000, -2.880000);

        let has = shortest_line_segment_between_lines(l00, l01, l10, l11);
        assert_eq!(has.is_some(), true);
        if let Some((r0, r1)) = has {
            assert_eq!(approx(r0, vec3f(4.823111, 0.751195, -3.155178), 0.01), true);
            assert_eq!(approx(r1, vec3f(4.969697, 2.242540, -2.777518), 0.01), true);
        }
    }
    {
        let l00 = vec3f(-9.920000, -1.050000, 9.680000);
        let l01 = vec3f(1.130000, 0.300000, 0.060000);
        let l10 = vec3f(-9.180000, 3.520000, -2.330000);
        let l11 = vec3f(1.870000, 4.870001, -11.950000);

        let has = shortest_line_segment_between_lines(l00, l01, l10, l11);
        assert_eq!(has.is_some(), false);
    }
    {
        let l00 = vec3f(-1.690000, 2.380000, 0.570000);
        let l01 = vec3f(6.160000, 2.900000, 6.400000);
        let l10 = vec3f(9.719999, -4.020000, 0.950000);
        let l11 = vec3f(17.570000, -3.500000, 6.780000);

        let has = shortest_line_segment_between_lines(l00, l01, l10, l11);
        assert_eq!(has.is_some(), false);
    }
    {
        let l00 = vec3f(-8.740000, -3.770000, 5.060000);
        let l01 = vec3f(8.129999, 2.700000, 2.380000);
        let l10 = vec3f(-6.340000, 0.340000, 2.260000);
        let l11 = vec3f(10.529999, 6.810000, -0.420000);

        let has = shortest_line_segment_between_lines(l00, l01, l10, l11);
        assert_eq!(has.is_some(), false);
    }
    {
        let l00 = vec3f(1.590000, 1.230000, -0.530000);
        let l01 = vec3f(3.850000, -2.830000, 2.720000);
        let l10 = vec3f(4.960000, 3.850000, -3.770000);
        let l11 = vec3f(7.220000, -0.210000, -0.520000);

        let has = shortest_line_segment_between_lines(l00, l01, l10, l11);
        assert_eq!(has.is_some(), false);
    }
}

#[test]
fn projection_tests() {
    {
        let view_proj = Mat4f::new(0.855010, 0.000000, 0.467094, 0.000000,
        0.398110, 1.520018, -0.728735, 0.000000,
        0.420820, -0.479521, -0.770305, 59.811974,
        0.420736, -0.479426, -0.770151, 60.000000);
        let p = vec3f(-5.080000, 0.420000, 9.870001);
        let vp = vec2f(1280.0, 720.0);
        let screen_point = project_to_sc(p, view_proj, vp);
        let unproj_point = unproject_sc(screen_point * vec3f(1.0, 1.0, 2.0) - vec3f(0.0, 0.0, 1.0), view_proj, vp);
        assert_eq!(approx(screen_point, vec3f(643.410583, 298.322327, 0.998102), 0.01), true);
        assert_eq!(approx(unproj_point, vec3f(-5.080000, 0.420000, 9.870001), 0.01), true);
    }
    {
        let view_proj = Mat4f::new(0.867752, 0.000000, -0.442972, 0.000000,
        -0.170388, 1.691024, -0.333778, 0.000000,
        -0.443986, -0.216407, -0.869738, 59.811977,
        -0.443897, -0.216364, -0.869564, 60.000004);
        let p = vec3f(-4.970000, -6.730000, 7.290001);
        let vp = vec2f(1280.0, 720.0);
        let screen_point = project_to_sc(p, view_proj, vp);
        let unproj_point = unproject_sc(screen_point * vec3f(1.0, 1.0, 2.0) - vec3f(0.0, 0.0, 1.0), view_proj, vp);
        assert_eq!(approx(screen_point, vec3f(555.795349, 278.564850, 0.998355), 0.01), true);
        assert_eq!(approx(unproj_point, vec3f(-4.970000, -6.730000, 7.290001), 0.01), true);
    }
    {
        let view_proj = Mat4f::new(-0.289494, 0.000000, -0.930275, -0.000002,
        -0.540549, 1.636921, 0.168214, 0.000000,
        -0.902573, -0.326913, 0.280873, 59.811981,
        -0.902393, -0.326848, 0.280817, 60.000008);
        let p = vec3f(-1.600000, -3.880000, -6.970000);
        let vp = vec2f(1280.0, 720.0);
        let screen_point = project_to_sc(p, view_proj, vp);
        let unproj_point = unproject_sc(screen_point * vec3f(1.0, 1.0, 2.0) - vec3f(0.0, 0.0, 1.0), view_proj, vp);
        assert_eq!(approx(screen_point, vec3f(713.182983, 320.543335, 0.998454), 0.01), true);
        assert_eq!(approx(unproj_point, vec3f(-1.600000, -3.880000, -6.970000), 0.01), true);
    }
    {
        let view_proj = Mat4f::new(-0.717769, -0.000000, 0.658807, -0.000004,
        0.365405, 1.645597, 0.398108, 0.000002,
        0.642576, -0.312051, 0.700085, 59.811974,
        0.642448, -0.311989, 0.699945, 60.000000);
        let p = vec3f(1.690000, 7.090000, 1.570000);
        let vp = vec2f(1280.0, 720.0);
        let screen_point = project_to_sc(p, view_proj, vp);
        let unproj_point = unproject_sc(screen_point * vec3f(1.0, 1.0, 2.0) - vec3f(0.0, 0.0, 1.0), view_proj, vp);
        assert_eq!(approx(screen_point, vec3f(638.092896, 437.494415, 0.998432), 0.01), true);
        assert_eq!(approx(unproj_point, vec3f(1.690000, 7.090000, 1.570000), 0.01), true);
    }
    {
        let view_proj = Mat4f::new(0.783699, 0.000000, 0.578822, 0.000002,
        0.329029, 1.641121, -0.445491, -0.000002,
        0.563026, -0.319815, -0.762312, 59.811974,
        0.562913, -0.319751, -0.762160, 60.000000);
        let p = vec3f(5.600000, -0.670000, 0.990000);
        let vp = vec2f(1280.0, 720.0);
        let screen_point = project_to_sc(p, view_proj, vp);
        let unproj_point = unproject_sc(screen_point * vec3f(1.0, 1.0, 2.0) - vec3f(0.0, 0.0, 1.0), view_proj, vp);
        assert_eq!(approx(screen_point, vec3f(690.717468, 361.736267, 0.998503), 0.01), true);
        assert_eq!(approx(unproj_point, vec3f(5.600000, -0.670000, 0.990000), 0.01), true);
    }
    {
        let view_proj = Mat4f::new(0.786060, -0.000000, 0.575611, 0.000000,
        0.990553, 0.434716, -1.352711, -0.000002,
        0.148312, -0.968185, -0.202537, 59.811974,
        0.148283, -0.967991, -0.202496, 60.000000);
        let p = vec3f(-7.220000, 8.160000, 3.350000);
        let vp = vec2f(1280.0, 720.0);
        let screen_point = project_to_sc(p, view_proj, vp);
        let unproj_point = unproject_sc(screen_point * vec3f(1.0, 1.0, 2.0) - vec3f(0.0, 0.0, 1.0), view_proj, vp);
        assert_eq!(approx(screen_point, vec3f(592.373108, 301.829865, 0.998114), 0.01), true);
        assert_eq!(approx(unproj_point, vec3f(-7.220000, 8.160000, 3.350000), 0.01), true);
    }
    {
        let view_proj = Mat4f::new(0.786060, -0.000000, 0.575611, 0.000000,
        0.990553, 0.434716, -1.352711, -0.000002,
        0.148312, -0.968185, -0.202537, 59.811974,
        0.148283, -0.967991, -0.202496, 60.000000);
        let p = vec3f(0.970000, 8.260000, 5.120000);
        let vp = vec2f(1280.0, 720.0);
        let screen_point = project_to_sc(p, view_proj, vp);
        let unproj_point = unproject_sc(screen_point * vec3f(1.0, 1.0, 2.0) - vec3f(0.0, 0.0, 1.0), view_proj, vp);
        assert_eq!(approx(screen_point, vec3f(686.450439, 343.276825, 0.998143), 0.01), true);
        assert_eq!(approx(unproj_point, vec3f(0.970000, 8.260000, 5.120000), 0.01), true);
    }
    {
        let view_proj = Mat4f::new(0.787158, 0.000000, 0.574109, 0.000000,
        0.562798, 1.444928, -0.771649, -0.000002,
        0.491681, -0.551528, -0.674141, 29.505840,
        0.491583, -0.551417, -0.674007, 29.699923);
        let p = vec3f(0.970000, 8.260000, 5.120000);
        let vp = vec2f(1280.0, 720.0);
        let screen_point = project_to_sc(p, view_proj, vp);
        let unproj_point = unproject_sc(screen_point * vec3f(1.0, 1.0, 2.0) - vec3f(0.0, 0.0, 1.0), view_proj, vp);
        assert_eq!(approx(screen_point, vec3f(746.891602, 498.507294, 0.995589), 0.01), true);
        assert_eq!(approx(unproj_point, vec3f(0.970000, 8.260000, 5.120000), 0.01), true);
    }
    {
        let view_proj = Mat4f::new(0.787158, 0.000000, 0.574109, 0.000000,
        0.562798, 1.444928, -0.771649, -0.000002,
        0.491681, -0.551528, -0.674141, 29.505840,
        0.491583, -0.551417, -0.674007, 29.699923);
        let p = vec3f(2.670000, 8.100000, 6.330000);
        let vp = vec2f(1280.0, 720.0);
        let screen_point = project_to_sc(p, view_proj, vp);
        let unproj_point = unproject_sc(screen_point * vec3f(1.0, 1.0, 2.0) - vec3f(0.0, 0.0, 1.0), view_proj, vp);
        assert_eq!(approx(screen_point, vec3f(804.766968, 494.470581, 0.995611), 0.01), true);
        assert_eq!(approx(unproj_point, vec3f(2.670000, 8.100000, 6.330000), 0.01), true);
    }
    {
        let view_proj = Mat4f::new(0.787158, 0.000000, 0.574109, 0.000000,
        0.562798, 1.444928, -0.771649, -0.000002,
        0.491681, -0.551528, -0.674141, 29.505840,
        0.491583, -0.551417, -0.674007, 29.699923);
        let p = vec3f(-0.210000, 1.490000, -4.210000);
        let vp = vec2f(1280.0, 720.0);
        let screen_point = project_to_sc(p, view_proj, vp);
        let unproj_point = unproject_sc(screen_point * vec3f(1.0, 1.0, 2.0) - vec3f(0.0, 0.0, 1.0), view_proj, vp);
        assert_eq!(approx(screen_point, vec3f(587.721069, 420.166504, 0.996936), 0.01), true);
        assert_eq!(approx(unproj_point, vec3f(-0.210000, 1.490000, -4.210000), 0.01), true);
    }
    {
        let view_proj = Mat4f::new(-0.780440, 0.000000, 0.583208, 0.000000,
        0.568056, 1.448952, 0.760165, -0.000003,
        0.500865, -0.547996, 0.670250, 29.505835,
        0.500765, -0.547886, 0.670116, 29.699917);
        let p = vec3f(-1.790000, 9.670000, -3.280000);
        let vp = vec2f(1280.0, 720.0);
        let screen_point = project_to_sc(p, view_proj, vp);
        let unproj_point = unproject_sc(screen_point * vec3f(1.0, 1.0, 2.0) - vec3f(0.0, 0.0, 1.0), view_proj, vp);
        assert_eq!(approx(screen_point, vec3f(624.503235, 537.422607, 0.995406), 0.01), true);
        assert_eq!(approx(unproj_point, vec3f(-1.790000, 9.670000, -3.280000), 0.01), true);
    }
    {
        let view_proj = Mat4f::new(-0.778339, 0.000000, -0.586010, 0.000001,
        -0.574464, 1.444928, 0.763003, 0.000000,
        -0.501874, -0.551528, 0.666588, 29.505838,
        -0.501773, -0.551417, 0.666455, 29.699921);
        let p = vec3f(3.930000, 3.360000, 4.850000);
        let vp = vec2f(1280.0, 720.0);
        let screen_point = project_to_sc(p, view_proj, vp);
        let unproj_point = unproject_sc(screen_point * vec3f(1.0, 1.0, 2.0) - vec3f(0.0, 0.0, 1.0), view_proj, vp);
        assert_eq!(approx(screen_point, vec3f(510.251587, 437.891876, 0.996664), 0.01), true);
        assert_eq!(approx(unproj_point, vec3f(3.930000, 3.360000, 4.850000), 0.01), true);
    }
    {
        let view_proj = Mat4f::new(-0.354547, 0.000000, -0.907477, 0.000001,
        -0.590134, 1.612012, 0.230563, -0.000001,
        -0.867056, -0.365868, 0.338755, 29.505838,
        -0.866883, -0.365795, 0.338687, 29.699921);
        let p = vec3f(7.450001, 2.280000, -9.090000);
        let vp = vec2f(1280.0, 720.0);
        let screen_point = project_to_sc(p, view_proj, vp);
        let unproj_point = unproject_sc(screen_point * vec3f(1.0, 1.0, 2.0) - vec3f(0.0, 0.0, 1.0), view_proj, vp);
        assert_eq!(approx(screen_point, vec3f(825.672546, 307.535034, 0.994926), 0.01), true);
        assert_eq!(approx(unproj_point, vec3f(7.450001, 2.280000, -9.090000), 0.01), true);
    }
    {
        let view_proj = Mat4f::new(-0.354547, 0.000000, -0.907477, 0.000001,
        -0.590134, 1.612012, 0.230563, -0.000001,
        -0.867056, -0.365868, 0.338755, 29.505838,
        -0.866883, -0.365795, 0.338687, 29.699921);
        let p = vec3f(-8.059999, -6.430000, 0.010000);
        let vp = vec2f(1280.0, 720.0);
        let screen_point = project_to_sc(p, view_proj, vp);
        let unproj_point = unproject_sc(screen_point * vec3f(1.0, 1.0, 2.0) - vec3f(0.0, 0.0, 1.0), view_proj, vp);
        assert_eq!(approx(screen_point, vec3f(686.695068, 308.304352, 0.997538), 0.01), true);
        assert_eq!(approx(unproj_point, vec3f(-8.059999, -6.430000, 0.010000), 0.01), true);
    }
    {
        let view_proj = Mat4f::new(-0.475640, 0.000000, 0.850286, 0.000000,
        0.552942, 1.612012, 0.309309, -0.000001,
        0.812412, -0.365868, 0.454454, 29.505840,
        0.812249, -0.365795, 0.454363, 29.699923);
        let p = vec3f(1.530000, -2.920000, 9.440001);
        let vp = vec2f(1280.0, 720.0);
        let screen_point = project_to_sc(p, view_proj, vp);
        let unproj_point = unproject_sc(screen_point * vec3f(1.0, 1.0, 2.0) - vec3f(0.0, 0.0, 1.0), view_proj, vp);
        assert_eq!(approx(screen_point, vec3f(768.687134, 350.665802, 0.997345), 0.01), true);
        assert_eq!(approx(unproj_point, vec3f(1.530000, -2.920000, 9.440001), 0.01), true);
    }
    {
        let view_proj = Mat4f::new(-0.475640, 0.000000, 0.850286, 0.000000,
        0.552942, 1.612012, 0.309309, -0.000001,
        0.812412, -0.365868, 0.454454, 29.505840,
        0.812249, -0.365795, 0.454363, 29.699923);
        let p = vec3f(6.680000, 4.900000, -8.760000);
        let vp = vec2f(1280.0, 720.0);
        let screen_point = project_to_sc(p, view_proj, vp);
        let unproj_point = unproject_sc(screen_point * vec3f(1.0, 1.0, 2.0) - vec3f(0.0, 0.0, 1.0), view_proj, vp);
        assert_eq!(approx(screen_point, vec3f(408.321289, 468.944641, 0.996693), 0.01), true);
        assert_eq!(approx(unproj_point, vec3f(6.680000, 4.900000, -8.760000), 0.01), true);
    }
}