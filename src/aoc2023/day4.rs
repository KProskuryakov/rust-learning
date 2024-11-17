use std::collections::HashSet;
use std::fs;
use std::error::Error;

pub fn solve() -> Result<(), Box<dyn Error>> {
  let date = "2023-04";
  let part1_sample_expected = 13;
  let part1_solution_expected = 23673;
  let part2_sample_expected = 30;
  let part2_solution_expected = 0;

  let part1_sample_file = format!("aoc-inputs/{date}/sample.txt");
  let part1_solution_file = format!("aoc-inputs/{date}/part1.txt");

  let part2_sample_file = &part1_sample_file;
  let part2_solution_file = &part1_solution_file;

  let sample_part1 = fs::read_to_string(&part1_sample_file)?;
  let part1_sample_result = part1(&sample_part1);

  println!("{date} Part 1 sample: {part1_sample_result}");
  assert_eq!(part1_sample_result, part1_sample_expected);

  let part1_input = fs::read_to_string(&part1_solution_file)?;
  let part1_result = part1(&part1_input);

  println!("{date} Part 1 result: {part1_result}");
  assert_eq!(part1_result, part1_solution_expected);

  let sample_part2 = fs::read_to_string(&part2_sample_file)?;
  let part2_sample_result = part2(&sample_part2);

  println!("{date} Part 2 sample: {part2_sample_result}");
  assert_eq!(part2_sample_result, part2_sample_expected);

  let part2_input = fs::read_to_string(&part2_solution_file)?;
  let part2_result = part2(&part2_input);

  println!("{date} Part 2 result: {}", part2_result);
  assert_eq!(part2_result, part2_solution_expected);

  return Ok(());
}

fn part1(inputstr: &str) -> i32 {
  let mut result = 0;

  for line in inputstr.lines() {
    result += process_line(line).unwrap_or(0);
  }
  
  return result;
}

fn process_line(line: &str) -> Option<i32> {
  let split_line = line.split(":").collect::<Vec<&str>>();
  let important_part = split_line.get(1)?;
  let split_str: Vec<&str> = important_part.split("|").collect();

  let winning_string = split_str.get(0)?;
  let my_string = split_str.get(1)?;

  let mut winning_nums = HashSet::new();

  for winning_num_str in winning_string.split_whitespace() {
    let winning_num = winning_num_str.parse::<i32>().ok()?;
    winning_nums.insert(winning_num);
  }

  let mut points = 0;

  for my_num_str in my_string.split_whitespace() {
    let my_num = my_num_str.parse::<i32>().ok()?;
    if winning_nums.contains(&my_num) {
      if points == 0 {
        points = 1;
      } else {
        points *= 2;
      }
    }
  }

  return Some(points);
}

fn part2(inputstr: &str) -> i32 {
  return 0;
}