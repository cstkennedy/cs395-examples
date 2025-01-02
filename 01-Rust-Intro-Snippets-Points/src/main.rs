use ordered_float::OrderedFloat;

fn procedural_example_1() {
    let point1: (f64, f64) = (0.0, 5.0);
    let point2: (f64, f64) = (8.0, 3.0);
    let point3: (f64, f64) = (1.0, 7.0);

    let points = vec![point1, point2, point3];

    for point in points.iter() {
        let distance = ((point.0).powf(2.0) + (point.1).powf(2.0)).sqrt();
        println!(
            "From (0, 0) to {:?} is {:4.2} (distance units)",
            point, distance
        );
    }
}

fn procedural_example_2() {
    let points: Vec<(f64, f64)> = vec![(0.0, 5.0), (8.0, 3.0), (1.0, 7.0)];

    for point in points.iter() {
        let distance = ((point.0).powf(2.0) + (point.1).powf(2.0)).sqrt();
        println!(
            "From (0, 0) to {:?} is {:4.2} (distance units)",
            point, distance
        );
    }
}

fn functional_example_1() {
    let points: Vec<(f64, f64)> = vec![(0.0, 5.0), (8.0, 3.0), (1.0, 7.0)];

    let distance_f = |point: &(f64, f64)| -> f64 {
        ((point.0).powf(2.0) + (point.1).powf(2.0)).sqrt()
    };

    let distances = points
        .iter()
        .map(|&pt| distance_f(&pt))
        .collect::<Vec<f64>>();

    let shortest_distance: f64 = *distances.iter().min_by_key(|c| OrderedFloat(**c)).unwrap();
    let largest_distance: f64 = *distances.iter().max_by_key(|c| OrderedFloat(**c)).unwrap();
    let average_distance: f64 = distances.iter().sum::<f64>() / (distances.len() as f64);

    println!("Min Distance: {:4.2}", shortest_distance);
    println!("Max Distance: {:4.2}", largest_distance);
    println!("Avg Distance: {:4.2}", average_distance);
}

fn main() {
    procedural_example_1();

    println!();
    procedural_example_2();

    println!();
    functional_example_1();

}
