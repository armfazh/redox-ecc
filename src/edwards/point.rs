use impl_ops::impl_op_ex;
use num_bigint::ToBigInt;
use num_traits::identities::{One, Zero};

use std::ops;

use crate::do_if_eq;
use crate::edwards::curve::Curve;
use crate::edwards::scalar::Scalar;
use crate::ellipticcurve::{EcPoint, EllipticCurve};
use crate::ops::ScMulRef;
use crate::primefield::FpElt;

#[derive(Clone)]
pub struct ProyCoordinates {
    pub x: FpElt,
    pub y: FpElt,
    pub t: FpElt,
    pub z: FpElt,
}
#[derive(Clone)]
pub struct Point {
    pub(crate) e: Curve,
    pub(crate) c: ProyCoordinates,
}

impl ScMulRef<Scalar> for Point {}
impl EcPoint<Scalar> for Point {
    fn is_zero(&self) -> bool {
        self.c.x.is_zero() && !self.c.y.is_zero() && self.c.t.is_zero() && !self.c.z.is_zero()
    }
    fn serialize(&self, _: bool) -> Vec<u8> {
        panic!("unimplemented!");
    }
}

impl Point {
    pub fn normalize(&mut self) {
        let inv_z = 1u32 / &self.c.z;
        self.c.x = &self.c.x * &inv_z;
        self.c.y = &self.c.y * &inv_z;
        self.c.t = &self.c.x * &self.c.y;
        self.c.z.set_one();
    }
    fn core_neg(&self) -> Point {
        self.e.new_point(ProyCoordinates {
            x: -&self.c.x,
            y: self.c.y.clone(),
            t: -&self.c.t,
            z: self.c.z.clone(),
        })
    }
    fn core_add(&self, p: &Point) -> Point {
        let (x1, y1, t1, z1) = (&self.c.x, &self.c.y, &self.c.t, &self.c.z);
        let (x2, y2, t2, z2) = (&p.c.x, &p.c.y, &p.c.t, &p.c.z);
        let (a_ec, d_ec) = (&self.e.a, &self.e.d);
        let aa = x1 * x2; // A = X1 * X2
        let bb = y1 * y2; // B = Y1 * Y2
        let cc = d_ec * t1 * t2; // C = d*T1 * T2
        let dd = z1 * z2; // D = Z1 * Z2
        let ee = (x1 + y1) * (x2 + y2) - &aa - &bb; // E = (X1 + Y1 ) * (X2 + Y2 ) - A - B
        let ff = &dd - &cc; // F = D - C
        let gg = dd + cc; // G = D + C
        let hh = bb - aa * a_ec; // H = B - a*A
        let x3 = &ee * &ff; // X3 = E * F
        let y3 = &gg * &hh; // Y3 = G * H
        let t3 = ee * hh; // T3 = E * H
        let z3 = ff * gg; // Z3 = F * G
        self.e.new_point(ProyCoordinates {
            x: x3,
            y: y3,
            t: t3,
            z: z3,
        })
    }
    fn core_mul(&self, k: &Scalar) -> Point {
        let mut q = self.e.identity();
        for ki in k.iter_lr() {
            q = &q + &q;
            if ki {
                q = q + self;
            }
        }
        q
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        let x1z2 = &self.c.x * &other.c.z;
        let z1x2 = &self.c.z * &other.c.x;
        let y1z2 = &self.c.y * &other.c.z;
        let z1y2 = &self.c.z * &other.c.y;
        let t1z2 = &self.c.t * &other.c.z;
        let z1t2 = &self.c.z * &other.c.t;
        self.e == other.e && x1z2 == z1x2 && y1z2 == z1y2 && t1z2 == z1t2
    }
}

impl_op_ex!(+|a: &Point , b: &Point | -> Point  {
    do_if_eq!(a.e == b.e, a.core_add(b), ERR_ADD_OP)
});
impl_op_ex!(-|a: &Point, b: &Point| -> Point { a + (-b) });
impl_op_ex!(-|a: &Point| -> Point { a.core_neg() });
impl_op_ex!(*|a: &Point, b: &Scalar| -> Point {
    let r = a.e.r.to_bigint().unwrap();
    do_if_eq!(r == b.r, a.core_mul(b), ERR_MUL_OP)
});

const ERR_MUL_OP: &str = "Scalar don't match with point";
const ERR_ADD_OP: &str = "points of different curves";

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "\nx: {}\ny: {}\nt: {}\nz: {}",
            self.c.x, self.c.y, self.c.t, self.c.z
        )
    }
}
