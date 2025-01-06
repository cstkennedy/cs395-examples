fn main() {
    // Dimensions in feet
    let length = 12_f64;
    let width = 10_f64;
    let height = 8_f64;

    // Price in dollars
    let price_per_gallon = 49.98;

    let area = compute_wall_surface_area(length, width, height);

    let gallons_required = determine_gallons_required(area, None, None);

    let (min_required, max_required) = gallons_required;
    let min_cost = (min_required as f64) * price_per_gallon;
    let max_cost = (max_required as f64) * price_per_gallon;

    println!("You will need to buy {min_required} to {max_required} gallons of paint.");
    println!("You will spend $ {min_cost:.2} to $ {max_cost:.2}");
}

fn compute_wall_surface_area(length: f64, width: f64, height: f64) -> f64 {
    let area_long_wall = length * height;
    let area_wide_wall = width * height;

    let area_four_walls = (2_f64 * area_long_wall) + (2_f64 * area_wide_wall);

    return area_four_walls;
}

fn determine_gallons_required(
    wall_area: f64,
    min_coverage: Option<f64>,
    max_coverage: Option<f64>,
) -> (u32, u32) {
    let min_coverage = match min_coverage {
        Some(coverage) => coverage,
        None => 350_f64,
    };
    let max_coverage = match max_coverage {
        Some(coverage) => coverage,
        None => 400_f64,
    };

    let max_gallons = (wall_area / min_coverage).ceil() as u32;
    let min_gallons = (wall_area / max_coverage).ceil() as u32;

    return (min_gallons, max_gallons);
}
