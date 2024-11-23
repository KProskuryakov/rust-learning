use std::error::Error;

use crate::aoc::verify;

pub fn solve() -> Result<(), Box<dyn Error>> {
    
    verify("sample.txt", "2023-05", 35, part1)?;
    verify("part1.txt", "2023-05", 579439039, part1)?;

    verify("sample.txt", "2023-05", 46, part2)?;
    verify("part1.txt", "2023-05", 0, part2)?;

    Ok(())
}

struct Mapping {
    dest_num: i64,
    source: i64,
    range: i64
}

fn line_to_mapping(line: &str) -> Result<Mapping, Box<dyn Error>> {
    let mut line_iter = line.split_ascii_whitespace();

    let dest_num = line_iter.next().ok_or("No dest_num in mapping line.")?.parse::<i64>()?;
    let source = line_iter.next().ok_or("No source in mapping line.")?.parse::<i64>()?;
    let range = line_iter.next().ok_or("No range in mapping line.")?.parse::<i64>()?;

    return Ok(Mapping {dest_num, source, range});
}

fn map_seed(seed: i64, mappings: &Vec<Vec<Mapping>>) -> i64 {
    let mut num = seed;
    'outer: for mapping_list in mappings {
        for mapping in mapping_list {
            if num >= mapping.source && num < mapping.source + mapping.range {
                num = num - mapping.source + mapping.dest_num;
                continue 'outer;
            }
        }
    }
    return num;
}

fn part1(inputstr: &str) -> Result<i64, Box<dyn Error>> {
    let lines: Vec<&str> = inputstr.lines().collect();

    let seeds: Vec<i64> = lines
        .first().ok_or("No supplied lines.")?
        .strip_prefix("seeds: ").ok_or("'seeds: ' section of the first line not found.")?
        .split(" ")
        .map(|s| s.parse::<i64>())
        .collect::<Result<Vec<i64>, _>>()?;

    let mappings = lines.split(|s| s.is_empty())
        .skip(1)
        .map(|block| block.iter()
            .skip(1)
            .map(|line| line_to_mapping(line))
            .collect::<Result<Vec<Mapping>, _>>())
        .collect::<Result<Vec<Vec<Mapping>>, _>>()?;

    let lowest_location = seeds.iter().map(|seed| map_seed(*seed, &mappings)).min().ok_or("Error calculating the lowest location.")?;

    return Ok(lowest_location);
}

fn pairs<T: Copy>(list: Vec<T>) -> Vec<(T, T)> {
    let first_iter = list.iter().enumerate().filter(|&(i, _)| i % 2 == 0);
    let second_iter = list.iter().enumerate().filter(|&(i, _)| i % 2 == 1);

    first_iter.zip(second_iter).map(|((_, v1), (_, v2))| (*v1, *v2)).collect()
}

fn map_seed_range(seed_pair: (i64, i64), mappings: &Vec<Vec<Mapping>>) -> i64 {
    let (seed_start, range) = seed_pair;
    let mut lowest = std::i64::MAX;

    let mut cur = 0;
    for i in seed_start..seed_start+range {
        let mapped = map_seed(i, mappings);
        if mapped < lowest {
            lowest = mapped;
        }
        cur += 1;
        if cur % 1000000 == 1 {
            println!("{cur}");
        }
    }
    
    return lowest;
}

fn part2(inputstr: &str) -> Result<i64, Box<dyn Error>> {
    let lines: Vec<&str> = inputstr.lines().collect();

    let seeds_as_individuals: Vec<i64> = lines
        .first().ok_or("No supplied lines.")?
        .strip_prefix("seeds: ").ok_or("'seeds: ' section of the first line not found.")?
        .split(" ")
        .map(|s| s.parse::<i64>())
        .collect::<Result<Vec<i64>, _>>()?;

    let seeds: Vec<(i64, i64)> = pairs(seeds_as_individuals);

    let mappings = lines.split(|s| s.is_empty())
        .skip(1)
        .map(|block| block.iter()
            .skip(1)
            .map(|line| line_to_mapping(line))
            .collect::<Result<Vec<Mapping>, _>>())
        .collect::<Result<Vec<Vec<Mapping>>, _>>()?;

    let total: i64 = seeds.iter().map(|p| p.1).sum();

    println!("{total}");

    let lowest_location = seeds.iter().map(|seed| map_seed_range(*seed, &mappings)).min().ok_or("Error calculating the lowest location.")?;

    return Ok(lowest_location);
}