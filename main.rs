use anyhow::Result;
use humantime::format_duration;
use std::{fs::read_to_string, time::Instant};

type ProcessedInput = Vec<i32>;
type Output = i32;

fn main() -> Result<()> {
    let input = read_to_string("input.txt")?;
    println!("Lines in Input: {}", input.lines().count());

    let processing_start = Instant::now();
    let processed = process_input(input)?;
    let time_processing = processing_start.elapsed();
    println!(
        "Processing Complete. (Took {:?})",
        format_duration(time_processing).to_string()
    );

    let part_one_start = Instant::now();
    let part_one = solve_part_one(processed.clone());
    let time_one = part_one_start.elapsed();
    println!(
        "Part One: {:?} (Took {:?})",
        part_one,
        format_duration(time_one).to_string()
    );

    let part_two_start = Instant::now();
    let part_two = solve_part_two(processed);
    let time_two = part_two_start.elapsed();
    println!(
        "Part Two: {:?} (Took {:?})",
        part_two,
        format_duration(time_two).to_string()
    );
    Ok(())
}

fn process_input(input: String) -> Result<ProcessedInput> {
    input
        .lines()
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i32>().with_context(|| "Failed to parse {x}"))
        .collect()
}

fn solve_part_one(data: ProcessedInput) -> Result<Output> {
    bail!("Part One Unimplemented")
}

fn solve_part_two(data: ProcessedInput) -> Result<Output> {
    bail!("Part Two Unimplemented")
}
