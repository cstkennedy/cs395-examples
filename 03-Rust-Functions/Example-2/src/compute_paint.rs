pub fn wall_surface_area(length: f64, width: f64, height: f64) -> f64 {
    let area_long_wall = length * height;
    let area_wide_wall = width * height;

    let area_four_walls = (2_f64 * area_long_wall) + (2_f64 * area_wide_wall);

    return area_four_walls;
}

pub fn gallons_required(
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
