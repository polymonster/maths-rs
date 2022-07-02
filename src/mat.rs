
use crate::vec::*;
use crate::num::*;

use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Mul;
use std::ops::MulAssign;

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
            pub fn set_column(&mut self, column: u32, value: $RowVecN<T>) {
                let ucol = column as usize;
                $(self.m[$col_field_index * $cols + ucol] = value.$col_field;)+
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
    fn mul(self, rhs: Mat2<T>) -> Mat2<T> {
        mul2x2(self, rhs)
    }
}

impl<T> MulAssign<Self> for Mat2<T> where T: Number {
    fn mul_assign(&mut self, rhs: Mat2<T>) {
        *self = mul2x2(*self, rhs);
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
    fn mul(self, rhs: Mat3<T>) -> Mat3<T> {
        mul3x3(self, rhs)
    }
}

impl<T> MulAssign<Self> for Mat3<T> where T: Number {
    fn mul_assign(&mut self, rhs: Mat3<T>) {
        *self = mul3x3(*self, rhs);
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
    fn mul(self, rhs: Mat4<T>) -> Mat4<T> {
        mul4x4(self, rhs)
    }
}

impl<T> MulAssign<Self> for Mat4<T> where T: Number {
    fn mul_assign(&mut self, rhs: Mat4<T>) {
        *self = mul4x4(*self, rhs);
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
    fn mul(self, rhs: Mat34<T>) -> Mat34<T> {
        mul3x4(self, rhs)
    }
}

impl<T> MulAssign<Self> for Mat34<T> where T: Number {
    fn mul_assign(&mut self, rhs: Mat34<T>) {
        *self = mul3x4(*self, rhs)
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
    fn mul(self, rhs: Mat4<T>) -> Mat4<T> {
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
    fn mul(self, rhs: Mat34<T>) -> Mat4<T> {
        mul4x4_3x4(self, rhs)
    }
}

impl<T> MulAssign<Mat34<T>> for Mat4<T> where T: Number {
    fn mul_assign(&mut self, rhs: Mat34<T>) {
        *self = mul4x4_3x4(*self, rhs)
    }
}

mat_impl!(Mat2, 2, 2, 4, Vec2 {x, 0, y, 1}, Vec2 {x, 0, y, 1});
mat_impl!(Mat3, 3, 3, 9, Vec3 {x, 0, y, 1, z, 2}, Vec3 {x, 0, y, 1, z, 2});
mat_impl!(Mat4, 4, 4, 16, Vec4 {x, 0, y, 1, z, 2, w, 3}, Vec4 {x, 0, y, 1, z, 2, w, 3});
mat_impl!(Mat34, 3, 4, 12, Vec4 {x, 0, y, 1, z, 2, w, 3}, Vec3 {x, 0, y, 1, z, 2});

// eq
// approx

// construct
//  translation
//  scale
//  rotation
//  from

// mul 
//  VecN

// transpose
// inverse
// det
