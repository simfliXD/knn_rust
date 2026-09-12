use std::collections::HashMap;

use crate::distance::euclidean_distance;
use crate::parse_data::Point;

pub struct KNearestNeighbors {
    pub k: usize,
    pub data_points: Vec<Point>,
}
impl KNearestNeighbors {
    pub fn new(k: usize, data_points: Vec<Point>) -> Self {
        Self { k, data_points }
    }

    pub fn distances(&self, query_point: &Point) -> Vec<f64> {
        let mut distances = Vec::new();

        for stored_point in &self.data_points {
            distances.push(euclidean_distance(stored_point, query_point));
        }

        distances
    }

    pub fn predict(&self, query_point: &Point) -> String {
        let distances = self.distances(query_point);
        let mut neighbors: Vec<(f64, String)> = Vec::new();

        for (i, _distance) in distances.iter().enumerate() {
            neighbors.push((distances[i], self.data_points[i].label.clone()));
        }

        let number_of_neighbors = self.k.min(neighbors.len());

        neighbors.select_nth_unstable_by(
            number_of_neighbors - 1,
            |a: &(f64, String), b: &(f64, String)| a.0.partial_cmp(&b.0).unwrap(),
        );

        let mut votes: HashMap<String, usize> = HashMap::new();

        for (_, label) in neighbors.iter().take(number_of_neighbors) {
            *votes.entry(label.clone()).or_insert(0) += 1;
        }

        let mut prediction = String::new();
        let mut most_votes = 0;

        for (label, count) in votes {
            if count > most_votes {
                prediction = label;
                most_votes = count;
            }
        }

        prediction
    }
}
