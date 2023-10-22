# aad64_Rust_Script
Rewrite a Python script in Rust

# Objective:
The objective of this project is to rewrite an existing Python script for data processing in Rust, highlighting improvements in speed and resource usage. For the same, I re-wrote my [Week 2 Python Pandas Mini Project](https://github.com/nogibjj/aad64_Pandas-Script) in Rust. This was a simple project with the following functions:
* Calculate Mean
* Calculate Median
* Calculate Standard Deviation
* Visualize a Scatter Plot 

## Project Structure:
```
aad64_Rust_Script/
├── .github/
│   └── workflows/actions.yml
├── .gitignore
├── src/
│   ├── lib.rs
│   └── main.rs
├── Cargo.toml
├── Makefile
├── README.md
└── scatter.png
```

## [Python Script to Compare](https://github.com/nogibjj/aad64_Pandas-Script)

## Build Time Difference:
As seen in the screenshots below, the build time for these two projects is drastically different. As research has also suggested, Rust is approximately twice as fast as Python, which was proven with the current project. 

### Python Build Time:
<p align = "center"><img width="556" alt="Screenshot 2023-10-21 at 8 25 55 PM" src="https://github.com/nogibjj/Rust_Script/assets/143753050/98d3f849-0641-41fb-a933-6a4a3bb0165c"></p>

### Rust Build Time:
<p align = "center"><img width="961" alt="Screenshot 2023-10-21 at 8 20 16 PM" src="https://github.com/nogibjj/Rust_Script/assets/143753050/866c7ec6-6db9-498b-ad77-b9bba36c3b2e"></p>

## Breakdown of the rest of the project:


### 1. [actions.yml](https://github.com/nogibjj/aad64_Rust_Script/actions/workflows/actions.yml)
This is a GitHub Actions workflow file named `Clippy`:
- **Name**: Clippy
- **Triggers**: It is triggered on both push events to the `main` branch and pull requests.
- **Jobs**:
  - **build**:
    - **Environment**: It runs on an Ubuntu environment.
    - **Steps**:
      - `actions/checkout@v1`: Checks out the repository.
      - `actions-rs/toolchain@v1`: Sets up the Rust toolchain with Clippy and Rustfmt.
      - **Format**: Executes the `make format` command.
      - **Lint**: Executes the `make lint` command.
      - **Test**: Executes the `make test` command.
This workflow automates the process of formatting, linting, and testing a Rust project using GitHub Actions whenever there is a push to the `main` branch or a pull request is opened.
As seen in the badge above, the project is passing the entire CI/CD build without any errors. Below is also a screenshot of the project passing the test function written in `lib.rs`.
<p align = 'center'><img width="837" alt="Screenshot 2023-10-11 at 4 26 29 PM" src="https://github.com/nogibjj/aad64_command_line/assets/143753050/5317cfaf-9df7-4e1d-884c-43d9f0a55e21"></p>

### 2. src Folder:
1. __[lib.rs](https://github.com/nogibjj/aad64_Rust_Script/src/lib.rs)__: Has four functions as previously mentioned. These are to:
   + Calculate the mean of a column in a dataset
   + Calculate the median of a given column
   + Calculate the standard deviation of a given column
   + Plot a scatterplot of a given column against another column of a dataset
  This file also has unit tests to test the functionality of the functions. 
3. __[main.rs](https://github.com/nogibjj/aad64_Rust_Script/src/main.rs)__: This file calls the functions written in lib.rs. It then outputs the results with reference to a dataset in this project, namely, [iris.csv](https://github.com/nogibjj/aad64_Rust_Script/iris.csv). Furthermore, this file also calculates the execution time for this project. 

### 3. [Cargo.toml](https://github.com/nogibjj/aad64_Rust_Script/Cargo.toml)
This file is the `Cargo.toml` manifest for a Rust project named `iris_analysis` with dependencies on `csv`, `serde`, `serde-derive`, and `plotters` libraries. It specifies the project version, edition, and library configuration.

### 4. [Makefile](https://github.com/nogibjj/aad64_Rust_Script/Makefile)
This is a `Makefile` used for automating various tasks in a Rust project:
- `format`: Invokes `cargo fmt` to automatically format the code according to Rust style guidelines.
- `lint`: Executes `cargo clippy` to perform linting and static analysis to catch potential issues or non-idiomatic code.
- `test`: Runs the project's test suite using `cargo test`.
- `run`: Launches the project with `cargo run`.
- `all`: Combines the tasks `format`, `lint`, `test`, and `run` to perform all common project tasks in sequence.



