use advent_of_code_2024::*;
use criterion::{criterion_group, criterion_main, Criterion};

macro_rules! bench_please {
    ($lib:ident) => {
        pub fn $lib(c: &mut Criterion) {
            let raw_input = read_input_file(&format!("input/2024/{}.txt", stringify!($lib)));
            c.bench_function(&format!("{} input parser", stringify!($lib)), |b| b.iter(|| $lib::input_generator(&raw_input)));
            let input = $lib::input_generator(&raw_input);
            c.bench_function(&format!("{} part 1", stringify!($lib)), |b| b.iter(|| $lib::part1(input.clone())));
            c.bench_function(&format!("{} part 2", stringify!($lib)), |b| b.iter(|| $lib::part2(input.clone())));
        }

    };
    ($lib:ident, $($func:ident),+) => {
        pub fn $lib(c: &mut Criterion) {
            let raw_input = read_input_file(&format!("input/2024/{}.txt", stringify!($lib)));
            c.bench_function(&format!("{} input parser", stringify!($lib)), |b| b.iter(|| $lib::input_generator(&raw_input)));
            let input = $lib::input_generator(&raw_input);
            c.bench_function(&format!("{} part 1", stringify!($lib)), |b| b.iter(|| $lib::part1(input.clone())));
            c.bench_function(&format!("{} part 2", stringify!($lib)), |b| b.iter(|| $lib::part2(input.clone())));
            $(c.bench_function(&format!("{} {}", stringify!($lib), stringify!($func)), |b| b.iter(|| $lib::$func(input.clone())));)*
        }
    }
}

bench_please!(day1, part1, part2);
bench_please!(day2, part1, part2);

criterion_group!(all, day1, day2);
criterion_group!(single, day2);
criterion_main!(single);
