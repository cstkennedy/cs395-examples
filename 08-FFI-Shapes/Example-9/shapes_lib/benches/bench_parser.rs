use std::io::BufReader;

use divan::{Bencher, black_box};

use std::str::FromStr;

use shapes_lib::prelude::MonoShape;
use shapes_lib::prelude::Parser;

#[divan::bench(min_time = 1)]
fn bench_read_shapes() {
    // The read function should handle (i.e., ignore) leading whitespace
    let raw_str = r#"
        Triangle
        Right Triangle
        Equilateral Triangle
        Square
        Circle
        1337 Haxor"#;

    let some_shapes = Parser::read_shapes(black_box(raw_str.as_bytes()));
}

const LINES: [&str; 6] =  [
    "Triangle; 1 2 3",
    "Right Triangle; 3 4",
    "Equilateral Triangle; 5",
    "Square; 5",
    "Circle; 5",
    "1337 Haxor; invalid input"
];

#[divan::bench(min_time = 1, args = LINES)]
fn test_bench_parse_shape_line(line: &str) {
    let _ = MonoShape::from_str(black_box(line));
}

#[divan::bench(min_time = 1)]
fn test_bench_shapes_with() {
    // The read function should handle (i.e., ignore) leading whitespace
    let raw_str = r#"
        Triangle; 1 2 3
        Right Triangle; 3 4
        Equilateral Triangle; 5
        Square; 5
        Circle; 5
        1337 Haxor; invalid input"#;

    let some_shapes = Parser::read_shapes_with(black_box(raw_str.as_bytes()));
}

fn main() {
    divan::main();
}
