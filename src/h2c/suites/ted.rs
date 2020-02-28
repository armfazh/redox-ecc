use crate::edwards::Curve as TeCurve;
use crate::edwards::CurveID;
use crate::ellipticcurve::EllipticCurve;
use crate::field::{FromFactory, Sgn0Endianness};
use crate::h2c::{EncodeToCurve, Encoding, HashID, MapToCurve};
use crate::instances::{CURVE25519, EDWARDS25519, EDWARDS448};
use crate::montgomery::Curve as MtCurve;
use crate::montgomery::Ell2;
// use crate::edwards::ratmap::Te2Mt25519;

#[derive(Copy, Clone)]
pub enum MapID {
    EDELL2,
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
    pub fn get(&self, dst: &[u8]) -> impl EncodeToCurve<E = TeCurve> {
        // let ratmap = match self.curve {
        //     EDWARDS25519 => {
        //         let e0 = Curve::from(EDWARDS25519);
        //         let e1 = MtCurve::from(CURVE25519);
        //         let invsqrD = e0.f.from(9);
        //         Te2Mt25519 { e0, e1, invsqrD }
        //     } // _ => unimplemented!(),
        // };

        let (h, l, ro) = (self.h, self.l, self.ro);
        let dst = dst.to_vec();
        let e = self.curve.get();
        let cofactor = e.new_scalar(e.get_cofactor());
        let map_to_curve: Box<dyn MapToCurve<E = TeCurve>> = match self.map {
            _ => unimplemented!(), // MapID::EDELL2 => Box::new(Ell2::new(e.clone(), e.f.from(self.z), self.s)),
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
    map: MapID::EDELL2,
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
    map: MapID::EDELL2,
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
    map: MapID::EDELL2,
    z: -1,
    l: 84,
    ro: false,
};
pub static EDWARDS448_SHA512_EDELL2_RO_: Suite = Suite {
    name: "edwards448-SHA512-EDELL2-RO-",
    ro: true,
    ..EDWARDS448_SHA512_EDELL2_NU_
};
