#[macro_use]
extern crate bencher;
use bencher::benchmark_group;
use bencher::Bencher;

extern crate num_bigint;
use num_bigint::BigUint;
use redox_ecc::field::PrimeField;

fn fp_add(b: &mut Bencher) {
    let f = PrimeField::new(BigUint::from(101u64));
    let mut x = f.elt(15u64);
    let y = f.elt(25u64);

    b.iter(|| {
        for _ in 0..1000 {
            x = &x + &y;
        }
    });
}

fn fp_mul(b: &mut Bencher) {
    let f = PrimeField::new(BigUint::from(101u64));
    let mut x = f.elt(15u64);
    let y = f.elt(25u64);

    b.iter(|| {
        for _ in 0..1000 {
            x = &x + &y + &y + &y + &y + &y;
        }
    });
}

benchmark_group!(field_arith, fp_add, fp_mul);
benchmark_main!(field_arith);
