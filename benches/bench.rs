use criterion::{criterion_group, criterion_main, Criterion};
use indexa::index::dict::ngram::ngram::Ngram;

fn bench(ctx: &mut Criterion) {
    let ngram = Ngram::<10>::try_from("abcaoesuro").unwrap();
    let serialized = bincode::serialize(&ngram).unwrap();

    ctx.bench_function("Deser Ngram", |b| {
        b.iter(|| {
            let _desered: Ngram<10> = bincode::deserialize(&serialized).unwrap();
        });
    });
}

criterion_group!(gp1, bench);
criterion_main!(gp1);
