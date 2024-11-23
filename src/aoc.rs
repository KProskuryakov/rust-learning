use std::error::Error;
use std::fs;

pub fn verify<F>(file_name: &str, aoc_date: &str, expected_value: i64, solve_func: F) -> Result<(), Box<dyn Error>> where F: Fn(&str) -> Result<i64, Box<dyn Error>> {
    let input_file_path = format!("aoc-inputs/{aoc_date}/{file_name}");
    let input_contents = fs::read_to_string(input_file_path)?;

    let calculated_value = solve_func(&input_contents)?;

    if calculated_value != expected_value {
        return Err(Box::<dyn Error>::from(format!("{aoc_date} Calculated result {calculated_value} should be {expected_value} but is not.")));
    }

    return Ok(());
}