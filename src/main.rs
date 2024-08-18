use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let sampleinput = "1abc2\n\
                             pqr3stu8vwx\n\
                             a1b2c3d4e5f\n\
                             treb7uchet";

    let adds = sampleinput.lines().fold(0, |acc, x| acc + parse_line(x));
    
    println!("{adds}");
    assert_eq!(adds, 142);

    let part1 = fs::read_to_string("aoc-inputs/2023-01/part1.txt")?;

    let part1_result = part1.lines().fold(0, |acc, x| acc + parse_line(x));

    println!("{part1_result}");
    assert_eq!(part1_result, 53651);

    let sampleinput2 ="two1nine\n\
                             eightwothree\n\
                             abcone2threexyz\n\
                             xtwone3four\n\
                             4nineeightseven2\n\
                             zoneight234\n\
                             7pqrstsixteen\n\
                             139\n\
                             fiveone2";

    let part2_sample = sampleinput2.lines().fold(0, |acc, x| acc + parse_line2(x));

    println!("{part2_sample}");
    assert_eq!(part2_sample, 352);

    let part2_result = part1.lines().fold(0, |acc, x| acc + parse_line2(x));

    println!("{part2_result}");
    assert_eq!(part2_result, 53894);

    Ok(())
}


fn parse_line(line: &str) -> i32 {
    let first_index = line.find(|c: char| c.is_digit(10));
    let last_index = line.rfind(|c: char| c.is_digit(10));

    let zipped = first_index.zip(last_index);

    let digits = match zipped {
        Some(pair) => line.get(pair.0..pair.0+1).unwrap_or("0").to_owned() + line.get(pair.1..pair.1+1).unwrap_or("0"),
        None => "0".to_owned()
    };

    let result = i32::from_str_radix(&digits, 10).unwrap_or(0);

    return result;
}


fn words_to_num(word: &str) -> Option<&str> {
    return match word {
        "one" => Some("1"),
        "two" => Some("2"),
        "three" => Some("3"),
        "four" => Some("4"),
        "five" => Some("5"),
        "six" => Some("6"),
        "seven" => Some("7"),
        "eight" => Some("8"),
        "nine" => Some("9"),
        _ => None
    }
}

const NUM_WORDS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn parse_line2(line: &str) -> i32 {
    let first_digit_index = line.find(|c: char| c.is_digit(10)).unwrap_or(line.len());
    let first_word_index = NUM_WORDS.iter().fold((line.len(), None), |a, x| find_word(a, x, line));

    let last_digit_index = line.rfind(|c: char| c.is_digit(10)).unwrap_or(0);
    let last_word_index = NUM_WORDS.iter().fold((0, None), |a, x| rfind_word(a, x, line));

    let first_digit: &str;
    let last_digit: &str;

    if first_digit_index <= first_word_index.0 {
        first_digit = line.get(first_digit_index..first_digit_index+1).unwrap_or("0");
    } else {
        first_digit = first_word_index.1.unwrap_or("0");
    }

    if last_digit_index >= last_word_index.0 {
        last_digit = line.get(last_digit_index..last_digit_index+1).unwrap_or("0");
    } else {
        last_digit = last_word_index.1.unwrap_or("0");
    }

    let result = i32::from_str_radix(&(first_digit.to_owned() + last_digit), 10);

    return result.unwrap_or(0);
}

fn find_word<'a>(earliest_pair: (usize, Option<&'a str>), num_word: &'a str, line: &'a str) -> (usize, Option<&'a str>) {
    match line.find(num_word) {
        Some(i) => if i < earliest_pair.0 {(i, words_to_num(num_word))} else { earliest_pair },
        None => earliest_pair
    }
}

fn rfind_word<'a>(last_pair: (usize, Option<&'a str>), num_word: &'a str, line: &'a str) -> (usize, Option<&'a str>) {
    match line.rfind(num_word) {
        Some(i) => if i > last_pair.0 {(i, words_to_num(num_word))} else { last_pair },
        None => last_pair
    }
}
