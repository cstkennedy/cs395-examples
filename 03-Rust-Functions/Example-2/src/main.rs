use estimate_paint::compute_paint;

fn main() {
    // Dimensions in feet
    let length = 12_f64;
    let width = 10_f64;
    let height = 8_f64;

    // Price in dollars
    let price_per_gallon = 49.98;

    let area = compute_paint::wall_surface_area(length, width, height);

    let (min_required, max_required) = compute_paint::gallons_required(area, None, None);
    let min_cost = (min_required as f64) * price_per_gallon;
    let max_cost = (max_required as f64) * price_per_gallon;

    println!("You will need to buy {min_required} to {max_required} gallons of paint.");
    println!("You will spend $ {min_cost:.2} to $ {max_cost:.2}.");
}


