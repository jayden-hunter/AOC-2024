use anyhow::{bail, Context, Result};
use humantime::format_duration;
use map::Map;
use num::integer::gcd;
use std::{f64, fs::read_to_string, time::Instant};
mod map;

type ProcessedInput = Vec<Game>;
type Output = i64;

#[derive(Clone, Debug)]
struct Game {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
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
        // We know that we have two equations to solve:
        // minimize(3a + b)
        // for a,b such that ax + bx = t_y and ay + by = t_y
        // We want to enumerate through costs in a clever way, because I want to.
        // so its like 0, 0. then 0, 1, then 0, 2 then 0, 3, then 1, 0 ... then 0, 4, then 0, 5.

        let (cost_a, cost_b) = token_costs;
        for tokens in 0_i64.. {
            for pushes in Game::generate_pairs_with_gcd(tokens, cost_a, cost_b, 0..101) {
                // println!("For {tokens}: {:?}", pushes);
                if self.is_valid_solution(pushes.0, pushes.1) {
                    return Some(tokens);
                }
                if pushes.0 >= 100 && pushes.1 >= 100 {
                    return None;
                }
            }
        }
        None
    }

    fn solve_part_two(&self, token_costs: (i64, i64)) -> Option<i64> {
        // Cramers rule to solve
        const MEASUREMENT: i64 = 10000000000000;
        let mut updated_costs = self.clone();
        updated_costs.prize.0 += MEASUREMENT;
        updated_costs.prize.1 += MEASUREMENT;
        println!("{:?} ->{:?}", self, updated_costs);
        let (pushes_a, pushes_b) =
            solve_2d_cramers(self.button_a, self.button_b, updated_costs.prize)?;
        let cost_a = token_costs.0 * pushes_a;
        let cost_b = token_costs.1 * pushes_b;
        let total = cost_a + cost_b;
        println!(
            "Solution for {:?}: 3*{pushes_a} + 1*{pushes_b} = {total}",
            updated_costs
        );
        Some(total)
    }
    fn generate_pairs_with_gcd(
        x: i64,
        a: i64,
        b: i64,
        range: std::ops::Range<i64>,
    ) -> Vec<(i64, i64)> {
        let g = gcd(a, b);

        // Check if x is divisible by gcd(a, b)
        if x % g != 0 {
            return vec![]; // No solution exists
        }

        // Scale the coefficients and x
        let (a, b, x) = (a / g, b / g, x / g);

        // Find one particular solution using the extended Euclidean algorithm
        let (mut i0, mut j0) = Game::extended_gcd(a, b);

        // Scale the particular solution to satisfy the equation
        i0 *= x;
        j0 *= x;

        // Generate all solutions using the parametric form
        range
            .clone()
            .filter_map(|k| {
                let i = i0 + k * b; // Parametric solution for i
                let j = j0 - k * a; // Parametric solution for j
                if range.contains(&i) && range.contains(&j) {
                    Some((i, j))
                } else {
                    None
                }
            })
            .collect()
    }

    // Extended Euclidean Algorithm
    fn extended_gcd(a: i64, b: i64) -> (i64, i64) {
        if b == 0 {
            (1, 0)
        } else {
            let (x, y) = Game::extended_gcd(b, a % b);
            (y, x - (a / b) * y)
        }
    }

    fn is_valid_solution(&self, pushes_a: i64, pushes_b: i64) -> bool {
        let ax = pushes_a * self.button_a.0;
        let bx = pushes_b * self.button_b.0;
        let ay = pushes_a * self.button_a.1;
        let by = pushes_b * self.button_b.1;
        let x = ax + bx;
        let y = ay + by;
        if pushes_a == 80 && pushes_b == 40 {
            // println!(
            //     "Pushing. {ax} + {bx}, {ay} + {by} = {x}, {y}. Trying to do this for {:#?}",
            //     self
            // );
        }
        x == self.prize.0 && y == self.prize.1
    }
}

fn solve_2d_cramers(a: (i64, i64), b: (i64, i64), c: (i64, i64)) -> Option<(i64, i64)> {
    println!("Cramers: a={:?}, b={:?}, c={:?}", a, b, c);
    let numerator_x = (c.0 * b.1) - (b.0 * c.1);
    let denominator_x = (a.0 * b.1) - (b.0 * a.1);
    let numerator_y = (a.0 * c.1) - (c.0 * a.1);
    let denominator_y = (a.0 * b.1) - (b.0 * a.1);
    println!("X: {numerator_x}/{denominator_y} \t Y:{numerator_y}/{denominator_y}");
    if numerator_x % denominator_x != 0 {
        return None;
    }
    if numerator_y % denominator_y != 0 {
        return None;
    }
    let x = numerator_x / denominator_x;
    let y = numerator_y / denominator_y;
    println!("X: {x}, \t Y: {y}");
    Some((x as i64, y as i64))
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

    println!("{:#?}", processed);
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
    let button_a_str = match game_iter.next() {
        Some(t) => t,
        None => return Ok(None),
    };
    let button_b_str = match game_iter.next() {
        Some(t) => t,
        None => return Ok(None),
    };
    let prize_str = match game_iter.next() {
        Some(t) => t,
        None => return Ok(None),
    };
    let (_, button_a_str) = match button_a_str.split_once("X+") {
        Some(t) => t,
        None => return Ok(None),
    };
    let (button_a_x, button_a_y) = match button_a_str.split_once(", Y+") {
        Some(t) => t,
        None => return Ok(None),
    };
    let button_a = (button_a_x.parse()?, button_a_y.parse()?);

    let (_, button_b_str) = match button_b_str.split_once("X+") {
        Some(t) => t,
        None => return Ok(None),
    };
    let (button_b_x, button_b_y) = match button_b_str.split_once(", Y+") {
        Some(t) => t,
        None => return Ok(None),
    };
    let button_b = (button_b_x.parse()?, button_b_y.parse()?);

    let (_, prize_str) = match prize_str.split_once("X=") {
        Some(t) => t,
        None => return Ok(None),
    };
    let (prize_x, prize_y) = match prize_str.split_once(", Y=") {
        Some(t) => t,
        None => return Ok(None),
    };
    let prize = (prize_x.parse()?, prize_y.parse()?);

    Ok(Some(Game::new(button_a, button_b, prize)))
}

fn solve_part_one(data: ProcessedInput) -> Result<Output> {
    let mut total_tokens = 0;
    for game in data {
        if let Some(tokens) = game.solve_part_one((3, 1)) {
            println!("Found Game Sol");
            total_tokens += tokens;
        }
    }
    Ok(total_tokens)
}

fn solve_part_two(data: ProcessedInput) -> Result<Output> {
    let mut total_tokens = 0;
    for game in data {
        if let Some(tokens) = game.solve_part_two((3, 1)) {
            println!("Found Game Sol");
            total_tokens += tokens;
        }
    }
    Ok(total_tokens)
}
