use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Point {
    pub values: Vec<f64>,
    pub label: String,
}

pub fn read_dataset(path: &str) -> Result<Vec<Point>, Box<dyn Error>> {
    let contents = fs::read_to_string(path);

    let mut data_points = Vec::new();

    for line in contents?.lines().skip(1) {
        // Split the data content into lines but skip the header
        let fields: Vec<&str> = line.split(',').collect();
        let field_count = fields.len() - 1;
        //println!("{field_count}");

        // Parse the values
        let mut values = Vec::new();
        for value in fields.iter().take(field_count) {
            let value: f64 = value.trim().parse().unwrap();
            values.push(value);
        }

        // Parse the label which is the last field
        let label: String = fields[field_count].trim().to_string();

        // Create new point with parsed values and push it to the points vector
        data_points.push(Point { values, label });
    }

    Ok(data_points)
}
