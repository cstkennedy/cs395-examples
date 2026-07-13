use std::io::BufReader;

use divan::{Bencher, black_box};

use shapes_py::parser::ShapeParser;

const LINES: [&str; 5] = [
    "Triangle; 1 2 3",
    "Right Triangle; 3 4",
    "Equilateral Triangle; 5",
    "Square; 5",
    "Circle; 5",
];

const ERROR_LINES: [&str; 2] = ["1337 Haxor; 1 2 3", "1337 Haxor; invalid input"];

#[divan::bench(min_time = 1, args = LINES)]
fn test_bench_parse_shape_line_ok(line: &str) {
    let _ = ShapeParser::read_shape(black_box(line));
}

#[divan::bench(min_time = 1, args = ERROR_LINES)]
fn test_bench_parse_shape_line_err(line: &str) {
    let _ = ShapeParser::read_shape(black_box(line));
}

fn main() {
    divan::main();
}
