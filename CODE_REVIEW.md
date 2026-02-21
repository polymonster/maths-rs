# Code Review - maths-rs 1.0.0 Readiness

Reviewed: 2026-02-21

## Blocking (must fix before 1.0)

- [x] **B1** `lib.rs:23-47` — `#[cfg(feature = "short_types")]` only applies to `Vec2f`, all other type aliases are always public
- [x] **B2** `quat.rs:90,126` — `Quat::from_matrix` variable `t` is always zero, causes division by zero / NaN
- [x] **B3** `lib.rs:1181` — `ray_vs_triangle` parallel check `a > eps && a < eps` is always false, should be `a > -eps && a < eps`
- [x] **B4** `vec.rs:455` — `reflect` formula has wrong operator precedence: `(i - 2) * n * dot` instead of `i - n * (2 * dot)`
- [ ] **B5** `vec.rs:527-531` — `Vec::three()` returns 4 (copy-paste from `four()`), affects `smoothstep`
- [ ] **B6** `lib.rs:786` — `point_inside_convex_hull` 2D cross product uses `-` instead of `*`: `v1.x * v2.y - v2.x - v1.y`
- [ ] **B7** `quat.rs:26-31` — Quat fields are private with no `new()` constructor, users can't construct or read components
- [ ] **B8** `lib.rs:1686` — `sinc` always returns NaN: `X::zero() * (...)` is always 0, then `sin(0)/0`. Should be `X::pi()`
- [ ] **B9** `quat.rs:312,325,338,351` — `Quat * Quat` y-component uses `self.z * other.z` instead of `self.z * other.x` in all 4 Mul impls

## Important (should fix before 1.0)

- [ ] **I1** `vec.rs`, `quat.rs` — `normalize` on zero-length vec/quat produces NaN silently
- [ ] **I2** `lib.rs:1000-1009,1043-1052` — OBB corner array has duplicate: index 3 is `(-1,1,1)` instead of `(-1,1,-1)`
- [ ] **I3** `lib.rs`, `vec.rs` — Redundant names in public API: `dist` vs `distance`, `mag` vs `length`
- [ ] **I4** `lib.rs:412,587,592,719,724,778,795` — Functions take `&Vec<T>` instead of idiomatic `&[T]`
- [ ] **I5** `lib.rs:1627` — `exp_sustained_impulse` requires `Ord` on `T`, uncallable with f32/f64
- [ ] **I6** `mat.rs:212` — `set_row` doc uses `//` instead of `///`, missing from rustdoc
- [ ] **I7** `vec.rs:344-358`, `mat.rs:85-101,234-251`, `quat.rs:185-202` — `unsafe` slice conversions lack `// SAFETY:` comments
- [ ] **I8** — No `#[must_use]` attributes on pure math functions
- [ ] **I9** `num.rs:196-212,333-345` — `isize` is `Number` but not `SignedNumber`, inconsistent
- [ ] **I10** `vec.rs:838-844` — Index out of bounds silently returns `x` component instead of panicking

## Minor (nice to have)

- [ ] **M1** Doc comment typos throughout:
  - `lib.rs:90` "paremeter" -> "parameter"
  - `lib.rs:100,105` "specificied" -> "specified"
  - `lib.rs:230` "perpedicular" -> "perpendicular"
  - `lib.rs:263` "chebyshevnormalized" -> "Chebyshev normalized"
  - `lib.rs:456` "placked" -> "packed", "constanr" -> "constant"
  - `lib.rs:471` "perfroming" -> "performing"
  - `lib.rs:633` "diection" -> "direction"
  - `lib.rs:750` "cirlcle" -> "circle"
  - `lib.rs:967` "intsercts" -> "intersects"
  - `lib.rs:981` "ture" -> "true"
  - `vec.rs:173` "levearging" -> "leveraging"
  - `quat.rs:205` "quationion" -> "quaternion"
  - `quat.rs:484` "lineraly" -> "linearly"
- [ ] **M2** — No prelude module; users must import from multiple submodules
- [ ] **M3** `mat.rs:309,320,332,344,356` — Multiple Mat2 `From` impl docs say "constructs Mat3"
- [ ] **M4** `Cargo.toml:19` — `serde_json` dependency is unnecessary, users bring their own format
- [ ] **M5** `Cargo.toml:12` — Uses `license-file` instead of `license = "MIT"` for crates.io
- [ ] **M6** — No crate-level docs on conventions (handedness, row/column major, angle units)
- [ ] **M7** — Quat missing `#[cfg_attr(feature="hash", derive(Hash))]` unlike vec/mat types
