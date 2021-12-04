use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_three::{calculate_life_system, calculate_power_consumption, DAY_THREE_INPUT};
use std::fs::File;
use std::io::prelude::*;

fn calculate_power_consumption_large(c: &mut Criterion) {
    let mut file = File::open(DAY_THREE_INPUT.to_string()).unwrap();
    let mut contents = String::new();

    let mut diagnostics = [[0; 12]; 1000];

    file.read_to_string(&mut contents).unwrap();

    let temp_diagnostics: Vec<Vec<u32>> = contents
        .lines()
        .map(|v| v.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    for (i, n) in temp_diagnostics.into_iter().enumerate() {
        for (j, m) in n.into_iter().enumerate() {
            diagnostics[i][j] = m;
        }
    }

    c.bench_function("calculate_power_consumption large", |b| {
        b.iter(|| calculate_power_consumption(black_box(&diagnostics)))
    });
}

fn calculate_power_consumption_small(c: &mut Criterion) {
    let diagnostics = [
        [0, 0, 1, 0, 0],
        [1, 1, 1, 1, 0],
        [1, 0, 1, 1, 0],
        [1, 0, 1, 1, 1],
        [1, 0, 1, 0, 1],
        [0, 1, 1, 1, 1],
        [0, 0, 1, 1, 1],
        [1, 1, 1, 0, 0],
        [1, 0, 0, 0, 0],
        [1, 1, 0, 0, 1],
        [0, 0, 0, 1, 0],
        [0, 1, 0, 1, 0],
    ];
    c.bench_function("calculate_power_consumption small", |b| {
        b.iter(|| calculate_power_consumption(black_box(&diagnostics)))
    });
}

fn calculate_life_system_large(c: &mut Criterion) {
    let mut file = File::open(DAY_THREE_INPUT.to_string()).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let diagnostics: Vec<i32> = contents
        .lines()
        .map(|s| i32::from_str_radix(s, 2).unwrap())
        .collect();
    let width = 12;
    c.bench_function("calculate_life_system large", |b| {
        b.iter(|| calculate_life_system(black_box(&diagnostics), black_box(width)))
    });
}

fn calculate_life_system_small(c: &mut Criterion) {
    let diagnostics = vec![
        0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000, 0b11001,
        0b00010, 0b01010,
    ];
    let width = 5;
    c.bench_function("calculate_life_system small", |b| {
        b.iter(|| calculate_life_system(black_box(&diagnostics), black_box(width)))
    });
}

criterion_group!(
    benches,
    calculate_power_consumption_small,
    calculate_power_consumption_large,
    calculate_life_system_small,
    calculate_life_system_large
);
criterion_main!(benches);
