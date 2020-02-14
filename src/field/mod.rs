//! This is documentation for the `field` module.
//!
//! The field module is meant to be used for bar.

extern crate num_bigint;
use num_bigint::{BigInt, BigUint, ToBigInt};

extern crate num_integer;
use num_integer::Integer;

use num_traits::identities::{One, Zero};

use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::Field;
use crate::{do_if_eq, impl_binary_op, impl_unary_op};

#[derive(Clone, std::cmp::PartialEq)]
pub struct Fp {
    n: BigInt,
    p: BigInt,
}

impl Fp {
    pub fn create(p: BigUint) -> Fp {
        // TODO: verify whether p is prime.
        let n = BigInt::zero();
        let p = p.to_bigint().unwrap();
        Fp { n, p }
    }
}
impl Field for Fp {
    type Elt = Fp;
    fn new(&self, n: BigInt) -> Self::Elt {
        let n = n.mod_floor(&self.p);
        let p = self.p.clone();
        Fp { n, p }
    }
    fn zero(&self) -> Self::Elt {
        self.new(BigInt::zero())
    }
    fn one(&self) -> Self::Elt {
        self.new(BigInt::one())
    }
}

macro_rules! impl_from_factory {
    ($target:ident, <$($other:ty)+> ) => {
     $(
         impl crate::FromFactory<$other> for $target{
            fn from(&self, n: $other) -> Self {
                self.new(BigInt::from(n))
            }
        }
    )+
    };
}

impl_from_factory!(Fp, <u8 u16 u32 u64 i8 i16 i32 i64>);

impl crate::FromFactory<&str> for Fp {
    fn from(&self, s: &str) -> Self {
        use std::str::FromStr;
        self.new(BigInt::from_str(s).unwrap())
    }
}

impl Fp {
    #[inline]
    fn neg_mod(&self) -> Fp {
        self.new(-&self.n)
    }
    #[inline]
    fn add_mod(&self, other: &Fp) -> Fp {
        self.new(&self.n + &other.n)
    }
    #[inline]
    fn sub_mod(&self, other: &Fp) -> Fp {
        self.new(&self.n - &other.n)
    }
    #[inline]
    fn mul_mod(&self, other: &Fp) -> Fp {
        self.new(&self.n * &other.n)
    }
    #[inline]
    fn inv_mod(&self) -> Fp {
        let p = &self.p;
        let p_minus_2 = p.sub(2u32);
        self.new(self.n.modpow(&p_minus_2, &p))
    }
}

impl<'a> Div<&'a Fp> for u32 {
    type Output = Fp;
    #[inline]
    fn div(self, other: &Fp) -> Self::Output {
        do_if_eq!(self, 1u32, other.inv_mod(), ERR_INV_OP)
    }
}

impl_binary_op!(Fp, Add, add, add_mod, p, ERR_BIN_OP);
impl_binary_op!(Fp, Sub, sub, sub_mod, p, ERR_BIN_OP);
impl_binary_op!(Fp, Mul, mul, mul_mod, p, ERR_BIN_OP);
impl_unary_op!(Fp, Neg, neg, neg_mod);

const ERR_BIN_OP: &str = "elements of different fields";
const ERR_INV_OP: &str = "numerator must be 1u32";

impl num_traits::identities::Zero for Fp {
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

impl num_traits::identities::One for Fp {
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

impl std::fmt::Display for Fp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let x = self.n.mod_floor(&self.p);
        write!(f, "{}", x)
    }
}
