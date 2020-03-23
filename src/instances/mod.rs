mod edw;
mod mont;
mod rational_maps;
mod weier;

pub use crate::instances::edw::{EdCurveID, EDWARDS25519, EDWARDS448};
pub use crate::instances::mont::{MtCurveID, CURVE25519, CURVE448};
pub use crate::instances::rational_maps::{
    edwards25519_to_curve25519, edwards448_to_curve448, get_isogeny_bls12381g1,
    get_isogeny_secp256k1,
};
pub use crate::instances::weier::{
    WeCurveID, BLS12381G1, BLS12381G1_11ISO, P256, P384, P521, SECP256K1, SECP256K1_3ISO,
};

use crate::ellipticcurve::EllipticCurve;

/// Obtains a curve from a curve identifier.
pub trait GetCurve {
    type E: EllipticCurve;
    fn get(&self) -> Self::E;
}
