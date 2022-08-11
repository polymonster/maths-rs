## Maths

Maths is a rust linear algebra crate which aims to ergonmic and fun to use for gamedev and graphics. If you like writing shaders you should feel right at home. In addition to the usual implementation of vectors, matrics and quaternions it includes a comprehensive array of utility functions, intersection tests, point tests, distance functions, graphs and signal processing functions.

### Vector

This is a narrow vector library with `Vec2`, `Vec3` and `Vec4` column-vector implementations. There are macros to create vectors of larger dimensions but the library is focused on games and graphics where 4-Dimensions are sufficient.

```rust
/// vectors are generic so for convenience I suggest you abbreviate your types
pub type Vec2f = Vec2<f32>;
pub type Vec3f = Vec3<f32>;
pub type Vec4f = Vec4<f32>;

// construct a vector
let v2 = Vec2f::new(2.0, 3.0);

// the short way 
let v2_short = vec2f(5.0, 6.0);

// splat a scalar
let v3_splat = splat3f(9.0);

// arithmentic
let result = v2 * v2_short;
let result = v2 + v2_short;
let result = v2 / v2_short;

// construct from tuples of various sizes
let v4 = Vec4f::from((v2, v2)); // vec4 from 2x v2's
let v3 = Vec3f::from((v2, 1.0)); // vec3 from 1x v2 and 1x scalar
let v2 = Vec2f::from((5.0, 6.0)); // vec2 from 2x scalars
// .. and so on
```

### Matrix

Column-major matrices with `Mat2`, `Mat3`, `Mat34` and `Mat4` implementations. Again wider matrices can be created using a macros which will do most of the work.

```

```

### Quaternion