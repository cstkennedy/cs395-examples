use std::io::prelude::*;
use std::fs::File;
use std::io::BufWriter;
use std::env;

use rand::prelude::*;
use rand::rngs::SmallRng;




pub fn random_file(num_lines: u64) -> String {
    const KNOWN_SHAPES: [(&'static str, usize); 5] = [
        ("Triangle", 3),
        ("Right Triangle", 2),
        ("Equilateral Triangle", 1),
        ("Square", 1),
        ("Circle", 1)
    ];

    const INVALID_SHAPE_RATE: f64 = 0.10;
    const INVALID_DIM_RATE: f64 = 0.10;

    let mut num_valid_shapes: u64 = 0;

    let random_shape_filename = format!("random-shapes-{num_lines}.txt");

    let mut random_shape_file = File::create(&random_shape_filename).unwrap();
    let mut random_shape_file = BufWriter::new(random_shape_file);

    let mut rng: SmallRng = rand::make_rng();

    for _ in 0..num_lines {
        // Create an unknown shape"
        if rng.random::<f64>() < INVALID_SHAPE_RATE {
            writeln!(random_shape_file, "LOL Not a Shape; 1 2 3 4 5");
            continue
        }

        let (shape, dim_count) = KNOWN_SHAPES.choose(&mut rng).unwrap();

        write!(random_shape_file, "{}", shape);

        for _ in 0..*dim_count {
            let dim = rng.random_range(0.0..10.0);

            write!(random_shape_file, " {dim}");

        }
        write!(random_shape_file, "\n");

        num_valid_shapes += 1

    }

    random_shape_filename
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let num_lines: u64 = args[1].parse().unwrap();

    random_file(num_lines);
}
