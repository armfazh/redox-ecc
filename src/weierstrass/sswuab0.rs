use num_traits::identities::Zero;

use crate::ellipticcurve::{EllipticCurve, Isogeny, MapToCurve};
use crate::field::{Field, Sgn0Endianness};
use crate::primefield::FpElt;
use crate::weierstrass::{Curve, SSWU};

pub struct SSWUAB0 {
    // e: Curve,
    iso: Box<dyn Isogeny<E0 = Curve, E1 = Curve>>,
    sswu: Box<dyn MapToCurve<E = Curve>>,
}

impl SSWUAB0 {
    pub fn new(
        e: Curve,
        z: FpElt,
        sgn0: Sgn0Endianness,
        iso: Box<dyn Isogeny<E0 = Curve, E1 = Curve>>,
    ) -> SSWUAB0 {
        if !SSWUAB0::verify(&e, iso.as_ref()) {
            panic!("wrong input parameters")
        } else {
            let sswu = Box::new(SSWU::new(iso.domain(), z, sgn0));
            SSWUAB0 { iso, sswu }
        }
    }
    fn verify(e: &Curve, iso: &dyn Isogeny<E0 = Curve, E1 = Curve>) -> bool {
        let cond0 = *e == iso.codomain();
        let cond1 = e.a.is_zero(); // A == 0
        let cond2 = e.b.is_zero(); // B == 0
        cond0 && (cond1 ^ cond2) // A == 0 xor B == 0
    }
}

impl MapToCurve for SSWUAB0 {
    type E = Curve;
    fn map(
        &self,
        u: &<<Self::E as EllipticCurve>::F as Field>::Elt,
    ) -> <Self::E as EllipticCurve>::Point {
        self.iso.push(self.sswu.map(u))
    }
}
