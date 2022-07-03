
use crate::vec::*;
use crate::num::*;

use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Deref;
use std::ops::DerefMut;

use std::cmp::PartialEq;

use std::fmt::Display;
use std::fmt::Formatter;

macro_rules! mat_impl {
    ($MatN:ident, $rows:expr, $cols:expr, $elems:expr, 
        $RowVecN:ident { $($row_field:ident, $row_field_index:expr),* },
        $ColVecN:ident { $($col_field:ident, $col_field_index:expr),* } ) => {
        #[derive(Debug, Copy, Clone)]
        pub struct $MatN<T> {
            pub m: [T; $elems]
        }

        /// index into matrix with tuple [(row, column)]
        impl<T> Index<(usize, usize)> for $MatN<T> {
            type Output = T;
            fn index(&self, rc: (usize, usize)) -> &Self::Output {
                &self.m[rc.0 * $cols + rc.1]
            }
        }

        /// mutably index into matrix with tuple [(row, column)]
        impl<T> IndexMut<(usize, usize)> for $MatN<T> {
            fn index_mut(&mut self, rc: (usize, usize)) -> &mut Self::Output {
                &mut self.m[rc.0 * $cols + rc.1]
            }
        }

        /// index into matrix raw array single [index]
        impl<T> Index<usize> for $MatN<T> {
            type Output = T;
            fn index(&self, index: usize) -> &Self::Output {
                &self.m[index]
            }
        }

        /// mutably index into matrix raw array with single [index]
        impl<T> IndexMut<usize> for $MatN<T> {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.m[index]
            }
        }

        impl<T> Deref for $MatN<T> where T: Number {
            type Target = [T];
            fn deref(&self) -> &Self::Target {
                self.as_slice()
            }
        }

        impl<T> DerefMut for $MatN<T> where T: Number {
            fn deref_mut(&mut self) -> &mut [T] {
                self.as_mut_slice()
            }
        }

        /// displays like [1.0, 0.0, 0.0]
        ///               [0.0, 1.0, 1.0]
        ///               [0.0, 0.0, 1.0]
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

        impl<T> $MatN<T> where T: Float {
            pub fn approx(lhs: Self, rhs: Self, eps: T) -> bool {
                for i in 0..$elems {
                    if !Float::approx(lhs.m[i], rhs.m[i], eps) {
                        return false;
                    }
                }
                true
            }
        }

        impl<T> $MatN<T> where T: Number {
            /// initialise matrix to all zero's
            pub fn zero() -> $MatN<T> {
                $MatN {
                    m: [T::zero(); $elems]
                }
            }

            /// initialise matrix to identity
            pub fn identity() -> $MatN<T> {
                let mut mat = Self::zero();
                for r in 0..$rows {
                    for c in 0..$cols {
                        if c == r {
                            mat.set(r, c, T::one());
                        }
                    }
                }
                mat
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

            // sets a single column of the matrix by an n sized vec, where n is the row count of the matrix
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

// mat2
// 00 01
// 02 03

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

// mat3
// 00 01 02
// 03 04 05
// 06 07 08

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

// mat4
// 00 01 02 03
// 04 05 06 07
// 08 09 10 11
// 12 13 14 15

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

/// performs multiplication with implicit w = 1.0, returning Vec3 and w in a tuple
impl<T> Mul<Vec3<T>> for Mat4<T> where T: Number {
    type Output = (Vec3<T>, T);
    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        (
            Vec3 {
                x: self.m[0] * rhs.x + self.m[1] * rhs.y + self.m[2] * rhs.z + self.m[3],
                y: self.m[4] * rhs.x + self.m[5] * rhs.y + self.m[6] * rhs.z + self.m[7],
                z: self.m[8] * rhs.x + self.m[9] * rhs.y + self.m[10] * rhs.z + self.m[11],
            },
            self.m[12] * rhs.x + self.m[13] * rhs.y + self.m[14] * rhs.z + self.m[15]
        )
    }
}

// mat34
// 00 01 02 03
// 04 05 06 07
// 08 09 10 11

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

// mat34 implicit last row
// 12 13 14 15
// 00 00 00 01

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

pub trait MatTranslate<V> {
    fn create_translation(t: V) -> Self;
}

impl<T> MatTranslate<Vec3<T>> for Mat4<T> where T: Number {
    fn create_translation(t: Vec3<T>) -> Self {
        let mut m = Mat4::identity();
        m.set_column(3, Vec4::from(t));
        m
    }
}

impl<T> MatTranslate<Vec3<T>> for Mat34<T> where T: Number {
    fn create_translation(t: Vec3<T>) -> Self {
        let mut m = Mat34::identity();
        m.set_column(3, t);
        m
    }
}

pub trait MatScale<V> {
    fn create_scale(t: V) -> Self;
}

impl<T> MatScale<Vec3<T>> for Mat4<T> where T: Number {
    fn create_scale(t: Vec3<T>) -> Self {
        let mut m = Mat4::identity();
        m.set(0, 0, t.x);
        m.set(1, 1, t.y);
        m.set(2, 2, t.z);
        m
    }
}

impl<T> MatScale<Vec3<T>> for Mat34<T> where T: Number {
    fn create_scale(t: Vec3<T>) -> Self {
        let mut m = Mat34::identity();
        m.set(0, 0, t.x);
        m.set(1, 1, t.y);
        m.set(2, 2, t.z);
        m
    }
}

impl<T> MatScale<Vec3<T>> for Mat3<T> where T: Number {
    fn create_scale(t: Vec3<T>) -> Self {
        let mut m = Mat3::identity();
        m.set(0, 0, t.x);
        m.set(1, 1, t.y);
        m.set(2, 2, t.z);
        m
    }
}

impl<T> MatScale<Vec2<T>> for Mat2<T> where T: Number {
    fn create_scale(t: Vec2<T>) -> Self {
        let mut m = Mat2::identity();
        m.set(0, 0, t.x);
        m.set(1, 1, t.y);
        m
    }
}

pub trait MatRotate2D<T> {
    fn create_z_rotation(theta: T) -> Self;
}

impl<T> MatRotate2D<T> for Mat2<T> where T: Float {
    fn create_z_rotation(theta: T) -> Self {
        let mut m = Mat2::identity();
        let cos_theta = Float::cos(theta);
        let sin_theta = Float::sin(theta);
        m.set(0, 0, cos_theta);
        m.set(0, 1, -sin_theta);
        m.set(1, 0, sin_theta);
        m.set(1, 1, cos_theta);
        m
    }
}

impl<T> MatRotate2D<T> for Mat3<T> where T: Float {
    fn create_z_rotation(theta: T) -> Self {
        let mut m = Mat3::identity();
        let cos_theta = Float::cos(theta);
        let sin_theta = Float::sin(theta);
        m.set(0, 0, cos_theta);
        m.set(0, 1, -sin_theta);
        m.set(1, 0, sin_theta);
        m.set(1, 1, cos_theta);
        m
    }
}

impl<T> MatRotate2D<T> for Mat4<T> where T: Float {
    fn create_z_rotation(theta: T) -> Self {
        let mut m = Mat4::identity();
        let cos_theta = Float::cos(theta);
        let sin_theta = Float::sin(theta);
        m.set(0, 0, cos_theta);
        m.set(0, 1, -sin_theta);
        m.set(1, 0, sin_theta);
        m.set(1, 1, cos_theta);
        m
    }
}

impl<T> MatRotate2D<T> for Mat34<T> where T: Float {
    fn create_z_rotation(theta: T) -> Self {
        let mut m = Mat34::identity();
        let cos_theta = Float::cos(theta);
        let sin_theta = Float::sin(theta);
        m.set(0, 0, cos_theta);
        m.set(0, 1, -sin_theta);
        m.set(1, 0, sin_theta);
        m.set(1, 1, cos_theta);
        m
    }
}

pub trait MatRotate3D<T, V> {
    fn create_x_rotation(theta: T) -> Self;
    fn create_y_rotation(theta: T) -> Self;
    fn create_rotation(axis: V, theta: T) -> Self;
}

impl<T> MatRotate3D<T, Vec3<T>> for Mat3<T> where T: Float {
    fn create_x_rotation(theta: T) -> Self {
        let mut m = Mat3::identity();
        let cos_theta = Float::cos(theta);
        let sin_theta = Float::sin(theta);
        m.set(1, 1, cos_theta);
        m.set(1, 2, -sin_theta);
        m.set(2, 1, sin_theta);
        m.set(2, 2, cos_theta);
        m
    }

    fn create_y_rotation(theta: T) -> Self {
        let mut m = Mat3::identity();
        let cos_theta = Float::cos(theta);
        let sin_theta = Float::sin(theta);
        m.set(1, 1, cos_theta);
        m.set(1, 3, sin_theta);
        m.set(3, 1, -sin_theta);
        m.set(3, 3, cos_theta);
        m
    }

    fn create_rotation(axis: Vec3<T>, theta: T) -> Self {
        let mut m = Mat3::identity();
        
        /*
        T theta_rad     = theta;
        T sin_theta     = sin(theta_rad);
        T cos_theta     = cos(theta_rad);
        T inv_cos_theta = 1 - cos(theta_rad);

        m.m[0] = inv_cos_theta * axis.x * axis.x + cos_theta;
        m.m[1] = inv_cos_theta * axis.x * axis.y - sin_theta * axis.z;
        m.m[2] = inv_cos_theta * axis.x * axis.z + sin_theta * axis.y;
        m.m[3] = 0;

        m.m[4] = inv_cos_theta * axis.x * axis.y + sin_theta * axis.z;
        m.m[5] = inv_cos_theta * axis.y * axis.y + cos_theta;
        m.m[6] = inv_cos_theta * axis.y * axis.z - sin_theta * axis.x;
        m.m[7] = 0;

        m.m[8]  = inv_cos_theta * axis.x * axis.z - sin_theta * axis.y;
        m.m[9]  = inv_cos_theta * axis.y * axis.z + sin_theta * axis.x;
        m.m[10] = inv_cos_theta * axis.z * axis.z + cos_theta;
        m.m[11] = 0;
        */

        let cos_theta = Float::cos(theta);
        let sin_theta = Float::sin(theta);
        let inv_cos_theta = T::one() - cos_theta;

        m.set_row(0, Vec3::new(
            inv_cos_theta * axis.x * axis.x + cos_theta,
            inv_cos_theta * axis.x * axis.y - sin_theta * axis.z,
            inv_cos_theta * axis.x * axis.z + sin_theta * axis.y,
        ));
        
        m
    }
}

// rotation 4x4
// rotation 3x4

mat_impl!(Mat2, 2, 2, 4, Vec2 {x, 0, y, 1}, Vec2 {x, 0, y, 1});
mat_impl!(Mat3, 3, 3, 9, Vec3 {x, 0, y, 1, z, 2}, Vec3 {x, 0, y, 1, z, 2});
mat_impl!(Mat4, 4, 4, 16, Vec4 {x, 0, y, 1, z, 2, w, 3}, Vec4 {x, 0, y, 1, z, 2, w, 3});
mat_impl!(Mat34, 3, 4, 12, Vec4 {x, 0, y, 1, z, 2, w, 3}, Vec3 {x, 0, y, 1, z, 2});


// from

// construct
//  rotation

// transpose
// inverse
// det
