use anyhow::Result;
use humantime::format_duration;
use regex::Regex;
use std::{fs::read_to_string, time::Instant};

type ProcessedInput = Vec<Operation>;
type Output = i32;

fn main() -> Result<()> {
    let input = read_to_string("input.txt")?;
    println!("Lines in Input: {}", input.len());

    let processing_start = Instant::now();
    let processed = process_input(input)?;
    let time_processing = processing_start.elapsed();

    let part_one = solve_part_one(processed.clone());
    let part_one_start = Instant::now();
    let time_one = part_one_start.elapsed();

    let part_two_start = Instant::now();
    let part_two = solve_part_two(processed);
    let time_two = part_two_start.elapsed();

    println!(
        "Processing Complete. (Took {:?})",
        format_duration(time_processing).to_string()
    );
    println!(
        "Part One: {:?} (Took {:?})",
        part_one,
        format_duration(time_one).to_string()
    );
    println!(
        "Part Two: {:?} (Took {:?})",
        part_two,
        format_duration(time_two).to_string()
    );
    Ok(())
}

#[derive(Clone)]
enum Operation {
    Mul((i32, i32)),
    Do(),
    Dont(),
}
fn process_input(input: String) -> Result<ProcessedInput> {
    let mut vec = Vec::new();
    input
        .lines()
        .filter(|x| !x.is_empty())
        .for_each(|x| vec.append(&mut process_line(x)));
    Ok(vec)
}

fn process_line(line: &str) -> Vec<Operation> {
    let regex = Regex::new(r"(mul\(([0-9]+),([0-9]+)\))|(do\(()()\))|(don't\(()()\))").unwrap(); // Matches "mul(454, 123)" or similar
    let mut operations: Vec<Operation> = vec![];
    for (_, [op_str, l, r]) in regex.captures_iter(line).map(|c| c.extract()) {
        if op_str.starts_with("mul") {
            operations.push(Operation::Mul((l.parse().unwrap(), r.parse().unwrap())));
        } else {
            match op_str {
                r"do()" => operations.push(Operation::Do()),
                r"don't()" => operations.push(Operation::Dont()),
                _ => continue,
            }
        }
    }
    operations
}

fn solve_part_one(data: ProcessedInput) -> Result<Output> {
    let sum = data
        .into_iter()
        .filter_map(|op| {
            if let Operation::Mul((l, r)) = op {
                Some(l * r)
            } else {
                None
            }
        })
        .sum();
    Ok(sum)
}

fn solve_part_two(data: ProcessedInput) -> Result<Output> {
    let mut active = true;
    let mut count = 0;
    for operation in data {
        match operation {
            Operation::Mul((l, r)) => {
                if active {
                    count += l * r
                }
            }
            Operation::Do() => active = true,
            Operation::Dont() => active = false,
        }
    }
    Ok(count)
}
