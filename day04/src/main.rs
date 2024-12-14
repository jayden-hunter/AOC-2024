use anyhow::anyhow;
use anyhow::bail;
use anyhow::Result;
use humantime::format_duration;
use std::char;
use std::str::Chars;
use std::{fs::read_to_string, time::Instant};

type ProcessedInput = Vec<Vec<char>>;
type Output = u32;

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

fn process_input(input: String) -> Result<ProcessedInput> {
    Ok(input
        .lines()
        .filter(|x| !x.is_empty())
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect())
}

fn solve_part_one(data: ProcessedInput) -> Result<Output> {
    let mut count = 0;
    let width = data.first().unwrap().len();
    for i in 0..data.len() {
        for j in 0..width {
            count += matches_in_wordsearch(&data, i, j)?
        }
    }
    Ok(count)
}

fn matches_in_wordsearch(data: &ProcessedInput, x: usize, y: usize) -> Result<u32> {
    let string_to_find = "XMAS".chars();
    let directions = [
        (0, 1),
        (1, 1),
        (0, -1),
        (-1, 1),
        (1, -1),
        (-1, -1),
        (1, 0),
        (-1, 0),
    ];
    let mut count = 0;
    for (dx, dy) in directions {
        if let Ok(true) = match_found_recursive(string_to_find.clone(), data, x, y, dx, dy) {
            count += 1;
        }
    }
    Ok(count)
}

fn match_found_recursive(
    mut string_to_find: Chars,
    data: &ProcessedInput,
    x: usize,
    y: usize,
    dx: i32,
    dy: i32,
) -> Result<bool> {
    let desired_char = string_to_find.next();
    let desired_char = match desired_char {
        Some(t) => t,
        None => return Ok(true),
    };
    let char = data
        .get(x)
        .ok_or(anyhow!("Exceeds height"))?
        .get(y)
        .ok_or(anyhow!("Exceeds width"))?;
    let next_x = x as i32 + dx;
    let next_y = y as i32 + dy;

    if desired_char == *char {
        return match_found_recursive(
            string_to_find,
            data,
            next_x as usize,
            next_y as usize,
            dx,
            dy,
        );
    }
    Ok(false)
}

fn solve_part_two(data: ProcessedInput) -> Result<Output> {
    let mut count = 0;
    let width = data.first().unwrap().len();
    for i in 0..data.len() {
        for j in 0..width {
            if let Ok(true) = mas_match(&data, i, j) {
                count += 1
            }
        }
    }
    Ok(count)
}

fn mas_match(data: &ProcessedInput, x: usize, y: usize) -> Result<bool> {
    let diag_one = check_diagonal(data, x, y, (1, 1))?;
    let diag_two = check_diagonal(data, x, y, (-1, 1))?;
    Ok(diag_one && diag_two)
}

fn check_diagonal(data: &ProcessedInput, x: usize, y: usize, dir: (i32, i32)) -> Result<bool> {
    let opp = (-dir.0, -dir.1);
    let rhs_center = "AS".chars();
    let lhs_center = "AM".chars();
    let normal_rhs = match_found_recursive(rhs_center.clone(), data, x, y, dir.0, dir.1)?;
    let oppposite_rhs = match_found_recursive(rhs_center, data, x, y, opp.0, opp.1)?;
    let normal_lhs = match_found_recursive(lhs_center.clone(), data, x, y, dir.0, dir.1)?;
    let oppposite_lhs = match_found_recursive(lhs_center, data, x, y, opp.0, opp.1)?;
    let diag_one = normal_lhs && oppposite_rhs;
    let diag_two = normal_rhs && oppposite_lhs;
    Ok(diag_one || diag_two)
}
