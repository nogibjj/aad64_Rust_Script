use iris_functions::*;
use std::time::Instant;
use std::mem;
use libc::{self, RUSAGE_SELF, timeval, rusage};

fn get_resource_usage() -> rusage {
    let mut usage: rusage = unsafe { mem::zeroed() };
    let _ = unsafe { libc::getrusage(RUSAGE_SELF, &mut usage) };
    usage
}

fn timeval_to_seconds(tv: timeval) -> f64 {
    tv.tv_sec as f64 + (tv.tv_usec as f64 / 1_000_000.0)
}

fn main() {
    let start_time = Instant::now();

    let usage_start = get_resource_usage();

    let data: Vec<IrisData> = read_iris_data("iris.csv").unwrap();

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

    let end_time = Instant::now();
    let execution_time = end_time.duration_since(start_time);

    let usage_end = get_resource_usage();

    let user_cpu_time = timeval_to_seconds(usage_end.ru_utime) - timeval_to_seconds(usage_start.ru_utime);
    let system_cpu_time = timeval_to_seconds(usage_end.ru_stime) - timeval_to_seconds(usage_start.ru_stime);

    println!("User CPU time: {:.6} seconds", user_cpu_time);
    println!("System CPU time: {:.6} seconds", system_cpu_time);

    println!("Execution time: {:?}", execution_time);
}
