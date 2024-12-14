use std::{cmp, error::Error, fs::read_to_string, time::Instant};

type ProcessedInput = Vec<Vec<i32>>;
type Output = i32;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;
    println!("Lines in Input: {}", input.len());
    let time_start = Instant::now();

    let processed = process_input(input)?;
    let time_processing = time_start.elapsed();

    let part_one = solve_part_one(processed.clone());
    let time_one = time_start.elapsed();

    let part_two = solve_part_two(processed);
    let time_two = time_start.elapsed();

    println!(
        "Processing Complete. (Took {:?}ms)",
        time_processing.as_millis()
    );
    println!(
        "Part One: {:?} (Took {:?}ms)",
        part_one,
        time_one.as_millis()
    );
    println!(
        "Part Two: {:?} (Took {:?}ms)",
        part_two,
        time_two.as_millis()
    );
    Ok(())
}

fn solve_part_two(processed: ProcessedInput) -> Result<Output, Box<dyn Error>> {
    let mut count = 0;
    for report in processed {
        if is_safe_report_dampened(report)? {
            count += 1;
        }
    }
    Ok(count)
}

fn is_safe_report_dampened(report: Vec<i32>) -> Result<bool, Box<dyn Error>> {
    for delete_index in 0..report.len() {
        let mut clone = report.clone();
        clone.remove(delete_index);
        if is_safe_report(clone)? {
            return Ok(true);
        }
    }
    Ok(false)
}

fn process_input(input: String) -> Result<ProcessedInput, Box<dyn Error>> {
    let lines = input.lines();
    let mut processed: ProcessedInput = Vec::new();
    for line in lines {
        let mut levels = Vec::new();
        let numbers = line.split_whitespace();
        for number in numbers {
            levels.push(number.parse()?)
        }
        processed.push(levels);
    }
    Ok(processed)
}

fn solve_part_one(input: ProcessedInput) -> Result<Output, Box<dyn Error>> {
    let mut count = 0;
    for report in input {
        if is_safe_report(report)? {
            count += 1;
        }
    }
    Ok(count)
}

fn is_safe_report(report: Vec<i32>) -> Result<bool, Box<dyn Error>> {
    let direction = get_direction(report.clone()).ok_or("Report must have at least one element")?;
    let mut iter = report.into_iter();
    let mut previous = iter
        .next()
        .ok_or("Report must contain at least one number")?;
    for item in iter {
        let near_direction = compare_directions(previous, item);
        if direction != near_direction {
            return Ok(false);
        }
        if !valid_neighbours(item, previous) {
            return Ok(false);
        }
        previous = item;
    }
    Ok(true)
}

fn valid_neighbours(item: i32, previous: i32) -> bool {
    let delta = item.abs_diff(previous);
    if delta < 1 {
        return false;
    }
    if delta > 3 {
        return false;
    }
    true
}
#[derive(cmp::PartialEq)]
enum Direction {
    Increasing,
    Decreasing,
}
fn get_direction(report: Vec<i32>) -> Option<Direction> {
    Some(compare_directions(*report.first()?, *report.last()?))
}

fn compare_directions(left: i32, right: i32) -> Direction {
    let direction = left - right;
    match direction > 0 {
        true => Direction::Decreasing,
        false => Direction::Increasing,
    }
}
