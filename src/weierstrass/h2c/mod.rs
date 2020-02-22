mod sswu;
mod svdw;

use crate::field::FpElt;
use crypto::digest::Digest;
use crypto::sha2::{Sha256, Sha512};

use crate::h2c::{EncodeToCurve, Encoding, MapToCurve};

use crate::weierstrass::{Curve, CurveID};
use crate::weierstrass::{P256, P384, P521, SECP256K1};
use crate::{EllipticCurve, FromFactory, Sgn0Choice};
use sswu::SSWU;
use svdw::SVDW;

pub struct Suite<D, M>
where
    D: Digest + Copy,
    M: MapToCurve<E = Curve>,
{
    name: &'static str,
    curve: CurveID,
    map: fn(_: Curve, _: FpElt, _: Sgn0Choice) -> M,
    h: fn() -> D,
    l: usize,
    z: i32,
    ro: bool,
    s: Sgn0Choice,
}

impl<D, M> Suite<D, M>
where
    D: Digest + Copy,
    M: MapToCurve<E = Curve>,
{
    pub fn new(&self, _dst: &[u8]) -> impl EncodeToCurve<E = Curve> {
        let e = Curve::from(self.curve);
        Encoding {
            map_to_curve: (self.map)(e.clone(), e.get_field().from(self.z), self.s),
            e,
            h: self.h,
            l: self.l,
            ro: self.ro,
        }
    }
}

impl<D, M> PartialEq for Suite<D, M>
where
    D: Digest + Copy,
    M: MapToCurve<E = Curve>,
{
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<D, M> std::fmt::Display for Suite<D, M>
where
    D: Digest + Copy,
    M: MapToCurve<E = Curve>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub static P256_SHA256_SSWU_NU_: Suite<Sha256, SSWU> = Suite {
    name: "P256-SHA256-SSWU-NU-",
    curve: P256,
    s: Sgn0Choice::Sgn0LE,
    h: Sha256::new,
    map: SSWU::new,
    z: -10,
    l: 48,
    ro: false,
};
pub static P256_SHA256_SSWU_RO_: Suite<Sha256, SSWU> = Suite {
    name: "P256-SHA256-SSWU-RO-",
    ro: true,
    ..P256_SHA256_SSWU_NU_
};

pub static P256_SHA256_SVDW_NU_: Suite<Sha256, SVDW> = Suite {
    name: "P256-SHA256-SVDW-NU-",
    curve: P256,
    h: Sha256::new,
    map: SVDW::new,
    s: Sgn0Choice::Sgn0LE,
    z: -3,
    l: 48,
    ro: false,
};
pub static P256_SHA256_SVDW_RO_: Suite<Sha256, SVDW> = Suite {
    name: "P256-SHA256-SVDW-RO-",
    ro: true,
    ..P256_SHA256_SVDW_NU_
};

pub static P384_SHA512_SSWU_NU_: Suite<Sha512, SSWU> = Suite {
    name: "P384-SHA512-SSWU-NU-",
    curve: P384,
    s: Sgn0Choice::Sgn0LE,
    h: Sha512::new,
    map: SSWU::new,
    z: -12,
    l: 72,
    ro: false,
};
pub static P384_SHA512_SSWU_RO_: Suite<Sha512, SSWU> = Suite {
    name: "P384-SHA512-SSWU-RO-",
    ro: true,
    ..P384_SHA512_SSWU_NU_
};

pub static P384_SHA512_SVDW_NU_: Suite<Sha512, SVDW> = Suite {
    name: "P384-SHA512-SVDW-NU-",
    curve: P384,
    h: Sha512::new,
    map: SVDW::new,
    s: Sgn0Choice::Sgn0LE,
    z: -1,
    l: 72,
    ro: false,
};
pub static P384_SHA512_SVDW_RO_: Suite<Sha512, SVDW> = Suite {
    name: "P384-SHA512-SVDW-RO-",
    ro: true,
    ..P384_SHA512_SVDW_NU_
};

pub static P521_SHA512_SSWU_NU_: Suite<Sha512, SSWU> = Suite {
    name: "P521-SHA512-SSWU-NU-",
    curve: P521,
    s: Sgn0Choice::Sgn0LE,
    h: Sha512::new,
    map: SSWU::new,
    z: -4,
    l: 96,
    ro: false,
};
pub static P521_SHA512_SSWU_RO_: Suite<Sha512, SSWU> = Suite {
    name: "P521-SHA512-SSWU-RO-",
    ro: true,
    ..P521_SHA512_SSWU_NU_
};

pub static P521_SHA512_SVDW_NU_: Suite<Sha512, SVDW> = Suite {
    name: "P521-SHA512-SVDW-NU-",
    curve: P521,
    h: Sha512::new,
    map: SVDW::new,
    s: Sgn0Choice::Sgn0LE,
    z: 1,
    l: 96,
    ro: false,
};
pub static P521_SHA512_SVDW_RO_: Suite<Sha512, SVDW> = Suite {
    name: "P521-SHA512-SVDW-RO-",
    ro: true,
    ..P521_SHA512_SVDW_NU_
};

pub static SECP256K1_SHA256_SSWU_NU_: Suite<Sha256, SSWU> = Suite {
    name: "secp256k1-SHA256-SSWU-NU-",
    curve: SECP256K1,
    s: Sgn0Choice::Sgn0LE,
    h: Sha256::new,
    map: SSWU::new,
    z: -11,
    l: 48,
    ro: false,
};
