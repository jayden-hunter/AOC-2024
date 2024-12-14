use anyhow::anyhow;
use anyhow::Result;
use humantime::format_duration;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::{fs::read_to_string, str::FromStr, time::Instant};

type ProcessedInput = Vec<Equation>;
type Output = i64;

#[derive(Clone)]
struct Equation {
    lhs: i64,
    rhs: Vec<i64>,
}

impl Equation {
    fn new(lhs: i64, rhs: Vec<i64>) -> Self {
        Self { lhs, rhs }
    }

    fn solve(&self, solve_method: &dyn Fn(Vec<i64>, i64) -> Vec<i64>) -> Result<bool> {
        let mut rhs_iter = self.rhs.clone().into_iter();
        let mut options = vec![rhs_iter
            .next()
            .ok_or(anyhow!("RHS Must contain at least one element"))?];
        for num in rhs_iter {
            options = solve_method(options, num);
        }
        Ok(options.contains(&self.lhs))
    }

    fn recursive_part_one(left_options: Vec<i64>, next: i64) -> Vec<i64> {
        let mut options = vec![];
        for left_opt in left_options {
            let addition = left_opt.checked_add(next);
            let multiplication = left_opt.checked_mul(next);
            if let Some(a) = addition {
                options.push(a);
            }
            if let Some(m) = multiplication {
                options.push(m);
            }
        }
        options
    }

    fn recursive_part_two(left_options: Vec<i64>, next: i64) -> Vec<i64> {
        let mut options = Equation::recursive_part_one(left_options.clone(), next);
        for left_opt in left_options {
            let mut combination = left_opt.to_string();
            combination.push_str(&next.to_string());
            if let Ok(c) = combination.parse() {
                options.push(c);
            }
        }
        options
    }
}

impl FromStr for Equation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (lhs, rhs_str) = s
            .split_once(":")
            .ok_or(anyhow!("Equation must have a ':' char."))?;
        let mut rhs_vec = vec![];
        for num_str in rhs_str.split(" ").filter(|s| !s.is_empty()) {
            rhs_vec.push(num_str.parse()?)
        }
        Ok(Equation::new(lhs.parse()?, rhs_vec))
    }
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
        .map(|x| x.parse())
        .collect()
}

fn solve_part_one(data: ProcessedInput) -> Result<Output> {
    Ok(data
        .par_iter()
        .filter(|e| e.solve(&Equation::recursive_part_one).is_ok_and(|v| v))
        .map(|e| e.lhs)
        .sum())
}

fn solve_part_two(data: ProcessedInput) -> Result<Output> {
    Ok(data
        .par_iter()
        .filter(|e| e.solve(&Equation::recursive_part_two).is_ok_and(|v| v))
        .map(|e| e.lhs)
        .sum())
}
