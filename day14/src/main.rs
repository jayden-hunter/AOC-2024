use anyhow::anyhow;
use anyhow::bail;
use anyhow::Result;
use core::f32;
use grid::Grid;
use humantime::format_duration;
use map::Map;
use std::{fs::read_to_string, time::Instant};
mod map;

type ProcessedInput = Map<Vec<Robot>>;
type Output = i32;
#[derive(Clone, Debug)]
struct Robot {
    velocity: (i32, i32),
}
const ROWS: usize = 101;
const COLS: usize = 103;

impl Robot {
    fn new(velocity: (i32, i32)) -> Self {
        Self { velocity }
    }

    fn tick(&self, pos: (usize, usize)) -> (usize, usize) {
        let x = pos.0 as i32 + self.velocity.0;
        let y = pos.1 as i32 + self.velocity.1;
        let wrapped_x = x.rem_euclid(ROWS as i32);
        let wrapped_y = y.rem_euclid(COLS as i32);
        (wrapped_x as usize, wrapped_y as usize)
    }
}

fn string_map(map: &Map<Vec<Robot>>) -> String {
    let mut string = String::new();
    for row in map.cells.iter_cols() {
        for cell in row.into_iter() {
            string.push_str(&format!("({})", cell.len()));
        }
        string.push_str("\n\r");
    }
    string
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
    // println!("{}", string_map(&processed));
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
    let lines = input.lines().filter(|x| !x.is_empty());
    let mut grid = Grid::new(ROWS, COLS);
    for line in lines {
        let (pos_str, vel_str) = line.split_once(" ").ok_or(anyhow!(
            "Input must have whitespace between pos and vel components"
        ))?;
        let pos_str = pos_str
            .split_once("p=")
            .ok_or(anyhow!("position component must be prefaced with 'p='"))?
            .1;
        let vel_str = vel_str
            .split_once("v=")
            .ok_or(anyhow!("velocity component must be prefaced with 'v='"))?
            .1;
        let (p0, p1) = pos_str
            .split_once(",")
            .ok_or(anyhow!("Pos digits must be split with ','"))?;
        let (v0, v1) = vel_str
            .split_once(",")
            .ok_or(anyhow!("Vel digits must be split with ','"))?;
        let p0: usize = p0.parse()?;
        let p1: usize = p1.parse()?;
        let v0 = v0.parse()?;
        let v1 = v1.parse()?;
        let robot = Robot::new((v0, v1));
        let pos_cell: &mut Vec<Robot> = grid.get_mut(p0, p1).ok_or(anyhow!(
            "Robot Position is out of bounds! ({p0}, {p1}) to bounds ({ROWS}, {COLS})"
        ))?;
        pos_cell.push(robot);
    }
    Ok(Map::new(grid))
}

fn solve_part_one(mut map: ProcessedInput) -> Result<Output> {
    for _ in 0..100 {
        // println!("Tick {i} complete");
        map = next_tick(&map);
    }
    let safety_factor = calculate_safety_factor(map);
    Ok(safety_factor)
}

fn next_tick(map: &Map<Vec<Robot>>) -> Map<Vec<Robot>> {
    let mut new_map = map.clone();
    for cell in new_map.cells.iter_mut() {
        *cell = Vec::new();
    }
    for (pos, robots) in map.cells.indexed_iter() {
        for robot in robots {
            let new_pos = robot.tick(pos);
            (*new_map.cells.get_mut(new_pos.0, new_pos.1).unwrap()).push(robot.to_owned());
        }
    }
    new_map
}

fn calculate_safety_factor(data: Map<Vec<Robot>>) -> i32 {
    // println!("{}", string_map(&data));
    let col_split = data.cells.cols() as f32 / 2. - 1.; // cols = 4, col_split = 1.
    let row_split = data.cells.rows() as f32 / 2. - 1.; // rows = 5, row_split = 1.5
    let mut nw_quad = 0;
    let mut ne_quad = 0;
    let mut se_quad = 0;
    let mut sw_quad = 0;
    for ((r, c), robots) in data.cells.indexed_iter() {
        let robot_count = robots.len();
        if robot_count == 0 {
            continue;
        }
        let r = r as f32;
        let c = c as f32;
        // In the case of row_split being 3.5, we want an index of 3 to read as north, 4 to be
        // neither, and 5 to be south.
        // In the case of 2, we want an index of 2 to read as north, 3 to be south.
        // South when +7.5
        let north = r <= row_split;
        let south = r > row_split + 0.75;
        let east = c <= col_split;
        let west = c > col_split + 0.75;
        if north && east {
            ne_quad += robot_count;
        } else if north && west {
            nw_quad += robot_count;
        } else if south && west {
            sw_quad += robot_count;
        } else if south && east {
            se_quad += robot_count;
        }
    }
    // println!("{nw_quad}, {ne_quad}, {se_quad}, {sw_quad}");
    let total = nw_quad * ne_quad * se_quad * sw_quad;
    total as Output
}

fn solve_part_two(mut map: ProcessedInput) -> Result<Output> {
    for i in 1..=10403 {
        map = next_tick(&map);
        if christmas_tree_displayed(&map) {
            return Ok(i);
        }
    }
    bail!("Maximum window exceeded.");
}

fn christmas_tree_displayed(map: &Map<Vec<Robot>>) -> bool {
    for row in map.cells.iter_rows() {
        let mut consecutive_cells = 0;
        const BOUND: i32 = 13;
        for cell in row {
            if !cell.is_empty() {
                consecutive_cells += 1;
            } else {
                consecutive_cells = 0;
            }
            if consecutive_cells >= BOUND {
                println!("{}", string_map(map));
                return true;
            }
        }
    }
    false
}
