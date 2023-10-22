use csv::ReaderBuilder;
use serde::Deserialize;
use std::error::Error;
use plotters::prelude::*;

#[derive(Debug, Deserialize)]
pub struct IrisData {
    #[serde(rename = "sepal.length")]
    pub sepal_length: f64,
    #[serde(rename = "sepal.width")]
    pub sepal_width: f64,
    #[serde(rename = "petal.length")]
    pub petal_length: f64,
    #[serde(rename = "petal.width")]
    pub petal_width: f64,
    #[serde(rename = "variety")]
    pub class: String,
}

pub fn read_iris_data(file_path: &str) -> Result<Vec<IrisData>, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new().has_headers(true).from_path(file_path)?;
    let mut data = Vec::new();
    for record in reader.deserialize() {
        let record: IrisData = record?;
        data.push(record);
    }
    Ok(data)
}

pub fn average(data: &[f64]) -> Option<f64> {
    if data.is_empty() {
        return None;
    }
    let sum: f64 = data.iter().sum();
    Some(sum / data.len() as f64)
}

pub fn med(data: &mut [f64]) -> Option<f64> {
    if data.is_empty() {
        return None;
    }
    data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = data.len() / 2;
    if data.len() % 2 == 0 {
        Some((data[mid - 1] + data[mid]) / 2.0)
    } else {
        Some(data[mid] as f64)
    }
}

pub fn standard_deviation(data: &[f64]) -> Option<f64> {
    if data.is_empty() {
        return None;
    }
    let mean = average(data)?;
    let variance: f64 = data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (data.len() - 1) as f64;
    Some(variance.sqrt())
}

pub fn visualize_data(data: &[IrisData], x_column: fn(&IrisData) -> f64, y_column: fn(&IrisData) -> f64) {
    let root = BitMapBackend::new("scatter.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .caption("Iris Data Scatter Plot", ("sans-serif", 30))
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0.0..10.0, 0.0..10.0)
        .unwrap();

    chart
        .configure_mesh()
        .x_labels(5)
        .y_labels(5)
        .disable_mesh()
        .x_label_formatter(&|x| format!("{:.2}", x))
        .y_label_formatter(&|y| format!("{:.2}", y))
        .draw()
        .unwrap();

    chart.draw_series(
        data.iter().map(|entry| {
            let x = x_column(entry);
            let y = y_column(entry);
            Circle::new((x, y), 3, BLACK.filled())
        })
    ).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_average() {
        let data = [1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(average(&data), Some(3.0));
    }

    #[test]
    fn test_med() {
        let mut data = [1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(med(&mut data), Some(3.0));
        let mut data = [1.0, 2.0, 3.0, 4.0];
        assert_eq!(med(&mut data), Some(2.5));
    }

    #[test]
    fn test_standard_deviation() {
        let data = [1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(standard_deviation(&data), Some(1.5811388300841898));
    }

}
