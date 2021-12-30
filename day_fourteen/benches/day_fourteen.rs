use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_fourteen::{get_instructions, mutate_polymer, DAY_FOURTEEN_EXAMPLE_INPUT};
use std::fs::File;
use std::io::prelude::*;

fn example_polymers_small(c: &mut Criterion) {
    let mut file = File::open(DAY_FOURTEEN_EXAMPLE_INPUT.to_string()).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();
    if let Some((start, rules)) = get_instructions(&contents) {
        c.bench_function("example_polymers small", |b| {
            b.iter(|| mutate_polymer(black_box(&start), black_box(rules.clone()), black_box(10)))
        });
    }
}

criterion_group!(benches, example_polymers_small,);
criterion_main!(benches);
