//! This is documentation for the `primefield` module.
//!
//! The primefield module is meant to be used for bar.

use hkdf::Hkdf;
use impl_ops::impl_op_ex;
use num_bigint::{BigInt, BigUint, Sign, ToBigInt};
use num_integer::Integer;
use num_traits::cast::ToPrimitive;
use num_traits::identities::{One, Zero};
use sha2::{Sha256, Sha384, Sha512};

use std::cell::RefCell;
use std::cmp::Ordering;
use std::ops;
use std::ops::{BitXor, Div};
use std::rc::Rc;

use crate::do_if_eq;
use crate::field::{CMov, Field, FieldElement, FromFactory, IntoFactory, Sgn0, Sqrt};
use crate::h2c::{HashID, HashToField};

struct Params {
    p: BigInt,
    sqrt_precmp: RefCell<SqrtPrecmp>,
}

impl PartialEq for Params {
    fn eq(&self, other: &Self) -> bool {
        self.p == other.p
    }
}

/// Fp implements a base field of prime characteristic.
#[derive(Clone, PartialEq)]
pub struct Fp(Rc<Params>);

impl Fp {
    /// Use `new` to generate a prime field instance.
    /// ```
    ///   let f = Fp::new(BigUint::from(101));
    /// ```
    /// The `modulus` should be a prime number.
    pub fn new(modulus: BigUint) -> Self {
        // TODO: verify whether p is prime.
        let p = modulus.to_bigint().unwrap();
        let f = Fp(Rc::new(Params {
            p,
            sqrt_precmp: RefCell::new(SqrtPrecmp::Empty),
        }));
        f.0.sqrt_precmp.replace(SqrtPrecmp::new(&f));
        f
    }
}

impl Field for Fp {
    type Elt = FpElt;
    fn elt(&self, n: BigInt) -> Self::Elt {
        let n = n.mod_floor(&self.0.p);
        let f = self.0.clone();
        FpElt { n, f }
    }
    fn zero(&self) -> Self::Elt {
        self.elt(BigInt::zero())
    }
    fn one(&self) -> Self::Elt {
        self.elt(BigInt::one())
    }
}

macro_rules! impl_from_factory {
    ($target:ident, <$($other:ty)+> ) => {
     $(
         impl FromFactory<$other> for $target{
            fn from(&self, n: $other) -> <Self as Field>::Elt {
                self.elt(BigInt::from(n))
            }
        }
    )+
    };
}

impl_from_factory!(Fp, <u8 u16 u32 u64 i8 i16 i32 i64>);

macro_rules! impl_into_factory {
    ($target:ident, <$($other:ty)+> ) => {
     $(
         impl IntoFactory<$target> for $other{
            fn lift(&self, fab: $target) ->  <$target as Field>::Elt {
                fab.elt(BigInt::from(*self))
            }
        }
    )+
    };
}

impl_into_factory!(Fp, <u8 u16 u32 u64 i8 i16 i32 i64>);

impl<'a> FromFactory<&'a str> for Fp {
    fn from(&self, s: &'a str) -> <Self as Field>::Elt {
        use std::str::FromStr;
        self.elt(BigInt::from_str(s).unwrap())
    }
}

/// FpElt is an element of a prime field.
#[derive(Clone, std::cmp::PartialEq)]
pub struct FpElt {
    n: BigInt,
    f: Rc<Params>,
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

impl FieldElement for FpElt {}

impl FpElt {
    #[inline]
    fn red(&self, n: BigInt) -> FpElt {
        let n = n.mod_floor(&self.f.p);
        let f = self.f.clone();
        FpElt { n, f }
    }
    #[inline]
    fn inv_mod(&self) -> FpElt {
        let p_minus_2 = &self.f.p.to_biguint().unwrap() - 2u32;
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
impl_op_ex!(/|a: &FpElt, b: &FpElt| -> FpElt { a * b.inv_mod() });
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

impl CMov for FpElt {}

#[derive(Clone, std::cmp::PartialEq)]
enum SqrtPrecmp {
    Empty,
    P3MOD4 { exp: BigInt },
    P5MOD8 { exp: BigInt, sqrt_minus_one: FpElt },
    P9MOD16,
    P1MOD16,
}

impl SqrtPrecmp {
    fn new(f: &Fp) -> SqrtPrecmp {
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

impl Sqrt for FpElt {
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

impl Sgn0 for FpElt {
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

impl std::fmt::Display for Fp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "GF({})", &self.0.p)
    }
}

impl HashToField<Fp> for Fp {
    fn hash(&self, h: HashID, msg: &[u8], dst: &[u8], ctr: u8, l: usize) -> FpElt {
        let info: [u8; 5] = [b'H', b'2', b'C', ctr, 1u8];
        let mut vmsg = msg.to_vec();
        vmsg.push(0u8);
        let mut v = Vec::new();
        v.resize(l, 0);
        match match h {
            HashID::SHA256 => Hkdf::<Sha256>::new(Some(dst), &vmsg).expand(&info, &mut v),
            HashID::SHA384 => Hkdf::<Sha384>::new(Some(dst), &vmsg).expand(&info, &mut v),
            HashID::SHA512 => Hkdf::<Sha512>::new(Some(dst), &vmsg).expand(&info, &mut v),
        } {
            Ok(_) => self.elt(BigInt::from_bytes_be(Sign::Plus, &v)),
            Err(e) => panic!(e),
        }
    }
}

const ERR_BIN_OP: &str = "elements of different fields";
const ERR_EXP_SQR_OP: &str = "exponent must be 2u32";
const ERR_EXP_INV_OP: &str = "exponent must be -1i32";
const ERR_INV_OP: &str = "numerator must be 1u32";