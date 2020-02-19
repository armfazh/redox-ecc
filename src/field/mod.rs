//! This is documentation for the `field` module.
//!
//! The field module is meant to be used for bar.

extern crate num_bigint;
use num_bigint::{BigInt, BigUint, ToBigInt};

extern crate num_integer;
use num_integer::Integer;

use num_traits::identities::{One, Zero};

use std::ops::{Add, Div, Mul, Neg, Sub};
use std::rc::Rc;

use crate::Field;
use crate::{do_if_eq, impl_binary_op, impl_unary_op};

#[derive(Clone, std::cmp::PartialEq)]
pub struct PrimeField {
    p: Rc<BigInt>,
}

impl PrimeField {
    pub fn create(p: BigUint) -> Self {
        // TODO: verify whether p is prime.
        let p = Rc::new(p.to_bigint().unwrap());
        PrimeField { p }
    }
}

#[derive(Clone, std::cmp::PartialEq)]
pub struct FpElt {
    n: BigInt,
    p: Rc<BigInt>,
}

impl Field for PrimeField {
    type Elt = FpElt;
    fn new(&self, n: BigInt) -> Self::Elt {
        let n = n.mod_floor(&self.p);
        let p = self.p.clone();
        FpElt { n, p }
    }
    fn zero(&self) -> Self::Elt {
        self.new(BigInt::zero())
    }
    fn one(&self) -> Self::Elt {
        self.new(BigInt::one())
    }
}

macro_rules! impl_from_factory {
    ($target:ident, $output:ident, <$($other:ty)+> ) => {
     $(
         impl crate::FromFactory<$other> for $target{
             type Output = $output;
            fn from(&self, n: $other) -> Self::Output {
                self.new(BigInt::from(n))
            }
        }
    )+
    };
}

impl_from_factory!(PrimeField, FpElt, <u8 u16 u32 u64 i8 i16 i32 i64>);

impl crate::FromFactory<&str> for PrimeField {
    type Output = FpElt;
    fn from(&self, s: &str) -> Self::Output {
        use std::str::FromStr;
        self.new(BigInt::from_str(s).unwrap())
    }
}

impl FpElt {
    #[inline]
    fn red(&self, n: BigInt) -> FpElt {
        let n = n.mod_floor(&self.p);
        let p = self.p.clone();
        FpElt { n, p }
    }
    #[inline]
    fn neg_mod(&self) -> FpElt {
        self.red(-&self.n)
    }
    #[inline]
    fn add_mod(&self, other: &FpElt) -> FpElt {
        self.red(&self.n + &other.n)
    }
    #[inline]
    fn sub_mod(&self, other: &FpElt) -> FpElt {
        self.red(&self.n - &other.n)
    }
    #[inline]
    fn mul_mod(&self, other: &FpElt) -> FpElt {
        self.red(&self.n * &other.n)
    }
    #[inline]
    fn inv_mod(&self) -> FpElt {
        let p_minus_2 = &*self.p - 2u32;
        self.red(self.n.modpow(&p_minus_2, &self.p))
    }
}

impl<'a> Div<&'a FpElt> for u32 {
    type Output = FpElt;
    #[inline]
    fn div(self, other: &FpElt) -> Self::Output {
        do_if_eq!(self, 1u32, other.inv_mod(), ERR_INV_OP)
    }
}

impl_binary_op!(FpElt, Add, add, add_mod, p, ERR_BIN_OP);
impl_binary_op!(FpElt, Sub, sub, sub_mod, p, ERR_BIN_OP);
impl_binary_op!(FpElt, Mul, mul, mul_mod, p, ERR_BIN_OP);
impl_unary_op!(FpElt, Neg, neg, neg_mod);

const ERR_BIN_OP: &str = "elements of different fields";
const ERR_INV_OP: &str = "numerator must be 1u32";

impl num_traits::identities::Zero for FpElt {
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

impl num_traits::identities::One for FpElt {
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

impl std::fmt::Display for FpElt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let x = self.n.mod_floor(&self.p);
        write!(f, "{}", x)
    }
}
