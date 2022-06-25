
use crate::vec::*;

#[derive(Debug, Copy, Clone)]
pub struct Mat4<T> {
    pub m: [T; 16]
}

impl<T> Mat4<T> where T: Number {
    pub fn at<'a>(&mut self, row: u32, column: u32) -> &mut T {
        let urow = row as usize;
        let ucol = column as usize;
        &mut self.m[urow * 4 + ucol]
    }
    pub fn get_row(&self, row: u32) -> Vec4<T> {
        let urow = row as usize;
        Vec4 {
            x: self.m[urow * 4],
            y: self.m[urow * 4 + 1],
            z: self.m[urow * 4 + 2],
            w: self.m[urow * 4 + 3]
        }
    }
    pub fn get_column(&self, column: u32) -> Vec4<T> {
        let ucol = column as usize;
        Vec4 {
            x: self.m[ucol],
            y: self.m[4 + ucol],
            z: self.m[8 + ucol],
            w: self.m[12 + ucol]
        }
    }
}

impl<T> std::ops::Mul<Self> for Mat4<T> where T: Number {
    type Output = Self;
    fn mul(self, rhs: Mat4<T>) -> Mat4<T> {
        let result = Mat4::<T> {
            m: [T::zero(); 16]
        };
        for r in 0..4 {
            for c in 0..4 {
                let elem = v4::dot(self.get_row(r), rhs.get_column(c));
            }
        }
        result
    }
}

// construct
// mul 
// mul assign
// transpose
// inverse
// det
// korneker
// add / sub
// from