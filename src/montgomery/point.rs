use impl_ops::impl_op_ex;
use num_bigint::ToBigInt;
use num_traits::identities::{One, Zero};

use std::ops;

use crate::do_if_eq;
use crate::ellipticcurve::{EcPoint, EllipticCurve, Encode};
use crate::montgomery::curve::Curve;
use crate::montgomery::scalar::Scalar;
use crate::ops::ScMulRef;
use crate::primefield::FpElt;
use crate::field::Sgn0;
use crate::ops::Serialize;

#[derive(Clone)]
pub struct ProyCoordinates {
    pub x: FpElt,
    pub y: FpElt,
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
        self.c.x.is_zero() && !self.c.y.is_zero() && self.c.z.is_zero()
    }
}
impl Encode for Point {
    fn encode(&self, compress: bool) -> Vec<u8> {
        // normalize the point to ensure that z = 1
        // clone so that we don't mutate the original point
        let mut p_normal = self.clone();
        p_normal.normalize();
        // if the point is the point at infinity, then return a single
        // zeroed byte
        if p_normal.is_zero() {
            return vec![0];
        }
        let coords = &p_normal.c;
        let x = &coords.x;
        let y = &coords.y;
        let mut x_bytes = x.to_bytes_be();
        let mut y_bytes = y.to_bytes_be();
        match compress {
            true => {
                let s = y.sgn0_le();
                // if sign == 1: tag = 0x02; elif sign == -1: tag = 0x03
                let tag = (((s>>1)&0x1)+2) as u8;
                let mut o = vec![tag];
                o.append(&mut x_bytes);
                o
            },
            _ => {
                let mut o: Vec<u8> = vec![0x04];
                o.append(&mut x_bytes);
                o.append(&mut y_bytes);
                o
            }
        }
    }
}

impl Point {
    pub fn normalize(&mut self) {
        let inv_z = 1u32 / &self.c.z;
        self.c.x = &self.c.x * &inv_z;
        self.c.y = &self.c.y * &inv_z;
        self.c.z.set_one();
    }
    fn core_neg(&self) -> Point {
        self.e.new_point(ProyCoordinates {
            x: self.c.x.clone(),
            y: -&self.c.y,
            z: self.c.z.clone(),
        })
    }
    fn core_add(&self, p: &Point) -> Point {
        let (x1, y1, z1) = (&self.c.x, &self.c.y, &self.c.z);
        let (x2, y2, z2) = (&p.c.x, &p.c.y, &p.c.z);
        let (a_ec, s_ec) = (&self.e.a, &self.e.s);
        let (t0, t1, t2) = (x1 * x2, y1 * y2, z1 * z2);
        let (t3, t4) = (x1 * y2, x2 * y1);
        let (t5, t6) = (y1 * z2, y2 * z1);
        let (t7, t8) = (x1 * z2, x2 * z1);
        let t9 = &t7 + &t8;
        let ta = &t9 + &(&t0 * a_ec);
        let rr = &t5 + &t6;
        let tt = &ta - &t1;
        let vv = t9 * a_ec + &t0 + &t0 + &t0 + &t2;
        let ss = (&t3 - &t4) * s_ec + t0 - t2;
        let uu = (t7 - t8) * s_ec - t3 - t4;
        let ww = (t5 - t6) * s_ec + ta + t1;
        let x3 = &rr * &ss - &tt * &uu;
        let y3 = tt * &ww - &vv * &ss;
        let z3 = vv * uu - rr * ww;
        self.e.new_point(ProyCoordinates {
            x: x3,
            y: y3,
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
    pub fn is_two_torsion(&self) -> bool {
        self.c.y.is_zero() && self.c.z.is_one()
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        let x1z2 = &self.c.x * &other.c.z;
        let z1x2 = &self.c.z * &other.c.x;
        let y1z2 = &self.c.y * &other.c.z;
        let z1y2 = &self.c.z * &other.c.y;
        self.e == other.e && x1z2 == z1x2 && y1z2 == z1y2
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

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\nx: {}\ny: {}\nz: {}", self.c.x, self.c.y, self.c.z)
    }
}

const ERR_MUL_OP: &str = "Scalar don't match with point";
const ERR_ADD_OP: &str = "points of different curves";
