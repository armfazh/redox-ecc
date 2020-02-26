use criterion::{criterion_group, criterion_main, Benchmark, Criterion};

use redox_ecc::field::FromFactory;
use redox_ecc::weierstrass;
use redox_ecc::weierstrass::{P256, P384, P521};

fn arith(c: &mut Criterion) {
    for id in [P256, P384, P521].iter() {
        let ec = weierstrass::Curve::from(*id);
        let mut x0 = ec.f.from(-1i64);
        let mut x1 = ec.f.from(-1i64);
        let mut x2 = ec.f.from(-1i64);
        let y0 = ec.f.from(15i64);
        let y1 = ec.f.from(15i64);

        c.bench(
            format!("{}/fp", id).as_str(),
            Benchmark::new("add", move |b| b.iter(|| x0 = &x0 + &y0))
                .with_function("mul", move |b| b.iter(|| x1 = &x1 * &y1))
                .with_function("inv", move |b| b.iter(|| x2 = 1u32 / &x2))
                .sample_size(10),
        );
    }
}

criterion_group!(field_bench, arith);
criterion_main!(field_bench);
