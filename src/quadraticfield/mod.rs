//! This is documentation for the `quadraticfield` module.
//!
//! The quadraticfield module is meant to be used for bar.

use atomic_refcell::AtomicRefCell;
use num_bigint::{BigInt, BigUint};
use num_traits::cast::ToPrimitive;
use num_traits::identities::{One, Zero};

use std::ops;
use std::ops::{BitXor, Div};
use std::sync::Arc;

use crate::do_if_eq;
use crate::field::{CMov, Field, FieldElement, Sgn0, Sqrt};
use crate::ops::{Deserialize, FromFactory, Serialize};
use crate::primefield::{Fp, FpElt};

struct Params {
    base: Fp,
    sqrt_precmp: AtomicRefCell<Option<SqrtPrecmp>>,
}

impl Eq for Params {}

impl PartialEq for Params {
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base
    }
}

/// Fp implements a base field of prime characteristic.
#[derive(Clone, PartialEq, Eq)]
pub struct Fp2(Arc<Params>);

impl Fp2 {
    /// Use `new` to generate a prime field instance.
    /// ```
    ///  use num_bigint::BigUint;
    ///  use redox_ecc::quadraticfield::Fp2;
    ///  let f = Fp2::new(BigUint::from(103u32));
    /// ```
    /// The `modulus` should be a prime number.
    pub fn new(modulus: BigUint) -> Self {
        let base = Fp::new(modulus);
        let sqrt_precmp = AtomicRefCell::new(None);
        Fp2(Arc::new(Params { base, sqrt_precmp }))
    }
}

impl Field for Fp2 {
    type Elt = Fp2Elt;
    fn elt(&self, n: BigInt) -> Self::Elt {
        let n0 = self.0.base.elt(n);
        let n1 = self.0.base.zero();
        let f = self.clone();
        Fp2Elt { n: vec![n0, n1], f }
    }
    fn zero(&self) -> Self::Elt {
        self.elt(BigInt::zero())
    }
    fn one(&self) -> Self::Elt {
        self.elt(BigInt::one())
    }
    fn get_modulus(&self) -> BigInt {
        self.0.base.get_modulus()
    }
    fn size_bytes(&self) -> usize {
        2 * self.0.base.size_bytes()
    }
}

impl Deserialize for Fp2 {
    type Deser = <Fp2 as Field>::Elt;
    fn from_bytes_be(&self, bytes: &[u8]) -> Result<Self::Deser, std::io::Error> {
        let len = self.size_bytes();
        if len != bytes.len() {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "wrong size"));
        }
        let size = len / 2;
        let n0 = self.0.base.from_bytes_be(&bytes[0..size]).unwrap();
        let n1 = self.0.base.from_bytes_be(&bytes[size..2 * size]).unwrap();
        Ok(Fp2Elt {
            n: vec![n0, n1],
            f: self.clone(),
        })
    }
    fn from_bytes_le(&self, bytes: &[u8]) -> Result<Self::Deser, std::io::Error> {
        let len = self.size_bytes();
        if len != bytes.len() {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "wrong size"));
        }
        let size = len / 2;
        let n0 = self.0.base.from_bytes_le(&bytes[0..size]).unwrap();
        let n1 = self.0.base.from_bytes_le(&bytes[size..2 * size]).unwrap();
        Ok(Fp2Elt {
            n: vec![n0, n1],
            f: self.clone(),
        })
    }
}

macro_rules! impl_from_factory {
    ($target:ident, <$($other:ty)+> ) => {
     $(
         impl FromFactory<$other> for $target{
            type Output = <Fp2 as Field>::Elt;
            fn from(&self, n: $other) -> Self::Output{
                self.elt(BigInt::from(n))
            }
        }
    )+
    };
}

impl_from_factory!(Fp2, <u8 u16 u32 u64 i8 i16 i32 i64>);

impl FromFactory<&str> for Fp2 {
    type Output = <Fp2 as Field>::Elt;
    fn from(&self, s: &str) -> Self::Output {
        let vs: Vec<&str> = s.splitn(2, ',').collect();
        let n0: FpElt = self.0.base.from(vs[0]);
        let n1: FpElt = self.0.base.from(vs[1]);
        Fp2Elt {
            n: vec![n0, n1],
            f: self.clone(),
        }
    }
}

/// Fp2Elt is an element of a prime field.
#[derive(Clone, PartialEq, Eq)]
pub struct Fp2Elt {
    n: Vec<FpElt>,
    f: Fp2,
}

impl FieldElement for Fp2Elt {}

impl Serialize for Fp2Elt {
    /// serializes the field element into big-endian bytes
    fn to_bytes_be(&self) -> Vec<u8> {
        let mut out = Vec::new();
        for x in self.n.iter() {
            out.append(&mut x.to_bytes_be())
        }
        out
    }
    /// serializes the field element into little-endian bytes
    fn to_bytes_le(&self) -> Vec<u8> {
        let mut out = Vec::new();
        for x in self.n.iter() {
            out.append(&mut x.to_bytes_le())
        }
        out
    }
}

impl<'a, 'b> std::ops::Add<&'b Fp2Elt> for &'a Fp2Elt {
    type Output = Fp2Elt;
    fn add(self, other: &'b Fp2Elt) -> Fp2Elt {
        do_if_eq!(
            self.f == other.f,
            self.elt(&self.n[0] + &other.n[0], &self.n[1] + &other.n[1]),
            ERR_BIN_OP
        )
    }
}
impl<'a> std::ops::Add<Fp2Elt> for &'a Fp2Elt {
    type Output = Fp2Elt;
    fn add(self, other: Fp2Elt) -> Fp2Elt {
        do_if_eq!(
            self.f == other.f,
            self.elt(&self.n[0] + &other.n[0], &self.n[1] + &other.n[1]),
            ERR_BIN_OP
        )
    }
}

impl Fp2Elt {
    #[inline]
    fn elt(&self, n0: FpElt, n1: FpElt) -> Fp2Elt {
        let n = vec![n0, n1];
        let f = self.f.clone();
        Fp2Elt { n, f }
    }
    #[inline]
    fn inv_mod(&self) -> Fp2Elt {
        let n0 = &self.n[0];
        let n1 = &self.n[1];
        let den = 1u32 / &(n0 * n0 + n1 * n1);
        self.elt(&den * n0, den * n1)
    }
}

impl_op_ex!(+|a: Fp2Elt, b: &Fp2Elt| -> Fp2Elt {
    do_if_eq!(a.f == b.f, a.elt(&a.n[0] + &b.n[0], &a.n[1] + &b.n[1]), ERR_BIN_OP)
});
impl_op_ex!(-|a: &Fp2Elt, b: &Fp2Elt| -> Fp2Elt {
    do_if_eq!(
        a.f == b.f,
        a.elt(&a.n[0] - &b.n[0], &a.n[1] - &b.n[1]),
        ERR_BIN_OP
    )
});
impl_op_ex!(*|a: &Fp2Elt, b: &Fp2Elt| -> Fp2Elt {
    do_if_eq!(
        a.f == b.f,
        a.elt(
            &a.n[0] * &b.n[0] - &a.n[1] * &b.n[1],
            &a.n[0] * &b.n[1] + &a.n[1] * &b.n[0],
        ),
        ERR_BIN_OP
    )
});

impl_op_ex!(/|a: &Fp2Elt, b: &Fp2Elt| -> Fp2Elt {
    #[allow(clippy::suspicious_arithmetic_impl)] {
        a * b.inv_mod()
    }
});
impl_op_ex!(-|a: &Fp2Elt| -> Fp2Elt { a.elt(-&a.n[0], -&a.n[1]) });
impl_op_ex!(^|a: &Fp2Elt, b: u32| -> Fp2Elt {
    do_if_eq!(b == 2u32, a * a, ERR_EXP_SQR_OP)
});
impl_op_ex!(^|a: &Fp2Elt, b: i32| -> Fp2Elt {
    do_if_eq!(b == -1i32, a.inv_mod(), ERR_EXP_INV_OP)
});

impl<'a> Div<&'a Fp2Elt> for u32 {
    type Output = Fp2Elt;
    #[inline]
    fn div(self, other: &Fp2Elt) -> Self::Output {
        do_if_eq!(self == 1u32, other.inv_mod(), ERR_INV_OP)
    }
}

impl<'a, 'b> BitXor<&'b BigUint> for &'a Fp2Elt {
    type Output = Fp2Elt;
    #[inline]
    fn bitxor(self, exp: &'b BigUint) -> Self::Output {
        let v = exp.to_u32_digits();
        let mut out = Fp2Elt::one();
        for vi in v.iter().rev() {
            for j in (0..31).rev() {
                out = &out * &out;
                let bit = (*vi >> j) & 1;
                if bit == 1 {
                    out = out * self;
                }
            }
        }
        out
    }
}

impl<'a, 'b> BitXor<&'b BigInt> for &'a Fp2Elt {
    type Output = Fp2Elt;
    #[inline]
    fn bitxor(self, exp: &'b BigInt) -> Self::Output {
        let expo = &exp.to_biguint().unwrap();
        self ^ expo
    }
}

impl CMov for Fp2Elt {}

#[derive(Clone, std::cmp::PartialEq)]
enum SqrtPrecmp {
    P3MOD4 { c1: BigInt, c2: BigInt },
}

impl Fp2 {
    fn get_sqrt_precmp(&self) -> SqrtPrecmp {
        self.0
            .sqrt_precmp
            .borrow_mut()
            .get_or_insert(self.calc_sqrt_precmp())
            .clone()
    }
    fn calc_sqrt_precmp(&self) -> SqrtPrecmp {
        let p = self.get_modulus();
        if 3u32 == (&p % 4u32).to_u32().unwrap() {
            let c1 = (&p - 3u32) >> 2usize;
            let c2 = (&p - 1u32) >> 1usize;
            SqrtPrecmp::P3MOD4 { c1, c2 }
        } else {
            unimplemented!()
        }
    }
}

impl Sqrt for Fp2Elt {
    #[inline]
    fn is_square(&self) -> bool {
        let n0 = &self.n[0];
        let n1 = &self.n[1];
        let t0 = n0 * n0 + n1 * n1;
        let exp = (self.f.get_modulus() - 1u32) >> 1usize;
        let t1 = &t0 ^ &exp;
        t1 == self.f.0.base.one()
    }
    fn sqrt(&self) -> Fp2Elt {
        let pre = self.f.get_sqrt_precmp();
        match pre {
            SqrtPrecmp::P3MOD4 { c1, c2 } => {
                let a1 = self ^ &c1;
                let a1a = &a1 * self;
                let alpha = a1 * &a1a;
                let x0 = a1a;
                let t = &alpha + self.f.one();
                if t.is_zero() {
                    let i = self.elt(self.f.0.base.from(0), self.f.0.base.from(1));
                    x0 * i
                } else {
                    let par = alpha + self.f.one();
                    let b = &par ^ &c2;
                    x0 * b
                }
            }
        }
    }
}

impl Sgn0 for Fp2Elt {
    fn sgn0(&self) -> i32 {
        let s0 = self.n[0].sgn0();
        let z0 = self.n[0].is_zero() as i32;
        let s1 = self.n[1].sgn0();
        s0 | (z0 ^ s1)
    }
}

impl num_traits::identities::Zero for Fp2Elt {
    fn zero() -> Self {
        unimplemented!()
    }
    fn is_zero(&self) -> bool {
        self.n[0].is_zero() && self.n[1].is_zero()
    }
    fn set_zero(&mut self) {
        self.n[0].set_zero();
        self.n[1].set_zero()
    }
}

impl num_traits::identities::One for Fp2Elt {
    fn one() -> Self {
        unimplemented!()
    }
    fn is_one(&self) -> bool {
        self.n[0].is_one() && self.n[1].is_zero()
    }
    fn set_one(&mut self) {
        self.n[0].set_one();
        self.n[1].set_zero();
    }
}

impl std::fmt::Display for Fp2Elt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}+i*{}", self.n[0], self.n[1])
    }
}

impl std::fmt::Display for Fp2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}^2", &self.0.base)
    }
}

const ERR_BIN_OP: &str = "elements of different fields";
const ERR_EXP_SQR_OP: &str = "exponent must be 2u32";
const ERR_EXP_INV_OP: &str = "exponent must be -1i32";
const ERR_INV_OP: &str = "numerator must be 1u32";
