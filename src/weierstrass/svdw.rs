use num_traits::identities::Zero;

use crate::ellipticcurve::{EllipticCurve, MapToCurve};
use crate::field::{CMov, Field, Sgn0, Sqrt};
use crate::ops::FromFactory;
use crate::primefield::FpElt;
use crate::weierstrass::Curve;

pub struct SVDW {
    e: Curve,
    c1: FpElt,
    c2: FpElt,
    c3: FpElt,
    c4: FpElt,
    z: FpElt,
}

impl SVDW {
    pub fn new(e: Curve, z: FpElt) -> SVDW {
        if !SVDW::verify(&e, &z) {
            panic!("wrong input parameters")
        } else {
            let f = e.get_field();
            let (f2, f3, f4) = (f.from(2u32), f.from(3u32), f.from(4u32));
            let gz = -SVDW::gx(&e, &z);
            let c1 = -&gz;
            let c2 = -&z * (1u32 / &f2);
            let t0 = (f3 * (&z ^ 2u32)) + &(&f4 * &e.a);
            let mut c3 = (&gz * &t0).sqrt();
            if c3.sgn0() == -1 {
                c3 = -c3;
            }
            let c4 = (f4 * gz) * (1u32 / &t0);
            SVDW {
                e,
                c1,
                c2,
                c3,
                c4,
                z,
            }
        }
    }
    fn gx(e: &Curve, x: &FpElt) -> FpElt {
        x * &((x ^ 2u32) + &e.a) + &e.b
    }
    fn verify(e: &Curve, z: &FpElt) -> bool {
        let f = e.get_field();
        let (f2, f3, f4) = (f.from(2u32), f.from(3u32), f.from(4u32));
        let gz = SVDW::gx(e, z);
        let gz2 = SVDW::gx(e, &((-z) * (1u32 / &f2)));
        let t0 = -(f3 * (z ^ 2u32) + &f4 * &e.a) * (1u32 / &(&f4 * &gz));
        let cond1 = !gz.is_zero(); //   g(Z) != 0
        let cond2 = !t0.is_zero(); //   -(3 * Z^2 + 4 * A) / (4 * g(Z)) != 0
        let cond3 = t0.is_square(); //  -(3 * Z^2 + 4 * A) / (4 * g(Z)) is square
        let cond4a = gz.is_square(); // At least one of g(Z) and g(-Z / 2) is square
        let cond4b = gz2.is_square();

        cond1 && cond2 && cond3 && (cond4a || cond4b)
    }
}

impl MapToCurve for SVDW {
    type E = Curve;
    fn map(
        &self,
        u: &<<Self::E as EllipticCurve>::F as Field>::Elt,
    ) -> <Self::E as EllipticCurve>::Point {
        let f = self.e.get_field();
        let cmov = FpElt::cmov;
        let mut t1 = u ^ 2u32; //           1.   t1 = u^2
        t1 = t1 * &self.c1; //              2.   t1 = t1 * c1
        let t2 = f.one() + &t1; //          3.   t2 = 1 + t1
        t1 = f.one() - &t1; //              4.   t1 = 1 - t1
        let mut t3 = &t1 * &t2; //          5.   t3 = t1 * t2
        t3 = 1u32 / &t3; //                 6.   t3 = inv0(t3)
        let mut t4 = u * &t1; //            7.   t4 = u * t1
        t4 = t4 * &t3; //                   8.   t4 = t4 * t3
        t4 = t4 * &self.c3; //              9.   t4 = t4 * c3
        let x1 = &self.c2 - &t4; //         10.  x1 = c2 - t4
        let mut gx1 = &x1 ^ 2u32; //        11. gx1 = x1^2
        gx1 = gx1 + &self.e.a; //           12. gx1 = gx1 + A
        gx1 = gx1 * &x1; //                 13. gx1 = gx1 * x1
        gx1 = gx1 + &self.e.b; //           14. gx1 = gx1 + B
        let e1 = gx1.is_square(); //        15.  e1 = is_square(gx1)
        let x2 = &self.c2 + &t4; //         16.  x2 = c2 + t4
        let mut gx2 = &x2 ^ 2u32; //        17. gx2 = x2^2
        gx2 = gx2 + &self.e.a; //           18. gx2 = gx2 + A
        gx2 = gx2 * &x2; //                 19. gx2 = gx2 * x2
        gx2 = gx2 + &self.e.b; //           20. gx2 = gx2 + B
        let e2 = gx2.is_square() && !e1; // 21.  e2 = is_square(gx2) AND NOT e1     // Avoid short-circuit logic ops
        let mut x3 = &t2 ^ 2u32; //         22.  x3 = t2^2
        x3 = x3 * t3; //                    23.  x3 = x3 * t3
        x3 = &x3 ^ 2u32; //                 24.  x3 = x3^2
        x3 = x3 * &self.c4; //              25.  x3 = x3 * c4
        x3 = x3 + &self.z; //               26.  x3 = x3 + Z
        let mut x = cmov(&x3, &x1, e1); //  27.   x = CMOV(x3, x1, e1)      // x = x1 if gx1 is square, else x = x3
        x = cmov(&x, &x2, e2); //           28.   x = CMOV(x, x2, e2)       // x = x2 if gx2 is square and gx1 is not
        let mut gx = &x ^ 2u32; //          29.  gx = x^2
        gx = gx + &self.e.a; //             30.  gx = gx + A
        gx = gx * &x; //                    31.  gx = gx * x
        gx = gx + &self.e.b; //             32.  gx = gx + B
        let mut y = gx.sqrt(); //           33.   y = sqrt(gx)
        let e3 = u.sgn0() == y.sgn0(); //   34.  e3 = sgn0(u) == sgn0(y)
        y = cmov(&(-&y), &y, e3); //        35.   y = CMOV(-y, y, e3)
        self.e.new_point(x, y)
    }
}
