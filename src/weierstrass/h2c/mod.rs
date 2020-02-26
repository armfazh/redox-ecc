mod sswu;
mod svdw;

use crate::ellipticcurve::EllipticCurve;
use crate::field::{FromFactory, Sgn0Endianness};
use crate::h2c::{EncodeToCurve, Encoding, HashID, MapToCurve};
use crate::weierstrass::{Curve, CurveID};
use crate::weierstrass::{P256, P384, P521, SECP256K1};
use sswu::SSWU;
use svdw::SVDW;

#[derive(Copy, Clone)]
pub enum MapID {
    SSWU,
    SVDW,
}

#[derive(Copy, Clone)]
pub struct Suite {
    name: &'static str,
    curve: CurveID,
    map: MapID,
    h: HashID,
    l: usize,
    z: i32,
    ro: bool,
    s: Sgn0Endianness,
}

impl Suite {
    pub fn get(&self, dst: &[u8]) -> impl EncodeToCurve<E = Curve> {
        let (h, l, ro) = (self.h, self.l, self.ro);
        let dst = dst.to_vec();
        let e = Curve::from(self.curve);
        let cofactor = e.new_scalar(e.get_cofactor());
        let map_to_curve: Box<dyn MapToCurve<E = Curve>> = match self.map {
            MapID::SSWU => Box::new(SSWU::new(e.clone(), e.f.from(self.z), self.s)),
            MapID::SVDW => Box::new(SVDW::new(e.clone(), e.f.from(self.z), self.s)),
        };
        Encoding {
            hash_to_field: Box::new(e.f),
            map_to_curve,
            dst,
            cofactor,
            h,
            l,
            ro,
        }
    }
}

impl std::fmt::Display for Suite {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub static P256_SHA256_SSWU_NU_: Suite = Suite {
    name: "P256-SHA256-SSWU-NU-",
    curve: P256,
    s: Sgn0Endianness::LittleEndian,
    h: HashID::SHA256,
    map: MapID::SSWU,
    z: -10,
    l: 48,
    ro: false,
};
pub static P256_SHA256_SSWU_RO_: Suite = Suite {
    name: "P256-SHA256-SSWU-RO-",
    ro: true,
    ..P256_SHA256_SSWU_NU_
};

pub static P256_SHA256_SVDW_NU_: Suite = Suite {
    name: "P256-SHA256-SVDW-NU-",
    curve: P256,
    h: HashID::SHA256,
    map: MapID::SVDW,
    s: Sgn0Endianness::LittleEndian,
    z: -3,
    l: 48,
    ro: false,
};
pub static P256_SHA256_SVDW_RO_: Suite = Suite {
    name: "P256-SHA256-SVDW-RO-",
    ro: true,
    ..P256_SHA256_SVDW_NU_
};

pub static P384_SHA512_SSWU_NU_: Suite = Suite {
    name: "P384-SHA512-SSWU-NU-",
    curve: P384,
    s: Sgn0Endianness::LittleEndian,
    h: HashID::SHA512,
    map: MapID::SSWU,
    z: -12,
    l: 72,
    ro: false,
};
pub static P384_SHA512_SSWU_RO_: Suite = Suite {
    name: "P384-SHA512-SSWU-RO-",
    ro: true,
    ..P384_SHA512_SSWU_NU_
};

pub static P384_SHA512_SVDW_NU_: Suite = Suite {
    name: "P384-SHA512-SVDW-NU-",
    curve: P384,
    h: HashID::SHA512,
    map: MapID::SVDW,
    s: Sgn0Endianness::LittleEndian,
    z: -1,
    l: 72,
    ro: false,
};
pub static P384_SHA512_SVDW_RO_: Suite = Suite {
    name: "P384-SHA512-SVDW-RO-",
    ro: true,
    ..P384_SHA512_SVDW_NU_
};

pub static P521_SHA512_SSWU_NU_: Suite = Suite {
    name: "P521-SHA512-SSWU-NU-",
    curve: P521,
    s: Sgn0Endianness::LittleEndian,
    h: HashID::SHA512,
    map: MapID::SSWU,
    z: -4,
    l: 96,
    ro: false,
};
pub static P521_SHA512_SSWU_RO_: Suite = Suite {
    name: "P521-SHA512-SSWU-RO-",
    ro: true,
    ..P521_SHA512_SSWU_NU_
};

pub static P521_SHA512_SVDW_NU_: Suite = Suite {
    name: "P521-SHA512-SVDW-NU-",
    curve: P521,
    h: HashID::SHA512,
    map: MapID::SVDW,
    s: Sgn0Endianness::LittleEndian,
    z: 1,
    l: 96,
    ro: false,
};
pub static P521_SHA512_SVDW_RO_: Suite = Suite {
    name: "P521-SHA512-SVDW-RO-",
    ro: true,
    ..P521_SHA512_SVDW_NU_
};

pub static SECP256K1_SHA256_SSWU_NU_: Suite = Suite {
    name: "secp256k1-SHA256-SSWU-NU-",
    curve: SECP256K1,
    s: Sgn0Endianness::LittleEndian,
    h: HashID::SHA256,
    map: MapID::SSWU,
    z: -11,
    l: 48,
    ro: false,
};
pub static SECP256K1_SHA256_SSWU_RO_: Suite = Suite {
    name: "secp256k1-SHA256-SSWU-RO-",
    ro: true,
    ..SECP256K1_SHA256_SSWU_NU_
};
