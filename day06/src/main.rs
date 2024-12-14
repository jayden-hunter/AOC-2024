use anyhow::anyhow;
use anyhow::bail;
use anyhow::Result;
use humantime::format_duration;
use ndarray::Array2;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::collections::HashSet;
use std::{fs::read_to_string, time::Instant};

type ProcessedInput = Map;
type Output = u32;

#[derive(Clone, PartialEq)]
struct Map {
    cells: Array2<WCell>,
    guard_pos: Option<(usize, usize)>,
    guard_facing: Facing,
}

impl Map {
    fn new(cells: Array2<WCell>, guard_pos: (usize, usize), guard_facing: Facing) -> Self {
        Self {
            cells,
            guard_pos: Some(guard_pos),
            guard_facing,
        }
    }
}

#[derive(Clone, PartialEq)]
enum WCell {
    Empty(Visited),
    Block,
    Guard(Facing),
}

type Visited = bool;
#[derive(Clone, PartialEq, Eq, Hash)]
enum Facing {
    North,
    East,
    South,
    West,
}
impl Facing {
    fn turn_right(&self) -> Facing {
        match self {
            Facing::North => Facing::East,
            Facing::East => Facing::South,
            Facing::South => Facing::West,
            Facing::West => Facing::North,
        }
    }
}

fn main() -> Result<()> {
    let input = read_to_string("/home/jayden/Documents/Code/AdventOfCode/2024/day06/input.txt")?;
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
    let width = input
        .lines()
        .next()
        .ok_or_else(|| anyhow!("Input must contain some text"))?
        .trim()
        .len();
    let height = input.lines().filter(|s| !s.is_empty()).count();

    // Create a flat vector of WCell
    let mut flat_map = Vec::with_capacity(width * height);
    for line in input.lines() {
        for char in line.chars() {
            let wcell_type = match char {
                '#' => WCell::Block,
                '.' => WCell::Empty(false),
                '^' => WCell::Guard(Facing::North),
                _ => bail!("Invalid character found in map"),
            };
            flat_map.push(wcell_type);
        }
    }
    let map_cells = Array2::from_shape_vec((height, width), flat_map)?;
    let guard = map_cells
        .indexed_iter()
        .find(|(_, c)| matches!(c, WCell::Guard(_)))
        .ok_or(anyhow!("No Guard on Map"))?
        .0;

    Ok(Map::new(map_cells, guard, Facing::North))
}

fn solve_part_one(mut map: ProcessedInput) -> Result<Output> {
    while map.guard_pos.is_some() {
        map.tick()?;
    }
    Ok(map
        .cells
        .iter()
        .filter(|c| matches!(c, WCell::Empty(true)))
        .count() as Output)
}
impl Map {
    fn tick(&mut self) -> Result<()> {
        let (y, x) = match self.guard_pos {
            Some(g) => (g.0 as i32, g.1 as i32),
            None => return Ok(()),
        };
        let new_index: (i32, i32) = match self.guard_facing {
            Facing::North => (y - 1, x),
            Facing::East => (y, x + 1),
            Facing::South => (y + 1, x),
            Facing::West => (y, x - 1),
        };
        let x = x as usize;
        let y = y as usize;
        let new_index = if new_index.0 < 0 || new_index.1 < 0 {
            self.cells[[y, x]] = WCell::Empty(true);
            self.guard_pos = None;
            return Ok(());
        } else {
            (new_index.0 as usize, new_index.1 as usize)
        };
        let new_cell = match self.cells.get(new_index) {
            Some(t) => t,
            None => {
                self.cells[[y, x]] = WCell::Empty(true);
                self.guard_pos = None;
                return Ok(());
            }
        };
        match new_cell {
            WCell::Empty(_) => {
                self.cells[[y, x]] = WCell::Empty(true);
                self.cells[[new_index.0, new_index.1]] = WCell::Guard(self.guard_facing.to_owned());
                self.guard_pos = Some(new_index);
                return Ok(());
            }
            WCell::Block => {
                let new_facing = self.guard_facing.turn_right();
                self.cells[[y, x]] = WCell::Guard(new_facing.clone());
                self.guard_facing = new_facing;
            }
            WCell::Guard(_) => panic!("Multiple Guards Found :("),
        }
        Ok(())
    }
}
fn solve_part_two(map: ProcessedInput) -> Result<Output> {
    let all_maps = generate_map_series(map)?;
    println!("All Versions Generated. Running...");
    let count = all_maps
        .par_iter()
        .filter_map(|m| is_looping(m.to_owned()).ok())
        .filter(|v| *v)
        .count();
    Ok(count as Output)
}

fn generate_map_series(mut map: Map) -> Result<Vec<Map>> {
    let fresh_map = map.clone();
    while map.guard_pos.is_some() {
        map.tick()?;
    }
    let mut all_maps = vec![];
    for (cell_index, _) in map
        .cells
        .indexed_iter()
        .filter(|(_, c)| matches!(c, WCell::Empty(true)))
    {
        let mut test_map = fresh_map.clone();
        test_map.cells[[cell_index.0, cell_index.1]] = WCell::Block;
        all_maps.push(test_map);
    }
    Ok(all_maps)
}

fn is_looping(mut map: Map) -> Result<bool> {
    let mut visited = HashSet::new();
    loop {
        map.tick()?;
        match map.guard_pos {
            Some(t) => {
                let insert_success = visited.insert((t, map.guard_facing.clone()));
                if !insert_success {
                    return Ok(true);
                }
            }
            None => return Ok(false),
        }
    }
}
