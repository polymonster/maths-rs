
use crate::vec::*;

pub struct Mat4<T> {
    pub m: [T; 16]
}

impl<T> Mat4<T> where T: Number {
    pub fn at(&self, row: u32, column: u32) -> T {
        let urow = row as usize;
        let ucol = column as usize;
        self.m[urow * 4 + ucol]
    }
    pub fn set(&mut self, row: u32, column: u32, value: T) {
        let urow = row as usize;
        let ucol = column as usize;
        self.m[urow * 4 + ucol] = value
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

// construct
// mul 
// mul assign
// transpose
// inverse
// det
// korneker
// add / sub
// from