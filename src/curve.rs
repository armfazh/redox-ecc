//! This is documentation for the `curve` module.
//!
//! The curve module is meant to be used for bar.

extern crate num_bigint;
use num_bigint::{BigInt, BigUint, ToBigInt};

use num_traits::identities::{One, Zero};
use std::ops::{Add, Mul, Neg};

use crate::field::{PrimeField, PrimeFieldElement};
use crate::scalar::Scalar;

/// This is an elliptic curve defined by the Weierstrass equation `y^2=x^3+ax+b`.
///
/// **Atention** This implementation only supports curves of prime order.
#[derive(Clone, std::cmp::PartialEq)]
pub struct WeierstrassCurve {
    pub f: PrimeField,
    pub a: PrimeFieldElement,
    pub b: PrimeFieldElement,
    pub r: BigUint,
}

impl std::fmt::Display for WeierstrassCurve {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Weierstrass Curve y^2=x^3+ax+b\na: {}\nb: {}",
            self.a, self.b,
        )
    }
}
impl WeierstrassCurve {
    pub fn new_point(
        &self,
        x: PrimeFieldElement,
        y: PrimeFieldElement,
        z: PrimeFieldElement,
    ) -> WeierstrassProjectivePoint {
        let e = self.clone();
        let p = WeierstrassProjectivePoint { e, x, y, z };
        do_if_eq!(self.is_on_curve(&p), true, p, ERR_ECC_NEW)
    }
    pub fn new_scalar(&self, k: BigInt) -> Scalar {
        let r = self.r.to_bigint().unwrap();
        Scalar { k, r }.reduce()
    }
    pub fn identity(&self) -> WeierstrassProjectivePoint {
        self.new_point(self.f.zero(), self.f.one(), self.f.zero())
    }
    pub fn is_on_curve(&self, p: &WeierstrassProjectivePoint) -> bool {
        let x3 = &p.x * &p.x * &p.x;
        let bz = &self.b * &p.z;
        let ax = &self.a * &p.x;
        let zz = &p.z * &(ax + &bz);
        let zy = &p.z * &(zz - &(&p.y * &p.y));
        let eq = x3 + &zy;
        eq.is_zero()
    }
    fn core_neg(&self, p: &WeierstrassProjectivePoint) -> WeierstrassProjectivePoint {
        self.new_point(p.x.clone(), -&p.y, p.z.clone())
    }
    /// core_add implements complete addition formulas for prime order groups.
    // Reference: "Complete addition formulas for prime order elliptic curves" by
    // Costello-Renes-Batina. [Alg.1] (eprint.iacr.org/2015/1060).
    fn core_add(
        &self,
        p: &WeierstrassProjectivePoint,
        q: &WeierstrassProjectivePoint,
    ) -> WeierstrassProjectivePoint {
        let a = &self.a;
        let b3 = &self.b * &self.f.elt(3i64);
        let (x1, x2) = (&p.x, &q.x);
        let (y1, y2) = (&p.y, &q.y);
        let (z1, z2) = (&p.z, &q.z);
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
        t4 = &t4 + &t2; // 32. t4 = t4 + t2
        t0 = &t1 * &t4; // 33. t0 = t1 * t4
        y3 = y3 + &t0; //  34. Y3 = Y3 + t0
        t0 = &t5 * &t4; // 35. t0 = t5 * t4
        x3 = &t3 * &x3; // 36. X3 = t3 * X3
        x3 = x3 - &t0; //  37. X3 = X3 - t0
        t0 = t3 * &t1; //  38. t0 = t3 * t1
        z3 = t5 * z3; //   39. Z3 = t5 * Z3
        z3 = z3 + t0; //   40. Z3 = Z3 + t0
        self.new_point(x3, y3, z3)
    }
    /// core_mul implements the double&add scalar multiplication method.
    /// This function run in non-constant time.
    fn core_mul(&self, p: &WeierstrassProjectivePoint, k: &Scalar) -> WeierstrassProjectivePoint {
        let mut q = self.identity();
        for ki in k.reduce().left_to_right() {
            q = &q + &q;
            if ki {
                q = q + p;
            }
        }
        q
    }
}

#[derive(Clone)]
pub struct WeierstrassProjectivePoint {
    e: WeierstrassCurve,
    x: PrimeFieldElement,
    y: PrimeFieldElement,
    z: PrimeFieldElement,
}

impl std::fmt::Display for WeierstrassProjectivePoint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\nx: {}\ny: {}\nz: {}", self.x, self.y, self.z)
    }
}

impl WeierstrassProjectivePoint {
    pub fn normalize(&mut self) {
        let inv_z = 1u32 / &self.z;
        self.x = &self.x * &inv_z;
        self.y = &self.y * &inv_z;
        self.z.set_one();
    }
    pub fn is_identity(&self) -> bool {
        self.x.is_zero() && !self.y.is_zero() && self.z.is_zero()
    }
}

impl PartialEq for WeierstrassProjectivePoint {
    fn eq(&self, other: &Self) -> bool {
        let x1z2 = &self.x * &other.z;
        let z1x2 = &self.z * &other.x;
        let y1z2 = &self.y * &other.z;
        let z1y2 = &self.z * &other.y;
        self.e == other.e && x1z2 == z1x2 && y1z2 == z1y2
    }
}

impl<'a, 'b> Mul<&'b Scalar> for &'a WeierstrassProjectivePoint {
    type Output = WeierstrassProjectivePoint;
    #[inline]
    fn mul(self, other: &Scalar) -> WeierstrassProjectivePoint {
        self.e.core_mul(&self, &other)
    }
}
impl<'a> Mul<&'a Scalar> for WeierstrassProjectivePoint {
    type Output = WeierstrassProjectivePoint;
    #[inline]
    fn mul(self, other: &'a Scalar) -> Self {
        self.e.core_mul(&self, &other)
    }
}
impl Mul<Scalar> for WeierstrassProjectivePoint {
    type Output = WeierstrassProjectivePoint;
    #[inline]
    fn mul(self, other: Scalar) -> Self {
        self.e.core_mul(&self, &other)
    }
}

macro_rules! impl_add_point {
    ($point:ident, $method:ident) => {
        impl<'a, 'b> Add<&'b $point> for &'a $point {
            type Output = $point;
            #[inline]
            fn add(self, other: &$point) -> $point {
                do_if_eq!(self.e, other.e, self.e.$method(&self, &other), ERR_ADD_OP)
            }
        }
        impl<'a> Add<&'a $point> for $point {
            type Output = $point;
            #[inline]
            fn add(self, other: &Self) -> Self::Output {
                do_if_eq!(self.e, other.e, self.e.$method(&self, &other), ERR_ADD_OP)
            }
        }
        impl Add for $point {
            type Output = $point;
            #[inline]
            fn add(self, other: Self) -> Self::Output {
                do_if_eq!(self.e, other.e, self.e.$method(&self, &other), ERR_ADD_OP)
            }
        }
    };
}

impl_add_point!(WeierstrassProjectivePoint, core_add);

impl<'a> Neg for &'a WeierstrassProjectivePoint {
    type Output = WeierstrassProjectivePoint;
    fn neg(self) -> Self::Output {
        self.e.core_neg(&self)
    }
}

const ERR_ADD_OP: &'static str = "points of different curves";
const ERR_ECC_NEW: &'static str = "not valid point";
