//! This is documentation for the `redox-ecc` crate.
//!
//! The foo crate is meant to be used for bar.

// #![warn(missing_docs)]

#[cfg(test)]
mod tests;

pub trait FromFactory<T>: Sized {
    type Output;
    fn from(&self, _: T) -> Self::Output;
}

macro_rules! do_if_eq {
    ($x:expr, $y:expr, $body:stmt, $error: expr) => {
        if $x == $y {
            $body
        } else {
            panic!($error)
        }
    };
}

macro_rules! impl_unary_op {
    ($target:ident,
     $trait:ident,
     $name:ident,
     $method:ident) => {
        impl<'a> $trait for &'a $target {
            type Output = $target;
            #[inline]
            fn $name(self) -> Self::Output {
                self.$method()
            }
        }
        impl $trait for $target {
            type Output = $target;
            #[inline]
            fn $name(self) -> Self::Output {
                self.$method()
            }
        }
    };
}

macro_rules! impl_binary_op {
    ($target:ident,
     $trait:ident,
     $name:ident,
     $method:ident,
     $field:ident,
     $error:ident) => {
        impl<'a, 'b> $trait<&'b $target> for &'a $target {
            type Output = $target;
            #[inline]
            fn $name(self, other: &$target) -> Self::Output {
                do_if_eq!(self.$field, other.$field, self.$method(&other), $error)
            }
        }
        impl<'a> $trait<&'a $target> for $target {
            type Output = $target;
            #[inline]
            fn $name(self, other: &'a Self) -> Self::Output {
                do_if_eq!(self.$field, other.$field, self.$method(&other), $error)
            }
        }
        impl $trait for $target {
            type Output = $target;
            #[inline]
            fn $name(self, other: Self) -> Self::Output {
                do_if_eq!(self.$field, other.$field, self.$method(&other), $error)
            }
        }
    };
}

pub mod field;

pub mod curve;

pub mod scalar;

/// Returns the version of the crate.
pub fn version() -> &'static str {
    private_version();
    env!("CARGO_PKG_VERSION")
}

fn private_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

use curve::{WeierstrassCurve, WeierstrassProjectivePoint};
use field::PrimeField;
use std::str::FromStr;
extern crate num_bigint;
use num_bigint::BigUint;

#[derive(PartialEq, Eq)]
pub struct CurveID {
    pub name: &'static str,
    p: &'static str,
    a: &'static str,
    b: &'static str,
    r: &'static str,
    gx: &'static str,
    gy: &'static str,
}

impl CurveID {
    pub fn get_field(&self) -> PrimeField {
        PrimeField::new(BigUint::from_str(self.p).unwrap())
    }
    pub fn get_curve(&self) -> WeierstrassCurve {
        let f = self.get_field();
        let a = f.from_str(self.a);
        let b = f.from_str(self.b);
        let r = BigUint::from_str(self.r).unwrap();
        WeierstrassCurve { f, a, b, r }
    }
    pub fn get_generator(&self) -> WeierstrassProjectivePoint {
        let e = self.get_curve();
        e.new_point(e.f.from_str(self.gx), e.f.from_str(self.gy), e.f.one())
    }
}

/// P256 is the NIST P-256 elliptic curve.
pub static P256: CurveID = CurveID {
    name: "P256",
    p: "115792089210356248762697446949407573530086143415290314195533631308867097853951",
    a: "-3",
    b: "41058363725152142129326129780047268409114441015993725554835256314039467401291",
    r: "115792089210356248762697446949407573529996955224135760342422259061068512044369",
    gx: "48439561293906451759052585252797914202762949526041747995844080717082404635286",
    gy: "36134250956749795798585127919587881956611106672985015071877198253568414405109",
};
/// P384 is the NIST P-384 elliptic curve.
pub static P384: CurveID = CurveID {
    name: "P384",
    p: "39402006196394479212279040100143613805079739270465446667948293404245721771496870329047266088258938001861606973112319",
    a: "-3",
    b: "27580193559959705877849011840389048093056905856361568521428707301988689241309860865136260764883745107765439761230575",
    r: "39402006196394479212279040100143613805079739270465446667946905279627659399113263569398956308152294913554433653942643",
    gx: "26247035095799689268623156744566981891852923491109213387815615900925518854738050089022388053975719786650872476732087",
    gy: "8325710961489029985546751289520108179287853048861315594709205902480503199884419224438643760392947333078086511627871",
};
/// P521 is the NIST P-521 elliptic curve.
pub static P521: CurveID = CurveID {
    name: "P521",
    p: "6864797660130609714981900799081393217269435300143305409394463459185543183397656052122559640661454554977296311391480858037121987999716643812574028291115057151",
    a: "-3",
    b: "1093849038073734274511112390766805569936207598951683748994586394495953116150735016013708737573759623248592132296706313309438452531591012912142327488478985984",
    r: "6864797660130609714981900799081393217269435300143305409394463459185543183397655394245057746333217197532963996371363321113864768612440380340372808892707005449",
    gx:"2661740802050217063228768716723360960729859168756973147706671368418802944996427808491545080627771902352094241225065558662157113545570916814161637315895999846",
    gy:"3757180025770020463545507224491183603594455134769762486694567779615544477440556316691234405012945539562144444537289428522585666729196580810124344277578376784",
};
/// SECP256K1 is a 256-bit elliptic curve knwon as secp256k1.
pub static SECP256K1: CurveID = CurveID {
    name: "secp256k1",
    p: "115792089237316195423570985008687907853269984665640564039457584007908834671663",
    a: "0",
    b: "7",
    r: "115792089237316195423570985008687907852837564279074904382605163141518161494337",
    gx: "55066263022277343669578718895168534326250603453777594175500187360389116729240",
    gy: "32670510020758816978083085130507043184471273380659243275938904335757337482424",
};
