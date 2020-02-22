//! This is documentation for the `field` module.
//!
//! The field module is meant to be used for bar.

use crypto::digest::Digest;
use crypto::hkdf::{hkdf_expand, hkdf_extract};
use num_bigint::{BigInt, BigUint, ToBigInt};
use num_integer::Integer;
use num_traits::cast::ToPrimitive;
use num_traits::identities::{One, Zero};

use std::cell::RefCell;
use std::cmp::Ordering;
use std::ops::BitXor;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::rc::Rc;

use crate::h2c::HashToField;
use crate::{do_if_eq, impl_binary_op, impl_unary_op};
use crate::{CMov, Field};

struct Params {
    p: BigInt,
    sqrt_precmp: RefCell<SqrtPrecmp>,
}

impl PartialEq for Params {
    fn eq(&self, other: &Self) -> bool {
        self.p == other.p
    }
}

#[derive(Clone, PartialEq)]
pub struct PrimeField(Rc<Params>);

impl PrimeField {
    pub fn create(p: BigUint) -> Self {
        // TODO: verify whether p is prime.
        let p = p.to_bigint().unwrap();
        let f = PrimeField(Rc::new(Params {
            p: p.clone(),
            sqrt_precmp: RefCell::new(SqrtPrecmp::Empty),
        }));
        f.0.sqrt_precmp.replace(SqrtPrecmp::new(&f));
        f
    }
}

#[derive(Clone, std::cmp::PartialEq)]
pub struct FpElt {
    n: BigInt,
    f: Rc<Params>,
}

impl Field for PrimeField {
    type Elt = FpElt;
    fn new(&self, n: BigInt) -> Self::Elt {
        let n = n.mod_floor(&self.0.p);
        let f = self.0.clone();
        FpElt { n, f }
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
            fn from(&self, n: $other) -> <Self as Field>::Elt {
                self.new(BigInt::from(n))
            }
        }
    )+
    };
}

impl_from_factory!(PrimeField, <u8 u16 u32 u64 i8 i16 i32 i64>);

macro_rules! impl_into_factory {
    ($target:ident, <$($other:ty)+> ) => {
     $(
         impl crate::IntoFactory<$target> for $other{
            fn lift(&self, fab: $target) ->  <$target as Field>::Elt {
                fab.new(BigInt::from(*self))
            }
        }
    )+
    };
}

impl_into_factory!(PrimeField, <u8 u16 u32 u64 i8 i16 i32 i64>);

impl crate::FromFactory<&str> for PrimeField {
    fn from(&self, s: &str) -> <Self as Field>::Elt {
        use std::str::FromStr;
        self.new(BigInt::from_str(s).unwrap())
    }
}

impl FpElt {
    #[inline]
    fn red(&self, n: BigInt) -> FpElt {
        let n = n.mod_floor(&self.f.p);
        let f = self.f.clone();
        FpElt { n, f }
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
        let p_minus_2 = &self.f.p.to_biguint().unwrap() - 2u32;
        self ^ &p_minus_2
    }
}

impl<'a> Div<&'a FpElt> for u32 {
    type Output = FpElt;
    #[inline]
    fn div(self, other: &FpElt) -> Self::Output {
        do_if_eq!(self, 1u32, other.inv_mod(), ERR_INV_OP)
    }
}

impl<'a> BitXor<u32> for &'a FpElt {
    type Output = FpElt;
    #[inline]
    fn bitxor(self, exp: u32) -> Self::Output {
        do_if_eq!(exp, 2u32, self * self, ERR_EXP_SQR_OP)
    }
}

impl<'a> BitXor<i32> for &'a FpElt {
    type Output = FpElt;
    #[inline]
    fn bitxor(self, exp: i32) -> Self::Output {
        do_if_eq!(exp, -1i32, self.inv_mod(), ERR_EXP_INV_OP)
    }
}

impl<'a, 'b> BitXor<&'b BigUint> for &'a FpElt {
    type Output = FpElt;
    #[inline]
    fn bitxor(self, exp: &'b BigUint) -> Self::Output {
        let exp = exp.to_bigint().unwrap();
        self.red(self.n.modpow(&exp, &self.f.p))
    }
}

impl<'a, 'b> BitXor<&'b BigInt> for &'a FpElt {
    type Output = FpElt;
    #[inline]
    fn bitxor(self, exp: &'b BigInt) -> Self::Output {
        let expo = &exp.mod_floor(&(&self.f.p - 1)).to_biguint().unwrap();
        self ^ expo
    }
}

impl_binary_op!(FpElt, Add, add, add_mod, f, ERR_BIN_OP);
impl_binary_op!(FpElt, Sub, sub, sub_mod, f, ERR_BIN_OP);
impl_binary_op!(FpElt, Mul, mul, mul_mod, f, ERR_BIN_OP);
impl_unary_op!(FpElt, Neg, neg, neg_mod);

const ERR_BIN_OP: &str = "elements of different fields";
const ERR_EXP_SQR_OP: &str = "exponent must be 2u32";
const ERR_EXP_INV_OP: &str = "exponent must be -1i32";
const ERR_INV_OP: &str = "numerator must be 1u32";

#[derive(Clone, std::cmp::PartialEq)]
enum SqrtPrecmp {
    Empty,
    P3MOD4 { exp: BigInt },
    P5MOD8 { exp: BigInt, sqrt_minus_one: FpElt },
    P9MOD16,
    P1MOD16,
}

impl SqrtPrecmp {
    fn new(f: &PrimeField) -> SqrtPrecmp {
        let p = &f.0.p;
        let res = (p % 16u32).to_u32().unwrap();
        if 3u32 == (res % 4u32) {
            let exp = (p + 1u32) >> 2usize;
            SqrtPrecmp::P3MOD4 { exp }
        } else if 5u32 == (res % 8u32) {
            let k = (p - 5u32) >> 3usize;
            let t = &(f.one() + f.one()) ^ &k; // t = 2^k
            let mut t0 = &t ^ 2u32; //  t^2
            t0 = &t0 + &t0; //          2t^2
            t0 = t0 + f.one(); //       2t^2+1
            t0 = t0 * t; //             t(2t^2+1)
            let exp = k + 1;
            let sqrt_minus_one = t0;
            SqrtPrecmp::P5MOD8 {
                exp,
                sqrt_minus_one,
            }
        } else if 9u32 == (res % 16u32) {
            SqrtPrecmp::P9MOD16
        } else {
            SqrtPrecmp::P1MOD16
        }
    }
}

impl crate::Sqrt for FpElt {
    #[inline]
    fn is_square(&self) -> bool {
        let p_minus_1_div_2 = (&self.f.p - 1) >> 1usize;
        let res: FpElt = self ^ &p_minus_1_div_2;
        res.is_one() || res.is_zero()
    }
    fn sqrt(&self) -> FpElt {
        let pre = &self.f.sqrt_precmp;
        match &*pre.borrow() {
            SqrtPrecmp::P3MOD4 { exp } => self ^ exp,
            SqrtPrecmp::P5MOD8 {
                exp,
                sqrt_minus_one,
            } => {
                let t0 = self ^ exp;
                let t1 = &t0 ^ 2u32;
                let e = *self == t1;
                let t1 = &t0 * sqrt_minus_one;
                FpElt::cmov(&t1, &t0, e)
            }
            SqrtPrecmp::P9MOD16 => unimplemented!(),
            SqrtPrecmp::P1MOD16 => unimplemented!(),
            SqrtPrecmp::Empty => unimplemented!(),
        }
    }
}

impl crate::Sgn0 for FpElt {
    fn sgn0_be(&self) -> i32 {
        let p_minus_1_div_2: BigInt = (&self.f.p - 1) >> 1usize;
        match &p_minus_1_div_2.cmp(&self.n) {
            Ordering::Equal | Ordering::Greater => 1,
            Ordering::Less => -1,
        }
    }
    fn sgn0_le(&self) -> i32 {
        let res = (&self.n % 2u32).to_i32().unwrap();
        1i32 - 2i32 * res
    }
}

impl crate::CMov for FpElt {
    #[inline]
    fn cmov(x: &FpElt, y: &FpElt, b: bool) -> FpElt {
        if b {
            y.clone()
        } else {
            x.clone()
        }
    }
}

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
        let x = self.n.mod_floor(&self.f.p);
        write!(f, "{}", x)
    }
}

impl std::fmt::Display for PrimeField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "GF({})", &self.0.p)
    }
}

impl HashToField for PrimeField {
    fn hash<D: Digest + Copy>(
        &self,
        hash_func: D,
        msg: &[u8],
        dst: &[u8],
        ctr: u8,
        l: usize,
    ) -> <Self as Field>::Elt {
        let info: [u8; 5] = [b'H', b'2', b'C', ctr, 1u8];
        let mut vmsg = msg.to_vec();
        vmsg.push(0u8);
        let mut msg_prime = Vec::new();
        msg_prime.resize(hash_func.output_bytes(), 0);
        hkdf_extract(hash_func, dst, &vmsg, &mut msg_prime);
        let mut v = Vec::new();
        v.resize(l, 0);
        hkdf_expand(hash_func, &msg_prime, &info, &mut v);
        self.new(BigInt::from_bytes_be(num_bigint::Sign::Plus, &v))
    }
}
