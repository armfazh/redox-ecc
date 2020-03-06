mod mont;
mod rational_maps;
mod ted;
mod weier;

pub use crate::instances::mont::{CURVE25519, CURVE448};
pub use crate::instances::rational_maps::{edwards25519_to_curve25519, edwards448_to_curve448};
pub use crate::instances::ted::{EDWARDS25519, EDWARDS448};
pub use crate::instances::weier::{P256, P384, P521, SECP256K1};