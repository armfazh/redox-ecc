//! This is documentation for the `field` module.
//!
//! The field module is meant to be used for bar.

extern crate num_bigint;
extern crate num_integer;

use num_bigint::{BigInt, BigUint, ToBigInt};
use num_integer::Integer;
use num_traits::identities::{One, Zero};
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::str::FromStr;

#[derive(Clone, std::cmp::PartialEq)]
pub struct PrimeField {
    modulus: BigInt,
}

#[derive(Clone, std::cmp::PartialEq)]
pub struct PrimeFieldElement {
    f: PrimeField,
    n: BigInt,
}

impl PrimeField {
    pub fn new(prime: BigUint) -> PrimeField {
        // TODO: verify whether modulus is a prime number.
        let modulus = prime.to_bigint().unwrap();
        PrimeField { modulus }
    }
    #[inline]
    pub fn new_elt(&self, n: BigInt) -> PrimeFieldElement {
        let n = n.mod_floor(&self.modulus);
        PrimeFieldElement { f: self.clone(), n }
    }
}

impl std::fmt::Display for PrimeField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "GF({})", self.modulus)
    }
}

impl PrimeField {
    pub fn new_elt_str(&self, s: &str) -> PrimeFieldElement {
        self.new_elt(BigInt::from_str(s).unwrap())
    }
    pub fn zero(&self) -> PrimeFieldElement {
        self.new_elt(BigInt::zero())
    }
    pub fn one(&self) -> PrimeFieldElement {
        self.new_elt(BigInt::one())
    }
}

macro_rules! impl_from_factory {
    ($target:ident produces $product:ident from $($other:ty)+ ) => {
    use std::convert::From;
     $(
        impl crate::FromFactory<$other> for $target {
            type Output = $product;
            fn from(&self, n: $other) -> Self::Output {
                self.new_elt(BigInt::from(n))
            }
        }
    )+
    };
}

impl_from_factory!(PrimeField produces PrimeFieldElement from u8 u16 u32 u64 i8 i16 i32 i64);

impl std::fmt::Display for PrimeFieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.n)
    }
}

impl PrimeFieldElement {
    #[inline]
    fn neg_mod(&self) -> PrimeFieldElement {
        self.f.new_elt(-&self.n)
    }
    #[inline]
    fn add_mod(&self, other: &PrimeFieldElement) -> PrimeFieldElement {
        self.f.new_elt(&self.n + &other.n)
    }
    #[inline]
    fn sub_mod(&self, other: &PrimeFieldElement) -> PrimeFieldElement {
        self.f.new_elt(&self.n - &other.n)
    }
    #[inline]
    fn mul_mod(&self, other: &PrimeFieldElement) -> PrimeFieldElement {
        self.f.new_elt(&self.n * &other.n)
    }
    #[inline]
    fn inv_mod(&self) -> PrimeFieldElement {
        let p = &self.f.modulus;
        let p_minus_2 = p.sub(2u32);
        self.f.new_elt(self.n.modpow(&p_minus_2, &p))
    }
}

impl<'a> Div<&'a PrimeFieldElement> for u32 {
    type Output = PrimeFieldElement;
    #[inline]
    fn div(self, other: &PrimeFieldElement) -> Self::Output {
        do_if_eq!(self, 1u32, other.inv_mod(), ERR_INV_OP)
    }
}

impl_binary_op!(PrimeFieldElement, Add, add, add_mod, f, ERR_BIN_OP);
impl_binary_op!(PrimeFieldElement, Sub, sub, sub_mod, f, ERR_BIN_OP);
impl_binary_op!(PrimeFieldElement, Mul, mul, mul_mod, f, ERR_BIN_OP);
impl_unary_op!(PrimeFieldElement, Neg, neg, neg_mod);

const ERR_BIN_OP: &'static str = "elements of different fields";
const ERR_INV_OP: &'static str = "numerator must be 1u32";

impl num_traits::identities::Zero for PrimeFieldElement {
    fn zero() -> Self {
        unimplemented!()
    }
    fn is_zero(&self) -> bool {
        self.n.is_zero()
    }
    fn set_zero(&mut self) {
        self.n.set_zero();
    }
}

impl num_traits::identities::One for PrimeFieldElement {
    fn one() -> Self {
        unimplemented!()
    }
    fn is_one(&self) -> bool {
        self.n.is_one()
    }
    fn set_one(&mut self) {
        self.n.set_one();
    }
}
