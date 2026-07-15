use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let num_lines: u64 = args[1].parse().unwrap();

    shapes_rand_file::random_file(num_lines).unwrap();
}
