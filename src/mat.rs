
use crate::vec::*;
use crate::num::*;
use crate::quat::*;

use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Deref;
use std::ops::DerefMut;

use std::cmp::PartialEq;

use std::fmt::Display;
use std::fmt::Formatter;

/// row major matrix index layouts
/// 
/// Mat2:
/// 00 01
/// 02 03 
/// 
/// Mat3:
/// 00 01 02
/// 03 04 05
/// 06 07 08
/// 
/// Mat34:
/// 00 01 02 03
/// 04 05 06 07
/// 08 09 10 11
/// 
/// Mat44:
/// 00 01 02 03
/// 04 05 06 07
/// 08 09 10 11
/// 12 13 14 15

/// Create a partial matrix class, this is the concreate matrix struct and indexing operations.
/// the partial implementation allows data only (no arithmetic or traits) to be created
/// to facilitate 3x4 to 4x3 trasposes for interoperation with other API's and shaders
macro_rules! mat_partial_impl {
    ($MatN:ident, $rows:expr, $cols:expr, $elems:expr, 
        $RowVecN:ident { $($row_field:ident, $row_field_index:expr),* },
        $ColVecN:ident { $($col_field:ident, $col_field_index:expr),* } ) => {
        #[derive(Debug, Copy, Clone)]
        #[repr(C)]
        pub struct $MatN<T> {
            pub m: [T; $elems]
        }

        /// index into matrix with tuple \[(row, column)\]
        impl<T> Index<(usize, usize)> for $MatN<T> {
            type Output = T;
            fn index(&self, rc: (usize, usize)) -> &Self::Output {
                &self.m[rc.0 * $cols + rc.1]
            }
        }

        /// mutably index into matrix with tuple \[(row, column)\]
        impl<T> IndexMut<(usize, usize)> for $MatN<T> {
            fn index_mut(&mut self, rc: (usize, usize)) -> &mut Self::Output {
                &mut self.m[rc.0 * $cols + rc.1]
            }
        }

        /// index into matrix raw array single \[index\]
        impl<T> Index<usize> for $MatN<T> {
            type Output = T;
            fn index(&self, index: usize) -> &Self::Output {
                &self.m[index]
            }
        }

        /// mutably index into matrix raw array with single \[index\]
        impl<T> IndexMut<usize> for $MatN<T> {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.m[index]
            }
        }

        /// deref matrix as a slice of T
        impl<T> Deref for $MatN<T> where T: Number {
            type Target = [T];
            fn deref(&self) -> &Self::Target {
                unsafe {
                    std::slice::from_raw_parts(&self.m[0], $elems)
                }
            }
        }

        /// mutably deref matrix as a slice of T
        impl<T> DerefMut for $MatN<T> where T: Number {
            fn deref_mut(&mut self) -> &mut [T] {
                unsafe {
                    std::slice::from_raw_parts_mut(&mut self.m[0], $elems)
                }
            }
        }

        /// displays like:
        /// [1.0, 0.0, 0.0]
        /// [0.0, 1.0, 1.0]
        /// [0.0, 0.0, 1.0]
        impl<T> Display for $MatN<T> where T: Display {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                let mut output = String::from("");
                for r in 0..$rows {
                    output += &String::from("[");
                    for c in 0..$cols {
                        output += &self[(r, c)].to_string();
                        if c < $cols-1 {
                            output += &String::from(", ");
                        }
                        
                    }
                    output += "]";
                    if r < $rows-1 {
                        output += "\n";
                    }
                }
                write!(f, "{}", output)
            }
        }

        impl<T> Eq for $MatN<T> where T: Eq  {}
        impl<T> PartialEq for $MatN<T> where T: PartialEq  {
            fn eq(&self, other: &Self) -> bool {
                for i in 0..$elems {
                    if self.m[i] != other.m[i] {
                        return false;
                    }
                }
                true
            }
        }

        impl<T> $MatN<T> where T: Float + FloatOps<T> {
            /// compare if matrices are approximately equal to account for floating point precision
            pub fn approx(lhs: Self, rhs: Self, eps: T) -> bool {
                for i in 0..$elems {
                    if !T::approx(lhs.m[i], rhs.m[i], eps) {
                        return false;
                    }
                }
                true
            }
        }
    }
}

/// creates the basic generic traits for row-major matrices 
macro_rules! mat_impl {
    ($MatN:ident, $rows:expr, $cols:expr, $elems:expr, 
        $RowVecN:ident { $($row_field:ident, $row_field_index:expr),* },
        $ColVecN:ident { $($col_field:ident, $col_field_index:expr),* } ) => {
        impl<T> $MatN<T> where T: Number {
            /// initialise matrix to all zero's
            pub fn zero() -> $MatN<T> {
                $MatN {
                    m: [T::zero(); $elems]
                }
            }

            /// initialise matrix to identity
            pub fn identity() -> $MatN<T> {
                unsafe {
                    let mut mat : $MatN<T> = std::mem::zeroed();
                    for r in 0..$rows {
                        for c in 0..$cols {
                            if c == r {
                                mat.set(r, c, T::one());
                            }
                        }
                    }
                    mat
                }
            }

            pub const fn get_num_rows(&self) -> usize {
                $rows
            }

            pub const fn get_num_columns(&self) -> usize {
                $cols
            }

            /// get single element from the matrix at row, column index
            pub fn at(self, row: u32, column: u32) -> T {
                let urow = row as usize;
                let ucol = column as usize;
                self.m[urow * $cols + ucol]
            }

            /// set a single element of the matrix at row, column index
            pub fn set(&mut self, row: u32, column: u32, value: T) {
                let urow = row as usize;
                let ucol = column as usize;
                self.m[urow * $cols + ucol] = value
            }

            /// gets a single row of the matrix in n sized vec where in is the column count of the matrix
            pub fn get_row(self, row: u32) -> $RowVecN<T> {
                let urow = row as usize;
                $RowVecN {
                    $($row_field: self.m[urow * $cols + $row_field_index],)+
                }
            }

            // sets a single row of the matrix by an n sized vec, where n is the column count of the matrix
            pub fn set_row(&mut self, row: u32, value: $RowVecN<T>) {
                let urow = row as usize;
                $(self.m[urow * $cols + $row_field_index] = value.$row_field;)+
            }

            /// gets a single column of the matrix in n sized vec where in is the row count of the matrix
            pub fn get_column(self, column: u32) -> $ColVecN<T> {
                let ucol = column as usize;
                $ColVecN {
                    $($col_field: self.m[$col_field_index * $cols + ucol],)+
                }
            }

            /// sets a single column of the matrix by an n sized vec, where n is the row count of the matrix
            pub fn set_column(&mut self, column: u32, value: $ColVecN<T>) {
                let ucol = column as usize;
                $(self.m[$col_field_index * $cols + ucol] = value.$col_field;)+
            }

            /// returns a slice T of the matrix
            pub fn as_slice(&self) -> &[T] {
                unsafe {
                    std::slice::from_raw_parts(&self.m[0], $elems)
                }
            }

            /// returns a mutable slice T of the matrix
            pub fn as_mut_slice(&mut self) -> &mut [T] {
                unsafe {
                    std::slice::from_raw_parts_mut(&mut self.m[0], $elems)
                }
            }

            /// returns a slice of bytes for the matrix
            pub fn as_u8_slice(&self) -> &[u8] {
                unsafe {
                    std::slice::from_raw_parts((&self.m[0] as *const T) as *const u8, std::mem::size_of::<$MatN<T>>())
                }
            }
        }
    }
}

macro_rules! mat_cast {
    ($MatN:ident, $count:expr, $t:ident, $u:ident) => {
        impl From<$MatN<$u>> for $MatN<$t> {
            /// cast matrix from type u to type t
            fn from(other: $MatN<$u>) -> $MatN<$t> {
                unsafe {
                    let mut mm : $MatN<$t> = std::mem::zeroed();
                    for i in 0..$count {
                        mm.m[i] = other.m[i] as $t;
                    }
                    mm
                }
            }
        }

        impl From<$MatN<$t>> for $MatN<$u> {
            /// cast matrix from type t to type u
            fn from(other: $MatN<$t>) -> $MatN<$u> {
                unsafe {
                    let mut mm : $MatN<$u> = std::mem::zeroed();
                    for i in 0..$count {
                        mm.m[i] = other.m[i] as $u;
                    }
                    mm
                }
            }
        }
    }
}

#[cfg(feature = "casts")]
mat_cast!(Mat2, 4, f32, f64);
mat_cast!(Mat3, 9, f32, f64);
mat_cast!(Mat34, 12, f32, f64);
mat_cast!(Mat4, 16, f32, f64);

//
// From
//

/// intialse matrix from tuple of 4 elements
impl<T> From<(T, T, T, T)> for Mat2<T> where T: Number {
    fn from(other: (T, T, T, T)) -> Mat2<T> {
        Mat2 {
            m: [
                other.0, other.1,
                other.2, other.3
            ]
        }
    }
}

/// constructs Mat3 from tuple of 2 2D row vectors
impl<T> From<(Vec2<T>, Vec2<T>)> for Mat2<T> where T: Number {
    fn from(other: (Vec2<T>, Vec2<T>)) -> Mat2<T> {
        Mat2 {
            m: [
                other.0.x, other.0.y,
                other.1.x, other.1.y
            ]
        }
    }
}

/// constructs Mat3 from 3x3 matrix truncating the 3rd row and column
impl<T> From<Mat3<T>> for Mat2<T> where T: Number {
    fn from(other: Mat3<T>) -> Mat2<T> {
        Mat2 {
            m: [
                other.m[0], other.m[1],
                other.m[3], other.m[4],
            ]
        }
    }
}

/// constructs Mat3 from 3x4 matrix truncating the 3rd row and 3rd and 4th column
impl<T> From<Mat34<T>> for Mat2<T> where T: Number {
    fn from(other: Mat34<T>) -> Mat2<T> {
        Mat2 {
            m: [
                other.m[0], other.m[1],
                other.m[4], other.m[5],
            ]
        }
    }
}

/// constructs Mat3 from 3x4 matrix truncating the 3rd and 4th row and 3rd and 4th column
impl<T> From<Mat4<T>> for Mat2<T> where T: Number {
    fn from(other: Mat4<T>) -> Mat2<T> {
        Mat2 {
            m: [
                other.m[0], other.m[1],
                other.m[4], other.m[5],
            ]
        }
    }
}

/// constructs Mat3 from a Mat2 initialising the 2x2 part and setting the 3rd column and row to identity
impl<T> From<Mat2<T>> for Mat3<T> where T: Number {
    fn from(other: Mat2<T>) -> Mat3<T> {
        Mat3 {
            m: [
                other.m[0], other.m[1], T::zero(),
                other.m[2], other.m[3], T::zero(),
                T::zero(), T::zero(), T::one()
            ]
        }
    }
}

/// construct a Mat3 from a Mat34 initialising the 3x3 part and truncation the 4th column
impl<T> From<Mat34<T>> for Mat3<T> where T: Number {
    fn from(other: Mat34<T>) -> Mat3<T> {
        Mat3 {
            m: [
                other.m[0], other.m[1], other.m[2],
                other.m[4], other.m[5], other.m[6],
                other.m[8], other.m[9], other.m[10],
            ]
        }
    }
}

/// construct a Mat3 from a Mat44 initialising the 3x3 part and truncation the 4th row and column
impl<T> From<Mat4<T>> for Mat3<T> where T: Number {
    fn from(other: Mat4<T>) -> Mat3<T> {
        Mat3 {
            m: [
                other.m[0], other.m[1], other.m[2],
                other.m[4], other.m[5], other.m[6],
                other.m[8], other.m[9], other.m[10],
            ]
        }
    }
}

/// construct a Mat3 from tuple of 9 elements
impl<T> From<(T, T, T, T, T, T, T, T, T)> for Mat3<T> where T: Number {
    fn from(other: (T, T, T, T, T, T, T, T, T)) -> Mat3<T> {
        Mat3 {
            m: [
                other.0, other.1, other.2, 
                other.3, other.4, other.5, 
                other.6, other.7, other.8,
            ]
        }
    }
}

/// constructs Mat3 from tuple of 3 3D row vectors
impl<T> From<(Vec3<T>, Vec3<T>, Vec3<T>)> for Mat3<T> where T: Number {
    fn from(other: (Vec3<T>, Vec3<T>, Vec3<T>)) -> Mat3<T> {
        Mat3 {
            m: [
                other.0.x, other.0.y, other.0.z,
                other.1.x, other.1.y, other.1.z,
                other.2.x, other.2.y, other.2.z,
            ]
        }
    }
}

/// constucts a mat3 rotation matrix from quaternion
impl<T> From<Quat<T>> for Mat3<T> where T: Float + SignedNumber + FloatOps<T> + NumberOps<T> + SignedNumberOps<T> {
    fn from(other: Quat<T>) -> Mat3<T> {
        other.get_matrix()
    }
}

/// construct a Mat34 from a Mat2 initialising the 2x2 part and setting the 3rd row to identity
impl<T> From<Mat2<T>> for Mat34<T> where T: Number {
    fn from(other: Mat2<T>) -> Mat34<T> {
        Mat34 {
            m: [
                other.m[0], other.m[1], T::zero(), T::zero(),
                other.m[2], other.m[3], T::zero(), T::zero(),
                T::zero(), T::zero(), T::one(), T::zero(),
            ]
        }
    }
}

/// construct a Mat34 from a Mat3 initialising the 3x3 part and setting the 4th row to zero
impl<T> From<Mat3<T>> for Mat34<T> where T: Number {
    fn from(other: Mat3<T>) -> Mat34<T> {
        Mat34 {
            m: [
                other.m[0], other.m[1], other.m[2], T::zero(),
                other.m[3], other.m[4], other.m[5], T::zero(),
                other.m[6], other.m[7], other.m[8], T::zero(),
            ]
        }
    }
}

/// constucts a mat34 rotation matrix from quaternion
impl<T> From<Quat<T>> for Mat34<T> where T: Float + SignedNumber + FloatOps<T> + NumberOps<T> + SignedNumberOps<T> {
    fn from(other: Quat<T>) -> Mat34<T> {
        Mat34::from(other.get_matrix())
    }
}

/// construct a Mat34 from a Mat4 initialising the 3x4 part and truncating the 4th row
impl<T> From<Mat4<T>> for Mat34<T> where T: Number {
    fn from(other: Mat4<T>) -> Mat34<T> {
        Mat34 {
            m: [
                other.m[0], other.m[1], other.m[2], other.m[3],
                other.m[4], other.m[5], other.m[6], other.m[7],
                other.m[8], other.m[9], other.m[10], other.m[11],
            ]
        }
    }
}

/// construct a Mat34 from tuple of 12 elements
impl<T> From<(T, T, T, T, T, T, T, T, T, T, T, T)> for Mat34<T> where T: Number {
    fn from(other: (T, T, T, T, T, T, T, T, T, T, T, T)) -> Mat34<T> {
        Mat34 {
            m: [
                other.0, other.1, other.2, other.3, 
                other.4, other.5, other.6, other.7, 
                other.8, other.9, other.10, other.11
            ]
        }
    }
}

/// constructs Mat34 from tuple of 3 4D row vectors
impl<T> From<(Vec4<T>, Vec4<T>, Vec4<T>)> for Mat34<T> where T: Number {
    fn from(other: (Vec4<T>, Vec4<T>, Vec4<T>)) -> Mat34<T> {
        Mat34 {
            m: [
                other.0.x, other.0.y, other.0.z, other.0.w,
                other.1.x, other.1.y, other.1.z, other.1.w,
                other.2.x, other.2.y, other.2.z, other.2.w,
            ]
        }
    }
}

/// construct a Mat4 from a Mat2 initialising the 2x2 part and setting the 3rd and 4th rows to identity
impl<T> From<Mat2<T>> for Mat4<T> where T: Number {
    fn from(other: Mat2<T>) -> Mat4<T> {
        Mat4 {
            m: [
                other.m[0], other.m[1], T::zero(), T::zero(),
                other.m[2], other.m[3], T::zero(), T::zero(),
                T::zero(), T::zero(), T::one(), T::zero(),
                T::zero(), T::zero(), T::zero(), T::one()
            ]
        }
    }
}

/// construct a Mat4 from a Mat3 initialising the 3x3 part and setting the 4th row/column to identity
impl<T> From<Mat3<T>> for Mat4<T> where T: Number {
    fn from(other: Mat3<T>) -> Mat4<T> {
        Mat4 {
            m: [
                other.m[0], other.m[1], other.m[2], T::zero(),
                other.m[3], other.m[4], other.m[5], T::zero(),
                other.m[6], other.m[7], other.m[8], T::zero(),
                T::zero(), T::zero(), T::zero(), T::one()
            ]
        }
    }
}

/// construct a Mat4 from a Mat34 initialising the 3x4 part and setting the 4th row to identity
impl<T> From<Mat34<T>> for Mat4<T> where T: Number {
    fn from(other: Mat34<T>) -> Mat4<T> {
        Mat4 {
            m: [
                other.m[0], other.m[1], other.m[2], other.m[3],
                other.m[4], other.m[5], other.m[6], other.m[7],
                other.m[8], other.m[9], other.m[10], other.m[11],
                T::zero(), T::zero(), T::zero(), T::one()
            ]
        }
    }
}

/// construct a Mat4 from tuple of 16 elements
impl<T> From<(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)> for Mat4<T> where T: Number {
    fn from(other: (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Mat4<T> {
        Mat4 {
            m: [
                other.0, other.1, other.2, other.3, 
                other.4, other.5, other.6, other.7, 
                other.8, other.9, other.10, other.11, 
                other.12, other.13, other.14, other.15
            ]
        }
    }
}

/// constructs Mat4 from tuple of 4 4D row vectors
impl<T> From<(Vec4<T>, Vec4<T>, Vec4<T>, Vec4<T>)> for Mat4<T> where T: Number {
    fn from(other: (Vec4<T>, Vec4<T>, Vec4<T>, Vec4<T>)) -> Mat4<T> {
        Mat4 {
            m: [
                other.0.x, other.0.y, other.0.z, other.0.w,
                other.1.x, other.1.y, other.1.z, other.1.w,
                other.2.x, other.2.y, other.2.z, other.2.w,
                other.3.x, other.3.y, other.3.z, other.3.w,
            ]
        }
    }
}

/// constucts a mat4 rotation matrix from quaternion
impl<T> From<Quat<T>> for Mat4<T> where T: Float + SignedNumber + FloatOps<T> + NumberOps<T> + SignedNumberOps<T> {
    fn from(other: Quat<T>) -> Mat4<T> {
        Mat4::from(other.get_matrix())
    }
}

//
// Mat2 Mul
//

/// multiply 2x2 * 2x2 matrix
fn mul2x2<T: Number>(lhs: Mat2<T>, rhs: Mat2<T>) -> Mat2<T> {
    Mat2 {
        m: [
            lhs.m[0] * rhs.m[0] + lhs.m[1] * rhs.m[2],
            lhs.m[0] * rhs.m[1] + lhs.m[1] * rhs.m[3],
            lhs.m[2] * rhs.m[0] + lhs.m[3] * rhs.m[2],
            lhs.m[2] * rhs.m[1] + lhs.m[3] * rhs.m[3],
        ]
    }
}

impl<T> Mul<Self> for Mat2<T> where T: Number {
    type Output = Self;
    fn mul(self, rhs: Mat2<T>) -> Self::Output {
        mul2x2(self, rhs)
    }
}

impl<T> MulAssign<Self> for Mat2<T> where T: Number {
    fn mul_assign(&mut self, rhs: Mat2<T>) {
        *self = mul2x2(*self, rhs);
    }
}

impl<T> Mul<Vec2<T>> for Mat2<T> where T: Number {
    type Output = Vec2<T>;
    fn mul(self, rhs: Vec2<T>) -> Self::Output {
        Vec2 {
            x: self.m[0] * rhs.x + self.m[1] * rhs.y,
            y: self.m[2] * rhs.x + self.m[3] * rhs.y
        }
    }
}

//
// Mat3 Mul
//

/// multiply 3x3 * 3x3 matrix
fn mul3x3<T: Number>(lhs: Mat3<T>, rhs: Mat3<T>) -> Mat3<T> {
    Mat3 {
        m: [
            lhs.m[0] * rhs.m[0] + lhs.m[1] * rhs.m[3] + lhs.m[2] * rhs.m[6],
            lhs.m[0] * rhs.m[1] + lhs.m[1] * rhs.m[4] + lhs.m[2] * rhs.m[7],
            lhs.m[0] * rhs.m[2] + lhs.m[1] * rhs.m[5] + lhs.m[2] * rhs.m[8],

            lhs.m[3] * rhs.m[0] + lhs.m[4] * rhs.m[3] + lhs.m[5] * rhs.m[6],
            lhs.m[3] * rhs.m[1] + lhs.m[4] * rhs.m[4] + lhs.m[5] * rhs.m[7],
            lhs.m[3] * rhs.m[2] + lhs.m[4] * rhs.m[5] + lhs.m[5] * rhs.m[8],

            lhs.m[6] * rhs.m[0] + lhs.m[7] * rhs.m[3] + lhs.m[8] * rhs.m[6],
            lhs.m[6] * rhs.m[1] + lhs.m[7] * rhs.m[4] + lhs.m[8] * rhs.m[7],
            lhs.m[6] * rhs.m[2] + lhs.m[7] * rhs.m[5] + lhs.m[8] * rhs.m[8],
        ]
    }
}

impl<T> Mul<Self> for Mat3<T> where T: Number {
    type Output = Self;
    fn mul(self, rhs: Mat3<T>) -> Self::Output {
        mul3x3(self, rhs)
    }
}

impl<T> MulAssign<Self> for Mat3<T> where T: Number {
    fn mul_assign(&mut self, rhs: Mat3<T>) {
        *self = mul3x3(*self, rhs);
    }
}

impl<T> Mul<Vec3<T>> for Mat3<T> where T: Number {
    type Output = Vec3<T>;
    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        Vec3 {
            x: self.m[0] * rhs.x + self.m[1] * rhs.y + self.m[2] * rhs.z,
            y: self.m[3] * rhs.x + self.m[4] * rhs.y + self.m[5] * rhs.z,
            z: self.m[6] * rhs.x + self.m[7] * rhs.y + self.m[8] * rhs.z,
        }
    }
}

//
// Mat34 Mul
//

/// multiply 3x4 * 3x4 matrix
fn mul3x4<T: Number>(lhs: Mat34<T>, rhs: Mat34<T>) -> Mat34<T> {
    Mat34 {
        m: [
            lhs.m[0] * rhs.m[0] + lhs.m[1] * rhs.m[4] + lhs.m[2] * rhs.m[8],
            lhs.m[0] * rhs.m[1] + lhs.m[1] * rhs.m[5] + lhs.m[2] * rhs.m[9],
            lhs.m[0] * rhs.m[2] + lhs.m[1] * rhs.m[6] + lhs.m[2] * rhs.m[10],
            lhs.m[0] * rhs.m[3] + lhs.m[1] * rhs.m[7] + lhs.m[2] * rhs.m[11] + lhs.m[3],

            lhs.m[4] * rhs.m[0] + lhs.m[5] * rhs.m[4] + lhs.m[6] * rhs.m[8],
            lhs.m[4] * rhs.m[1] + lhs.m[5] * rhs.m[5] + lhs.m[6] * rhs.m[9],
            lhs.m[4] * rhs.m[2] + lhs.m[5] * rhs.m[6] + lhs.m[6] * rhs.m[10],
            lhs.m[4] * rhs.m[3] + lhs.m[5] * rhs.m[7] + lhs.m[6] * rhs.m[11] + lhs.m[7],

            lhs.m[8] * rhs.m[0] + lhs.m[9] * rhs.m[4] + lhs.m[10] * rhs.m[8],
            lhs.m[8] * rhs.m[1] + lhs.m[9] * rhs.m[5] + lhs.m[10] * rhs.m[9],
            lhs.m[8] * rhs.m[2] + lhs.m[9] * rhs.m[6] + lhs.m[10] * rhs.m[10],
            lhs.m[8] * rhs.m[3] + lhs.m[9] * rhs.m[7] + lhs.m[10] * rhs.m[11] + lhs.m[11],
        ]
    }
}

impl<T> Mul<Self> for Mat34<T> where T: Number {
    type Output = Self;
    fn mul(self, rhs: Mat34<T>) -> Self::Output {
        mul3x4(self, rhs)
    }
}

impl<T> MulAssign<Self> for Mat34<T> where T: Number {
    fn mul_assign(&mut self, rhs: Mat34<T>) {
        *self = mul3x4(*self, rhs)
    }
}

impl<T> Mul<Vec3<T>> for Mat34<T> where T: Number {
    type Output = Vec3<T>;
    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        Vec3 {
            x: self.m[0] * rhs.x + self.m[1] * rhs.y + self.m[2] * rhs.z + self.m[3],
            y: self.m[4] * rhs.x + self.m[5] * rhs.y + self.m[6] * rhs.z + self.m[7],
            z: self.m[8] * rhs.x + self.m[9] * rhs.y + self.m[10] * rhs.z + self.m[11]
        }
    }
}

/// multiples vector with implicit 4th row in matrix 0,0,0,1
impl<T> Mul<Vec4<T>> for Mat34<T> where T: Number {
    type Output = Vec4<T>;
    fn mul(self, rhs: Vec4<T>) -> Self::Output {
        Vec4 {
            x: self.m[0] * rhs.x + self.m[1] * rhs.y + self.m[2] * rhs.z + self.m[3] * rhs.w,
            y: self.m[4] * rhs.x + self.m[5] * rhs.y + self.m[6] * rhs.z + self.m[7] * rhs.w,
            z: self.m[8] * rhs.x + self.m[9] * rhs.y + self.m[10] * rhs.z + self.m[11] * rhs.w,
            w: rhs.w,
        }
    }
}

//
// Mat4 Mul
//

/// multiply 4x4 * 4x4 matrix
fn mul4x4<T: Number>(lhs: Mat4<T>, rhs: Mat4<T>) -> Mat4<T> {
    Mat4 {
        m: [
            lhs.m[0] * rhs.m[0] + lhs.m[1] * rhs.m[4] + lhs.m[2] * rhs.m[8] + lhs.m[3] * rhs.m[12],
            lhs.m[0] * rhs.m[1] + lhs.m[1] * rhs.m[5] + lhs.m[2] * rhs.m[9] + lhs.m[3] * rhs.m[13],
            lhs.m[0] * rhs.m[2] + lhs.m[1] * rhs.m[6] + lhs.m[2] * rhs.m[10] + lhs.m[3] * rhs.m[14],
            lhs.m[0] * rhs.m[3] + lhs.m[1] * rhs.m[7] + lhs.m[2] * rhs.m[11] + lhs.m[3] * rhs.m[15],

            lhs.m[4] * rhs.m[0] + lhs.m[5] * rhs.m[4] + lhs.m[6] * rhs.m[8] + lhs.m[7] * rhs.m[12],
            lhs.m[4] * rhs.m[1] + lhs.m[5] * rhs.m[5] + lhs.m[6] * rhs.m[9] + lhs.m[7] * rhs.m[13],
            lhs.m[4] * rhs.m[2] + lhs.m[5] * rhs.m[6] + lhs.m[6] * rhs.m[10] + lhs.m[7] * rhs.m[14],
            lhs.m[4] * rhs.m[3] + lhs.m[5] * rhs.m[7] + lhs.m[6] * rhs.m[11] + lhs.m[7] * rhs.m[15],

            lhs.m[8] * rhs.m[0] + lhs.m[9] * rhs.m[4] + lhs.m[10] * rhs.m[8] + lhs.m[11] * rhs.m[12],
            lhs.m[8] * rhs.m[1] + lhs.m[9] * rhs.m[5] + lhs.m[10] * rhs.m[9] + lhs.m[11] * rhs.m[13],
            lhs.m[8] * rhs.m[2] + lhs.m[9] * rhs.m[6] + lhs.m[10] * rhs.m[10] + lhs.m[11] * rhs.m[14],
            lhs.m[8] * rhs.m[3] + lhs.m[9] * rhs.m[7] + lhs.m[10] * rhs.m[11] + lhs.m[11] * rhs.m[15],

            lhs.m[12] * rhs.m[0] + lhs.m[13] * rhs.m[4] + lhs.m[14] * rhs.m[8] + lhs.m[15] * rhs.m[12],
            lhs.m[12] * rhs.m[1] + lhs.m[13] * rhs.m[5] + lhs.m[14] * rhs.m[9] + lhs.m[15] * rhs.m[13],
            lhs.m[12] * rhs.m[2] + lhs.m[13] * rhs.m[6] + lhs.m[14] * rhs.m[10] + lhs.m[15] * rhs.m[14],
            lhs.m[12] * rhs.m[3] + lhs.m[13] * rhs.m[7] + lhs.m[14] * rhs.m[11] + lhs.m[15] * rhs.m[15],
        ]
    }
}

impl<T> Mul<Self> for Mat4<T> where T: Number {
    type Output = Self;
    fn mul(self, rhs: Mat4<T>) -> Self::Output {
        mul4x4(self, rhs)
    }
}

impl<T> MulAssign<Self> for Mat4<T> where T: Number {
    fn mul_assign(&mut self, rhs: Mat4<T>) {
        *self = mul4x4(*self, rhs);
    }
}

impl<T> Mul<Vec4<T>> for Mat4<T> where T: Number {
    type Output = Vec4<T>;
    fn mul(self, rhs: Vec4<T>) -> Self::Output {
        Vec4 {
            x: self.m[0] * rhs.x + self.m[1] * rhs.y + self.m[2] * rhs.z + self.m[3] * rhs.w,
            y: self.m[4] * rhs.x + self.m[5] * rhs.y + self.m[6] * rhs.z + self.m[7] * rhs.w,
            z: self.m[8] * rhs.x + self.m[9] * rhs.y + self.m[10] * rhs.z + self.m[11] * rhs.w,
            w: self.m[12] * rhs.x + self.m[13] * rhs.y + self.m[14] * rhs.z + self.m[15] * rhs.w,
        }
    }
}

/// performs multiplication on vec3 with implicit w = 1.0, returning Vec3 which has been divided by w
impl<T> Mul<Vec3<T>> for Mat4<T> where T: Number {
    type Output = Vec3<T>;
    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        let w = self.m[12] * rhs.x + self.m[13] * rhs.y + self.m[14] * rhs.z + self.m[15];
        Vec3 {
            x: self.m[0] * rhs.x + self.m[1] * rhs.y + self.m[2] * rhs.z + self.m[3],
            y: self.m[4] * rhs.x + self.m[5] * rhs.y + self.m[6] * rhs.z + self.m[7],
            z: self.m[8] * rhs.x + self.m[9] * rhs.y + self.m[10] * rhs.z + self.m[11],
        } / w
    }
}

// mat34 implicit last row
// 12 13 14 15
// 00 00 00 01

/// multiply 3x4 * 4x4 matrix
fn mul3x4_4x4<T: Number>(lhs: Mat34<T>, rhs: Mat4<T>) -> Mat4<T> {
    Mat4 {
        m: [
            lhs.m[0] * rhs.m[0] + lhs.m[1] * rhs.m[4] + lhs.m[2] * rhs.m[8] + lhs.m[3] * rhs.m[12],
            lhs.m[0] * rhs.m[1] + lhs.m[1] * rhs.m[5] + lhs.m[2] * rhs.m[9] + lhs.m[3] * rhs.m[13],
            lhs.m[0] * rhs.m[2] + lhs.m[1] * rhs.m[6] + lhs.m[2] * rhs.m[10] + lhs.m[3] * rhs.m[14],
            lhs.m[0] * rhs.m[3] + lhs.m[1] * rhs.m[7] + lhs.m[2] * rhs.m[11] + lhs.m[3] * rhs.m[15],

            lhs.m[4] * rhs.m[0] + lhs.m[5] * rhs.m[4] + lhs.m[6] * rhs.m[8] + lhs.m[7] * rhs.m[12],
            lhs.m[4] * rhs.m[1] + lhs.m[5] * rhs.m[5] + lhs.m[6] * rhs.m[9] + lhs.m[7] * rhs.m[13],
            lhs.m[4] * rhs.m[2] + lhs.m[5] * rhs.m[6] + lhs.m[6] * rhs.m[10] + lhs.m[7] * rhs.m[14],
            lhs.m[4] * rhs.m[3] + lhs.m[5] * rhs.m[7] + lhs.m[6] * rhs.m[11] + lhs.m[7] * rhs.m[15],

            lhs.m[8] * rhs.m[0] + lhs.m[9] * rhs.m[4] + lhs.m[10] * rhs.m[8] + lhs.m[11] * rhs.m[12],
            lhs.m[8] * rhs.m[1] + lhs.m[9] * rhs.m[5] + lhs.m[10] * rhs.m[9] + lhs.m[11] * rhs.m[13],
            lhs.m[8] * rhs.m[2] + lhs.m[9] * rhs.m[6] + lhs.m[10] * rhs.m[10] + lhs.m[11] * rhs.m[14],
            lhs.m[8] * rhs.m[3] + lhs.m[9] * rhs.m[7] + lhs.m[10] * rhs.m[11] + lhs.m[11] * rhs.m[15],

            rhs.m[12],
            rhs.m[13],
            rhs.m[14],
            rhs.m[15],
        ]
    }
}

impl<T> Mul<Mat4<T>> for Mat34<T> where T: Number {
    type Output = Mat4<T>;
    fn mul(self, rhs: Mat4<T>) -> Self::Output {
        mul3x4_4x4(self, rhs)
    }
}

// mat34 implicit last row
// 12 13 14 15
// 00 00 00 01

/// multiply 4x4 * 3x4 matrix
fn mul4x4_3x4<T: Number>(lhs: Mat4<T>, rhs: Mat34<T>) -> Mat4<T> {
    Mat4 {
        m: [
            lhs.m[0] * rhs.m[0] + lhs.m[1] * rhs.m[4] + lhs.m[2] * rhs.m[8],
            lhs.m[0] * rhs.m[1] + lhs.m[1] * rhs.m[5] + lhs.m[2] * rhs.m[9],
            lhs.m[0] * rhs.m[2] + lhs.m[1] * rhs.m[6] + lhs.m[2] * rhs.m[10],
            lhs.m[0] * rhs.m[3] + lhs.m[1] * rhs.m[7] + lhs.m[2] * rhs.m[11] + lhs.m[3],

            lhs.m[4] * rhs.m[0] + lhs.m[5] * rhs.m[4] + lhs.m[6] * rhs.m[8],
            lhs.m[4] * rhs.m[1] + lhs.m[5] * rhs.m[5] + lhs.m[6] * rhs.m[9],
            lhs.m[4] * rhs.m[2] + lhs.m[5] * rhs.m[6] + lhs.m[6] * rhs.m[10],
            lhs.m[4] * rhs.m[3] + lhs.m[5] * rhs.m[7] + lhs.m[6] * rhs.m[11] + lhs.m[7],

            lhs.m[8] * rhs.m[0] + lhs.m[9] * rhs.m[4] + lhs.m[10] * rhs.m[8],
            lhs.m[8] * rhs.m[1] + lhs.m[9] * rhs.m[5] + lhs.m[10] * rhs.m[9],
            lhs.m[8] * rhs.m[2] + lhs.m[9] * rhs.m[6] + lhs.m[10] * rhs.m[10],
            lhs.m[8] * rhs.m[3] + lhs.m[9] * rhs.m[7] + lhs.m[10] * rhs.m[11] + lhs.m[11],

            lhs.m[12] * rhs.m[0] + lhs.m[10] * rhs.m[4] + lhs.m[14] * rhs.m[8],
            lhs.m[12] * rhs.m[1] + lhs.m[10] * rhs.m[5] + lhs.m[14] * rhs.m[9],
            lhs.m[12] * rhs.m[2] + lhs.m[10] * rhs.m[6] + lhs.m[14] * rhs.m[10],
            lhs.m[12] * rhs.m[3] + lhs.m[10] * rhs.m[7] + lhs.m[14] * rhs.m[11] + lhs.m[15],
        ]
    }
}

impl<T> Mul<Mat34<T>> for Mat4<T> where T: Number {
    type Output = Self;
    fn mul(self, rhs: Mat34<T>) -> Self::Output {
        mul4x4_3x4(self, rhs)
    }
}

impl<T> MulAssign<Mat34<T>> for Mat4<T> where T: Number {
    fn mul_assign(&mut self, rhs: Mat34<T>) {
        *self = mul4x4_3x4(*self, rhs)
    }
}

/// base matrix trait for arithmetic ops
pub trait MatN<T: Number, V: VecN<T>>: 
    Sized + Display + Copy + Clone +
    Mul<V, Output=V> + Mul<Self, Output=Self> + MulAssign<Self> + 
{
}

impl<T> MatN<T, Vec2<T>> for Mat2<T> where T: Number {}
impl<T> MatN<T, Vec3<T>> for Mat3<T> where T: Number {}
impl<T> MatN<T, Vec3<T>> for Mat34<T> where T: Number {}
impl<T> MatN<T, Vec4<T>> for Mat4<T> where T: Number {}
impl<T> MatN<T, Vec3<T>> for Mat4<T> where T: Number {}


/// trait for minimum of 4 column matrices to create translation
pub trait MatTranslate<V> {
    /// create a translation matrix from 3-dimensional vector `t`
    fn from_translation(t: V) -> Self;
}

impl<T> MatTranslate<Vec3<T>> for Mat4<T> where T: Number {
    fn from_translation(t: Vec3<T>) -> Self {
        let mut m = Mat4::identity();
        m.set_column(3, Vec4::from(t));
        m.m[15] = T::one();
        m
    }
}

impl<T> MatTranslate<Vec3<T>> for Mat34<T> where T: Number {
    fn from_translation(t: Vec3<T>) -> Self {
        let mut m = Mat34::identity();
        m.set_column(3, t);
        m
    }
}

/// trait for all matrices to cerate a scaling matrix
pub trait MatScale<V> {
    /// create a scale matrix from n-dimensional vector `t`
    fn from_scale(t: V) -> Self;
}

impl<T> MatScale<Vec3<T>> for Mat4<T> where T: Number {
    fn from_scale(t: Vec3<T>) -> Self {
        let mut m = Mat4::identity();
        m.set(0, 0, t.x);
        m.set(1, 1, t.y);
        m.set(2, 2, t.z);
        m
    }
}

impl<T> MatScale<Vec3<T>> for Mat34<T> where T: Number {
    fn from_scale(t: Vec3<T>) -> Self {
        let mut m = Mat34::identity();
        m.set(0, 0, t.x);
        m.set(1, 1, t.y);
        m.set(2, 2, t.z);
        m
    }
}

impl<T> MatScale<Vec3<T>> for Mat3<T> where T: Number {
    fn from_scale(t: Vec3<T>) -> Self {
        let mut m = Mat3::identity();
        m.set(0, 0, t.x);
        m.set(1, 1, t.y);
        m.set(2, 2, t.z);
        m
    }
}

impl<T> MatScale<Vec2<T>> for Mat2<T> where T: Number {
    fn from_scale(t: Vec2<T>) -> Self {
        let mut m = Mat2::identity();
        m.set(0, 0, t.x);
        m.set(1, 1, t.y);
        m
    }
}

/// trait for minimum of 2x2 matrices applying rotation in z-axis
pub trait MatRotate2D<T> {
    /// create rotation about the z axis by `theta` radians
    fn from_z_rotation(theta: T) -> Self;
}

impl<T> MatRotate2D<T> for Mat2<T> where T: Float + FloatOps<T>  {
    fn from_z_rotation(theta: T) -> Self {
        let mut m = Mat2::identity();
        let cos_theta = T::cos(theta);
        let sin_theta = T::sin(theta);
        m.set(0, 0, cos_theta);
        m.set(0, 1, -sin_theta);
        m.set(1, 0, sin_theta);
        m.set(1, 1, cos_theta);
        m
    }
}

impl<T> MatRotate2D<T> for Mat3<T> where T: Float + FloatOps<T>  {
    fn from_z_rotation(theta: T) -> Self {
        let mut m = Mat3::identity();
        let cos_theta = T::cos(theta);
        let sin_theta = T::sin(theta);
        m.set(0, 0, cos_theta);
        m.set(0, 1, -sin_theta);
        m.set(1, 0, sin_theta);
        m.set(1, 1, cos_theta);
        m
    }
}

impl<T> MatRotate2D<T> for Mat4<T> where T: Float + FloatOps<T>  {
    fn from_z_rotation(theta: T) -> Self {
        let mut m = Mat4::identity();
        let cos_theta = T::cos(theta);
        let sin_theta = T::sin(theta);
        m.set(0, 0, cos_theta);
        m.set(0, 1, -sin_theta);
        m.set(1, 0, sin_theta);
        m.set(1, 1, cos_theta);
        m
    }
}

impl<T> MatRotate2D<T> for Mat34<T> where T: Float + FloatOps<T> {
    fn from_z_rotation(theta: T) -> Self {
        let mut m = Mat34::identity();
        let cos_theta = T::cos(theta);
        let sin_theta = T::sin(theta);
        m.set(0, 0, cos_theta);
        m.set(0, 1, -sin_theta);
        m.set(1, 0, sin_theta);
        m.set(1, 1, cos_theta);
        m
    }
}

/// given the normalised vector `n`, constructs an orthonormal basis returned as tuple
pub fn get_orthonormal_basis_hughes_moeller<T: Float + SignedNumberOps<T> + FloatOps<T>>(n: Vec3<T>) -> (Vec3<T>, Vec3<T>) {
    // choose a vector orthogonal to n as the direction of b2.
    let b2 = if T::abs(n.x) > T::abs(n.z) {
        Vec3::new(-n.y, n.x, T::zero())
    }
    else {
        Vec3::new(T::zero(), -n.z, n.y)
    };
    
    // normalise b2
    let b2 = b2 * T::rsqrt(Vec3::dot(b2, b2));
    
    // construct b1 using cross product
    let b1 = Vec3::cross(b2, n);

    (b1, b2)
}

/// given the normalised vector `n` construct an orthonormal basis without sqrt..
pub fn get_orthonormal_basis_frisvad<T: Float + SignedNumberOps<T> + FloatOps<T> + From<f64>>(n: Vec3<T>) -> (Vec3<T>, Vec3<T>) {
    let epsilon = T::from(-0.99999999);
    if n.z < epsilon {
        (-Vec3::unit_y(), -Vec3::unit_x())
    }
    else {
        let a = T::one()/(T::one() + n.z);
        let b = -n.x * n.y * a;
        (Vec3::new(T::one() - n.x * n.x * a, b, -n.x), Vec3::new(b, T::one() - n.y * n.y * a, -n.y))
    }
}


/// trait for minimum of 3x3 matrices applying rotation to x, y or aribtrary 3D axes
pub trait MatRotate3D<T, V> {
    /// create rotation about the x axis by `theta` radians
    fn from_x_rotation(theta: T) -> Self;
    /// create rotation about the y axis by `theta` radians
    fn from_y_rotation(theta: T) -> Self;
    /// create rotation about the abitrary `axis` by `theta` radians
    fn from_rotation(axis: V, theta: T) -> Self;
    /// create an othonormal basis from the `normal` vector
    fn from_orthonormal_basis(normal: Vec3<T>) -> Self;
}

impl<T> MatRotate3D<T, Vec3<T>> for Mat3<T> where T: Float + FloatOps<T> + SignedNumberOps<T> {
    fn from_x_rotation(theta: T) -> Self {
        let mut m = Mat3::identity();
        let cos_theta = T::cos(theta);
        let sin_theta = T::sin(theta);
        m.set(1, 1, cos_theta);
        m.set(1, 2, -sin_theta);
        m.set(2, 1, sin_theta);
        m.set(2, 2, cos_theta);
        m
    }

    fn from_y_rotation(theta: T) -> Self {
        let mut m = Mat3::identity();
        let cos_theta = T::cos(theta);
        let sin_theta = T::sin(theta);
        m.set(0, 0, cos_theta);
        m.set(0, 2, sin_theta);
        m.set(2, 0, -sin_theta);
        m.set(2, 2, cos_theta);
        m
    }

    fn from_rotation(axis: Vec3<T>, theta: T) -> Self {
        let cos_theta = T::cos(theta);
        let sin_theta = T::sin(theta);
        let inv_cos_theta = T::one() - cos_theta;
        Mat3::from((
            Vec3::new(
                inv_cos_theta * axis.x * axis.x + cos_theta,
                inv_cos_theta * axis.x * axis.y - sin_theta * axis.z,
                inv_cos_theta * axis.x * axis.z + sin_theta * axis.y
            ),
            Vec3::new(
                inv_cos_theta * axis.x * axis.y + sin_theta * axis.z,
                inv_cos_theta * axis.y * axis.y + cos_theta,
                inv_cos_theta * axis.y * axis.z - sin_theta * axis.x
            ),
            Vec3::new(
                inv_cos_theta * axis.x * axis.z - sin_theta * axis.y,
                inv_cos_theta * axis.y * axis.z + sin_theta * axis.x,
                inv_cos_theta * axis.z * axis.z + cos_theta
            )
        ))
    }

    fn from_orthonormal_basis(normal: Vec3<T>) -> Self {
        let (b1, b2) = get_orthonormal_basis_hughes_moeller(normal);
        Mat3::from((
            b1,
            normal,
            b2
        ))
    }
}

impl<T> MatRotate3D<T, Vec3<T>> for Mat34<T> where T: Float + FloatOps<T> + SignedNumberOps<T> {
    fn from_x_rotation(theta: T) -> Self {
        Mat34::from(Mat3::from_x_rotation(theta))
    }

    fn from_y_rotation(theta: T) -> Self {
        Mat34::from(Mat3::from_y_rotation(theta))
    }

    fn from_rotation(axis: Vec3<T>, theta: T) -> Self {
        Mat34::from(Mat3::from_rotation(axis, theta))
    }

    fn from_orthonormal_basis(normal: Vec3<T>) -> Self {
        Mat34::from(Mat3::from_orthonormal_basis(normal))
    }
}

impl<T> MatRotate3D<T, Vec3<T>> for Mat4<T> where T: Float + FloatOps<T> + SignedNumberOps<T> {
    fn from_x_rotation(theta: T) -> Self {
        Mat4::from(Mat3::from_x_rotation(theta))
    }

    fn from_y_rotation(theta: T) -> Self {
        Mat4::from(Mat3::from_y_rotation(theta))
    }

    fn from_rotation(axis: Vec3<T>, theta: T) -> Self {
        Mat4::from(Mat3::from_rotation(axis, theta))
    }

    fn from_orthonormal_basis(normal: Vec3<T>) -> Self {
        Mat4::from(Mat3::from_orthonormal_basis(normal))
    }
}

/// trait for square matrices to compute determinant
pub trait MatDeterminant<T> {
    /// return the determinant of the matrix as scalar `T`
    fn determinant(&self) -> T;
}

impl<T> MatDeterminant<T> for Mat2<T> where T: Number {
    fn determinant(&self) -> T {
        self.m[0] * self.m[3] - self.m[1] * self.m[2]
    }
}

impl<T> MatDeterminant<T> for Mat3<T> where T: Number {
    fn determinant(&self) -> T {
        self.m[0] * (self.m[4] * self.m[8] - self.m[5] * self.m[7]) -
        self.m[1] * (self.m[3] * self.m[8] - self.m[5] * self.m[6]) +
        self.m[2] * (self.m[3] * self.m[7] - self.m[4] * self.m[6])
    }
}

/// returns the 4x4 determinant using laplace expansion theorum
#[allow(clippy::zero_prefixed_literal)] 
impl<T> MatDeterminant<T> for Mat4<T> where T: Number {
    fn determinant(&self) -> T {
        let s0 = (self.m[00] * self.m[05]) - (self.m[01] * self.m[04]);
        let s1 = (self.m[00] * self.m[06]) - (self.m[02] * self.m[04]);
        let s2 = (self.m[00] * self.m[07]) - (self.m[03] * self.m[04]);
        let s3 = (self.m[01] * self.m[06]) - (self.m[02] * self.m[05]);
        let s4 = (self.m[01] * self.m[07]) - (self.m[03] * self.m[05]);
        let s5 = (self.m[02] * self.m[07]) - (self.m[03] * self.m[06]);
        let c5 = (self.m[10] * self.m[15]) - (self.m[11] * self.m[14]);
        let c4 = (self.m[09] * self.m[15]) - (self.m[11] * self.m[13]);
        let c3 = (self.m[09] * self.m[14]) - (self.m[10] * self.m[13]);
        let c2 = (self.m[08] * self.m[15]) - (self.m[11] * self.m[12]);
        let c1 = (self.m[08] * self.m[14]) - (self.m[10] * self.m[12]);
        let c0 = (self.m[08] * self.m[13]) - (self.m[09] * self.m[12]);
        (s0 * c5) - (s1 * c4) + (s2 * c3) + (s3 * c2) - (s4 * c1) + (s5 * c0)
    }
}

/// trait for all kinds of matrices to calculate inverse
pub trait MatInverse<T> {
    /// returns the inverse of the matrix
    fn inverse(&self) -> Self;
}

impl<T> MatInverse<T> for Mat2<T> where T: SignedNumber {
    fn inverse(&self) -> Self {
        let det = self.determinant();
        let inv_det = T::one()/det;
        Mat2 { 
            m: [
                inv_det * self.m[3], inv_det * -self.m[1],
                inv_det * -self.m[2], inv_det * self.m[0],
            ] 
        }
    }
}

impl<T> MatInverse<T> for Mat3<T> where T: SignedNumber {
    fn inverse(&self) -> Self {
        let det = self.determinant();
        let inv_det = T::one() / det;
        Mat3 {
            m: [
                (self.m[4] * self.m[8] - self.m[5] * self.m[7]) * inv_det,
                -(self.m[1] * self.m[8] - self.m[2] * self.m[7]) * inv_det,
                (self.m[1] * self.m[5] - self.m[2] * self.m[4]) * inv_det,

                -(self.m[3] * self.m[8] - self.m[5] * self.m[6]) * inv_det,
                (self.m[0] * self.m[8] - self.m[2] * self.m[6]) * inv_det,
                -(self.m[0] * self.m[5] - self.m[2] * self.m[3]) * inv_det,

                (self.m[3] * self.m[7] - self.m[4] * self.m[6]) * inv_det,
                -(self.m[0] * self.m[7] - self.m[1] * self.m[6]) * inv_det,
                (self.m[0] * self.m[4] - self.m[1] * self.m[3]) * inv_det
            ]
        }
    }
}

impl<T> MatInverse<T> for Mat34<T> where T: SignedNumber {
    fn inverse(&self) -> Self {        
        let m3_inv = Mat3::<T>::from(*self).inverse();
        let mut m34 = Mat34::<T>::from(m3_inv);
        let t = Vec3::<T>::from((-self.m[3], -self.m[7], -self.m[11]));
        let inv_t = Vec3::<T> {
            x: t.x * m3_inv.m[0] + t.y * m3_inv.m[1] + t.z * m3_inv.m[2],
            y: t.x * m3_inv.m[3] + t.y * m3_inv.m[4] + t.z * m3_inv.m[5],
            z: t.x * m3_inv.m[6] + t.y * m3_inv.m[7] + t.z * m3_inv.m[8],
        };
        m34.set_column(3, inv_t);
        m34
    }
}

#[allow(clippy::zero_prefixed_literal)] 
impl<T> MatInverse<T> for Mat4<T> where T: SignedNumber {
    fn inverse(&self) -> Self {
        let s0 = (self.m[00] * self.m[05]) - (self.m[01] * self.m[04]);
        let s1 = (self.m[00] * self.m[06]) - (self.m[02] * self.m[04]);
        let s2 = (self.m[00] * self.m[07]) - (self.m[03] * self.m[04]);
        let s3 = (self.m[01] * self.m[06]) - (self.m[02] * self.m[05]);
        let s4 = (self.m[01] * self.m[07]) - (self.m[03] * self.m[05]);
        let s5 = (self.m[02] * self.m[07]) - (self.m[03] * self.m[06]);
        let c5 = (self.m[10] * self.m[15]) - (self.m[11] * self.m[14]);
        let c4 = (self.m[09] * self.m[15]) - (self.m[11] * self.m[13]);
        let c3 = (self.m[09] * self.m[14]) - (self.m[10] * self.m[13]);
        let c2 = (self.m[08] * self.m[15]) - (self.m[11] * self.m[12]);
        let c1 = (self.m[08] * self.m[14]) - (self.m[10] * self.m[12]);
        let c0 = (self.m[08] * self.m[13]) - (self.m[09] * self.m[12]);
        let det = (s0 * c5) - (s1 * c4) + (s2 * c3) + (s3 * c2) - (s4 * c1) + (s5 * c0);
        let inv_det = T::one() / det;
        Mat4 {
            m: [
                (self.m[5] * c5 - self.m[6] * c4 + self.m[7] * c3) * inv_det,
                -(self.m[1] * c5 - self.m[2] * c4 + self.m[3] * c3) * inv_det,
                (self.m[13] * s5 - self.m[14] * s4 + self.m[15] * s3) * inv_det,
                -(self.m[9] * s5 - self.m[10] * s4 + self.m[11] * s3) * inv_det,
                -(self.m[4] * c5 - self.m[6] * c2 + self.m[7] * c1) * inv_det,
                (self.m[0] * c5 - self.m[2] * c2 + self.m[3] * c1) * inv_det,
                -(self.m[12] * s5 - self.m[14] * s2 + self.m[15] * s1) * inv_det,
                (self.m[8] * s5 - self.m[10] * s2 + self.m[11] * s1) * inv_det,
                (self.m[4] * c4 - self.m[5] * c2 + self.m[7] * c0) * inv_det,
                 -(self.m[0] * c4 - self.m[1] * c2 + self.m[3] * c0) * inv_det,
                (self.m[12] * s4 - self.m[13] * s2 + self.m[15] * s0) * inv_det,
                 -(self.m[8] * s4 - self.m[9] * s2 + self.m[11] * s0) * inv_det,
                 -(self.m[4] * c3 - self.m[5] * c1 + self.m[6] * c0) * inv_det,
                (self.m[0] * c3 - self.m[1] * c1 + self.m[2] * c0) * inv_det,
                 -(self.m[12] * s3 - self.m[13] * s1 + self.m[14] * s0) * inv_det,
                (self.m[8] * s3 - self.m[9] * s1 + self.m[10] * s0) * inv_det,
            ]
        }
    }
}

/// trait for matrix transpose
pub trait MatTranspose<T, Other> {
    fn transpose(&self) -> Other;
}

impl<T> MatTranspose<T, Self> for Mat2<T> where T: Number {
    fn transpose(&self) -> Self {
        let mut t = *self;
        for r in 0..t.get_num_rows() as u32 {
            for c in 0..t.get_num_columns() as u32 {
                t.set(c, r, self.at(r, c));
            }
        }
        t
    }
}

impl<T> MatTranspose<T, Self> for Mat3<T> where T: Number {
    fn transpose(&self) -> Self {
        let mut t = *self;
        for r in 0..t.get_num_rows() as u32 {
            for c in 0..t.get_num_columns() as u32 {
                t.set(c, r, self.at(r, c));
            }
        }
        t
    }
}

impl<T> MatTranspose<T, Self> for Mat4<T> where T: Number {
    fn transpose(&self) -> Self {
        let mut t = *self;
        for r in 0..t.get_num_rows() as u32 {
            for c in 0..t.get_num_columns() as u32 {
                t.set(c, r, self.at(r, c));
            }
        }
        t
    }
}

impl<T> MatTranspose<T, Mat43<T>> for Mat34<T> where T: Number {
    fn transpose(&self) -> Mat43<T> {
        let mut t = Mat43 {
            m: [T::zero(); 12]
        };
        for r in 0..3 {
            for c in 0..4 {
                let i = c * 3 + r;
                let v = self.at(r as u32, c as u32);
                t.m[i] = v;
            }
        }
        t
    }
}

impl<T> MatTranspose<T, Mat34<T>> for Mat43<T> where T: Number {
    fn transpose(&self) -> Mat34<T> {
        let mut t = Mat34 {
            m: [T::zero(); 12]
        };
        for r in 0..4 {
            for c in 0..3 {
                let i = r * 3 + c;
                t.set(c, r, self.m[i as usize]);
            }
        }
        t
    }
}

/// trait for 4x4 projection matrices
pub trait MatProjection<T> {
    /// returns 6 frustum planes as `Vec4`'s in the form `.xyz = normal, .w = plane distance`
    fn get_frustum_planes(&self) -> [Vec4<T>; 6];
    /// returns 8 points which are the corners of the frustum first 4 near, second 4 far
    fn get_frustum_corners(&self) -> [Vec3<T>; 8];
    /// returns an orthogrpahic projection matrix defined by `left`, `right`, `top`, `bottom` edges and `near` - `far` depth range
    fn create_ortho_matrix(left: T, right: T, bottom: T, top: T, near: T, far: T) -> Self;
    /// returns a perespective projection matrix (left hand coordinate system with y-up) from `fov` (radians), `aspect` ratio and `near` - `far` depth
    fn create_perspective_projection_lh_yup(fov: T, aspect: T, near: T, far: T) -> Self; 
    /// returns a perespective projection matrix (right hand coordinate system with y-up) from `fov` (radians), `aspect` ratio and `near` - `far` depth
    fn create_perspective_projection_rh_yup(fov: T, aspect: T, near: T, far: T) -> Self;
}

/// internal utility function to extract a plane in the form `.xyz=normal, w=constant` from plane corners (of a frustum)
fn plane_from_vectors<T: Float + FloatOps<T>>(plane_vectors: &[Vec3<T>; 18], offset: usize) -> Vec4<T> {
    let v1 = super::normalize(plane_vectors[offset + 1] - plane_vectors[offset]);
    let v2 = super::normalize(plane_vectors[offset + 2] - plane_vectors[offset]);
    let pn = super::cross(v1, v2);
    let pd = super::plane_distance(plane_vectors[offset], pn);
    Vec4::from((pn, pd))
}

fn create_perspective_matrix_internal_lh<T: Float + FloatOps<T>>(left: T, right: T, bottom: T, top: T, near: T, far: T) -> Mat4<T> {
    Mat4::from((
        Vec4::new((T::two() * near) / (right - left), T::zero(), (right + left) / (right - left), T::zero()),
        Vec4::new(T::zero(), (T::two() * near) / (top - bottom), (top + bottom) / (top - bottom), T::zero()),
        Vec4::new(T::zero(), T::zero(), (-far - near) / (far - near), (-(T::two() * near) * far) / (far - near)),
        Vec4::new(T::zero(), T::zero(), T::minus_one(), T::zero())
    ))
}

fn create_perspective_matrix_internal_rh<T: Float + FloatOps<T>>(left: T, right: T, bottom: T, top: T, near: T, far: T) -> Mat4<T> {
    Mat4::from((
        Vec4::new((T::two() * near) / (right - left), T::zero(), (right + left) / (right - left), T::zero()),
        Vec4::new(T::zero(), (T::two() * near) / (top - bottom), (top + bottom) / (top - bottom), T::zero()),
        Vec4::new(T::zero(), T::zero(), (-far - near) / (far - near), (-(T::two() * near) * far) / (far - near)),
        Vec4::new(T::zero(), T::zero(), T::one(), T::zero())
    ))
}

impl<T> MatProjection<T> for Mat4<T> where T: Float + FloatOps<T>, Vec3<T>: FloatOps<T> {
    fn get_frustum_planes(&self) -> [Vec4<T>; 6] {
        // unproject matrix to get frustum corners grouped as 4 near, 4 far.
        let ndc_coords = [
            Vec2::<T>::new(T::zero(), T::one()),
            Vec2::<T>::new(T::one(), T::one()),
            Vec2::<T>::new(T::zero(), T::zero()),
            Vec2::<T>::new(T::one(), T::zero()),
        ];

        // construct corner points
        let corners = [[
            super::unproject_sc(Vec3::from((ndc_coords[0], T::zero())), *self, Vec2::one()),
            super::unproject_sc(Vec3::from((ndc_coords[1], T::zero())), *self, Vec2::one()),
            super::unproject_sc(Vec3::from((ndc_coords[2], T::zero())), *self, Vec2::one()),
            super::unproject_sc(Vec3::from((ndc_coords[3], T::zero())), *self, Vec2::one()),
        ],
        [
            super::unproject_sc(Vec3::from((ndc_coords[0], T::one())), *self, Vec2::one()),
            super::unproject_sc(Vec3::from((ndc_coords[1], T::one())), *self, Vec2::one()),
            super::unproject_sc(Vec3::from((ndc_coords[2], T::one())), *self, Vec2::one()),
            super::unproject_sc(Vec3::from((ndc_coords[3], T::one())), *self, Vec2::one()),
        ]];

        // construct vectors to obtain normals
        let plane_vectors = [
            corners[0][0], corners[1][0], corners[0][2], // left
            corners[0][0], corners[0][1], corners[1][0], // top
            corners[0][1], corners[0][3], corners[1][1], // right
            corners[0][2], corners[1][2], corners[0][3], // bottom
            corners[0][0], corners[0][2], corners[0][1], // near
            corners[1][0], corners[1][1], corners[1][2]  // far
        ];

        // return array of planes
        [
            plane_from_vectors(&plane_vectors, 0),
            plane_from_vectors(&plane_vectors, 3),
            plane_from_vectors(&plane_vectors, 6),
            plane_from_vectors(&plane_vectors, 9),
            plane_from_vectors(&plane_vectors, 12),
            plane_from_vectors(&plane_vectors, 15),
        ]
    }

    fn get_frustum_corners(&self) -> [Vec3<T>; 8] {
        // unproject matrix to get frustum corners grouped as 4 near, 4 far.
        let ndc_coords = [
            Vec2::<T>::new(T::zero(), T::one()),
            Vec2::<T>::new(T::one(), T::one()),
            Vec2::<T>::new(T::zero(), T::zero()),
            Vec2::<T>::new(T::one(), T::zero()),
        ];

        // construct corner points
        [
            super::unproject_sc(Vec3::from((ndc_coords[0], T::zero())), *self, Vec2::one()),
            super::unproject_sc(Vec3::from((ndc_coords[1], T::zero())), *self, Vec2::one()),
            super::unproject_sc(Vec3::from((ndc_coords[2], T::zero())), *self, Vec2::one()),
            super::unproject_sc(Vec3::from((ndc_coords[3], T::zero())), *self, Vec2::one()),
            super::unproject_sc(Vec3::from((ndc_coords[0], T::one())), *self, Vec2::one()),
            super::unproject_sc(Vec3::from((ndc_coords[1], T::one())), *self, Vec2::one()),
            super::unproject_sc(Vec3::from((ndc_coords[2], T::one())), *self, Vec2::one()),
            super::unproject_sc(Vec3::from((ndc_coords[3], T::one())), *self, Vec2::one()),
        ]
    }

    fn create_ortho_matrix(left: T, right: T, bottom: T, top: T, near: T, far: T) -> Mat4<T> {
        Mat4::from((
            Vec4::new(T::two() / (right - left), T::zero(), T::zero(), T::zero()),
            Vec4::new(T::zero(), T::two() / (top - bottom), T::zero(), T::zero()),
            Vec4::new(T::zero(), T::zero(), T::one() / (near - far), T::zero()),
            Vec4::new((right + left) / (left - right), (top + bottom) / (bottom - top), near / (near - far), T::one()),
        ))
    }

    fn create_perspective_projection_lh_yup(fov: T, aspect: T, near: T, far: T) -> Mat4<T> {
        let tfov = T::tan(fov * T::point_five());
        let right = tfov * aspect * near;
        let left = -right;
        let top = tfov * near;
        let bottom = -top;
        create_perspective_matrix_internal_lh(left, right, bottom, top, near, far)
    }
    
    fn create_perspective_projection_rh_yup(fov: T, aspect: T, near: T, far: T) -> Mat4<T> {
        let tfov = T::tan(fov * T::point_five());
        let right = tfov * aspect * near;
        let left = -right;
        let top = tfov * near;
        let bottom = -top;
        create_perspective_matrix_internal_rh(left, right, bottom, top, near, far)
    }
}

/// trait to construct matrix from 4 scalars
pub trait MatNew2<T> {
    fn new(
        m00: T, m01: T, 
        m10: T, m11: T
    ) -> Self;
}

impl<T> MatNew2<T> for Mat2<T> where T: Number {
    fn new(
        m00: T, m01: T, 
        m10: T, m11: T
    ) -> Self {
        Self {  
            m: [
                m00, m01,
                m10, m11
            ]
        }
    }
}

#[allow(clippy::too_many_arguments)] 
/// trait to construct matrix from 9 scalars
pub trait MatNew3<T> {
    fn new(
        m00: T, m01: T, m02: T,
        m10: T, m11: T, m12: T,
        m20: T, m21: T, m22: T,
    ) -> Self;
}

impl<T> MatNew3<T> for Mat3<T> where T: Number {
    fn new(
        m00: T, m01: T, m02: T,
        m10: T, m11: T, m12: T,
        m20: T, m21: T, m22: T,
    ) -> Self {
        Self {
            m: [
                m00, m01, m02,
                m10, m11, m12,
                m20, m21, m22,
            ]
        }
    }
}

#[allow(clippy::too_many_arguments)] 
/// trait to construct matrix from 12 scalars
pub trait MatNew34<T> {
    fn new(
        m00: T, m01: T, m02: T, m03: T,
        m10: T, m11: T, m12: T, m13: T,
        m20: T, m21: T, m22: T, m23: T,
    ) -> Self;
}

impl<T> MatNew34<T> for Mat34<T> where T: Number {
    fn new(
        m00: T, m01: T, m02: T, m03: T,
        m10: T, m11: T, m12: T, m13: T,
        m20: T, m21: T, m22: T, m23: T,
    ) -> Self {
        Self {
            m: [
                m00, m01, m02, m03,
                m10, m11, m12, m13,
                m20, m21, m22, m23
            ]
        }
    }
}

#[allow(clippy::too_many_arguments)] 
/// trait to construct matrix from 12 scalars
pub trait MatNew43<T> {
    fn new(
        m00: T, m01: T, m02: T, 
        m03: T, m10: T, m11: T,
        m12: T, m13: T, m20: T, 
        m21: T, m22: T, m23: T,
    ) -> Self;
}

impl<T> MatNew43<T> for Mat43<T> where T: Number {
    fn new(
        m00: T, m01: T, m02: T, 
        m03: T, m10: T, m11: T, 
        m12: T, m13: T, m20: T, 
        m21: T, m22: T, m23: T,
    ) -> Self {
        Self {
            m: [
                m00, m01, m02, 
                m03, m10, m11, 
                m12, m13, m20, 
                m21, m22, m23
            ]
        }
    }
}

#[allow(clippy::too_many_arguments)] 
/// trait to construct matrix from 16 scalars
pub trait MatNew4<T> {
    fn new(
        m00: T, m01: T, m02: T, m03: T,
        m10: T, m11: T, m12: T, m13: T,
        m20: T, m21: T, m22: T, m23: T,
        m30: T, m31: T, m32: T, m33: T,
    ) -> Self;
}

impl<T> MatNew4<T> for Mat4<T> where T: Number {
    fn new(
        m00: T, m01: T, m02: T, m03: T,
        m10: T, m11: T, m12: T, m13: T,
        m20: T, m21: T, m22: T, m23: T,
        m30: T, m31: T, m32: T, m33: T,
    ) -> Self {
        Self {
            m: [
                m00, m01, m02, m03,
                m10, m11, m12, m13,
                m20, m21, m22, m23,
                m30, m31, m32, m33
            ]
        }
    }
}

mat_partial_impl!(Mat2, 2, 2, 4, Vec2 {x, 0, y, 1}, Vec2 {x, 0, y, 1});
mat_impl!(Mat2, 2, 2, 4, Vec2 {x, 0, y, 1}, Vec2 {x, 0, y, 1});

mat_partial_impl!(Mat3, 3, 3, 9, Vec3 {x, 0, y, 1, z, 2}, Vec3 {x, 0, y, 1, z, 2});
mat_impl!(Mat3, 3, 3, 9, Vec3 {x, 0, y, 1, z, 2}, Vec3 {x, 0, y, 1, z, 2});

mat_partial_impl!(Mat4, 4, 4, 16, Vec4 {x, 0, y, 1, z, 2, w, 3}, Vec4 {x, 0, y, 1, z, 2, w, 3});
mat_impl!(Mat4, 4, 4, 16, Vec4 {x, 0, y, 1, z, 2, w, 3}, Vec4 {x, 0, y, 1, z, 2, w, 3});

mat_partial_impl!(Mat34, 3, 4, 12, Vec4 {x, 0, y, 1, z, 2, w, 3}, Vec3 {x, 0, y, 1, z, 2});
mat_impl!(Mat34, 3, 4, 12, Vec4 {x, 0, y, 1, z, 2, w, 3}, Vec3 {x, 0, y, 1, z, 2});

mat_partial_impl!(Mat43, 4, 3, 12, Vec4 {x, 0, y, 1, z, 2}, Vec3 {x, 0, y, 1, z, 2, w, 3});