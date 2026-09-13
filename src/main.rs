mod distance;
mod knn;
mod metrics;
mod parse_data;

use std::error::Error;

use crate::knn::KNearestNeighbors;
use crate::metrics::{accuracy, precision};

fn main() -> Result<(), Box<dyn Error>> {
    let knn = KNearestNeighbors::new(
	    9,	// K value
        parse_data::read_dataset("data.csv")?, // Data points
    );

    for i in &knn.data_points {
        println!("{:?}", i);
        // println!("{:?}", knn.distances(&i));
        println!("Prediction: {:?}", knn.predict(i));
    }

    let true_labels = knn
        .data_points
        .iter()
        .map(|i| i.label.clone())
        .collect::<Vec<String>>();

    let predicted_labels = knn
        .data_points
        .iter()
        .map(|i| knn.predict(i))
        .collect::<Vec<String>>();
    
    let accuracy = accuracy(&true_labels, &predicted_labels);
    println!("ACCURACY((tp + tf) / total) IS: {accuracy}");

    let precision = precision(&true_labels, &predicted_labels, "höger");
    println!("PRECISION FOR höger (tp / (tp + fp)) IS: {precision}");

    Ok(())
}
