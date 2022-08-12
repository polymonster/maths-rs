## Maths

Maths is a linear algebra crate which aims to be ergonmic and fun to use for gamedev and graphics. If you like writing shaders you should feel right at home. In addition to the usual implementation of vectors, matrics and quaternions it includes a comprehensive array of utility functions, intersection tests, point tests, distance functions, graphs and signal processing functions.

### Vector

This is a narrow vector library with `Vec2`, `Vec3` and `Vec4` column-vector implementations. There are macros to create vectors of larger dimensions but the library is focused on games and graphics where 4-Dimensions are sufficient.

```rust
/// abbrivated tupes are not included them, but you define them for convenience 
pub type Vec2f = Vec2<f32>;
pub type Vec3f = Vec3<f32>;
pub type Vec4f = Vec4<f32>;

// construct a vector
let v2 = Vec2f::new(2.0, 3.0);

// the short way 
let v2_short = vec2f(5.0, 6.0);

// splat a scalar
let v3_splat = splat3f(9.0);

// arithmentic / operators
let result = v2 * v2_short;
let result = v2 + v2_short;
let result = v2 / v2_short;
let result = v2 - v2_short;
let result = -v2;

// quick constructors
let y = Vec3f::unit_y();
let b = Vec3f::blue();
let z = Vec3f::zero();
let o = Vec3f::one();
// + more

// construct from tuples and vectors of various sizes
let v4 = Vec4f::from((v2, v2)); // vec4 from 2x v2's
let v3 = Vec3f::from((v2, 1.0)); // vec3 from 1x v2 and 1x scalar
let v2 = Vec2f::from((5.0, 6.0)); // vec2 from 2x scalars
let v4 = Vec4f::from((v2, 0.0, 1.0)); // vec4 from 1x v2 and 2x scalars
let v4 = Vec4f::from(v2); // vec4 from vec2 (splat 0's)
let v2 = Vec2f::from(v4); // vec2 from vec4 (truncate)
// .. and so on
```

### Matrix

Column-major matrices with `Mat2`, `Mat3`, `Mat34` and `Mat4` implementations. Again wider matrices can be created using a macros which will do most of the work.

```rust
// abbrivated types
pub type Mat2f = Mat2<f32>;
pub type Mat3f = Mat3<f32>;
pub type Mat34f = Mat34<f32>;
pub type Mat4f = Mat4<f32>; 

// construct from floats
let m4 = Mat4f::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 1.0, 0.0,
    0.0, 0.0, 0.0, 1.0
);

// construct from common matrices
let m34 = Mat34f::create_translation(vec3f(50.0, -10.0, 20.0));
let m4 = Mat4f::create_z_rotation(f32::deg_to_rad(90.0));
let x4 = Mat4f::create_scale(vec3f(10.0, 2.0, 30.0));
let m3 = Mat34f::create_scale(vec3f(10.0, 2.0, 30.0));

// arithmetic
// matrix multiplication
let result = x4 * m4;

// vector transformation
let v4 = vec4f(0.0, 1.0, 0.0, 1.0);
let result = m4 * v4;

// functions
let det = m4.determinant();
let inv = m4.inverse();

// construct rows from tuples
let m3v = Mat3f::from((
    vec3f(1.0, 2.0, 3.0),
    vec3f(4.0, 5.0, 6.0),
    vec3f(7.0, 8.0, 9.0)
));

let m4v = Mat4f::from((
    vec4f(1.0, 2.0, 3.0, 4.0),
    vec4f(5.0, 6.0, 7.0, 8.0),
    vec4f(9.0, 10.0, 11.0, 12.0),
    vec4f(13.0, 14.0, 15.0, 16.0),
));

let m4rot = Mat4f::from(m3v); // mat4 from mat3
let m4r = Mat3f::from(quat); // from quaternions
```

### Quaternion

Generic floating-point quaternion.

```rust
// abbreviations
pub type Quatf = Quat<f32>;

// construct from euler angles
let q = Quatf::from_euler_angles(x, y, z);

// construct from axis angle
let q2 = Quatf::from_axis_angle(axis, angle);

// arithmetic
let q3 = q * q2;
let q4 = q + q2;
let q5 = q - q2;
let q6 = -q;

// functions
let rev = q.reverse();
let inv = q.inverse();
```

### Generic Functions

You can use generic functions on different sized vectors or scalars:

```rust
// numeric ops
let int_abs = abs(-1);
let float_abs = abs(-1.0);
let int_max = max(5, 6);
let float_max = max(1.0, 2.0);
let vec_mac = max(vec3f(1.0, 1.0, 1.0), vec3f(2.0, 2.0, 2.0));
let vec_min = min(vec4f(8.0, 7.0, 6.0, 5.0), vec4f(-8.0, -7.0, -6.0, -5.0));

// float ops
let fsat = saturate(22.0);
let vsar = saturate(vec3f(22.0, 22.0, 22.0));
let f = floor(5.5);
let vc = ceil(vec3f(5.0, 5.0, 5.0));

// vector ops
let dp = dot(vec2, vec2);
let dp = dot(vec3, vec3);
let cp = cross(vec3, Vec3::unit_y());
let n = normalize(vec3);
let qn = normalize(quat);
let m = mag(vec4);

// interpolation
let vi = lerp(vec2, vec2, 0.5);
let qi = lerp(quat1, quat2, 0.75);
let qs = slerp(quat1, quat2, 0.1);
let vn = nlerp(vec2, vec2, 0.3);
let f = smoothstep(5.0, 1.0, f);
```

The full set of functions consists of: min, max, clamp, step, signum, copysign, abs, deg_to_rad, rad_to_deg, ceil, round, approx, sqrt, powi, powf, sqrt, frac, trunc, modf, rsqrt, recip lerp, nlerp, slerp, smoothstep.

### Trigenometry and Logarithmic Functions

These functions are availble for all floating point scalar or vector types, they do not have generic implementations as they are not as commonly used, you can access them like so:

```rust
// trig
let s = Vec3f::sin(x);
let x = Vec3f::cos(y);
let f = f32::acos(x);
let d = f64::atan2(y, x);
```

The full list consists of: cos, sin, tan, acos, asin, atan, cosh, sinh, tanh, sin_cos, atan2, exp, exp2, log2, log10.

### Point / Distance Tests

### Intersection Tests

### Graphs / Signal Processing Functions