use std::fs;
use std::error::Error;

pub fn solve() -> Result<(), Box<dyn Error>> {
    let sample = fs::read_to_string("aoc-inputs/2023-02/sample.txt")?;

    let part1_input = fs::read_to_string("aoc-inputs/2023-02/input.txt")?;

    let part1_sample_result = part1(&sample);

    println!("2023-02 Part 1 sample: {}", part1_sample_result);
    assert_eq!(part1_sample_result, 8);

    let part1_result = part1(&part1_input);

    println!("2023-02 Part 1 result: {}", part1_result);
    assert_eq!(part1_result, 2164);

    let part2_sample_result = part2(&sample);

    println!("2023-02 Part 2 sample: {}", part2_sample_result);
    assert_eq!(part2_sample_result, 2286);

    let part2_result = part2(&part1_input);

    println!("2023-02 Part 2 result: {}", part2_result);
    assert_eq!(part2_result, 69929);

    Ok(())
}


fn part1(text: &str) -> i32 {
    let reds = 12;
    let blues = 14;
    let greens = 13;

    let mut counter = 0;

    'line: for line in text.lines() {
        let split_line: Vec<&str> = line.split(": ").collect();
        let game: i32 = split_line[0].split(" ").collect::<Vec<&str>>()[1].parse().expect("Game number not found.");

        for handful in split_line[1].split("; ") {
            let mut picked_reds = 0;
            let mut picked_blues = 0;
            let mut picked_greens = 0;

            for picked_type in handful.split(", ") {
                let split_picked_type: Vec<&str> = picked_type.split(" ").collect();
                let color = split_picked_type[1];
                let count: i32 = split_picked_type[0].parse().expect("Cannot parse the number of squares picked.");

                if color == "red" {
                    picked_reds += count;
                } else if color == "blue" {
                    picked_blues += count;
                } else if color == "green" {
                    picked_greens += count;
                } else {
                    panic!("Invalid color of picked square.")
                }
            }

            if picked_reds > reds || picked_blues > blues || picked_greens > greens {
                continue 'line;
            }
        }

        counter += game;
    }

    return counter;
}

fn part2(text: &str) -> u64 {
    let mut counter: u64 = 0;

    for line in text.lines() {
        let split_line: Vec<&str> = line.split(": ").collect();

        let mut picked_reds: u64 = 1;
        let mut picked_blues: u64 = 1;
        let mut picked_greens: u64 = 1;

        for handful in split_line[1].split("; ") {
            for picked_type in handful.split(", ") {
                let split_picked_type: Vec<&str> = picked_type.split(" ").collect();
                let color = split_picked_type[1];
                let count: u64 = split_picked_type[0].parse().expect("Cannot parse the number of squares picked.");

                if color == "red" {
                    picked_reds = count.max(picked_reds);
                } else if color == "blue" {
                    picked_blues = count.max(picked_blues);
                } else if color == "green" {
                    picked_greens = count.max(picked_greens);
                } else {
                    panic!("Invalid color of picked square.")
                }
            }
        }
        
        counter += picked_reds * picked_blues * picked_greens;
    }

    return counter;
}