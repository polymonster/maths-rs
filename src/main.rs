pub mod vec;

use vec::*;

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