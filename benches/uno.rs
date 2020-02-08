#[macro_use]
extern crate bencher;
use bencher::Bencher;

extern crate num_bigint;
use num_bigint::BigUint;
use redox_ecc::field::new_field;

fn add(b: &mut Bencher) {
    let f = new_field(BigUint::from(101u64));
    let mut x = f.elt(15u64);
    let y = f.elt(25u64);

    b.iter(|| {
        for _ in 0..1000 {
            x = &x + &y;
        }
    });
}

fn mul(b: &mut Bencher) {
    let f = new_field(BigUint::from(101u64));
    let mut x = f.elt(15u64);
    let y = f.elt(25u64);

    b.iter(|| {
        for _ in 0..1000 {
            x = &x + &y + &y + &y + &y + &y;
        }
    });
}

benchmark_group!(benches, add, mul);
benchmark_main!(benches);
