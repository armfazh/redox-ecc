use crate::edwards;
use crate::edwards::Curve as EdCurve;
use crate::edwards::Point as TePoint;
use crate::ellipticcurve::{EcPoint, EllipticCurve, RationalMap};
use crate::field::Field;
use crate::instances::{GetCurve, CURVE25519, CURVE448, EDWARDS25519, EDWARDS448};
use crate::montgomery;
use crate::montgomery::Curve as MtCurve;
use crate::montgomery::Point as MtPoint;
use crate::ops::FromFactory;
use crate::primefield::FpElt;

pub fn edwards25519_to_curve25519() -> impl RationalMap<E0 = EdCurve, E1 = MtCurve> {
    let e0 = EDWARDS25519.get();
    let e1 = CURVE25519.get();
    let f = e0.get_field();
    let invsqr_d =
        f.from("6853475219497561581579357271197624642482790079785650197046958215289687604742");
    Ed2Mt25519 { e0, e1, invsqr_d }
}

struct Ed2Mt25519 {
    e0: EdCurve,
    e1: MtCurve,
    invsqr_d: FpElt,
}

impl RationalMap for Ed2Mt25519 {
    type E0 = EdCurve;
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
            self.e1.new_proy_point(montgomery::ProyCoordinates {
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
            let ff = self.e0.get_field();
            self.e0.new_proy_point(edwards::ProyCoordinates {
                x: ff.zero(),
                y: -ff.one(),
                t: ff.zero(),
                z: ff.one(),
            })
        } else {
            let (x, y, z) = (&p.c.x, &p.c.y, &p.c.z);
            let add = x + z;
            let sub = x - z;
            let xx = &self.invsqr_d * x * &add;
            let yy = y * &sub;
            let tt = &self.invsqr_d * x * sub;
            let zz = y * add;
            self.e0.new_proy_point(edwards::ProyCoordinates {
                x: xx,
                y: yy,
                t: tt,
                z: zz,
            })
        }
    }
}

pub fn edwards448_to_curve448() -> impl RationalMap<E0 = EdCurve, E1 = MtCurve> {
    let e0 = EDWARDS448.get();
    let e1 = CURVE448.get();
    Ed4isoMt448 { e0, e1 }
}

struct Ed4isoMt448 {
    e0: EdCurve,
    e1: MtCurve,
}

impl RationalMap for Ed4isoMt448 {
    type E0 = EdCurve;
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
            let ff = self.e1.get_field();
            let f2 = ff.from(2);
            let x2 = x ^ 2u32;
            let y2 = y ^ 2u32;
            let z2 = z ^ 2u32;
            let zz = x * &x2; // zz = x^3
            let xx = x * &y2; // xx = x*y^2
            let yy = y * (f2 * z2 - y2 - x2); // yy = y*(2z^2-y^2-x^2)
            self.e1.new_proy_point(montgomery::ProyCoordinates {
                x: xx,
                y: yy,
                z: zz,
            })
        }
    }
    fn pull(&self, p: MtPoint) -> TePoint {
        if p.is_zero() {
            self.e0.identity()
        } else {
            let ff = self.e0.get_field();
            let (f2, f4) = (&ff.from(2), &ff.from(4));
            let (x, y, z) = (&p.c.x, &p.c.y, &p.c.z);
            let x2 = x ^ 2u32;
            let x3 = &x2 * x;
            let x4 = &x2 ^ 2u32;
            let x5 = &x3 * &x2;
            let y2 = y ^ 2u32;
            let z2 = z ^ 2u32;
            let z3 = &z2 * z;
            let z4 = &z2 ^ 2u32;
            let xx = f4 * y * z * (&x2 - &z2);
            let z0 = x4 - f2 * &x2 * &z2 + &z4 + f4 * &y2 * &z2;
            let yy = -(&x5 - f2 * &x3 * &z2 + x * &z4 - f4 * x * &y2 * &z2);
            let z1 = x5 - f2 * &x3 * &z2 + x * z4 - f2 * x2 * &y2 * z - f2 * y2 * z3;
            let tt = &xx * &yy;
            let xx = &xx * &z1;
            let yy = &yy * &z0;
            let zz = z0 * z1;
            self.e0.new_proy_point(edwards::ProyCoordinates {
                x: xx,
                y: yy,
                t: tt,
                z: zz,
            })
        }
    }
}
