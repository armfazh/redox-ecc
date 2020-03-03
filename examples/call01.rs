use redox_ecc::ellipticcurve::EllipticCurve;
use redox_ecc::h2c::HashToCurve;
use redox_ecc::instances::{P256, P384};
use redox_ecc::suites::{
    EDWARDS25519_SHA256_EDELL2_NU_, EDWARDS25519_SHA256_EDELL2_RO_, EDWARDS25519_SHA512_EDELL2_NU_,
    EDWARDS25519_SHA512_EDELL2_RO_, EDWARDS448_SHA512_EDELL2_NU_, EDWARDS448_SHA512_EDELL2_RO_,
};
use redox_ecc::version;

fn main() {
    println!("{}", version());
    println!("Example!");

    let ec = P256.get();
    let g0 = ec.get_generator();
    let g1 = ec.get_generator();
    println!("G: {} ", g0 + g1);
    let ec = P384.get();
    let g0 = ec.get_generator();
    let g1 = ec.get_generator();
    println!("G: {} ", g0 + g1);
    let msg = "This is a message string".as_bytes();
    let dst = "QUUX-V01-CS02".as_bytes();
    for suite in [
        EDWARDS25519_SHA256_EDELL2_NU_,
        EDWARDS25519_SHA256_EDELL2_RO_,
        EDWARDS25519_SHA512_EDELL2_NU_,
        EDWARDS25519_SHA512_EDELL2_RO_,
        EDWARDS448_SHA512_EDELL2_NU_,
        EDWARDS448_SHA512_EDELL2_RO_,
    ]
    .iter()
    {
        let h = suite.get(dst);
        let mut p = h.hash(msg);
        p.normalize();
        println!("enc: {} {} ", suite, p);
    }
}
