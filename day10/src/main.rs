mod map;
use anyhow::{anyhow, Result};
use grid::Grid;
use humantime::format_duration;
use map::{Direction, Map};
use std::{collections::HashSet, fs::read_to_string, time::Instant};

type ProcessedInput = Map<u8>;

type Output = usize;

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
    let cols = string
        .lines()
        .find(|e| !e.is_empty())
        .ok_or(anyhow!("Input must contain at least one line"))?
        .len();
    let rows = string.lines().filter(|e| !e.is_empty()).count();
    let mut grid = Grid::new(rows, cols);
    for (y, line) in string.lines().filter(|e| !e.is_empty()).enumerate() {
        for (x, char) in line.chars().enumerate() {
            let digit =
                char.to_digit(10)
                    .ok_or(anyhow!("All characters must be digits [0-9]"))? as u8;
            // println!("{x} and {y}");
            *grid.get_mut(x, y).unwrap() = digit;
        }
    }
    Ok(Map::new(grid))
}

fn solve_part_one(map: ProcessedInput) -> Result<Output> {
    let trail_starters = map.cells.indexed_iter().filter(|(_, &f)| f == 0);
    let mut count = 0;
    for (start_pos, num) in trail_starters {
        let trail_score = score_trails_part_one(start_pos, *num, &map).len();
        count += trail_score
    }
    Ok(count)
}

fn score_trails_part_one(
    pos: (usize, usize),
    num: u8,
    map: &ProcessedInput,
) -> HashSet<(usize, usize)> {
    let mut hashset = HashSet::new();
    if num == 9 {
        hashset.insert(pos);
        return hashset;
    };
    for dir in Direction::cardinals().into_iter() {
        let cell = map.get_relative_cell(&pos, dir);
        let (cell_pos, &new_num) = match cell {
            Some(c) => c,
            None => continue,
        };
        if new_num != num + 1 {
            continue;
        }
        // Cell is one higher, and cardinally adjacent.
        hashset.extend(score_trails_part_one(cell_pos, new_num, map).iter());
    }
    hashset
}

fn solve_part_two(map: ProcessedInput) -> Result<Output> {
    let trail_starters = map.cells.indexed_iter().filter(|(_, &f)| f == 0);
    let mut count = 0;
    for (start_pos, num) in trail_starters {
        let distinct_trails = score_trails_part_two(start_pos, *num, &map).len();
        count += distinct_trails;
    }
    Ok(count)
}

type TrailPath = Vec<(usize, usize)>;

fn score_trails_part_two(pos: (usize, usize), num: u8, map: &ProcessedInput) -> Vec<TrailPath> {
    // println!("Scoring Part Two: at {:?}[{num}]", pos);
    let mut paths: Vec<TrailPath> = Vec::new();
    if num == 9 {
        let final_path = vec![pos];
        paths.push(final_path);
        // println!("Found a 9, added positions to trailpath. [{:?}]", paths);
        return paths;
    };
    for dir in Direction::cardinals().into_iter() {
        let cell = map.get_relative_cell(&pos, dir);
        let (cell_pos, &new_num) = match cell {
            Some(c) => c,
            None => continue,
        };
        if new_num != num + 1 {
            continue;
        }
        // Cell is one higher, and cardinally adjacent. Add it to each TrailPath.
        let mut new_paths = score_trails_part_two(cell_pos, new_num, map);
        for path in new_paths.iter_mut() {
            path.push(pos);
        }
        paths.append(&mut new_paths);
    }
    // println!("After all directions [{:?}]", paths);
    paths
}
