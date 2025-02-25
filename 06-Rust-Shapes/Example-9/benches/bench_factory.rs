
use criterion::{black_box, criterion_group, criterion_main, Criterion};

use shapes::prelude::Factory;

use shapes::circle::Circle;
use shapes::equilateral_triangle::EquilateralTriangle;
use shapes::right_triangle::RightTriangle;
use shapes::square::Square;
use shapes::triangle::Triangle;

fn bench_is_known(c: &mut Criterion) {
    c.bench_function("is_known", |b| {
        b.iter(|| {
            let _ = Factory::is_known(black_box("Circle"));
            let _ = Factory::is_known(black_box("Square"));
            let _ = Factory::is_known(black_box("Triangle"));
            let _ = Factory::is_known(black_box("Right Triangle"));
            let _ = Factory::is_known(black_box("Equilateral Triangle"));
        })
    });
}

fn bench_number_known(c: &mut Criterion) {
    c.bench_function("number_known", |b| {
        b.iter(|| {
            let _ = Factory::number_known();
        })
    });
}

fn bench_list_known(c: &mut Criterion) {
    c.bench_function("list_known", |b| {
        b.iter(|| {
            let _ = Factory::list_known();
        })
    });
}

fn bench_create(c: &mut Criterion) {
    // I need to write this test...
}

fn bench_create_with(c: &mut Criterion) {
    c.bench_function("create_with - all shapes", |b| {
        b.iter(|| {
            let _ = Factory::create_with(black_box("Triangle"), black_box(&[3.0, 4.0, 5.0]));
            let _ = Factory::create_with(black_box("Right Triangle"), black_box(&[3.0, 4.0]));
            let _ = Factory::create_with(black_box("Equilateral Triangle"), black_box(&[5.0]));
            let _ = Factory::create_with(black_box("Circle"), black_box(&[5.0]));
            let _ = Factory::create_with(black_box("Square"), black_box(&[5.0]));
        })
    });
}

criterion_group!(benches, bench_is_known, bench_number_known, bench_list_known, bench_create_with);
criterion_main!(benches);
