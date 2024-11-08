use std::collections::HashSet;
use std::fs;
use std::error::Error;

pub fn solve() -> Result<(), Box<dyn Error>> {
    let sample = fs::read_to_string("aoc-inputs/2023-03/sample.txt")?;

    let part1_sample_result = part1(&sample);

    println!("2023-03 Part 1 sample: {}", part1_sample_result);
    assert_eq!(part1_sample_result, 4361);

    let part1_input = fs::read_to_string("aoc-inputs/2023-03/part1.txt")?;

    let part1_result = part1(&part1_input);

    println!("2023-03 Part 1 result: {}", part1_result);
    assert_eq!(part1_result, 525119);

    return Ok(());
}

#[derive(PartialEq, Eq, Hash)]
struct SymbolLocation {
    x: i32,
    y: i32,
}

struct PartNumber {
    value: i32,
    char_len: i32,
    x: i32,
    y: i32,
}

fn part1(inputstr: &str) -> i32 {
    let mut symbols: HashSet<SymbolLocation> = HashSet::new();

    let mut possible_part_numbers: Vec<PartNumber> = Vec::new();

    let mut y = 0;

    let mut in_progress_part_number = String::with_capacity(3);

    for line in inputstr.lines() {
        let mut x = 0;
        for c in line.chars() {
            if c.is_ascii_digit() {
                in_progress_part_number.push(c);
            } else {
                if c != '.' {
                    symbols.insert(SymbolLocation { x, y });
                }
                
                if ! in_progress_part_number.is_empty() {
                    let value = i32::from_str_radix(&in_progress_part_number, 10).unwrap();
                    let char_len = in_progress_part_number.len().try_into().unwrap();
                    let start_x = x - char_len;
                    possible_part_numbers.push(PartNumber { value, char_len, x: start_x, y});
                    in_progress_part_number.clear();
                }
            }

            x += 1;
        }

        if ! in_progress_part_number.is_empty() {
            let value = i32::from_str_radix(&in_progress_part_number, 10).unwrap();
            let char_len = in_progress_part_number.len().try_into().unwrap();
            let start_x = x - char_len;
            possible_part_numbers.push(PartNumber { value, char_len, x: start_x, y});
            in_progress_part_number.clear();
        }

        y += 1;
    }

    let mut part_numbers: Vec<PartNumber> = Vec::new();

    for part_num in possible_part_numbers {
        let mut symbol_adj = false;

        let mut locations_to_check: Vec<SymbolLocation> = Vec::new();

        for i in (part_num.x - 1) ..= (part_num.x + part_num.char_len) {
            locations_to_check.push(SymbolLocation { x: i, y: part_num.y - 1});
            locations_to_check.push(SymbolLocation { x: i, y: part_num.y + 1});
        }
        locations_to_check.push(SymbolLocation { x: part_num.x - 1, y: part_num.y });
        locations_to_check.push(SymbolLocation { x: part_num.x + part_num.char_len, y: part_num.y });

        for loc in locations_to_check {
            if symbols.contains(&loc) {
                symbol_adj = true;
            }
        }

        if symbol_adj {
            part_numbers.push(part_num);
        }
    }

    return part_numbers.iter().fold(0, |a, b| a + b.value);
}