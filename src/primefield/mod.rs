//! This is documentation for the `primefield` module.
//!
//! The primefield module is meant to be used for bar.

use atomic_refcell::AtomicRefCell;
use impl_ops::impl_op_ex;
use num_bigint::{BigInt, BigUint, ToBigInt};
use num_integer::Integer;
use num_traits::cast::ToPrimitive;
use num_traits::identities::{One, Zero};

use std::ops;
use std::ops::{BitXor, Div};
use std::sync::Arc;

use crate::do_if_eq;
use crate::field::{CMov, Field, FieldElement, Sgn0, Sqrt};
use crate::ops::{Deserialize, FromFactory, Serialize};

struct Params {
    p: BigInt,
    sqrt_precmp: AtomicRefCell<Option<SqrtPrecmp>>,
}

impl Eq for Params {}

impl PartialEq for Params {
    fn eq(&self, other: &Self) -> bool {
        self.p == other.p
    }
}

/// Fp implements a base field of prime characteristic.
#[derive(Clone, PartialEq, Eq)]
pub struct Fp(Arc<Params>);

impl Fp {
    /// Use `new` to generate a prime field instance.
    /// ```
    ///  use num_bigint::BigUint;
    ///  use redox_ecc::primefield::Fp;
    ///  let f = Fp::new(BigUint::from(101u32));
    /// ```
    /// The `modulus` should be a prime number.
    pub fn new(modulus: BigUint) -> Self {
        // TODO: verify whether p is prime.
        Fp(Arc::new(Params {
            p: modulus.to_bigint().unwrap(),
            sqrt_precmp: AtomicRefCell::new(None),
        }))
    }
}

impl Field for Fp {
    type Elt = FpElt;
    fn elt(&self, n: BigInt) -> Self::Elt {
        let n = n.mod_floor(&self.0.p);
        let f = self.clone();
        FpElt { n, f }
    }
    fn zero(&self) -> Self::Elt {
        self.elt(BigInt::zero())
    }
    fn one(&self) -> Self::Elt {
        self.elt(BigInt::one())
    }
    fn get_modulus(&self) -> BigInt {
        self.0.p.clone()
    }
    fn size_bytes(&self) -> usize {
        (self.0.p.bits() as usize + 7) / 8
    }
}

impl Deserialize for Fp {
    type Deser = <Fp as Field>::Elt;
    fn from_bytes_be(&self, bytes: &[u8]) -> Result<Self::Deser, std::io::Error> {
        let n = BigUint::from_bytes_be(bytes);
        Ok(self.elt(n.to_bigint().unwrap()))
    }
    fn from_bytes_le(&self, bytes: &[u8]) -> Result<Self::Deser, std::io::Error> {
        let n = BigUint::from_bytes_le(bytes);
        Ok(self.elt(n.to_bigint().unwrap()))
    }
}

macro_rules! impl_from_factory {
    ($target:ident, <$($other:ty)+> ) => {
     $(
         impl FromFactory<$other> for $target{
            type Output = <Fp as Field>::Elt;
            fn from(&self, n: $other) -> Self::Output{
                self.elt(BigInt::from(n))
            }
        }
    )+
    };
}

impl_from_factory!(Fp, <u8 u16 u32 u64 i8 i16 i32 i64>);

impl FromFactory<&str> for Fp {
    type Output = <Fp as Field>::Elt;
    fn from(&self, s: &str) -> Self::Output {
        let mut sl = &s[0..];
        if sl.is_empty() {
            return self.zero();
        }
        let neg = if sl.starts_with('-') {
            sl = &sl[1..];
            -1
        } else {
            1
        };
        let radix = if sl.len() > 1 {
            match &sl[0..2] {
                "0o" => {
                    sl = &sl[2..];
                    8
                }
                "0x" => {
                    sl = &sl[2..];
                    16
                }
                "0b" => {
                    sl = &sl[2..];
                    2
                }
                _ => 10,
            }
        } else {
            10
        };
        self.elt(neg * BigInt::parse_bytes(sl.as_bytes(), radix).unwrap())
    }
}

/// FpElt is an element of a prime field.
#[derive(Clone, PartialEq, Eq)]
pub struct FpElt {
    n: BigInt,
    f: Fp,
}

impl FieldElement for FpElt {}

impl Serialize for FpElt {
    /// serializes the field element into big-endian bytes
    fn to_bytes_be(&self) -> Vec<u8> {
        let field_len = self.f.size_bytes();
        let mut bytes = self.n.to_biguint().unwrap().to_bytes_be();
        let mut out = vec![0; field_len - bytes.len()];
        if !out.is_empty() {
            out.append(&mut bytes);
        } else {
            out = bytes;
        }
        out
    }
    /// serializes the field element into little-endian bytes
    fn to_bytes_le(&self) -> Vec<u8> {
        let mut bytes = self.to_bytes_be();
        bytes.reverse();
        bytes
    }
}

// impl<'b> EltOps<&'b FpElt, FpElt> for FpElt {}
// impl<'a> EltOps<FpElt, FpElt> for &'a FpElt {}
impl<'a, 'b> std::ops::Add<&'b FpElt> for &'a FpElt {
    type Output = FpElt;
    fn add(self, other: &'b FpElt) -> FpElt {
        do_if_eq!(self.f == other.f, self.red(&self.n + &other.n), ERR_BIN_OP)
    }
}
impl<'a> std::ops::Add<FpElt> for &'a FpElt {
    type Output = FpElt;
    fn add(self, other: FpElt) -> FpElt {
        do_if_eq!(self.f == other.f, self.red(&self.n + &other.n), ERR_BIN_OP)
    }
}

impl FpElt {
    #[inline]
    fn red(&self, n: BigInt) -> FpElt {
        let n = n.mod_floor(&self.f.0.p);
        let f = self.f.clone();
        FpElt { n, f }
    }
    #[inline]
    fn inv_mod(&self) -> FpElt {
        let p_minus_2 = &self.f.0.p - 2u32;
        self ^ &p_minus_2
    }
}

impl_op_ex!(+|a: FpElt, b: &FpElt| -> FpElt {
    do_if_eq!(a.f == b.f, a.red(&a.n + &b.n), ERR_BIN_OP)
});
impl_op_ex!(-|a: &FpElt, b: &FpElt| -> FpElt {
    do_if_eq!(a.f == b.f, a.red(&a.n - &b.n), ERR_BIN_OP)
});
impl_op_ex!(*|a: &FpElt, b: &FpElt| -> FpElt {
    do_if_eq!(a.f == b.f, a.red(&a.n * &b.n), ERR_BIN_OP)
});

impl_op_ex!(/|a: &FpElt, b: &FpElt| -> FpElt {
    #[allow(clippy::suspicious_arithmetic_impl)] {
        a * b.inv_mod()
    }
});
impl_op_ex!(-|a: &FpElt| -> FpElt { a.red(-&a.n) });
impl_op_ex!(^|a: &FpElt, b: u32| -> FpElt {
    do_if_eq!(b == 2u32, a * a, ERR_EXP_SQR_OP)
});
impl_op_ex!(^|a: &FpElt, b: i32| -> FpElt {
    do_if_eq!(b == -1i32, a.inv_mod(), ERR_EXP_INV_OP)
});

impl<'a> Div<&'a FpElt> for u32 {
    type Output = FpElt;
    #[inline]
    fn div(self, other: &FpElt) -> Self::Output {
        do_if_eq!(self == 1u32, other.inv_mod(), ERR_INV_OP)
    }
}

impl<'a, 'b> BitXor<&'b BigUint> for &'a FpElt {
    type Output = FpElt;
    #[inline]
    fn bitxor(self, exp: &'b BigUint) -> Self::Output {
        let exp = exp.to_bigint().unwrap();
        self.red(self.n.modpow(&exp, &self.f.0.p))
    }
}

impl<'a, 'b> BitXor<&'b BigInt> for &'a FpElt {
    type Output = FpElt;
    #[inline]
    fn bitxor(self, exp: &'b BigInt) -> Self::Output {
        let expo = &exp.mod_floor(&(&self.f.0.p - 1)).to_biguint().unwrap();
        self ^ expo
    }
}

impl CMov for FpElt {}

#[derive(Clone, std::cmp::PartialEq)]
enum SqrtPrecmp {
    P3MOD4 { exp: BigInt },
    P5MOD8 { exp: BigInt, sqrt_minus_one: FpElt },
    P9MOD16,
    P1MOD16,
}
impl Fp {
    fn get_sqrt_precmp(&self) -> SqrtPrecmp {
        self.0
            .sqrt_precmp
            .borrow_mut()
            .get_or_insert(self.calc_sqrt_precmp())
            .clone()
    }
    fn calc_sqrt_precmp(&self) -> SqrtPrecmp {
        let p = &self.0.p;
        let res = (p % 16u32).to_u32().unwrap();
        if 3u32 == (res % 4u32) {
            let exp = (p + 1u32) >> 2usize;
            SqrtPrecmp::P3MOD4 { exp }
        } else if 5u32 == (res % 8u32) {
            let k = (p - 5u32) >> 3usize;
            let t = &(self.one() + self.one()) ^ &k; // t = 2^k
            let mut t0 = &t ^ 2u32; //  t^2
            t0 = &t0 + &t0; //          2t^2
            t0 = t0 + self.one(); //       2t^2+1
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

impl Sqrt for FpElt {
    #[inline]
    fn is_square(&self) -> bool {
        let p_minus_1_div_2 = (&self.f.0.p - 1) >> 1usize;
        let res: FpElt = self ^ &p_minus_1_div_2;
        res.is_one() || res.is_zero()
    }
    fn sqrt(&self) -> FpElt {
        let pre = self.f.get_sqrt_precmp();
        match pre {
            SqrtPrecmp::P3MOD4 { exp } => self ^ &exp,
            SqrtPrecmp::P5MOD8 {
                exp,
                sqrt_minus_one,
            } => {
                let t0 = self ^ &exp;
                let t1 = &t0 ^ 2u32;
                let e = *self == t1;
                let t1 = &t0 * sqrt_minus_one;
                FpElt::cmov(&t1, &t0, e)
            }
            SqrtPrecmp::P9MOD16 => unimplemented!(),
            SqrtPrecmp::P1MOD16 => unimplemented!(),
        }
    }
}

impl Sgn0 for FpElt {
    fn sgn0(&self) -> i32 {
        let res = (&self.n % 2u32).to_i32().unwrap();
        1i32 - 2i32 * res
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
        let x = self.n.mod_floor(&self.f.0.p);
        write!(f, "{}", x)
    }
}

impl std::fmt::Display for Fp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "GF({})", &self.0.p)
    }
}

const ERR_BIN_OP: &str = "elements of different fields";
const ERR_EXP_SQR_OP: &str = "exponent must be 2u32";
const ERR_EXP_INV_OP: &str = "exponent must be -1i32";
const ERR_INV_OP: &str = "numerator must be 1u32";
