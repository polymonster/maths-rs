pub mod vec;

use vec::*;

#[test]
fn v2_construct() {
    let v2 : Vec2<f32> = Vec2 {
        x: 2.0,
        y: 3.0
    };
    let v2_short = vec2f(2.0, 3.0);
    assert_eq!(v2.x, v2_short.x);
    assert_eq!(v2.y, v2_short.y);
}

#[test]
fn v2_len() {
    assert_eq!(Vec2::<f32>::len(), 2);
}

#[test]
fn v3_construct() {
    let v3 : Vec3<f32> = Vec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0
    };
    let v3_short = vec3f(1.0, 2.0, 3.0);
    assert_eq!(v3.x, v3_short.x);
    assert_eq!(v3.y, v3_short.y);
    assert_eq!(v3.z, v3_short.z);
}

#[test]
fn v3_len() {
    assert_eq!(Vec3::<f32>::len(), 3);
}

fn main() {
    let v2 : Vec2<f32> = Vec2 {
        x: 2.0,
        y: 3.0
    };

    let ve : Vec2<f32> = Vec2 {
        x: 2.0,
        y: 3.0
    };

    let vf : Vec3<f32> = Vec3 {
        x: 1.0,
        y: 0.0,
        z: 1.0
    };

    let dot3 = v3::dot(&vf, &vf);
    println!("dot3 = {}", dot3);

    let dot2 = v2::dot(&ve, &ve);
    println!("dot2 = {}", dot2);

    let vneg = -v2;
    println!("neg = {}", vneg);

    let v2v2 = ve * ve;
    println!("ve * ve = {}", v2v2);

    let v2scalar = ve * 20.0;
    println!("ve * 20.0 = {}", v2scalar);

    if v2 == ve {
        println!("equals!");
    }

    let dp = v2::dot(&v2, &v2);
    println!("dot = {}", dp);
}