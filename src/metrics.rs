pub fn accuracy(true_labels: &[String], predicted_labels: &[String]) -> f64 {
    let total = true_labels.len();
    let correct = true_labels
        .iter()
        .zip(predicted_labels)
        .filter(|(t, p)| t == p)
        .count();

    correct as f64 / total as f64
}

pub fn precision(true_labels: &[String], predicted_labels: &[String], positive_label: &str) -> f64 {
    let mut tp = 0;
    let mut fp = 0;

    for (actual, predicted) in true_labels.iter().zip(predicted_labels) {
        if predicted == positive_label {
            if actual == positive_label {
                tp += 1;
            } else {
                fp += 1;
            }
        }
    }

    if tp + fp == 0 {
        0.0
    } else {
        tp as f64 / (tp + fp) as f64
    }
}
