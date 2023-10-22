use iris_functions::*;
use std::error::Error as StdError;
use std::time::Instant;

fn main() -> Result<(), Box<dyn StdError>> {
    // Measure start time
    let start_time = Instant::now();

    let data: Vec<IrisData> = read_iris_data("iris.csv")?;

    let sepal_lengths: Vec<f64> = data.iter().map(|entry| entry.sepal_length).collect();

    if let Some(mean) = average(&sepal_lengths) {
        println!("Mean Sepal Length: {:.2}", mean);
    }

    if let Some(median) = med(&mut sepal_lengths.clone()) {
        println!("Median Sepal Length: {:.2}", median);
    }

    if let Some(std_dev) = standard_deviation(&sepal_lengths) {
        println!("Standard Deviation Sepal Length: {:.2}", std_dev);
    }

    visualize_data(&data, |entry| entry.sepal_length, |entry| entry.sepal_width);

    // Measure end time
    let end_time = Instant::now();

    // Calculate and print execution time
    let execution_time = end_time.duration_since(start_time);
    println!("Execution time: {:?}", execution_time);

    Ok(())
}
