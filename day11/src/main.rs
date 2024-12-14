use anyhow::Result;
use cached::proc_macro::cached;
use humantime::format_duration;
use std::{collections::HashMap, fs::read_to_string, time::Instant};

type ProcessedInput = Vec<u64>;
type Output = u64;

fn main() -> Result<()> {
    let string = read_to_string("input.txt")?;

    let time_proc = Instant::now();
    let processed = process_input(string.clone())?;
    let time_proc = format_duration(time_proc.elapsed());
    println!(
        "Processed input [{:?} lines] (Took: {})",
        string.lines().count(),
        time_proc
    );

    let time_one = Instant::now();
    let part_one = solve_part_one(processed.clone());
    let time_one = format_duration(time_one.elapsed());
    println!("Part One: {:?} (Took: {})", part_one, time_one);

    let time_two = Instant::now();
    let part_two = solve_part_two(processed);
    let time_two = format_duration(time_two.elapsed());

    println!("Part Two: {:?} (Took: {})", part_two, time_two);
    Ok(())
}

fn process_input(string: String) -> Result<ProcessedInput> {
    let mut vec = vec![];
    for num_word in string.split(char::is_whitespace) {
        vec.push(num_word.parse()?);
    }
    Ok(vec)
}

fn solve_part_one(mut data: ProcessedInput) -> Result<Output> {
    const NUM_BLINKS: i32 = 25;
    for _ in 0..NUM_BLINKS {
        data = tick_part_one(&data);
        // println!("{:?}", data);
    }
    Ok(data.len() as Output)
}

fn tick_part_one(data: &[u64]) -> Vec<u64> {
    let mut new_vec = vec![];
    for stone in data {
        new_vec.append(&mut tick_one_stone(*stone));
    }
    new_vec
}

#[cached]
fn tick_stone_recursive(stone: u64, ticks: u64) -> HashMap<u64, u64> {
    // println!("\t\tRecusive called: Simulating {stone} for {ticks} ticks");
    let result = tick_one_stone(stone);
    let mut hashmap = HashMap::new();
    for r in result {
        hashmap.entry(r).and_modify(|e| *e += 1).or_insert(1);
    }

    if ticks == 1 {
        // println!("\tSimulated {stone} for 1 tick. -> {:?}", hashmap);
        return hashmap;
    }
    let mut new_map = HashMap::new();
    for (child_stone, count) in hashmap.iter() {
        // Last tick we generated 3 2's. What happens if we run that now?
        let sub_children = tick_stone_recursive(*child_stone, ticks - 1);
        for entry in sub_children {
            let mult_total = entry.1 * count;
            new_map
                .entry(entry.0)
                .and_modify(|e| *e += mult_total)
                .or_insert(mult_total);
        }
    }
    // println!("\tSimulated {stone} for {ticks} ticks. -> {:?}", new_map);
    new_map
}

#[cached]
fn tick_one_stone(stone: u64) -> Vec<u64> {
    let mut new_vec = Vec::new();
    if stone == 0 {
        new_vec.push(1);
    } else if let Some((l, r)) = split_digits_evenly(stone) {
        new_vec.push(l);
        new_vec.push(r);
    } else {
        let new_stone = stone * 2024;
        new_vec.push(new_stone);
    }
    new_vec
}

fn split_digits_evenly(num: u64) -> Option<(u64, u64)> {
    let digits = num.checked_ilog10()? + 1;
    if digits % 2 != 0 {
        // println!("{num} is has {digits} digits");
        return None;
    }
    let midpoint = digits / 2;
    let midpoint_pow10 = 10_u64.pow(midpoint);
    let left = num / midpoint_pow10;
    let right = num % midpoint_pow10;
    Some((left, right))
}

fn solve_part_two(data: ProcessedInput) -> Result<Output> {
    let mut count = 0;
    for stone in data {
        count += tick_stone_recursive(stone, 75).values().sum::<u64>();
    }
    Ok(count as Output)
}
