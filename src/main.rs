use my_crate::average;
use my_crate::med;
use my_crate::standard_deviation;
use my_crate::summary_stats;
use my_crate::visualize_data;

fn main() {
    let file_path = Path::new("iris.csv");
    let df = CsvReader::from_path(file_path).unwrap().finish().unwrap();

    println!("Mean of Sepal Lengths in iris.csv: {:?}", average(&df.select("sepal.length")));
    println!("Median of Sepal Lengths in iris.csv: {:?}", med(&df.select("sepal.length")));
    println!("Standard Deviation of Sepal Lengths in iris.csv: {:?}", standard_deviation(&df.select("sepal.length")));
    println!("Overall summary statistics of full dataset iris.csv: {:?}", summary_stats(&df));

    visualize_data(&df, "Iris Dataset", "Sepal Length", "Frequency");
}