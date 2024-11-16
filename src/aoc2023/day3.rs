use std::collections::HashSet;
use std::collections::HashMap;
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


    let part2_sample_result = part2(&sample);

    println!("2023-03 Part 2 sample: {}", part2_sample_result);
    assert_eq!(part2_sample_result, 467835);

    let part2_result = part2(&part1_input);

    println!("2023-03 Part 2 result: {}", part2_result);
    assert_eq!(part2_result, 76504829);

    return Ok(());
}

#[derive(PartialEq, Eq, Hash)]
struct SymbolLocation {
    x: i32,
    y: i32,
}

#[derive(PartialEq, Eq, Hash)]
struct PartNumber {
    value: i32,
    char_len: i32,
    x: i32,
    y: i32,
}

fn parse_input(inputstr: &str) -> (HashSet<SymbolLocation>, Vec<PartNumber>) {
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
                    let char_len = in_progress_part_number.len() as i32;
                    let start_x = x - char_len;
                    possible_part_numbers.push(PartNumber { value, char_len, x: start_x, y});
                    in_progress_part_number.clear();
                }
            }

            x += 1;
        }

        if ! in_progress_part_number.is_empty() {
            let value = i32::from_str_radix(&in_progress_part_number, 10).unwrap();
            let char_len = in_progress_part_number.len() as i32;
            let start_x = x - char_len;
            possible_part_numbers.push(PartNumber { value, char_len, x: start_x, y});
            in_progress_part_number.clear();
        }

        y += 1;
    }

    return (symbols, possible_part_numbers);
}

fn part1(inputstr: &str) -> i32 {
    let (symbols, possible_part_numbers) = parse_input(inputstr);

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

fn part2(inputstr: &str) -> i32 {
    let (symbols, possible_part_numbers) = parse_input(inputstr);

    let mut locs_to_parts: HashMap<SymbolLocation, &PartNumber> = HashMap::new();

    for p in &possible_part_numbers {
        let y = p.y;
        for x in p.x .. p.x + p.char_len {
            locs_to_parts.insert(SymbolLocation {x, y}, p);
        }
    }

    let mut running_count = 0;

    for s in symbols {
        let mut parts_around_symbol: HashSet<&PartNumber> = HashSet::new();
        for x in s.x-1..=s.x+1 {
            for y in s.y-1..=s.y+1 {
                if s.x != x || s.y != y {
                    let part_at_loc = locs_to_parts.get(&SymbolLocation {x, y});
                    if part_at_loc.is_some() {
                        parts_around_symbol.insert(*part_at_loc.unwrap());
                    }
                }
            }
        }
        if parts_around_symbol.len() == 2 {
            running_count += parts_around_symbol.iter().fold(1, |a, b| a * b.value);
        }
    }

    return running_count;
}