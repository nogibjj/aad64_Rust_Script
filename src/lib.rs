// Importing module polars for my function.
use polars::prelude::*;
use polars::frame::group_by::GroupBy;
use polars::frame::DataFrame;
use polars::io::prelude::*;
use polars::lazy::prelude::*;
use polars::series::NamedFrom;

use std::path::Path;

fn main() {
    let file_path = Path::new("iris.csv");
    let df = CsvReader::from_path(file_path).unwrap().finish().unwrap();

    fn average(data: &DataFrame) -> Option<f64> {
        data.mean().get(0).map(|x| *x)
    }

    fn med(data: &DataFrame) -> Option<f64> {
        data.median().get(0).map(|x| *x)
    }

    fn standard_deviation(data: &DataFrame) -> Option<f64> {
        data.std().get(0).map(|x| *x)
    }

    fn summary_stats(data: &DataFrame) -> DataFrame {
        data.describe().transpose().expect("Error in describe")
    }

    println!("Mean of Sepal Lengths in iris.csv: {:?}", average(&df.select("sepal.length")));
    println!("Median of Sepal Lengths in iris.csv: {:?}", med(&df.select("sepal.length")));
    println!("Standard Deviation of Sepal Lengths in iris.csv: {:?}", standard_deviation(&df.select("sepal.length")));
    println!("Overall summary statistics of full dataset iris.csv: {:?}", summary_stats(&df));

    fn visualize_data(data: &DataFrame, title: &str, xlabel: &str, ylabel: &str) {
        let plot = data.violin_plot("sepal.length").unwrap();
        plot.set_title(title);
        plot.set_xlabel(xlabel);
        plot.set_ylabel(ylabel);
        plot.show();
    }

    visualize_data(&df, "Iris Dataset", "Sepal Length", "Frequency");
}