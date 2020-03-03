extern crate num_bigint;

use criterion::{criterion_group, criterion_main, Benchmark, Criterion};

use redox_ecc::h2c::HashToCurve;
use redox_ecc::suites::{P256_SHA256_SSWU_NU_, P384_SHA512_SSWU_NU_, P521_SHA512_SSWU_NU_};

fn h2c(c: &mut Criterion) {
    let msg = "message to be hashed".as_bytes();
    let dst = "domain separation tag".as_bytes();
    for suite in vec![
        P256_SHA256_SSWU_NU_,
        P384_SHA512_SSWU_NU_,
        P521_SHA512_SSWU_NU_,
    ] {
        let h = suite.get(dst);
        c.bench(
            format!("{}", suite).as_str(),
            Benchmark::new("hash", move |b| {
                b.iter(|| {
                    h.hash(msg);
                })
            })
            .sample_size(10),
        );
    }
}

criterion_group!(h2c_bench, h2c);
criterion_main!(h2c_bench);
