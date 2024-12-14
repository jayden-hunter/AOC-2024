use std::{collections::HashMap, error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;
    println!("Lines in Input: {}", input.len());
    let output = process_input(input)?;
    println!("{:?}", output);
    Ok(())
}

fn process_input(input: String) -> Result<String, Box<dyn Error>> {
    let lines = input.lines().filter(|l| !l.is_empty());
    let mut left = vec![];
    let mut right = vec![];
    for line in lines {
        let mut line_vec = line.split_whitespace();
        let first = line_vec.next().ok_or("Invalid Input Line".to_owned())?;
        let last = line_vec.next().ok_or("Invalid Input line".to_owned())?;
        left.push(first.parse()?);
        right.push(last.parse()?);
    }
    solve_part_two(left, right)
    // solve_part_one(left, right)
}

fn solve_part_two(left: Vec<i32>, right: Vec<i32>) -> Result<String, Box<dyn Error>> {
    let right = sum_vec_into_countmap(right);
    let sum: i32 = left
        .into_iter()
        .map(|x| x * right.get(&x).unwrap_or(&0))
        .sum();
    Ok(sum.to_string())
}

fn sum_vec_into_countmap(right: Vec<i32>) -> HashMap<i32, i32> {
    let mut count_map: HashMap<i32, i32> = HashMap::new();

    for &num in &right {
        *count_map.entry(num).or_insert(0) += 1;
    }
    count_map
}

fn solve_part_one(mut left: Vec<i32>, mut right: Vec<i32>) -> Result<String, Box<dyn Error>> {
    left.sort();
    right.sort();
    let sum: u32 = left
        .into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum();
    Ok(sum.to_string())
}
