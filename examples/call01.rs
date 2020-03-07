use redox_ecc::ellipticcurve::EllipticCurve;
use redox_ecc::instances::{GetCurve, CURVE25519, P256, P384};

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
    let ec = CURVE25519.get();
    let g0 = ec.get_generator();
    let g1 = ec.get_generator();
    println!("G: {} ", g0 + g1);
}
