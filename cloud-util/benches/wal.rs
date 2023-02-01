use cloud_util::wal::{LogType, Wal};
use criterion::{criterion_group, criterion_main, Criterion};

async fn benchmark() {
    let mut wal = Wal::create("../target/benchmark").await.unwrap();
    let _ = wal
        .save(0, LogType::FinalizeBlock, &[0; 102400000])
        .await
        .map_err(|e| {
            panic!("wal_save_message: failed: {e}");
        });
    wal.clear_file()
        .await
        .map_err(|e| {
            panic!("exec_block(0) wal clear_file error: {e}");
        })
        .unwrap();
}

fn runner() {
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(benchmark());
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("wal", |b| b.iter(runner));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
