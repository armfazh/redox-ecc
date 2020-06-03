use num_traits::identities::Zero;

use crate::ellipticcurve::{EllipticCurve, MapToCurve};
use crate::field::{CMov, Field, Sgn0, Sqrt};
use crate::montgomery::Curve;
use crate::ops::FromFactory;
use crate::primefield::FpElt;

pub struct Ell2 {
    e: Curve,
    z: FpElt,
    ca: FpElt,
    cb: FpElt,
}

impl Ell2 {
    pub fn new(e: Curve, z: FpElt) -> Ell2 {
        if !Ell2::verify(&e) {
            panic!("wrong input parameters")
        } else {
            let inb = 1u32 / &e.b;
            let ca = &e.a / &inb;
            let cb = inb ^ 2u32;
            Ell2 { e, z, ca, cb }
        }
    }
    fn verify(e: &Curve) -> bool {
        let cond1 = !e.a.is_zero();
        let cond2 = !e.b.is_zero();
        cond1 && cond2
    }
}

impl MapToCurve for Ell2 {
    type E = Curve;
    fn map(
        &self,
        u: &<<Self::E as EllipticCurve>::F as Field>::Elt,
    ) -> <Self::E as EllipticCurve>::Point {
        let f = self.e.get_field();
        let cmov = FpElt::cmov;
        let mut t1 = u ^ 2u32; //          1.   t1 = u^2
        t1 = &self.z * t1; //              2.   t1 = Z * t1              // Z * u^2
        let e1 = t1 == f.from(-1); //      3.   e1 = t1 == -1            // exceptional case: Z * u^2 == -1
        t1 = cmov(&t1, &f.zero(), e1); //  4.   t1 = CMOV(t1, 0, e1)     // if t1 == -1, set t1 = 0
        let mut x1 = &t1 + f.one(); //     5.   x1 = t1 + 1
        x1 = 1u32 / &x1; //                6.   x1 = inv0(x1)
        x1 = -&self.ca * &x1; //           7.   x1 = -A * x1             // x1 = -A / (1 + Z * u^2)
        let mut gx1 = &x1 + &self.ca; //   8.  gx1 = x1 + A
        gx1 = gx1 * &x1; //                9.  gx1 = gx1 * x1
        gx1 = gx1 + &self.cb; //           10. gx1 = gx1 + B
        gx1 = gx1 * &x1; //                11. gx1 = gx1 * x1            // gx1 = x1^3 + A * x1^2 + B * x1
        let x2 = -&x1 - &self.ca; //       12.  x2 = -x1 - A
        let gx2 = t1 * &gx1; //            13. gx2 = t1 * gx1
        let e2 = gx1.is_square(); //       14.  e2 = is_square(gx1)
        let mut x = cmov(&x2, &x1, e2); // 15.   x = CMOV(x2, x1, e2)    // If is_square(gx1), x = x1, else x = x2
        let y2 = cmov(&gx2, &gx1, e2); //  16.  y2 = CMOV(gx2, gx1, e2)  // If is_square(gx1), y2 = gx1, else y2 = gx2
        let mut y = y2.sqrt(); //          17.   y = sqrt(y2)
        let e3 = y.sgn0() == 1; //         18.  e3 = sgn0(y) == 1        // Fix sign of y
        y = cmov(&(-&y), &y, e2 ^ e3); //  19.   y = CMOV(-y, y, e2 xor e3)
        x = x * &self.e.b;
        y = y * &self.e.b;
        self.e.new_point(x, y)
    }
}
