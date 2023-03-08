
use crate::vec::*;
use crate::num::*;

/// swizzle combinations of x and y
pub trait Vec2Swizzle<T: Number>: VecN<T> {
    fn xx(self) -> Vec2<T>;
    fn xy(self) -> Vec2<T>;
    fn yx(self) -> Vec2<T>;
    fn yy(self) -> Vec2<T>;
    fn set_xy(&mut self, other: Vec2<T>);
    fn set_yx(&mut self, other: Vec2<T>);
}

/// swizzle combinations of xyz
pub trait Vec3Swizzle<T: Number>: VecN<T> {
    fn xz(self) -> Vec2<T>;
    fn yz(self) -> Vec2<T>;
    fn zx(self) -> Vec2<T>;
    fn zy(self) -> Vec2<T>;
    fn zz(self) -> Vec2<T>;
    fn xxx(self) -> Vec3<T>;
    fn xxy(self) -> Vec3<T>;
    fn xxz(self) -> Vec3<T>;
    fn xyx(self) -> Vec3<T>;
    fn xyy(self) -> Vec3<T>;
    fn xyz(self) -> Vec3<T>;
    fn xzx(self) -> Vec3<T>;
    fn xzy(self) -> Vec3<T>;
    fn xzz(self) -> Vec3<T>;
    fn yxx(self) -> Vec3<T>;
    fn yxy(self) -> Vec3<T>;
    fn yxz(self) -> Vec3<T>;
    fn yyx(self) -> Vec3<T>;
    fn yyy(self) -> Vec3<T>;
    fn yyz(self) -> Vec3<T>;
    fn yzx(self) -> Vec3<T>;
    fn yzy(self) -> Vec3<T>;
    fn yzz(self) -> Vec3<T>;
    fn zxx(self) -> Vec3<T>;
    fn zxy(self) -> Vec3<T>;
    fn zxz(self) -> Vec3<T>;
    fn zyx(self) -> Vec3<T>;
    fn zyy(self) -> Vec3<T>;
    fn zyz(self) -> Vec3<T>;
    fn zzx(self) -> Vec3<T>;
    fn zzy(self) -> Vec3<T>;
    fn zzz(self) -> Vec3<T>;
    fn set_xz(&mut self, other: Vec2<T>);
    fn set_yz(&mut self, other: Vec2<T>);
    fn set_zx(&mut self, other: Vec2<T>);
    fn set_zy(&mut self, other: Vec2<T>);
    fn set_xyz(&mut self, other: Vec3<T>);
    fn set_xzy(&mut self, other: Vec3<T>);
    fn set_yxz(&mut self, other: Vec3<T>);
    fn set_yzx(&mut self, other: Vec3<T>);
    fn set_zxy(&mut self, other: Vec3<T>);
    fn set_zyx(&mut self, other: Vec3<T>);
}

/// swizzle combinations of xyzw
pub trait Vec4Swizzle<T: Number>: VecN<T> {
    fn xw(self) -> Vec2<T>;
    fn yw(self) -> Vec2<T>;
    fn zw(self) -> Vec2<T>;
    fn wx(self) -> Vec2<T>;
    fn wy(self) -> Vec2<T>;
    fn wz(self) -> Vec2<T>;
    fn ww(self) -> Vec2<T>;
    fn xxw(self) -> Vec3<T>;
    fn xyw(self) -> Vec3<T>;
    fn xzw(self) -> Vec3<T>;
    fn xwx(self) -> Vec3<T>;
    fn xwy(self) -> Vec3<T>;
    fn xwz(self) -> Vec3<T>;
    fn xww(self) -> Vec3<T>;
    fn yxw(self) -> Vec3<T>;
    fn yyw(self) -> Vec3<T>;
    fn yzw(self) -> Vec3<T>;
    fn ywx(self) -> Vec3<T>;
    fn ywy(self) -> Vec3<T>;
    fn ywz(self) -> Vec3<T>;
    fn yww(self) -> Vec3<T>;
    fn zxw(self) -> Vec3<T>;
    fn zyw(self) -> Vec3<T>;
    fn zzw(self) -> Vec3<T>;
    fn zwx(self) -> Vec3<T>;
    fn zwy(self) -> Vec3<T>;
    fn zwz(self) -> Vec3<T>;
    fn zww(self) -> Vec3<T>;
    fn wxx(self) -> Vec3<T>;
    fn wxy(self) -> Vec3<T>;
    fn wxz(self) -> Vec3<T>;
    fn wxw(self) -> Vec3<T>;
    fn wyx(self) -> Vec3<T>;
    fn wyy(self) -> Vec3<T>;
    fn wyz(self) -> Vec3<T>;
    fn wyw(self) -> Vec3<T>;
    fn wzx(self) -> Vec3<T>;
    fn wzy(self) -> Vec3<T>;
    fn wzz(self) -> Vec3<T>;
    fn wzw(self) -> Vec3<T>;
    fn wwx(self) -> Vec3<T>;
    fn wwy(self) -> Vec3<T>;
    fn wwz(self) -> Vec3<T>;
    fn www(self) -> Vec3<T>;
    fn xxxx(self) -> Vec4<T>;
    fn xxxy(self) -> Vec4<T>;
    fn xxxz(self) -> Vec4<T>;
    fn xxxw(self) -> Vec4<T>;
    fn xxyx(self) -> Vec4<T>;
    fn xxyy(self) -> Vec4<T>;
    fn xxyz(self) -> Vec4<T>;
    fn xxyw(self) -> Vec4<T>;
    fn xxzx(self) -> Vec4<T>;
    fn xxzy(self) -> Vec4<T>;
    fn xxzz(self) -> Vec4<T>;
    fn xxzw(self) -> Vec4<T>;
    fn xxwx(self) -> Vec4<T>;
    fn xxwy(self) -> Vec4<T>;
    fn xxwz(self) -> Vec4<T>;
    fn xxww(self) -> Vec4<T>;
    fn xyxx(self) -> Vec4<T>;
    fn xyxy(self) -> Vec4<T>;
    fn xyxz(self) -> Vec4<T>;
    fn xyxw(self) -> Vec4<T>;
    fn xyyx(self) -> Vec4<T>;
    fn xyyy(self) -> Vec4<T>;
    fn xyyz(self) -> Vec4<T>;
    fn xyyw(self) -> Vec4<T>;
    fn xyzx(self) -> Vec4<T>;
    fn xyzy(self) -> Vec4<T>;
    fn xyzz(self) -> Vec4<T>;
    fn xyzw(self) -> Vec4<T>;
    fn xywx(self) -> Vec4<T>;
    fn xywy(self) -> Vec4<T>;
    fn xywz(self) -> Vec4<T>;
    fn xyww(self) -> Vec4<T>;
    fn xzxx(self) -> Vec4<T>;
    fn xzxy(self) -> Vec4<T>;
    fn xzxz(self) -> Vec4<T>;
    fn xzxw(self) -> Vec4<T>;
    fn xzyx(self) -> Vec4<T>;
    fn xzyy(self) -> Vec4<T>;
    fn xzyz(self) -> Vec4<T>;
    fn xzyw(self) -> Vec4<T>;
    fn xzzx(self) -> Vec4<T>;
    fn xzzy(self) -> Vec4<T>;
    fn xzzz(self) -> Vec4<T>;
    fn xzzw(self) -> Vec4<T>;
    fn xzwx(self) -> Vec4<T>;
    fn xzwy(self) -> Vec4<T>;
    fn xzwz(self) -> Vec4<T>;
    fn xzww(self) -> Vec4<T>;
    fn xwxx(self) -> Vec4<T>;
    fn xwxy(self) -> Vec4<T>;
    fn xwxz(self) -> Vec4<T>;
    fn xwxw(self) -> Vec4<T>;
    fn xwyx(self) -> Vec4<T>;
    fn xwyy(self) -> Vec4<T>;
    fn xwyz(self) -> Vec4<T>;
    fn xwyw(self) -> Vec4<T>;
    fn xwzx(self) -> Vec4<T>;
    fn xwzy(self) -> Vec4<T>;
    fn xwzz(self) -> Vec4<T>;
    fn xwzw(self) -> Vec4<T>;
    fn xwwx(self) -> Vec4<T>;
    fn xwwy(self) -> Vec4<T>;
    fn xwwz(self) -> Vec4<T>;
    fn xwww(self) -> Vec4<T>;
    fn yxxx(self) -> Vec4<T>;
    fn yxxy(self) -> Vec4<T>;
    fn yxxz(self) -> Vec4<T>;
    fn yxxw(self) -> Vec4<T>;
    fn yxyx(self) -> Vec4<T>;
    fn yxyy(self) -> Vec4<T>;
    fn yxyz(self) -> Vec4<T>;
    fn yxyw(self) -> Vec4<T>;
    fn yxzx(self) -> Vec4<T>;
    fn yxzy(self) -> Vec4<T>;
    fn yxzz(self) -> Vec4<T>;
    fn yxzw(self) -> Vec4<T>;
    fn yxwx(self) -> Vec4<T>;
    fn yxwy(self) -> Vec4<T>;
    fn yxwz(self) -> Vec4<T>;
    fn yxww(self) -> Vec4<T>;
    fn yyxx(self) -> Vec4<T>;
    fn yyxy(self) -> Vec4<T>;
    fn yyxz(self) -> Vec4<T>;
    fn yyxw(self) -> Vec4<T>;
    fn yyyx(self) -> Vec4<T>;
    fn yyyy(self) -> Vec4<T>;
    fn yyyz(self) -> Vec4<T>;
    fn yyyw(self) -> Vec4<T>;
    fn yyzx(self) -> Vec4<T>;
    fn yyzy(self) -> Vec4<T>;
    fn yyzz(self) -> Vec4<T>;
    fn yyzw(self) -> Vec4<T>;
    fn yywx(self) -> Vec4<T>;
    fn yywy(self) -> Vec4<T>;
    fn yywz(self) -> Vec4<T>;
    fn yyww(self) -> Vec4<T>;
    fn yzxx(self) -> Vec4<T>;
    fn yzxy(self) -> Vec4<T>;
    fn yzxz(self) -> Vec4<T>;
    fn yzxw(self) -> Vec4<T>;
    fn yzyx(self) -> Vec4<T>;
    fn yzyy(self) -> Vec4<T>;
    fn yzyz(self) -> Vec4<T>;
    fn yzyw(self) -> Vec4<T>;
    fn yzzx(self) -> Vec4<T>;
    fn yzzy(self) -> Vec4<T>;
    fn yzzz(self) -> Vec4<T>;
    fn yzzw(self) -> Vec4<T>;
    fn yzwx(self) -> Vec4<T>;
    fn yzwy(self) -> Vec4<T>;
    fn yzwz(self) -> Vec4<T>;
    fn yzww(self) -> Vec4<T>;
    fn ywxx(self) -> Vec4<T>;
    fn ywxy(self) -> Vec4<T>;
    fn ywxz(self) -> Vec4<T>;
    fn ywxw(self) -> Vec4<T>;
    fn ywyx(self) -> Vec4<T>;
    fn ywyy(self) -> Vec4<T>;
    fn ywyz(self) -> Vec4<T>;
    fn ywyw(self) -> Vec4<T>;
    fn ywzx(self) -> Vec4<T>;
    fn ywzy(self) -> Vec4<T>;
    fn ywzz(self) -> Vec4<T>;
    fn ywzw(self) -> Vec4<T>;
    fn ywwx(self) -> Vec4<T>;
    fn ywwy(self) -> Vec4<T>;
    fn ywwz(self) -> Vec4<T>;
    fn ywww(self) -> Vec4<T>;
    fn zxxx(self) -> Vec4<T>;
    fn zxxy(self) -> Vec4<T>;
    fn zxxz(self) -> Vec4<T>;
    fn zxxw(self) -> Vec4<T>;
    fn zxyx(self) -> Vec4<T>;
    fn zxyy(self) -> Vec4<T>;
    fn zxyz(self) -> Vec4<T>;
    fn zxyw(self) -> Vec4<T>;
    fn zxzx(self) -> Vec4<T>;
    fn zxzy(self) -> Vec4<T>;
    fn zxzz(self) -> Vec4<T>;
    fn zxzw(self) -> Vec4<T>;
    fn zxwx(self) -> Vec4<T>;
    fn zxwy(self) -> Vec4<T>;
    fn zxwz(self) -> Vec4<T>;
    fn zxww(self) -> Vec4<T>;
    fn zyxx(self) -> Vec4<T>;
    fn zyxy(self) -> Vec4<T>;
    fn zyxz(self) -> Vec4<T>;
    fn zyxw(self) -> Vec4<T>;
    fn zyyx(self) -> Vec4<T>;
    fn zyyy(self) -> Vec4<T>;
    fn zyyz(self) -> Vec4<T>;
    fn zyyw(self) -> Vec4<T>;
    fn zyzx(self) -> Vec4<T>;
    fn zyzy(self) -> Vec4<T>;
    fn zyzz(self) -> Vec4<T>;
    fn zyzw(self) -> Vec4<T>;
    fn zywx(self) -> Vec4<T>;
    fn zywy(self) -> Vec4<T>;
    fn zywz(self) -> Vec4<T>;
    fn zyww(self) -> Vec4<T>;
    fn zzxx(self) -> Vec4<T>;
    fn zzxy(self) -> Vec4<T>;
    fn zzxz(self) -> Vec4<T>;
    fn zzxw(self) -> Vec4<T>;
    fn zzyx(self) -> Vec4<T>;
    fn zzyy(self) -> Vec4<T>;
    fn zzyz(self) -> Vec4<T>;
    fn zzyw(self) -> Vec4<T>;
    fn zzzx(self) -> Vec4<T>;
    fn zzzy(self) -> Vec4<T>;
    fn zzzz(self) -> Vec4<T>;
    fn zzzw(self) -> Vec4<T>;
    fn zzwx(self) -> Vec4<T>;
    fn zzwy(self) -> Vec4<T>;
    fn zzwz(self) -> Vec4<T>;
    fn zzww(self) -> Vec4<T>;
    fn zwxx(self) -> Vec4<T>;
    fn zwxy(self) -> Vec4<T>;
    fn zwxz(self) -> Vec4<T>;
    fn zwxw(self) -> Vec4<T>;
    fn zwyx(self) -> Vec4<T>;
    fn zwyy(self) -> Vec4<T>;
    fn zwyz(self) -> Vec4<T>;
    fn zwyw(self) -> Vec4<T>;
    fn zwzx(self) -> Vec4<T>;
    fn zwzy(self) -> Vec4<T>;
    fn zwzz(self) -> Vec4<T>;
    fn zwzw(self) -> Vec4<T>;
    fn zwwx(self) -> Vec4<T>;
    fn zwwy(self) -> Vec4<T>;
    fn zwwz(self) -> Vec4<T>;
    fn zwww(self) -> Vec4<T>;
    fn wxxx(self) -> Vec4<T>;
    fn wxxy(self) -> Vec4<T>;
    fn wxxz(self) -> Vec4<T>;
    fn wxxw(self) -> Vec4<T>;
    fn wxyx(self) -> Vec4<T>;
    fn wxyy(self) -> Vec4<T>;
    fn wxyz(self) -> Vec4<T>;
    fn wxyw(self) -> Vec4<T>;
    fn wxzx(self) -> Vec4<T>;
    fn wxzy(self) -> Vec4<T>;
    fn wxzz(self) -> Vec4<T>;
    fn wxzw(self) -> Vec4<T>;
    fn wxwx(self) -> Vec4<T>;
    fn wxwy(self) -> Vec4<T>;
    fn wxwz(self) -> Vec4<T>;
    fn wxww(self) -> Vec4<T>;
    fn wyxx(self) -> Vec4<T>;
    fn wyxy(self) -> Vec4<T>;
    fn wyxz(self) -> Vec4<T>;
    fn wyxw(self) -> Vec4<T>;
    fn wyyx(self) -> Vec4<T>;
    fn wyyy(self) -> Vec4<T>;
    fn wyyz(self) -> Vec4<T>;
    fn wyyw(self) -> Vec4<T>;
    fn wyzx(self) -> Vec4<T>;
    fn wyzy(self) -> Vec4<T>;
    fn wyzz(self) -> Vec4<T>;
    fn wyzw(self) -> Vec4<T>;
    fn wywx(self) -> Vec4<T>;
    fn wywy(self) -> Vec4<T>;
    fn wywz(self) -> Vec4<T>;
    fn wyww(self) -> Vec4<T>;
    fn wzxx(self) -> Vec4<T>;
    fn wzxy(self) -> Vec4<T>;
    fn wzxz(self) -> Vec4<T>;
    fn wzxw(self) -> Vec4<T>;
    fn wzyx(self) -> Vec4<T>;
    fn wzyy(self) -> Vec4<T>;
    fn wzyz(self) -> Vec4<T>;
    fn wzyw(self) -> Vec4<T>;
    fn wzzx(self) -> Vec4<T>;
    fn wzzy(self) -> Vec4<T>;
    fn wzzz(self) -> Vec4<T>;
    fn wzzw(self) -> Vec4<T>;
    fn wzwx(self) -> Vec4<T>;
    fn wzwy(self) -> Vec4<T>;
    fn wzwz(self) -> Vec4<T>;
    fn wzww(self) -> Vec4<T>;
    fn wwxx(self) -> Vec4<T>;
    fn wwxy(self) -> Vec4<T>;
    fn wwxz(self) -> Vec4<T>;
    fn wwxw(self) -> Vec4<T>;
    fn wwyx(self) -> Vec4<T>;
    fn wwyy(self) -> Vec4<T>;
    fn wwyz(self) -> Vec4<T>;
    fn wwyw(self) -> Vec4<T>;
    fn wwzx(self) -> Vec4<T>;
    fn wwzy(self) -> Vec4<T>;
    fn wwzz(self) -> Vec4<T>;
    fn wwzw(self) -> Vec4<T>;
    fn wwwx(self) -> Vec4<T>;
    fn wwwy(self) -> Vec4<T>;
    fn wwwz(self) -> Vec4<T>;
    fn wwww(self) -> Vec4<T>;
    fn set_xw(&mut self, other: Vec2<T>);
    fn set_yw(&mut self, other: Vec2<T>);
    fn set_zw(&mut self, other: Vec2<T>);
    fn set_wx(&mut self, other: Vec2<T>);
    fn set_wy(&mut self, other: Vec2<T>);
    fn set_wz(&mut self, other: Vec2<T>);
    fn set_xyw(&mut self, other: Vec3<T>);
    fn set_xzw(&mut self, other: Vec3<T>);
    fn set_xwy(&mut self, other: Vec3<T>);
    fn set_xwz(&mut self, other: Vec3<T>);
    fn set_yxw(&mut self, other: Vec3<T>);
    fn set_yzw(&mut self, other: Vec3<T>);
    fn set_ywx(&mut self, other: Vec3<T>);
    fn set_ywz(&mut self, other: Vec3<T>);
    fn set_zxw(&mut self, other: Vec3<T>);
    fn set_zyw(&mut self, other: Vec3<T>);
    fn set_zwx(&mut self, other: Vec3<T>);
    fn set_zwy(&mut self, other: Vec3<T>);
    fn set_wxy(&mut self, other: Vec3<T>);
    fn set_wxz(&mut self, other: Vec3<T>);
    fn set_wyx(&mut self, other: Vec3<T>);
    fn set_wyz(&mut self, other: Vec3<T>);
    fn set_wzx(&mut self, other: Vec3<T>);
    fn set_wzy(&mut self, other: Vec3<T>);
    fn set_xyzw(&mut self, other: Vec4<T>);
    fn set_xywz(&mut self, other: Vec4<T>);
    fn set_xzyw(&mut self, other: Vec4<T>);
    fn set_xzwy(&mut self, other: Vec4<T>);
    fn set_xwyz(&mut self, other: Vec4<T>);
    fn set_xwzy(&mut self, other: Vec4<T>);
    fn set_yxzw(&mut self, other: Vec4<T>);
    fn set_yxwz(&mut self, other: Vec4<T>);
    fn set_yzxw(&mut self, other: Vec4<T>);
    fn set_yzwx(&mut self, other: Vec4<T>);
    fn set_ywxz(&mut self, other: Vec4<T>);
    fn set_ywzx(&mut self, other: Vec4<T>);
    fn set_zxyw(&mut self, other: Vec4<T>);
    fn set_zxwy(&mut self, other: Vec4<T>);
    fn set_zyxw(&mut self, other: Vec4<T>);
    fn set_zywx(&mut self, other: Vec4<T>);
    fn set_zwxy(&mut self, other: Vec4<T>);
    fn set_zwyx(&mut self, other: Vec4<T>);
    fn set_wxyz(&mut self, other: Vec4<T>);
    fn set_wxzy(&mut self, other: Vec4<T>);
    fn set_wyxz(&mut self, other: Vec4<T>);
    fn set_wyzx(&mut self, other: Vec4<T>);
    fn set_wzxy(&mut self, other: Vec4<T>);
    fn set_wzyx(&mut self, other: Vec4<T>);
}

macro_rules! v2_swizzle_impl {
    ($VecN:ident) => {
        impl<T> Vec2Swizzle<T> for $VecN<T> where T: Number {
            fn xx(self) -> Vec2<T> {
                Vec2 {
                    x: self.x,
                    y: self.x
                }
            }
            
            fn xy(self) -> Vec2<T> {
                Vec2 {
                    x: self.x,
                    y: self.y
                }
            }
            
            fn yx(self) -> Vec2<T> {
                Vec2 {
                    x: self.y,
                    y: self.x
                }
            }
            
            fn yy(self) -> Vec2<T> {
                Vec2 {
                    x: self.y,
                    y: self.y
                }
            }

            fn set_xy(&mut self, other: Vec2<T>) {
                self.x = other.x;
                self.y = other.y;
            }
            
            fn set_yx(&mut self, other: Vec2<T>) {
                self.x = other.y;
                self.y = other.x;
            }
        }
    }
}

macro_rules! v3_swizzle_impl {
    ($VecN:ident) => {
        impl<T> Vec3Swizzle<T> for $VecN<T> where T: Number {
            fn xz(self) -> Vec2<T> {
                Vec2::new(self.x, self.z)
            }
            
            fn yz(self) -> Vec2<T> {
                Vec2::new(self.y, self.z)
            }
            
            fn zx(self) -> Vec2<T> {
                Vec2::new(self.z, self.x)
            }
            
            fn zy(self) -> Vec2<T> {
                Vec2::new(self.z, self.y)
            }
            
            fn zz(self) -> Vec2<T> {
                Vec2::new(self.z, self.z)
            }
            
            fn xxx(self) -> Vec3<T> {
                Vec3 {
                    x: self.x,
                    y: self.x,
                    z: self.x,
                }
            }
            
            fn xxy(self) -> Vec3<T> {
                Vec3 {
                    x: self.x,
                    y: self.x,
                    z: self.y,
                }
            }
            
            fn xxz(self) -> Vec3<T> {
                Vec3 {
                    x: self.x,
                    y: self.x,
                    z: self.z,
                }
            }
            
            fn xyx(self) -> Vec3<T> {
                Vec3 {
                    x: self.x,
                    y: self.y,
                    z: self.x,
                }
            }
            
            fn xyy(self) -> Vec3<T> {
                Vec3 {
                    x: self.x,
                    y: self.y,
                    z: self.y,
                }
            }
            
            fn xyz(self) -> Vec3<T> {
                Vec3 {
                    x: self.x,
                    y: self.y,
                    z: self.z,
                }
            }
            
            fn xzx(self) -> Vec3<T> {
                Vec3 {
                    x: self.x,
                    y: self.z,
                    z: self.x,
                }
            }
            
            fn xzy(self) -> Vec3<T> {
                Vec3 {
                    x: self.x,
                    y: self.z,
                    z: self.y,
                }
            }
            
            fn xzz(self) -> Vec3<T> {
                Vec3 {
                    x: self.x,
                    y: self.z,
                    z: self.z,
                }
            }
            
            fn yxx(self) -> Vec3<T> {
                Vec3 {
                    x: self.y,
                    y: self.x,
                    z: self.x,
                }
            }
            
            fn yxy(self) -> Vec3<T> {
                Vec3 {
                    x: self.y,
                    y: self.x,
                    z: self.y,
                }
            }
            
            fn yxz(self) -> Vec3<T> {
                Vec3 {
                    x: self.y,
                    y: self.x,
                    z: self.z,
                }
            }
            
            fn yyx(self) -> Vec3<T> {
                Vec3 {
                    x: self.y,
                    y: self.y,
                    z: self.x,
                }
            }
            
            fn yyy(self) -> Vec3<T> {
                Vec3 {
                    x: self.y,
                    y: self.y,
                    z: self.y,
                }
            }
            
            fn yyz(self) -> Vec3<T> {
                Vec3 {
                    x: self.y,
                    y: self.y,
                    z: self.z,
                }
            }
            
            fn yzx(self) -> Vec3<T> {
                Vec3 {
                    x: self.y,
                    y: self.z,
                    z: self.x,
                }
            }
            
            fn yzy(self) -> Vec3<T> {
                Vec3 {
                    x: self.y,
                    y: self.z,
                    z: self.y,
                }
            }
            
            fn yzz(self) -> Vec3<T> {
                Vec3 {
                    x: self.y,
                    y: self.z,
                    z: self.z,
                }
            }
            
            fn zxx(self) -> Vec3<T> {
                Vec3 {
                    x: self.z,
                    y: self.x,
                    z: self.x,
                }
            }
            
            fn zxy(self) -> Vec3<T> {
                Vec3 {
                    x: self.z,
                    y: self.x,
                    z: self.y,
                }
            }
            
            fn zxz(self) -> Vec3<T> {
                Vec3 {
                    x: self.z,
                    y: self.x,
                    z: self.z,
                }
            }
            
            fn zyx(self) -> Vec3<T> {
                Vec3 {
                    x: self.z,
                    y: self.y,
                    z: self.x,
                }
            }
            
            fn zyy(self) -> Vec3<T> {
                Vec3 {
                    x: self.z,
                    y: self.y,
                    z: self.y,
                }
            }
            
            fn zyz(self) -> Vec3<T> {
                Vec3 {
                    x: self.z,
                    y: self.y,
                    z: self.z,
                }
            }
            
            fn zzx(self) -> Vec3<T> {
                Vec3 {
                    x: self.z,
                    y: self.z,
                    z: self.x,
                }
            }
            
            fn zzy(self) -> Vec3<T> {
                Vec3 {
                    x: self.z,
                    y: self.z,
                    z: self.y,
                }
            }
            
            fn zzz(self) -> Vec3<T> {
                Vec3 {
                    x: self.z,
                    y: self.z,
                    z: self.z,
                }
            }

            fn set_xz(&mut self, other: Vec2<T>) {
                self.x = other.x; self.z = other.y;
            }
            
            fn set_yz(&mut self, other: Vec2<T>) {
                self.y = other.x; self.z = other.y;
            }
            
            fn set_zx(&mut self, other: Vec2<T>) {
                self.z = other.x; self.x = other.y;
            }
            
            fn set_zy(&mut self, other: Vec2<T>) {
                self.z = other.x; self.y = other.y;
            }

            fn set_xyz(&mut self, other: Vec3<T>) {
                self.x = other.x;
                self.y = other.y;
                self.z = other.z;
            }
            
            fn set_xzy(&mut self, other: Vec3<T>) {
                self.x = other.x;
                self.y = other.z;
                self.z = other.y;
            }
            
            fn set_yxz(&mut self, other: Vec3<T>) {
                self.x = other.y;
                self.y = other.x;
                self.z = other.z;
            }
            
            fn set_yzx(&mut self, other: Vec3<T>) {
                self.x = other.y;
                self.y = other.z;
                self.z = other.x;
            }
            
            fn set_zxy(&mut self, other: Vec3<T>) {
                self.x = other.z;
                self.y = other.x;
                self.z = other.y;
            }
            
            fn set_zyx(&mut self, other: Vec3<T>) {
                self.x = other.z;
                self.y = other.y;
                self.z = other.x;
            }
        }
    }
}

macro_rules! v4_swizzle_impl {
    ($VecN:ident) => {
        impl<T> Vec4Swizzle<T> for $VecN<T> where T: Number {
            fn xw(self) -> Vec2<T> {
                Vec2::new(self.x, self.w)
            }

            fn yw(self) -> Vec2<T> {
                Vec2::new(self.y, self.w)
            }

            fn zw(self) -> Vec2<T> {
                Vec2::new(self.z, self.w)
            }

            fn wx(self) -> Vec2<T> {
                Vec2::new(self.w, self.x)
            }

            fn wy(self) -> Vec2<T> {
                Vec2::new(self.w, self.y)
            }

            fn wz(self) -> Vec2<T> {
                Vec2::new(self.w, self.z)
            }

            fn ww(self) -> Vec2<T> {
                Vec2::new(self.w, self.w)
            }

            fn xxw(self) -> Vec3<T> {
                Vec3::new(self.x, self.x, self.w)
            }
            
            fn xyw(self) -> Vec3<T> {
                Vec3::new(self.x, self.y, self.w)
            }
            
            fn xzw(self) -> Vec3<T> {
                Vec3::new(self.x, self.z, self.w)
            }
            
            fn xwx(self) -> Vec3<T> {
                Vec3::new(self.x, self.w, self.x)
            }
            
            fn xwy(self) -> Vec3<T> {
                Vec3::new(self.x, self.w, self.y)
            }
            
            fn xwz(self) -> Vec3<T> {
                Vec3::new(self.x, self.w, self.z)
            }
            
            fn xww(self) -> Vec3<T> {
                Vec3::new(self.x, self.w, self.w)
            }
            
            fn yxw(self) -> Vec3<T> {
                Vec3::new(self.y, self.x, self.w)
            }
            
            fn yyw(self) -> Vec3<T> {
                Vec3::new(self.y, self.y, self.w)
            }
            
            fn yzw(self) -> Vec3<T> {
                Vec3::new(self.y, self.z, self.w)
            }
            
            fn ywx(self) -> Vec3<T> {
                Vec3::new(self.y, self.w, self.x)
            }
            
            fn ywy(self) -> Vec3<T> {
                Vec3::new(self.y, self.w, self.y)
            }
            
            fn ywz(self) -> Vec3<T> {
                Vec3::new(self.y, self.w, self.z)
            }
            
            fn yww(self) -> Vec3<T> {
                Vec3::new(self.y, self.w, self.w)
            }
            
            fn zxw(self) -> Vec3<T> {
                Vec3::new(self.z, self.x, self.w)
            }
            
            fn zyw(self) -> Vec3<T> {
                Vec3::new(self.z, self.y, self.w)
            }
            
            fn zzw(self) -> Vec3<T> {
                Vec3::new(self.z, self.z, self.w)
            }
            
            fn zwx(self) -> Vec3<T> {
                Vec3::new(self.z, self.w, self.x)
            }
            
            fn zwy(self) -> Vec3<T> {
                Vec3::new(self.z, self.w, self.y)
            }
            
            fn zwz(self) -> Vec3<T> {
                Vec3::new(self.z, self.w, self.z)
            }
            
            fn zww(self) -> Vec3<T> {
                Vec3::new(self.z, self.w, self.w)
            }
            
            fn wxx(self) -> Vec3<T> {
                Vec3::new(self.w, self.x, self.x)
            }
            
            fn wxy(self) -> Vec3<T> {
                Vec3::new(self.w, self.x, self.y)
            }
            
            fn wxz(self) -> Vec3<T> {
                Vec3::new(self.w, self.x, self.z)
            }
            
            fn wxw(self) -> Vec3<T> {
                Vec3::new(self.w, self.x, self.w)
            }
            
            fn wyx(self) -> Vec3<T> {
                Vec3::new(self.w, self.y, self.x)
            }
            
            fn wyy(self) -> Vec3<T> {
                Vec3::new(self.w, self.y, self.y)
            }
            
            fn wyz(self) -> Vec3<T> {
                Vec3::new(self.w, self.y, self.z)
            }
            
            fn wyw(self) -> Vec3<T> {
                Vec3::new(self.w, self.y, self.w)
            }
            
            fn wzx(self) -> Vec3<T> {
                Vec3::new(self.w, self.z, self.x)
            }
            
            fn wzy(self) -> Vec3<T> {
                Vec3::new(self.w, self.z, self.y)
            }
            
            fn wzz(self) -> Vec3<T> {
                Vec3::new(self.w, self.z, self.z)
            }
            
            fn wzw(self) -> Vec3<T> {
                Vec3::new(self.w, self.z, self.w)
            }
            
            fn wwx(self) -> Vec3<T> {
                Vec3::new(self.w, self.w, self.x)
            }
            
            fn wwy(self) -> Vec3<T> {
                Vec3::new(self.w, self.w, self.y)
            }
            
            fn wwz(self) -> Vec3<T> {
                Vec3::new(self.w, self.w, self.z)
            }
            
            fn www(self) -> Vec3<T> {
                Vec3::new(self.w, self.w, self.w)
            }

            fn xxxx(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.x,
                    z: self.x,
                    w: self.x,
                }
            }
            
            fn xxxy(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.x,
                    z: self.x,
                    w: self.y,
                }
            }
            
            fn xxxz(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.x,
                    z: self.x,
                    w: self.z,
                }
            }
            
            fn xxxw(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.x,
                    z: self.x,
                    w: self.w,
                }
            }
            
            fn xxyx(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.x,
                    z: self.y,
                    w: self.x,
                }
            }
            
            fn xxyy(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.x,
                    z: self.y,
                    w: self.y,
                }
            }
            
            fn xxyz(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.x,
                    z: self.y,
                    w: self.z,
                }
            }
            
            fn xxyw(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.x,
                    z: self.y,
                    w: self.w,
                }
            }
            
            fn xxzx(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.x,
                    z: self.z,
                    w: self.x,
                }
            }
            
            fn xxzy(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.x,
                    z: self.z,
                    w: self.y,
                }
            }
            
            fn xxzz(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.x,
                    z: self.z,
                    w: self.z,
                }
            }
            
            fn xxzw(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.x,
                    z: self.z,
                    w: self.w,
                }
            }
            
            fn xxwx(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.x,
                    z: self.w,
                    w: self.x,
                }
            }
            
            fn xxwy(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.x,
                    z: self.w,
                    w: self.y,
                }
            }
            
            fn xxwz(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.x,
                    z: self.w,
                    w: self.z,
                }
            }
            
            fn xxww(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.x,
                    z: self.w,
                    w: self.w,
                }
            }
            
            fn xyxx(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.y,
                    z: self.x,
                    w: self.x,
                }
            }
            
            fn xyxy(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.y,
                    z: self.x,
                    w: self.y,
                }
            }
            
            fn xyxz(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.y,
                    z: self.x,
                    w: self.z,
                }
            }
            
            fn xyxw(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.y,
                    z: self.x,
                    w: self.w,
                }
            }
            
            fn xyyx(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.y,
                    z: self.y,
                    w: self.x,
                }
            }
            
            fn xyyy(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.y,
                    z: self.y,
                    w: self.y,
                }
            }
            
            fn xyyz(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.y,
                    z: self.y,
                    w: self.z,
                }
            }
            
            fn xyyw(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.y,
                    z: self.y,
                    w: self.w,
                }
            }
            
            fn xyzx(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.y,
                    z: self.z,
                    w: self.x,
                }
            }
            
            fn xyzy(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.y,
                    z: self.z,
                    w: self.y,
                }
            }
            
            fn xyzz(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.y,
                    z: self.z,
                    w: self.z,
                }
            }
            
            fn xyzw(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.y,
                    z: self.z,
                    w: self.w,
                }
            }
            
            fn xywx(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.y,
                    z: self.w,
                    w: self.x,
                }
            }
            
            fn xywy(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.y,
                    z: self.w,
                    w: self.y,
                }
            }
            
            fn xywz(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.y,
                    z: self.w,
                    w: self.z,
                }
            }
            
            fn xyww(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.y,
                    z: self.w,
                    w: self.w,
                }
            }
            
            fn xzxx(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.z,
                    z: self.x,
                    w: self.x,
                }
            }
            
            fn xzxy(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.z,
                    z: self.x,
                    w: self.y,
                }
            }
            
            fn xzxz(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.z,
                    z: self.x,
                    w: self.z,
                }
            }
            
            fn xzxw(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.z,
                    z: self.x,
                    w: self.w,
                }
            }
            
            fn xzyx(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.z,
                    z: self.y,
                    w: self.x,
                }
            }
            
            fn xzyy(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.z,
                    z: self.y,
                    w: self.y,
                }
            }
            
            fn xzyz(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.z,
                    z: self.y,
                    w: self.z,
                }
            }
            
            fn xzyw(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.z,
                    z: self.y,
                    w: self.w,
                }
            }
            
            fn xzzx(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.z,
                    z: self.z,
                    w: self.x,
                }
            }
            
            fn xzzy(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.z,
                    z: self.z,
                    w: self.y,
                }
            }
            
            fn xzzz(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.z,
                    z: self.z,
                    w: self.z,
                }
            }
            
            fn xzzw(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.z,
                    z: self.z,
                    w: self.w,
                }
            }
            
            fn xzwx(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.z,
                    z: self.w,
                    w: self.x,
                }
            }
            
            fn xzwy(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.z,
                    z: self.w,
                    w: self.y,
                }
            }
            
            fn xzwz(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.z,
                    z: self.w,
                    w: self.z,
                }
            }
            
            fn xzww(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.z,
                    z: self.w,
                    w: self.w,
                }
            }
            
            fn xwxx(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.w,
                    z: self.x,
                    w: self.x,
                }
            }
            
            fn xwxy(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.w,
                    z: self.x,
                    w: self.y,
                }
            }
            
            fn xwxz(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.w,
                    z: self.x,
                    w: self.z,
                }
            }
            
            fn xwxw(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.w,
                    z: self.x,
                    w: self.w,
                }
            }
            
            fn xwyx(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.w,
                    z: self.y,
                    w: self.x,
                }
            }
            
            fn xwyy(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.w,
                    z: self.y,
                    w: self.y,
                }
            }
            
            fn xwyz(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.w,
                    z: self.y,
                    w: self.z,
                }
            }
            
            fn xwyw(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.w,
                    z: self.y,
                    w: self.w,
                }
            }
            
            fn xwzx(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.w,
                    z: self.z,
                    w: self.x,
                }
            }
            
            fn xwzy(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.w,
                    z: self.z,
                    w: self.y,
                }
            }
            
            fn xwzz(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.w,
                    z: self.z,
                    w: self.z,
                }
            }
            
            fn xwzw(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.w,
                    z: self.z,
                    w: self.w,
                }
            }
            
            fn xwwx(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.w,
                    z: self.w,
                    w: self.x,
                }
            }
            
            fn xwwy(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.w,
                    z: self.w,
                    w: self.y,
                }
            }
            
            fn xwwz(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.w,
                    z: self.w,
                    w: self.z,
                }
            }
            
            fn xwww(self) -> Vec4<T> {
                Vec4 {
                    x: self.x,
                    y: self.w,
                    z: self.w,
                    w: self.w,
                }
            }
            
            fn yxxx(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.x,
                    z: self.x,
                    w: self.x,
                }
            }
            
            fn yxxy(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.x,
                    z: self.x,
                    w: self.y,
                }
            }
            
            fn yxxz(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.x,
                    z: self.x,
                    w: self.z,
                }
            }
            
            fn yxxw(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.x,
                    z: self.x,
                    w: self.w,
                }
            }
            
            fn yxyx(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.x,
                    z: self.y,
                    w: self.x,
                }
            }
            
            fn yxyy(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.x,
                    z: self.y,
                    w: self.y,
                }
            }
            
            fn yxyz(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.x,
                    z: self.y,
                    w: self.z,
                }
            }
            
            fn yxyw(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.x,
                    z: self.y,
                    w: self.w,
                }
            }
            
            fn yxzx(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.x,
                    z: self.z,
                    w: self.x,
                }
            }
            
            fn yxzy(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.x,
                    z: self.z,
                    w: self.y,
                }
            }
            
            fn yxzz(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.x,
                    z: self.z,
                    w: self.z,
                }
            }
            
            fn yxzw(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.x,
                    z: self.z,
                    w: self.w,
                }
            }
            
            fn yxwx(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.x,
                    z: self.w,
                    w: self.x,
                }
            }
            
            fn yxwy(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.x,
                    z: self.w,
                    w: self.y,
                }
            }
            
            fn yxwz(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.x,
                    z: self.w,
                    w: self.z,
                }
            }
            
            fn yxww(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.x,
                    z: self.w,
                    w: self.w,
                }
            }
            
            fn yyxx(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.y,
                    z: self.x,
                    w: self.x,
                }
            }
            
            fn yyxy(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.y,
                    z: self.x,
                    w: self.y,
                }
            }
            
            fn yyxz(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.y,
                    z: self.x,
                    w: self.z,
                }
            }
            
            fn yyxw(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.y,
                    z: self.x,
                    w: self.w,
                }
            }
            
            fn yyyx(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.y,
                    z: self.y,
                    w: self.x,
                }
            }
            
            fn yyyy(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.y,
                    z: self.y,
                    w: self.y,
                }
            }
            
            fn yyyz(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.y,
                    z: self.y,
                    w: self.z,
                }
            }
            
            fn yyyw(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.y,
                    z: self.y,
                    w: self.w,
                }
            }
            
            fn yyzx(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.y,
                    z: self.z,
                    w: self.x,
                }
            }
            
            fn yyzy(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.y,
                    z: self.z,
                    w: self.y,
                }
            }
            
            fn yyzz(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.y,
                    z: self.z,
                    w: self.z,
                }
            }
            
            fn yyzw(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.y,
                    z: self.z,
                    w: self.w,
                }
            }
            
            fn yywx(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.y,
                    z: self.w,
                    w: self.x,
                }
            }
            
            fn yywy(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.y,
                    z: self.w,
                    w: self.y,
                }
            }
            
            fn yywz(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.y,
                    z: self.w,
                    w: self.z,
                }
            }
            
            fn yyww(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.y,
                    z: self.w,
                    w: self.w,
                }
            }
            
            fn yzxx(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.z,
                    z: self.x,
                    w: self.x,
                }
            }
            
            fn yzxy(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.z,
                    z: self.x,
                    w: self.y,
                }
            }
            
            fn yzxz(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.z,
                    z: self.x,
                    w: self.z,
                }
            }
            
            fn yzxw(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.z,
                    z: self.x,
                    w: self.w,
                }
            }
            
            fn yzyx(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.z,
                    z: self.y,
                    w: self.x,
                }
            }
            
            fn yzyy(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.z,
                    z: self.y,
                    w: self.y,
                }
            }
            
            fn yzyz(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.z,
                    z: self.y,
                    w: self.z,
                }
            }
            
            fn yzyw(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.z,
                    z: self.y,
                    w: self.w,
                }
            }
            
            fn yzzx(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.z,
                    z: self.z,
                    w: self.x,
                }
            }
            
            fn yzzy(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.z,
                    z: self.z,
                    w: self.y,
                }
            }
            
            fn yzzz(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.z,
                    z: self.z,
                    w: self.z,
                }
            }
            
            fn yzzw(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.z,
                    z: self.z,
                    w: self.w,
                }
            }
            
            fn yzwx(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.z,
                    z: self.w,
                    w: self.x,
                }
            }
            
            fn yzwy(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.z,
                    z: self.w,
                    w: self.y,
                }
            }
            
            fn yzwz(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.z,
                    z: self.w,
                    w: self.z,
                }
            }
            
            fn yzww(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.z,
                    z: self.w,
                    w: self.w,
                }
            }
            
            fn ywxx(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.w,
                    z: self.x,
                    w: self.x,
                }
            }
            
            fn ywxy(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.w,
                    z: self.x,
                    w: self.y,
                }
            }
            
            fn ywxz(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.w,
                    z: self.x,
                    w: self.z,
                }
            }
            
            fn ywxw(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.w,
                    z: self.x,
                    w: self.w,
                }
            }
            
            fn ywyx(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.w,
                    z: self.y,
                    w: self.x,
                }
            }
            
            fn ywyy(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.w,
                    z: self.y,
                    w: self.y,
                }
            }
            
            fn ywyz(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.w,
                    z: self.y,
                    w: self.z,
                }
            }
            
            fn ywyw(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.w,
                    z: self.y,
                    w: self.w,
                }
            }
            
            fn ywzx(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.w,
                    z: self.z,
                    w: self.x,
                }
            }
            
            fn ywzy(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.w,
                    z: self.z,
                    w: self.y,
                }
            }
            
            fn ywzz(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.w,
                    z: self.z,
                    w: self.z,
                }
            }
            
            fn ywzw(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.w,
                    z: self.z,
                    w: self.w,
                }
            }
            
            fn ywwx(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.w,
                    z: self.w,
                    w: self.x,
                }
            }
            
            fn ywwy(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.w,
                    z: self.w,
                    w: self.y,
                }
            }
            
            fn ywwz(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.w,
                    z: self.w,
                    w: self.z,
                }
            }
            
            fn ywww(self) -> Vec4<T> {
                Vec4 {
                    x: self.y,
                    y: self.w,
                    z: self.w,
                    w: self.w,
                }
            }
            
            fn zxxx(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.x,
                    z: self.x,
                    w: self.x,
                }
            }
            
            fn zxxy(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.x,
                    z: self.x,
                    w: self.y,
                }
            }
            
            fn zxxz(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.x,
                    z: self.x,
                    w: self.z,
                }
            }
            
            fn zxxw(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.x,
                    z: self.x,
                    w: self.w,
                }
            }
            
            fn zxyx(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.x,
                    z: self.y,
                    w: self.x,
                }
            }
            
            fn zxyy(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.x,
                    z: self.y,
                    w: self.y,
                }
            }
            
            fn zxyz(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.x,
                    z: self.y,
                    w: self.z,
                }
            }
            
            fn zxyw(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.x,
                    z: self.y,
                    w: self.w,
                }
            }
            
            fn zxzx(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.x,
                    z: self.z,
                    w: self.x,
                }
            }
            
            fn zxzy(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.x,
                    z: self.z,
                    w: self.y,
                }
            }
            
            fn zxzz(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.x,
                    z: self.z,
                    w: self.z,
                }
            }
            
            fn zxzw(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.x,
                    z: self.z,
                    w: self.w,
                }
            }
            
            fn zxwx(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.x,
                    z: self.w,
                    w: self.x,
                }
            }
            
            fn zxwy(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.x,
                    z: self.w,
                    w: self.y,
                }
            }
            
            fn zxwz(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.x,
                    z: self.w,
                    w: self.z,
                }
            }
            
            fn zxww(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.x,
                    z: self.w,
                    w: self.w,
                }
            }
            
            fn zyxx(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.y,
                    z: self.x,
                    w: self.x,
                }
            }
            
            fn zyxy(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.y,
                    z: self.x,
                    w: self.y,
                }
            }
            
            fn zyxz(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.y,
                    z: self.x,
                    w: self.z,
                }
            }
            
            fn zyxw(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.y,
                    z: self.x,
                    w: self.w,
                }
            }
            
            fn zyyx(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.y,
                    z: self.y,
                    w: self.x,
                }
            }
            
            fn zyyy(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.y,
                    z: self.y,
                    w: self.y,
                }
            }
            
            fn zyyz(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.y,
                    z: self.y,
                    w: self.z,
                }
            }
            
            fn zyyw(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.y,
                    z: self.y,
                    w: self.w,
                }
            }
            
            fn zyzx(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.y,
                    z: self.z,
                    w: self.x,
                }
            }
            
            fn zyzy(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.y,
                    z: self.z,
                    w: self.y,
                }
            }
            
            fn zyzz(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.y,
                    z: self.z,
                    w: self.z,
                }
            }
            
            fn zyzw(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.y,
                    z: self.z,
                    w: self.w,
                }
            }
            
            fn zywx(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.y,
                    z: self.w,
                    w: self.x,
                }
            }
            
            fn zywy(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.y,
                    z: self.w,
                    w: self.y,
                }
            }
            
            fn zywz(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.y,
                    z: self.w,
                    w: self.z,
                }
            }
            
            fn zyww(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.y,
                    z: self.w,
                    w: self.w,
                }
            }
            
            fn zzxx(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.z,
                    z: self.x,
                    w: self.x,
                }
            }
            
            fn zzxy(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.z,
                    z: self.x,
                    w: self.y,
                }
            }
            
            fn zzxz(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.z,
                    z: self.x,
                    w: self.z,
                }
            }
            
            fn zzxw(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.z,
                    z: self.x,
                    w: self.w,
                }
            }
            
            fn zzyx(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.z,
                    z: self.y,
                    w: self.x,
                }
            }
            
            fn zzyy(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.z,
                    z: self.y,
                    w: self.y,
                }
            }
            
            fn zzyz(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.z,
                    z: self.y,
                    w: self.z,
                }
            }
            
            fn zzyw(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.z,
                    z: self.y,
                    w: self.w,
                }
            }
            
            fn zzzx(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.z,
                    z: self.z,
                    w: self.x,
                }
            }
            
            fn zzzy(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.z,
                    z: self.z,
                    w: self.y,
                }
            }
            
            fn zzzz(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.z,
                    z: self.z,
                    w: self.z,
                }
            }
            
            fn zzzw(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.z,
                    z: self.z,
                    w: self.w,
                }
            }
            
            fn zzwx(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.z,
                    z: self.w,
                    w: self.x,
                }
            }
            
            fn zzwy(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.z,
                    z: self.w,
                    w: self.y,
                }
            }
            
            fn zzwz(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.z,
                    z: self.w,
                    w: self.z,
                }
            }
            
            fn zzww(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.z,
                    z: self.w,
                    w: self.w,
                }
            }
            
            fn zwxx(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.w,
                    z: self.x,
                    w: self.x,
                }
            }
            
            fn zwxy(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.w,
                    z: self.x,
                    w: self.y,
                }
            }
            
            fn zwxz(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.w,
                    z: self.x,
                    w: self.z,
                }
            }
            
            fn zwxw(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.w,
                    z: self.x,
                    w: self.w,
                }
            }
            
            fn zwyx(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.w,
                    z: self.y,
                    w: self.x,
                }
            }
            
            fn zwyy(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.w,
                    z: self.y,
                    w: self.y,
                }
            }
            
            fn zwyz(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.w,
                    z: self.y,
                    w: self.z,
                }
            }
            
            fn zwyw(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.w,
                    z: self.y,
                    w: self.w,
                }
            }
            
            fn zwzx(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.w,
                    z: self.z,
                    w: self.x,
                }
            }
            
            fn zwzy(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.w,
                    z: self.z,
                    w: self.y,
                }
            }
            
            fn zwzz(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.w,
                    z: self.z,
                    w: self.z,
                }
            }
            
            fn zwzw(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.w,
                    z: self.z,
                    w: self.w,
                }
            }
            
            fn zwwx(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.w,
                    z: self.w,
                    w: self.x,
                }
            }
            
            fn zwwy(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.w,
                    z: self.w,
                    w: self.y,
                }
            }
            
            fn zwwz(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.w,
                    z: self.w,
                    w: self.z,
                }
            }
            
            fn zwww(self) -> Vec4<T> {
                Vec4 {
                    x: self.z,
                    y: self.w,
                    z: self.w,
                    w: self.w,
                }
            }
            
            fn wxxx(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.x,
                    z: self.x,
                    w: self.x,
                }
            }
            
            fn wxxy(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.x,
                    z: self.x,
                    w: self.y,
                }
            }
            
            fn wxxz(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.x,
                    z: self.x,
                    w: self.z,
                }
            }
            
            fn wxxw(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.x,
                    z: self.x,
                    w: self.w,
                }
            }
            
            fn wxyx(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.x,
                    z: self.y,
                    w: self.x,
                }
            }
            
            fn wxyy(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.x,
                    z: self.y,
                    w: self.y,
                }
            }
            
            fn wxyz(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.x,
                    z: self.y,
                    w: self.z,
                }
            }
            
            fn wxyw(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.x,
                    z: self.y,
                    w: self.w,
                }
            }
            
            fn wxzx(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.x,
                    z: self.z,
                    w: self.x,
                }
            }
            
            fn wxzy(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.x,
                    z: self.z,
                    w: self.y,
                }
            }
            
            fn wxzz(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.x,
                    z: self.z,
                    w: self.z,
                }
            }
            
            fn wxzw(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.x,
                    z: self.z,
                    w: self.w,
                }
            }
            
            fn wxwx(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.x,
                    z: self.w,
                    w: self.x,
                }
            }
            
            fn wxwy(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.x,
                    z: self.w,
                    w: self.y,
                }
            }
            
            fn wxwz(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.x,
                    z: self.w,
                    w: self.z,
                }
            }
            
            fn wxww(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.x,
                    z: self.w,
                    w: self.w,
                }
            }
            
            fn wyxx(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.y,
                    z: self.x,
                    w: self.x,
                }
            }
            
            fn wyxy(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.y,
                    z: self.x,
                    w: self.y,
                }
            }
            
            fn wyxz(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.y,
                    z: self.x,
                    w: self.z,
                }
            }
            
            fn wyxw(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.y,
                    z: self.x,
                    w: self.w,
                }
            }
            
            fn wyyx(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.y,
                    z: self.y,
                    w: self.x,
                }
            }
            
            fn wyyy(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.y,
                    z: self.y,
                    w: self.y,
                }
            }
            
            fn wyyz(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.y,
                    z: self.y,
                    w: self.z,
                }
            }
            
            fn wyyw(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.y,
                    z: self.y,
                    w: self.w,
                }
            }
            
            fn wyzx(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.y,
                    z: self.z,
                    w: self.x,
                }
            }
            
            fn wyzy(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.y,
                    z: self.z,
                    w: self.y,
                }
            }
            
            fn wyzz(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.y,
                    z: self.z,
                    w: self.z,
                }
            }
            
            fn wyzw(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.y,
                    z: self.z,
                    w: self.w,
                }
            }
            
            fn wywx(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.y,
                    z: self.w,
                    w: self.x,
                }
            }
            
            fn wywy(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.y,
                    z: self.w,
                    w: self.y,
                }
            }
            
            fn wywz(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.y,
                    z: self.w,
                    w: self.z,
                }
            }
            
            fn wyww(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.y,
                    z: self.w,
                    w: self.w,
                }
            }
            
            fn wzxx(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.z,
                    z: self.x,
                    w: self.x,
                }
            }
            
            fn wzxy(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.z,
                    z: self.x,
                    w: self.y,
                }
            }
            
            fn wzxz(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.z,
                    z: self.x,
                    w: self.z,
                }
            }
            
            fn wzxw(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.z,
                    z: self.x,
                    w: self.w,
                }
            }
            
            fn wzyx(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.z,
                    z: self.y,
                    w: self.x,
                }
            }
            
            fn wzyy(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.z,
                    z: self.y,
                    w: self.y,
                }
            }
            
            fn wzyz(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.z,
                    z: self.y,
                    w: self.z,
                }
            }
            
            fn wzyw(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.z,
                    z: self.y,
                    w: self.w,
                }
            }
            
            fn wzzx(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.z,
                    z: self.z,
                    w: self.x,
                }
            }
            
            fn wzzy(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.z,
                    z: self.z,
                    w: self.y,
                }
            }
            
            fn wzzz(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.z,
                    z: self.z,
                    w: self.z,
                }
            }
            
            fn wzzw(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.z,
                    z: self.z,
                    w: self.w,
                }
            }
            
            fn wzwx(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.z,
                    z: self.w,
                    w: self.x,
                }
            }
            
            fn wzwy(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.z,
                    z: self.w,
                    w: self.y,
                }
            }
            
            fn wzwz(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.z,
                    z: self.w,
                    w: self.z,
                }
            }
            
            fn wzww(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.z,
                    z: self.w,
                    w: self.w,
                }
            }
            
            fn wwxx(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.w,
                    z: self.x,
                    w: self.x,
                }
            }
            
            fn wwxy(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.w,
                    z: self.x,
                    w: self.y,
                }
            }
            
            fn wwxz(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.w,
                    z: self.x,
                    w: self.z,
                }
            }
            
            fn wwxw(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.w,
                    z: self.x,
                    w: self.w,
                }
            }
            
            fn wwyx(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.w,
                    z: self.y,
                    w: self.x,
                }
            }
            
            fn wwyy(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.w,
                    z: self.y,
                    w: self.y,
                }
            }
            
            fn wwyz(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.w,
                    z: self.y,
                    w: self.z,
                }
            }
            
            fn wwyw(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.w,
                    z: self.y,
                    w: self.w,
                }
            }
            
            fn wwzx(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.w,
                    z: self.z,
                    w: self.x,
                }
            }
            
            fn wwzy(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.w,
                    z: self.z,
                    w: self.y,
                }
            }
            
            fn wwzz(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.w,
                    z: self.z,
                    w: self.z,
                }
            }
            
            fn wwzw(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.w,
                    z: self.z,
                    w: self.w,
                }
            }
            
            fn wwwx(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.w,
                    z: self.w,
                    w: self.x,
                }
            }
            
            fn wwwy(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.w,
                    z: self.w,
                    w: self.y,
                }
            }
            
            fn wwwz(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.w,
                    z: self.w,
                    w: self.z,
                }
            }
            
            fn wwww(self) -> Vec4<T> {
                Vec4 {
                    x: self.w,
                    y: self.w,
                    z: self.w,
                    w: self.w,
                }
            }

            fn set_xw(&mut self, other: Vec2<T>) {
                self.x = other.x; self.w = other.y;
            }

            fn set_yw(&mut self, other: Vec2<T>) {
                self.y = other.x; self.w = other.y;
            }

            fn set_zw(&mut self, other: Vec2<T>) {
                self.z = other.x; self.w = other.y;
            }

            fn set_wx(&mut self, other: Vec2<T>) {
                self.w = other.x; self.x = other.y;
            }

            fn set_wy(&mut self, other: Vec2<T>) {
                self.w = other.x; self.y = other.y;
            }

            fn set_wz(&mut self, other: Vec2<T>) {
                self.w = other.x; self.z = other.y;
            }

              
            // output v4-v3
            fn set_xyw(&mut self, other: Vec3<T>) {
                self.x = other.x; self.y = other.y; self.w = other.z;
            }
            
            fn set_xzw(&mut self, other: Vec3<T>) {
                self.x = other.x; self.z = other.y; self.w = other.z;
            }
            
            fn set_xwy(&mut self, other: Vec3<T>) {
                self.x = other.x; self.w = other.y; self.y = other.z;
            }
            
            fn set_xwz(&mut self, other: Vec3<T>) {
                self.x = other.x; self.w = other.y; self.z = other.z;
            }
            
            fn set_yxw(&mut self, other: Vec3<T>) {
                self.y = other.x; self.x = other.y; self.w = other.z;
            }
            
            fn set_yzw(&mut self, other: Vec3<T>) {
                self.y = other.x; self.z = other.y; self.w = other.z;
            }
            
            fn set_ywx(&mut self, other: Vec3<T>) {
                self.y = other.x; self.w = other.y; self.x = other.z;
            }
            
            fn set_ywz(&mut self, other: Vec3<T>) {
                self.y = other.x; self.w = other.y; self.z = other.z;
            }
            
            fn set_zxw(&mut self, other: Vec3<T>) {
                self.z = other.x; self.x = other.y; self.w = other.z;
            }
            
            fn set_zyw(&mut self, other: Vec3<T>) {
                self.z = other.x; self.y = other.y; self.w = other.z;
            }
            
            fn set_zwx(&mut self, other: Vec3<T>) {
                self.z = other.x; self.w = other.y; self.x = other.z;
            }
            
            fn set_zwy(&mut self, other: Vec3<T>) {
                self.z = other.x; self.w = other.y; self.y = other.z;
            }
            
            fn set_wxy(&mut self, other: Vec3<T>) {
                self.w = other.x; self.x = other.y; self.y = other.z;
            }
            
            fn set_wxz(&mut self, other: Vec3<T>) {
                self.w = other.x; self.x = other.y; self.z = other.z;
            }
            
            fn set_wyx(&mut self, other: Vec3<T>) {
                self.w = other.x; self.y = other.y; self.x = other.z;
            }
            
            fn set_wyz(&mut self, other: Vec3<T>) {
                self.w = other.x; self.y = other.y; self.z = other.z;
            }
            
            fn set_wzx(&mut self, other: Vec3<T>) {
                self.w = other.x; self.z = other.y; self.x = other.z;
            }
            
            fn set_wzy(&mut self, other: Vec3<T>) {
                self.w = other.x; self.z = other.y; self.y = other.z;
            }

            fn set_xyzw(&mut self, other: Vec4<T>) {
                self.x = other.x;
                self.y = other.y;
                self.z = other.z;
                self.w = other.w;
            }
            
            fn set_xywz(&mut self, other: Vec4<T>) {
                self.x = other.x;
                self.y = other.y;
                self.z = other.w;
                self.w = other.z;
            }
            
            fn set_xzyw(&mut self, other: Vec4<T>) {
                self.x = other.x;
                self.y = other.z;
                self.z = other.y;
                self.w = other.w;
            }
            
            fn set_xzwy(&mut self, other: Vec4<T>) {
                self.x = other.x;
                self.y = other.z;
                self.z = other.w;
                self.w = other.y;
            }
            
            fn set_xwyz(&mut self, other: Vec4<T>) {
                self.x = other.x;
                self.y = other.w;
                self.z = other.y;
                self.w = other.z;
            }
            
            fn set_xwzy(&mut self, other: Vec4<T>) {
                self.x = other.x;
                self.y = other.w;
                self.z = other.z;
                self.w = other.y;
            }
            
            fn set_yxzw(&mut self, other: Vec4<T>) {
                self.x = other.y;
                self.y = other.x;
                self.z = other.z;
                self.w = other.w;
            }
            
            fn set_yxwz(&mut self, other: Vec4<T>) {
                self.x = other.y;
                self.y = other.x;
                self.z = other.w;
                self.w = other.z;
            }
            
            fn set_yzxw(&mut self, other: Vec4<T>) {
                self.x = other.y;
                self.y = other.z;
                self.z = other.x;
                self.w = other.w;
            }
            
            fn set_yzwx(&mut self, other: Vec4<T>) {
                self.x = other.y;
                self.y = other.z;
                self.z = other.w;
                self.w = other.x;
            }
            
            fn set_ywxz(&mut self, other: Vec4<T>) {
                self.x = other.y;
                self.y = other.w;
                self.z = other.x;
                self.w = other.z;
            }
            
            fn set_ywzx(&mut self, other: Vec4<T>) {
                self.x = other.y;
                self.y = other.w;
                self.z = other.z;
                self.w = other.x;
            }
            
            fn set_zxyw(&mut self, other: Vec4<T>) {
                self.x = other.z;
                self.y = other.x;
                self.z = other.y;
                self.w = other.w;
            }
            
            fn set_zxwy(&mut self, other: Vec4<T>) {
                self.x = other.z;
                self.y = other.x;
                self.z = other.w;
                self.w = other.y;
            }
            
            fn set_zyxw(&mut self, other: Vec4<T>) {
                self.x = other.z;
                self.y = other.y;
                self.z = other.x;
                self.w = other.w;
            }
            
            fn set_zywx(&mut self, other: Vec4<T>) {
                self.x = other.z;
                self.y = other.y;
                self.z = other.w;
                self.w = other.x;
            }
            
            fn set_zwxy(&mut self, other: Vec4<T>) {
                self.x = other.z;
                self.y = other.w;
                self.z = other.x;
                self.w = other.y;
            }
            
            fn set_zwyx(&mut self, other: Vec4<T>) {
                self.x = other.z;
                self.y = other.w;
                self.z = other.y;
                self.w = other.x;
            }
            
            fn set_wxyz(&mut self, other: Vec4<T>) {
                self.x = other.w;
                self.y = other.x;
                self.z = other.y;
                self.w = other.z;
            }
            
            fn set_wxzy(&mut self, other: Vec4<T>) {
                self.x = other.w;
                self.y = other.x;
                self.z = other.z;
                self.w = other.y;
            }
            
            fn set_wyxz(&mut self, other: Vec4<T>) {
                self.x = other.w;
                self.y = other.y;
                self.z = other.x;
                self.w = other.z;
            }
            
            fn set_wyzx(&mut self, other: Vec4<T>) {
                self.x = other.w;
                self.y = other.y;
                self.z = other.z;
                self.w = other.x;
            }
            
            fn set_wzxy(&mut self, other: Vec4<T>) {
                self.x = other.w;
                self.y = other.z;
                self.z = other.x;
                self.w = other.y;
            }
            
            fn set_wzyx(&mut self, other: Vec4<T>) {
                self.x = other.w;
                self.y = other.z;
                self.z = other.y;
                self.w = other.x;
            }
        }
    }
}

v2_swizzle_impl!(Vec2);
v2_swizzle_impl!(Vec3);
v2_swizzle_impl!(Vec4);
v3_swizzle_impl!(Vec3);
v3_swizzle_impl!(Vec4);
v4_swizzle_impl!(Vec4);