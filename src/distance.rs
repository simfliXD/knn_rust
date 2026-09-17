use crate::parse_data;

pub fn squared_euclidean_distance(point_a: &parse_data::Point, point_b: &parse_data::Point) -> f64 {
    let mut sum = 0.0;

    // Calculate differance between each vector value and square it.
    for i in 0..point_a.values.len() {
        let diff = point_a.values[i] - point_b.values[i];
        sum += diff * diff;
    }

    sum // No need to square root.
}
