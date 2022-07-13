use crate::num::Number;
use crate::num::SignedNumber;
use crate::num::Float;
use crate::num::NumberOps;
use crate::num::SignedNumberOps;
use crate::num::FloatOps;

pub fn min<T: Number, V: NumberOps<T>>(a: V, b: V) -> V {
    V::min(a, b)
}

pub fn max<T: Number, V: NumberOps<T>>(a: V, b: V) -> V {
    V::max(a, b)
}

pub fn clamp<T: Number, V: NumberOps<T>>(x: V, min: V, max: V) -> V {
    V::clamp(x, min, max)
}

pub fn step<T: Number, V: NumberOps<T>>(a: V, b: V) -> V {
    V::step(a, b)
}

pub fn sign<T: SignedNumber, V: SignedNumberOps<T>>(a: V) -> V {
    V::sign(a)
}

pub fn abs<T: SignedNumber, V: SignedNumberOps<T>>(a: V) -> V {
    V::abs(a)
}

pub fn deg_to_rad<T: Float, V: FloatOps<T>>(a: V) -> V {
    V::deg_to_rad(a)
}

pub fn rad_to_deg<T: Float, V: FloatOps<T>>(a: V) -> V {
    V::deg_to_rad(a)
}

pub fn floor<T: Float, V: FloatOps<T>>(a: V) -> V {
    V::floor(a)
}

pub fn ceil<T: Float, V: FloatOps<T>>(a: V) -> V {
    V::ceil(a)
}

pub fn round<T: Float, V: FloatOps<T>>(a: V) -> V {
    V::round(a)
}

pub fn sqrt<T: Float, V: FloatOps<T>>(a: V) -> V {
    V::sqrt(a)
}

pub fn powi<T: Float, V: FloatOps<T>>(a: V, b: i32) -> V {
    V::powi(a, b)
}

pub fn powf<T: Float, V: FloatOps<T>>(a: V, b: T) -> V {
    V::powf(a, b)
}

pub fn lerp<T: Float, V: FloatOps<T>>(e0: V, e1: V, t: T) -> V {
    V::lerp(e0, e1, t)
}

pub fn smoothstep<T: Float, V: FloatOps<T>>(e0: V, e1: V, t: T) -> V {
    V::smoothstep(e0, e1, t)
}

pub fn saturate<T: Float, V: FloatOps<T>>(x: V) -> V {
    V::saturate(x)
}