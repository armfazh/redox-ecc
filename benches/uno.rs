#[macro_use]
extern crate bencher;
use bencher::Bencher;

use redox_ecc::version;

fn bench_add_two(b: &mut Bencher) {
    b.iter(|| {
        for _ in 0..50 {
            version();
        }
    });
}

benchmark_group!(benches, bench_add_two);
benchmark_main!(benches);
