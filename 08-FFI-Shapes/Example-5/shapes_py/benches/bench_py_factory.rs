use divan::{Bencher, black_box};

use shapes::circle::Circle;
use shapes::triangle::Triangle;

use shapes_py::circle::CircleWrapper;
use shapes_py::factory::ShapeFactory;

const SHAPE_TUPLES: &'static [(&str, &[f64])] = &[
    (&"Triangle", &[3.0, 4.0, 5.0]),
    (&"Right Triangle", &[3.0, 4.0]),
    (&"Equilateral Triangle", &[5.0]),
    (&"Circle", &[5.0]),
    (&"Square", &[5.0]),
];

const SHAPE_NAMES: &'static [&str] = &[
    &"Triangle",
    &"Right Triangle",
    &"Equilateral Triangle",
    &"Circle",
    &"Square",
];

#[divan::bench(min_time = 1, args = ["Circle", "Square", "Triangle", "Right Triangle", "Equilateral Triangle"])]
fn bench_is_known_valid(name: &str) {
    let _ = ShapeFactory::is_known(black_box(name));
}

#[divan::bench(min_time = 1, args = ["Not Known"])]
fn bench_is_known_invalid(name: &str) {
    let _ = ShapeFactory::is_known(black_box(name));
}

#[divan::bench(min_time = 1)]
fn bench_number_known() {
    let _ = ShapeFactory::number_known();
}

#[divan::bench(min_time = 1)]
fn bench_list_known() {
    let _ = ShapeFactory::list_known();
}

#[divan::bench(min_time = 1, args = SHAPE_TUPLES)]
fn bench_create(name_and_vals: (&str, &[f64])) {
    let (name, _) = name_and_vals;

    let _ = ShapeFactory::create(black_box(&name));
}

#[divan::bench(min_time = 1, args = SHAPE_TUPLES)]
fn bench_create_with(name_and_vals: (&str, &[f64])) {
    let (name, dims) = name_and_vals;
    let _ = ShapeFactory::create_with(black_box(&name), black_box(dims.to_vec()));
}

#[divan::bench(min_time = 1)]
fn bench_create_with_invalid_name(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let dims = vec![1.0, 3.0];

            dims
        })
        .bench_values(|dims| {
            let _ = ShapeFactory::create_with(black_box("Unknown Shape"), black_box(dims));
        });
}

#[divan::bench(min_time = 1, args = SHAPE_NAMES)]
fn bench_create_with_invalid_dims(bencher: Bencher, name: &str) {
    bencher
        .with_inputs(|| {
            let dims = vec![1.0, 3.0, 5.0, 7.0, 8.0, 9.0];

            dims
        })
        .bench_values(|dims| {
            let _ = ShapeFactory::create_with(black_box(&name), black_box(dims));
        });
}

fn main() {
    divan::main();
}
