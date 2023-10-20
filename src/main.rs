use iris_analysis::{create_scatter_plot, mean, median, standard_deviation};
use polars::prelude::*;
use std::error::Error;
use std::path::Path;
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "iris.csv";
    let df = CsvReader::from_path(file_path)
        .unwrap()
        .infer_schema(None)
        .has_header(true)
        .finish()
        .unwrap();

    let column_name = "sepal.length";

    match df.column(column_name) {
        Ok(column) => {
            match mean(&column.f64().unwrap()) {
                Some(mean) => println!("Mean of {}: {:.2}", column_name, mean),
                None => eprintln!("No data to calculate mean"),
            }

            match median(&column.f64().unwrap()) {
                Some(median) => println!("Median of {}: {:.2}", column_name, median),
                None => eprintln!("No data to calculate median"),
            }

            match standard_deviation(&column.f64().unwrap()) {
                Some(std_dev) => println!("Standard Deviation of {}: {:.2}", column_name, std_dev),
                None => eprintln!("No data to calculate standard deviation"),
            }
        },
        Err(e) => eprintln!("Error accessing column {}: {:?}", column_name, e),
    }

    let x_data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y_data = vec![2.0, 3.0, 3.5, 4.0, 4.5];

    create_scatter_plot(&x_data, &y_data, "scatter_plot.png")?;

    Ok(())
}