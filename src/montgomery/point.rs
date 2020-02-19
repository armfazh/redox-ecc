extern crate num_bigint;
use num_bigint::ToBigInt;

use num_traits::identities::{One, Zero};

use std::ops::{Add, Mul, Neg};

use crate::EllipticCurve;

use crate::field::FpElt;
use crate::montgomery::curve::Curve;
use crate::montgomery::scalar::Scalar;
use crate::{do_if_eq, impl_binary_op, impl_unary_op};

#[derive(Clone)]
pub struct ProyCoordinates {
    pub x: FpElt,
    pub y: FpElt,
    pub z: FpElt,
}
#[derive(Clone)]
pub struct Point {
    pub(super) e: Curve,
    pub(super) c: ProyCoordinates,
}

impl Point {
    pub fn normalize(&mut self) {
        let inv_z = 1u32 / &self.c.z;
        self.c.x = &self.c.x * &inv_z;
        self.c.y = &self.c.y * &inv_z;
        self.c.z.set_one();
    }
    pub fn is_identity(&self) -> bool {
        self.c.x.is_zero() && !self.c.y.is_zero() && self.c.z.is_zero()
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
        let r = &t5 + &t6;
        let t = &ta - &t1;
        let v = t9 * a_ec + &t0 + &t0 + &t0 + &t2;
        let s = (&t3 - &t4) * s_ec + t0 - t2;
        let u = (t7 - t8) * s_ec - t3 - t4;
        let w = (t5 - t6) * s_ec + ta + t1;
        let x3 = &r * &s - &t * &u;
        let y3 = t * &w - &v * &s;
        let z3 = v * u - r * w;
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

impl<'a, 'b> Mul<&'b Scalar> for &'a Point {
    type Output = Point;
    #[inline]
    fn mul(self, other: &Scalar) -> Self::Output {
        let r = self.e.get_order().to_bigint().unwrap();
        do_if_eq!(r, other.r, self.core_mul(&other), ERR_MUL_OP)
    }
}
impl<'a> Mul<&'a Scalar> for Point {
    type Output = Point;
    #[inline]
    fn mul(self, other: &'a Scalar) -> Self::Output {
        let r = self.e.get_order().to_bigint().unwrap();
        do_if_eq!(r, other.r, self.core_mul(&other), ERR_MUL_OP)
    }
}
impl Mul<Scalar> for Point {
    type Output = Point;
    #[inline]
    fn mul(self, other: Scalar) -> Self::Output {
        let r = self.e.get_order().to_bigint().unwrap();
        do_if_eq!(r, other.r, self.core_mul(&other), ERR_MUL_OP)
    }
}

const ERR_MUL_OP: &str = "Scalar don't match with point";
const ERR_ADD_OP: &str = "points of different curves";

impl_binary_op!(Point, Add, add, core_add, e, ERR_ADD_OP);
impl_unary_op!(Point, Neg, neg, core_neg);

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\nx: {}\ny: {}\nz: {}", self.c.x, self.c.y, self.c.z)
    }
}
