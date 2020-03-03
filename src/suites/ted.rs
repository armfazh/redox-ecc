use crate::edwards::Curve as TeCurve;
use crate::edwards::{CurveID, Ell2};
use crate::ellipticcurve::EllipticCurve;
use crate::ellipticcurve::RationalMap;
use crate::field::{FromFactory, Sgn0Endianness};
use crate::h2c::{Encoding, HashID, HashToCurve, MapToCurve};
use crate::instances::{edwards25519_to_curve25519, edwards448_to_curve448};
use crate::instances::{EDWARDS25519, EDWARDS448};
use crate::montgomery::Curve as MtCurve;

#[derive(Copy, Clone)]
pub enum MapID {
    ELL2,
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
    pub fn get(&self, dst: &[u8]) -> impl HashToCurve<E = TeCurve> {
        let ratmap: Option<Box<dyn RationalMap<E0 = TeCurve, E1 = MtCurve>>> =
            match self.curve.0.name {
                "edwards25519" => Some(Box::new(edwards25519_to_curve25519())),
                "edwards448" => Some(Box::new(edwards448_to_curve448())),
                _ => None,
            };
        let (h, l, ro) = (self.h, self.l, self.ro);
        let dst = dst.to_vec();
        let e = self.curve.get();
        let cofactor = e.new_scalar(e.get_cofactor());
        let map_to_curve: Box<dyn MapToCurve<E = TeCurve>> = match self.map {
            MapID::ELL2 => Box::new(Ell2::new(e.clone(), e.f.from(self.z), self.s, ratmap)),
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

pub static EDWARDS25519_SHA256_EDELL2_NU_: Suite = Suite {
    name: "edwards25519-SHA256-EDELL2-NU-",
    curve: EDWARDS25519,
    s: Sgn0Endianness::LittleEndian,
    h: HashID::SHA256,
    map: MapID::ELL2,
    z: 2,
    l: 48,
    ro: false,
};
pub static EDWARDS25519_SHA256_EDELL2_RO_: Suite = Suite {
    name: "edwards25519-SHA256-EDELL2-RO-",
    ro: true,
    ..EDWARDS25519_SHA256_EDELL2_NU_
};

pub static EDWARDS25519_SHA512_EDELL2_NU_: Suite = Suite {
    name: "edwards25519-SHA512-EDELL2-NU-",
    curve: EDWARDS25519,
    s: Sgn0Endianness::LittleEndian,
    h: HashID::SHA512,
    map: MapID::ELL2,
    z: 2,
    l: 48,
    ro: false,
};
pub static EDWARDS25519_SHA512_EDELL2_RO_: Suite = Suite {
    name: "edwards25519-SHA512-EDELL2-RO-",
    ro: true,
    ..EDWARDS25519_SHA512_EDELL2_NU_
};

pub static EDWARDS448_SHA512_EDELL2_NU_: Suite = Suite {
    name: "edwards448-SHA512-EDELL2-NU-",
    curve: EDWARDS448,
    s: Sgn0Endianness::LittleEndian,
    h: HashID::SHA512,
    map: MapID::ELL2,
    z: -1,
    l: 84,
    ro: false,
};
pub static EDWARDS448_SHA512_EDELL2_RO_: Suite = Suite {
    name: "edwards448-SHA512-EDELL2-RO-",
    ro: true,
    ..EDWARDS448_SHA512_EDELL2_NU_
};
