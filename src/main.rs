use std::vec;

use Rgorithm::rolling_finite_mean;

fn main() {
    let raw_data = vec![f64::NAN, 2.71, f64::NAN, f64::NAN, 9.81, -10.];
    let window = 3;

    let mva_data = rolling_finite_mean(raw_data, window);

    println!("{:?}", mva_data);
}