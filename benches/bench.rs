use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion
};

fn bench(c: &mut Criterion) {
    c.bench_function("zx", |b| {
        b.iter(|| {
            black_box({
                let _result = std::process::Command::new("./target/release/zx")
                    .arg("-a")
                    .output()
                    .expect("command failed");
            });
        })
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
