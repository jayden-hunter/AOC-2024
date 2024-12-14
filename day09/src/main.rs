use anyhow::{anyhow, Result};
use humantime::format_duration;
use std::{fs::read_to_string, time::Instant};

type ProcessedInput = Vec<u32>;
type Output = i64;

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

    let time_two = Instant::now();
    let part_two = solve_part_two(processed);
    let time_two = format_duration(time_two.elapsed());

    println!("Part One: {:?} (Took: {})", part_one, time_one);
    println!("Part Two: {:?} (Took: {})", part_two, time_two);
    Ok(())
}

fn process_input(string: String) -> Result<ProcessedInput> {
    string
        .chars()
        .map(|c| c.to_digit(10).ok_or(anyhow!("Must be a valid digit")))
        .collect()
}

fn solve_part_one(data: ProcessedInput) -> Result<Output> {
    let mut vec = construct_initial_alloc(data);
    defrag_part_one(&mut vec);
    // println!("{:?}", vec);
    Ok(calculate_checksum(vec))
}

fn defrag_part_one(vec: &mut [Option<u32>]) {
    // println!("{:?}", vec);
    let mut forward_index = 0;
    let mut backwards_index = vec.len() - 1;
    while forward_index < backwards_index {
        // println!("Iterating: {forward_index} and {backwards_index}");
        let front = vec[forward_index];
        let back = vec[backwards_index];
        // println!("Inspecting {:?} and {:?}", front, back);
        if front.is_none() && back.is_some() {
            // println!("Swapping {:?} and {:?}", front, back);
            vec[forward_index] = back;
            vec[backwards_index] = None;
            backwards_index -= 1;
            forward_index += 1;
        } else if front.is_none() && back.is_none() {
            backwards_index -= 1;
        } else if front.is_some() && back.is_none() {
            forward_index += 1;
            backwards_index -= 1;
        } else if front.is_some() && back.is_some() {
            forward_index += 1;
        }
    }
}

fn calculate_checksum(vec: Vec<Option<u32>>) -> i64 {
    let mut sum = 0;
    for (i, val) in vec.iter().enumerate() {
        sum += i as i64 * val.unwrap_or(0) as i64;
    }
    sum
}

fn construct_initial_alloc(data: Vec<u32>) -> Vec<Option<u32>> {
    let mut vec = vec![];
    let mut iter = data.into_iter();
    let mut index: u32 = 0;
    loop {
        let data_size = match iter.next() {
            Some(t) => t,
            None => return vec,
        };
        let mut data_vec = vec![Some(index); data_size as usize];
        vec.append(&mut data_vec);
        index += 1;

        let empty_space = match iter.next() {
            Some(t) => t,
            None => return vec,
        };
        let mut empty_vec = vec![None; empty_space as usize];
        vec.append(&mut empty_vec);
    }
}

fn solve_part_two(data: ProcessedInput) -> Result<Output> {
    let mut vec = construct_initial_alloc(data.clone());
    // println!("{:?}", vec);
    let mut dict = construct_initial_map(data);
    defrag_part_two(&mut vec, &mut dict);
    // println!("{:?}", vec);
    Ok(calculate_checksum(vec))
}

fn defrag_part_two(vec: &mut [Option<u32>], dict: &mut [u32]) {
    for (id, item_size) in dict.iter().enumerate().rev() {
        // println!("Attempting to move: {id}");
        move_one_element(id, *item_size, vec);
    }
}

fn move_one_element(id: usize, item_size: u32, vec: &mut [Option<u32>]) {
    // Lets find a space
    let start_index = match find_start_index(vec, item_size, id as u32) {
        Some(t) => t,
        None => return,
    };
    // println!(
    //     "\tSpace found at {start_index} for {id}. Currently: {:?}",
    //     vec[start_index]
    // );
    for x in &mut *vec {
        if x.is_some_and(|y| y == id as u32) {
            *x = None;
        }
    }
    // We have now removed the original elements. Let us insert
    for x in 0..item_size {
        vec[x as usize + start_index as usize] = Some(id as u32);
    }
}

fn find_start_index(vec: &[Option<u32>], item_size: u32, id: u32) -> Option<usize> {
    // Given an item size, find the earliest
    let windows = vec.windows(item_size as usize);
    for (start_index, vals) in windows.enumerate() {
        if vals.iter().any(|v| v.is_some_and(|v| v == id)) {
            return None;
        }
        if vals.iter().all(|v| v.is_none()) {
            return Some(start_index);
        }
    }
    None
}

fn construct_initial_map(data: Vec<u32>) -> Vec<u32> {
    let mut vec = vec![];
    let mut iter = data.into_iter();
    loop {
        let data_size = match iter.next() {
            Some(t) => t,
            None => return vec,
        };
        vec.push(data_size);
        let _ = iter.next();
    }
}
