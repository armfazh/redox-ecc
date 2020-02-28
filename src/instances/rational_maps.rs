use crate::edwards;
use crate::edwards::Curve as TeCurve;
use crate::edwards::Point as TePoint;
use crate::ellipticcurve::{EcPoint, EllipticCurve, RationalMap};
use crate::field::Field;
use crate::montgomery;
use crate::montgomery::Curve as MtCurve;
use crate::montgomery::Point as MtPoint;
use crate::primefield::FpElt;

pub struct Te2Mt25519 {
    e0: TeCurve,
    e1: MtCurve,
    invsqr_d: FpElt,
}

impl RationalMap for Te2Mt25519 {
    type E0 = TeCurve;
    type E1 = MtCurve;

    fn domain(&self) -> Self::E0 {
        self.e0.clone()
    }
    fn codomain(&self) -> Self::E1 {
        self.e1.clone()
    }
    fn push(&self, p: TePoint) -> MtPoint {
        if p.is_zero() {
            self.e1.identity()
        } else {
            let (x, y, z) = (&p.c.x, &p.c.y, &p.c.z);
            let t0 = z + y;
            let xx = x * &t0;
            let yy = &self.invsqr_d * z * t0;
            let zz = x * (z - y);
            self.e1.new_point(montgomery::ProyCoordinates {
                x: xx,
                y: yy,
                z: zz,
            })
        }
    }
    fn pull(&self, p: MtPoint) -> TePoint {
        if p.is_zero() {
            self.e0.identity()
        } else if p.is_two_torsion() {
            let f = &self.e0.f;
            let (x, y, t, z) = (f.zero(), -f.one(), f.zero(), f.one());
            self.e0.new_point(edwards::ProyCoordinates { x, y, t, z })
        } else {
            let (x, y, z) = (&p.c.x, &p.c.y, &p.c.z);
            let add = x + z;
            let sub = x - z;
            let xx = &self.invsqr_d * x * &add;
            let yy = y * &sub;
            let tt = &self.invsqr_d * x * sub;
            let zz = y * add;
            self.e0.new_point(edwards::ProyCoordinates {
                x: xx,
                y: yy,
                t: tt,
                z: zz,
            })
        }
    }
}
