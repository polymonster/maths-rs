// use crate::vec::*;
// use crate::num::*;

// pub type Quat<T> = Vec4<T>;

/*
impl<T> Slerp<T> for Quat<T> where T: Float + FloatOps<T> + NumberOps<T> {
    fn slerp(e0: Self, e1: Self, t: T) -> Self {
        T w1, x1, y1, z1, w2, x2, y2, z2;
        T theta, mult1, mult2;

        w1 = q1.w; x1 = q1.x; y1 = q1.y; z1 = q1.z;
        w2 = q2.w; x2 = q2.x; y2 = q2.y; z2 = q2.z;

        // reverse the sign of q2 if q1.q2 < 0.
        if (w1*w2 + x1*x2 + y1*y2 + z1*z2 < 0)
        {
            w2 = -w2; x2 = -x2; y2 = -y2; z2 = -z2;
        }
        
        theta = acos(w1*w2 + x1*x2 + y1*y2 + z1*z2);

        constexpr T k_epsilon = (T)0.000001;
        if (theta > k_epsilon)
        {
            mult1 = sin( (1-t)*theta ) / sin( theta );
            mult2 = sin( t*theta ) / sin( theta );
        }
        else
        {
            mult1 = 1 - t;
            mult2 = t;
        }

        Quat<T> out_quat;
        out_quat.w = mult1*w1 + mult2*w2;
        out_quat.x = mult1*x1 + mult2*x2;
        out_quat.y = mult1*y1 + mult2*y2;
        out_quat.z = mult1*z1 + mult2*z2;

        return out_quat;
    }
}
*/