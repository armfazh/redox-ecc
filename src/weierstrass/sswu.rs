use num_traits::identities::Zero;

use crate::ellipticcurve::{EllipticCurve, MapToCurve};
use crate::field::{CMov, Field, Sgn0, Sqrt};
use crate::ops::FromFactory;
use crate::primefield::FpElt;
use crate::weierstrass::Curve;

#[derive(Clone)]
pub struct SSWU {
    e: Curve,
    c1: FpElt,
    c2: FpElt,
    z: FpElt,
}

impl SSWU {
    pub fn new(e: Curve, z: FpElt) -> SSWU {
        if !SSWU::verify(&e, &z) {
            panic!("wrong input parameters")
        } else {
            let c1 = -&e.b * (1u32 / &e.a);
            let c2 = -(1u32 / &z);
            SSWU { e, c1, c2, z }
        }
    }
    fn verify(e: &Curve, z: &FpElt) -> bool {
        let precond1 = !e.a.is_zero(); //              A != 0
        let precond2 = !e.b.is_zero(); //              B != 0
        let cond1 = !z.is_square(); //                 Z is non-square
        let cond2 = *z != e.get_field().from(-1); //               Z != -1
        let x = &e.b * &(1u32 / &(z * &e.a)); //       B/(Z*A)
        let gx = &x * &((&x ^ 2u32) + &e.a) + &e.b; // g(B/(Z*A))
        let cond4 = gx.is_square(); //                 g(B/(Z*A)) is square
        precond1 && precond2 && cond1 && cond2 && cond4
    }
}

impl MapToCurve for SSWU {
    type E = Curve;
    fn map(
        &self,
        u: &<<Self::E as EllipticCurve>::F as Field>::Elt,
    ) -> <Self::E as EllipticCurve>::Point {
        let f = self.e.get_field();
        let cmov = FpElt::cmov;
        let mut t1 = u ^ 2u32; //         0.   t1 = u^2
        t1 = &self.z * &t1; //            1.   t1 = Z * u^2
        let mut t2 = &t1 ^ 2u32; //       2.   t2 = t1^2
        let mut x1 = &t1 + &t2; //        3.   x1 = t1 + t2
        x1 = 1u32 / &x1; //               4.   x1 = inv0(x1)
        let e1 = x1.is_zero(); //         5.   e1 = x1 == 0
        x1 = x1 + f.one(); //             6.   x1 = x1 + 1
        x1 = cmov(&x1, &self.c2, e1); //  7.   x1 = CMOV(x1, c2, e1)
        x1 = x1 * &self.c1; //            8.   x1 = x1 * c1
        let mut gx1 = &x1 ^ 2u32; //      9.  gx1 = x1^2
        gx1 = gx1 + &self.e.a; //         10. gx1 = gx1 + A
        gx1 = gx1 * &x1; //               11. gx1 = gx1 * x1
        gx1 = gx1 + &self.e.b; //         12. gx1 = gx1 + B
        let x2 = &t1 * &x1; //            13.  x2 = t1 * x1
        t2 = t1 * t2; //                  14.  t2 = t1 * t2
        let gx2 = &gx1 * &t2; //          15. gx2 = gx1 * t2
        let e2 = gx1.is_square(); //      16.  e2 = is_square(gx1)
        let x = cmov(&x2, &x1, e2); //    17.   x = CMOV(x2, x1, e2)
        let y2 = cmov(&gx2, &gx1, e2); // 18.  y2 = CMOV(gx2, gx1, e2)
        let mut y = y2.sqrt(); //         19.   y = sqrt(y2)
        let e3 = u.sgn0() == y.sgn0(); // 20.  e3 = sgn0(u) == sgn0(y)
        y = cmov(&(-&y), &y, e3); //      21.   y = CMOV(-y, y, e3)
        self.e.new_point(x, y)
    }
}
