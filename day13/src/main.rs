use anyhow::{anyhow, Result};
use humantime::format_duration;
use nom::{character::is_digit, complete::tag, sequence::delimited};
use serde::Deserialize;
use std::{fs::read_to_string, time::Instant};
mod map;

type ProcessedInput = Vec<Game>;
type Output = i64;

#[derive(Clone, Debug, Deserialize)]
struct Game {
    #[serde(rename = "Button A")]
    button_a: Coordinates,
    #[serde(rename = "Button B")]
    button_b: Coordinates,
    #[serde(rename = "Prize")]
    prize: Coordinates,
}

impl Game {
    fn new(button_a: (i64, i64), button_b: (i64, i64), prize: (i64, i64)) -> Self {
        Self {
            button_a,
            button_b,
            prize,
        }
    }

    fn solve_part_one(&self, token_costs: (i64, i64)) -> Option<Output> {
        let (pushes_a, pushes_b) = solve_2d_cramers(self.button_a, self.button_b, self.prize)?;
        let cost_a = token_costs.0 * pushes_a;
        let cost_b = token_costs.1 * pushes_b;
        let total = cost_a + cost_b;
        Some(total)
    }

    fn solve_part_two(&self, token_costs: (i64, i64)) -> Option<i64> {
        // Cramers rule to solve
        const MEASUREMENT: i64 = 10000000000000;
        let mut updated_costs = self.clone();
        updated_costs.prize.0 += MEASUREMENT;
        updated_costs.prize.1 += MEASUREMENT;
        updated_costs.solve_part_one(token_costs)
    }
}

fn solve_2d_cramers(a: (i64, i64), b: (i64, i64), c: (i64, i64)) -> Option<(i64, i64)> {
    // println!("Cramers: a={:?}, b={:?}, c={:?}", a, b, c);
    let numerator_x = (c.0 * b.1) - (b.0 * c.1);
    let denominator_x = (a.0 * b.1) - (b.0 * a.1);
    let numerator_y = (a.0 * c.1) - (c.0 * a.1);
    let denominator_y = (a.0 * b.1) - (b.0 * a.1);
    // println!("X: {numerator_x}/{denominator_y} \t Y:{numerator_y}/{denominator_y}");
    if numerator_x % denominator_x != 0 {
        return None;
    }
    if numerator_y % denominator_y != 0 {
        return None;
    }
    let x = numerator_x / denominator_x;
    let y = numerator_y / denominator_y;
    // println!("X: {x}, \t Y: {y}");
    Some((x, y))
}

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

    // println!("{:#?}", processed);
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
    let mut games = vec![];
    let mut game_iter = input.lines();
    loop {
        let game = parse_game(&mut game_iter)?;
        match game {
            Some(t) => games.push(t),
            None => return Ok(games),
        }
        let _ = game_iter.next();
    }
}

fn parse_game(game_iter: &mut std::str::Lines<'_>) -> Result<Option<Game>> {
    let mut input = String::new();
    for _ in 0..3 {
        input.push_str(game_iter.next().ok_or(anyhow!("Game input not finished"))?);
    }
    let formatted_input = input
        .replace(": X+", ": {\"X\": ")
        .replace(", Y+", ", \"Y\": ")
        .replace(": X=", ": {\"X\": ")
        .replace(", Y=", ", \"Y\": ")
        .replace("\n", "}, \n")
        + "}";

    match serde_json::from_str::<Game>(&formatted_input) {
        Ok(game) => println!("Parsed successfully: {:#?}", game),
        Err(err) => eprintln!("Failed to parse input: {:#?}", err),
    }
}

fn solve_part_one(data: ProcessedInput) -> Result<Output> {
    let mut total_tokens = 0;
    for game in data {
        if let Some(tokens) = game.solve_part_one((3, 1)) {
            total_tokens += tokens;
        }
    }
    Ok(total_tokens)
}

fn solve_part_two(data: ProcessedInput) -> Result<Output> {
    let mut total_tokens = 0;
    for game in data {
        if let Some(tokens) = game.solve_part_two((3, 1)) {
            total_tokens += tokens;
        }
    }
    Ok(total_tokens)
}
