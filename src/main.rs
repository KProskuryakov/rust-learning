use std::error::Error;

mod aoc;
mod aoc2023;

fn main() -> Result<(), Box<dyn Error>> {

    crate::aoc2023::day1::solve()?;
    crate::aoc2023::day2::solve()?;
    crate::aoc2023::day3::solve()?;

    Ok(())
}