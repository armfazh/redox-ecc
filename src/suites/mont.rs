use crate::ellipticcurve::EllipticCurve;
use crate::field::{FromFactory, Sgn0Endianness};
use crate::h2c::{Encoding, HashID, HashToCurve, MapToCurve};
use crate::instances::{CURVE25519, CURVE448};
use crate::montgomery::Ell2;
use crate::montgomery::{Curve, CurveID};

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
    pub fn get(&self, dst: &[u8]) -> impl HashToCurve<E = Curve> {
        let (h, l, ro) = (self.h, self.l, self.ro);
        let dst = dst.to_vec();
        let e = self.curve.get();
        let cofactor = e.new_scalar(e.get_cofactor());
        let map_to_curve: Box<dyn MapToCurve<E = Curve>> = match self.map {
            MapID::ELL2 => Box::new(Ell2::new(e.clone(), e.f.from(self.z), self.s)),
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
pub static CURVE25519_SHA256_ELL2_NU_: Suite = Suite {
    name: "curve25519-SHA256-ELL2-NU-",
    curve: CURVE25519,
    s: Sgn0Endianness::LittleEndian,
    h: HashID::SHA256,
    map: MapID::ELL2,
    z: 2,
    l: 48,
    ro: false,
};
pub static CURVE25519_SHA256_ELL2_RO_: Suite = Suite {
    name: "curve25519-SHA256-ELL2-RO-",
    ro: true,
    ..CURVE25519_SHA256_ELL2_NU_
};

pub static CURVE25519_SHA512_ELL2_NU_: Suite = Suite {
    name: "curve25519-SHA512-ELL2-NU-",
    curve: CURVE25519,
    s: Sgn0Endianness::LittleEndian,
    h: HashID::SHA512,
    map: MapID::ELL2,
    z: 2,
    l: 48,
    ro: false,
};
pub static CURVE25519_SHA512_ELL2_RO_: Suite = Suite {
    name: "curve25519-SHA512-ELL2-RO-",
    ro: true,
    ..CURVE25519_SHA512_ELL2_NU_
};

pub static CURVE448_SHA512_ELL2_NU_: Suite = Suite {
    name: "curve448-SHA512-ELL2-NU-",
    curve: CURVE448,
    s: Sgn0Endianness::LittleEndian,
    h: HashID::SHA512,
    map: MapID::ELL2,
    z: -1,
    l: 84,
    ro: false,
};
pub static CURVE448_SHA512_ELL2_RO_: Suite = Suite {
    name: "curve448-SHA512-ELL2-RO-",
    ro: true,
    ..CURVE448_SHA512_ELL2_NU_
};
