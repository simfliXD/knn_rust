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

        // Vector list of points with distances from query point,
        // requires that distances are calculated in same order.
        let mut neighbors: Vec<(f64, String)> = self
            .data_points
            .iter()	// For every point
            .zip(distances) 	// Pair each point with it's distance from query point
            .filter(|(stored_point, _)| !std::ptr::eq(*stored_point, query_point)) // Remove point if it's the same as the query point stored in memory
            .map(|(stored_point, distance)| (distance, stored_point.label.clone())) // crate new tuple with (distance, label)
            .collect();

        // Use at most `k` of the available neighbors
        let k = self.k.min(neighbors.len());

        // If k or dataset is empty/0
        if k == 0 {
            return String::new();
        }

        //neighbors.sort_by(|a, b| a.0.total_cmp(&b.0));
        
        // Partition neighbors so the first `k` entries are the nearest by distance
        neighbors.select_nth_unstable_by(k - 1, |a, b| a.0.total_cmp(&b.0));
        
        let mut votes: HashMap<String, usize> = HashMap::new();

        for (_, label) in neighbors.iter().take(k) {
            *votes.entry(label.clone()).or_insert(0) += 1; // make an vote for your label but if it dosen't exist yet , insert it with count of 0
        }

        let mut prediction = String::new();
        let mut most_votes = 0;

        for (label, count) in votes {
            if count > most_votes || (count == most_votes && label < prediction) {
                prediction = label;
                most_votes = count;
            }
        }

        prediction
    }
}
