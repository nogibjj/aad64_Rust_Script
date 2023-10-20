use plotters::backend::BitMapBackend;
use plotters::prelude::*;
use polars::prelude::*;
use std::error::Error;

pub fn mean(column: &Series) -> Option<f64> {
    Some(column.mean().unwrap())
}

pub fn median(column: &Series) -> Option<f64> {
    Some(column.median().unwrap())
}

pub fn standard_deviation(column: &Series) -> Option<f64> {
    Some(column.std().unwrap())
}

pub fn create_scatter_plot(x_data: &[f64], y_data: &[f64], output_file: &str) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(output_file, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0f64..10f64, 0f64..10f64)?;

    chart.configure_mesh().draw()?;
    chart.draw_series(
        x_data.iter()
            .zip(y_data.iter())
            .map(|(x, y)| Circle::new((*x, *y), 5, &BLUE.mix(0.8).filled())),
    )?;

    Ok(())
}