//! This is documentation for the `curve` module.
//!
//! The curve module is meant to be used for bar.

use impl_ops::impl_op_ex;
use num_bigint::ToBigInt;
use num_traits::identities::{One, Zero};

use std::ops;

use crate::do_if_eq;
use crate::ellipticcurve::{EcPoint, EllipticCurve, Encode};
use crate::ops::ScMulRef;
use crate::primefield::FpElt;
use crate::field::Sgn0;
use crate::weierstrass::curve::Curve;
use crate::weierstrass::scalar::Scalar;
use crate::ops::Serialize;

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
    fn core_neg(&self) -> <Curve as EllipticCurve>::Point {
        self.e.new_point(ProyCoordinates {
            x: self.c.x.clone(),
            y: -&self.c.y,
            z: self.c.z.clone(),
        })
    }
    /// core_add implements complete addition formulas for prime order groups.
    // Reference: "Complete addition formulas for prime order elliptic curves" by
    // Costello-Renes-Batina. [Alg.1] (eprint.iacr.org/2015/1060).
    fn core_add(&self, p: &<Curve as EllipticCurve>::Point) -> <Curve as EllipticCurve>::Point {
        let a = &self.e.a;
        let b3 = &self.e.b + &self.e.b + &self.e.b;
        let (x1, x2) = (&self.c.x, &p.c.x);
        let (y1, y2) = (&self.c.y, &p.c.y);
        let (z1, z2) = (&self.c.z, &p.c.z);
        let (mut x3, mut y3, mut z3);
        let (mut t0, mut t1, mut t2, mut t3, mut t4, mut t5);
        t0 = x1 * x2; //    1. t0 = X1 * X2
        t1 = y1 * y2; //    2. t1 = Y1 * Y2
        t2 = z1 * z2; //    3. t2 = Z1 * Z2
        t3 = x1 + y1; //    4. t3 = X1 + Y1
        t4 = x2 + y2; //    5. t4 = X2 + Y2
        t3 = &t3 * &t4; //  6. t3 = t3 * t4
        t4 = &t0 + &t1; //  7. t4 = t0 + t1
        t3 = &t3 - &t4; //  8. t3 = t3 - t4
        t4 = x1 + z1; //    9. t4 = X1 + Z1
        t5 = x2 + z2; //   10. t5 = X2 + Z2
        t4 = &t4 * &t5; // 11. t4 = t4 * t5
        t5 = &t0 + &t2; // 12. t5 = t0 + t2
        t4 = &t4 - &t5; // 13. t4 = t4 - t5
        t5 = y1 + z1; //   14. t5 = Y1 + Z1
        x3 = y2 + z2; //   15. X3 = Y2 + Z2
        t5 = &t5 * &x3; // 16. t5 = t5 * X3
        x3 = &t1 + &t2; // 17. X3 = t1 + t2
        t5 = &t5 - &x3; // 18. t5 = t5 - X3
        z3 = a * &t4; //   19. Z3 =  a * t4
        x3 = &b3 * &t2; // 20. X3 = b3 * t2
        z3 = x3 + &z3; //  21. Z3 = X3 + Z3
        x3 = &t1 - &z3; // 22. X3 = t1 - Z3
        z3 = &t1 + &z3; // 23. Z3 = t1 + Z3
        y3 = &x3 * &z3; // 24. Y3 = X3 * Z3
        t1 = &t0 + &t0; // 25. t1 = t0 + t0
        t1 = &t1 + &t0; // 26. t1 = t1 + t0
        t2 = a * &t2; //   27. t2 =  a * t2
        t4 = b3 * &t4; //  28. t4 = b3 * t4
        t1 = &t1 + &t2; // 29. t1 = t1 + t2
        t2 = &t0 - &t2; // 30. t2 = t0 - t2
        t2 = a * &t2; //   31. t2 =  a * t2
        t4 = &t4 + t2; //  32. t4 = t4 + t2
        t0 = &t1 * &t4; // 33. t0 = t1 * t4
        y3 = y3 + &t0; //  34. Y3 = Y3 + t0
        t0 = &t5 * t4; //  35. t0 = t5 * t4
        x3 = &t3 * &x3; // 36. X3 = t3 * X3
        x3 = x3 - &t0; //  37. X3 = X3 - t0
        t0 = t3 * t1; //   38. t0 = t3 * t1
        z3 = t5 * z3; //   39. Z3 = t5 * Z3
        z3 = z3 + t0; //   40. Z3 = Z3 + t0
        self.e.new_point(ProyCoordinates {
            x: x3,
            y: y3,
            z: z3,
        })
    }
    /// core_mul implements the double&add Scalar multiplication method.
    /// This function run in non-constant time.
    fn core_mul(&self, k: &Scalar) -> <Curve as EllipticCurve>::Point {
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
