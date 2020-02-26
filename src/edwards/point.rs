extern crate num_bigint;
use num_bigint::ToBigInt;

use num_traits::identities::{One, Zero};

use std::ops::{Add, Mul, Neg};

use crate::edwards::curve::Curve;
use crate::edwards::scalar::Scalar;
use crate::field::FpElt;
use crate::EllipticCurve;
use crate::{do_if_eq, impl_binary_op, impl_unary_op};

#[derive(Clone)]
pub struct ProyCoordinates {
    pub x: FpElt,
    pub y: FpElt,
    pub t: FpElt,
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
        self.c.t = &self.c.x * &self.c.y;
        self.c.z.set_one();
    }
    pub fn is_identity(&self) -> bool {
        self.c.x.is_zero() && !self.c.y.is_zero() && self.c.t.is_zero() && !self.c.z.is_zero()
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
        let a = x1 * x2; // A = X1 * X2
        let b = y1 * y2; // B = Y1 * Y2
        let c = d_ec * t1 * t2; // C = d*T1 * T2
        let d = z1 * z2; // D = Z1 * Z2
        let e = (x1 + y1) * (x2 + y2) - &a - &b; // E = (X1 + Y1 ) * (X2 + Y2 ) - A - B
        let f = &d - &c; // F = D - C
        let g = d + c; // G = D + C
        let h = b - a * a_ec; // H = B - a*A
        let x3 = &e * &f; // X3 = E * F
        let y3 = &g * &h; // Y3 = G * H
        let t3 = e * h; // T3 = E * H
        let z3 = f * g; // Z3 = F * G
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
        write!(
            f,
            "\nx: {}\ny: {}\nt: {}\nz: {}",
            self.c.x, self.c.y, self.c.t, self.c.z
        )
    }
}
