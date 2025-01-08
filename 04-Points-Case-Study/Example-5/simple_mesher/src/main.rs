use rust_points::prelude::*;
use simple_mesher::*;

fn main() {
    let square: [Point2D<f64>; 4] = [
        (0_f64, 0_f64).into(),
        (0_f64, 1_f64).into(),
        (1_f64, 1_f64).into(),
        (1_f64, 0_f64).into(),
    ];

    let (triangle_1, triangle_2) = square_to_triangles(&square);

    println!("Triangle 1: ");
    println!("  {:?}", triangle_1);
    println!("  centroid: {:?}", triangle_1.centroid());
    println!();

    println!("Triangle 2: ");
    println!("  {:?}", triangle_2);
    println!("  centroid: {:?}", triangle_2.centroid());
}
