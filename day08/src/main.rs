use anyhow::{anyhow, Result};
use grid::Grid;
use humantime::format_duration;
use std::{cell, collections::HashMap, fs::read_to_string, time::Instant};

type ProcessedInput = Map;
type Output = usize;

#[derive(Clone)]
struct Map {
    cells: Grid<ACell>,
    frequencies: HashMap<char, Vec<(usize, usize)>>,
}

impl Map {
    fn new(cells: Grid<ACell>, frequencies: HashMap<char, Vec<(usize, usize)>>) -> Self {
        Self { cells, frequencies }
    }

    fn calculate_antinodes(&mut self, factor_harmonics: bool) {
        for (_, positions) in self.frequencies.clone() {
            for pos in positions.iter() {
                self.update_antinode(pos, &positions, factor_harmonics);
            }
        }
        println!("{}", self.print());
    }

    fn print(&self) -> String {
        let mut str = String::new();
        for row in self.cells.iter_rows() {
            for col in row {
                let mut char = col.antenna.unwrap_or('.');
                if char == '.' && col.antinode_present {
                    char = '#';
                }
                str.push(char);
            }
            str.push('\n');
        }
        str
    }

    fn update_antinode(
        &mut self,
        pos: &(usize, usize),
        positions: &Vec<(usize, usize)>,
        factor_harmonics: bool,
    ) {
        for other_pos in positions {
            self.update_antinode_sub(pos, other_pos, factor_harmonics);
        }
    }
    fn update_antinode_sub(
        &mut self,
        pos1: &(usize, usize),
        pos2: &(usize, usize),
        factor_harmonics: bool,
    ) {
        if pos1 == pos2 {
            return;
        }
        if factor_harmonics {
            self.cells.get_mut(pos1.0, pos1.1).unwrap().antinode_present = true;
        }
        let delta_pos = (pos1.0 as i32 - pos2.0 as i32, pos1.1 as i32 - pos2.1 as i32);
        let mut antinode_pos = (pos1.0 as i32, pos1.1 as i32);
        loop {
            antinode_pos = (antinode_pos.0 + delta_pos.0, antinode_pos.1 + delta_pos.1);
            if antinode_pos.0 < 0 || antinode_pos.1 < 0 {
                return;
            }
            let antinode_pos = (antinode_pos.0 as usize, antinode_pos.1 as usize);
            if antinode_pos.0 >= self.cells.size().0 || antinode_pos.1 >= self.cells.size().1 {
                return;
            }
            self.cells
                .get_mut(antinode_pos.0, antinode_pos.1)
                .unwrap()
                .antinode_present = true;
            if !factor_harmonics {
                return;
            }
        }
    }
}

#[derive(Default, Clone)]
struct ACell {
    antinode_present: bool,
    antenna: Option<char>,
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
    let width = input
        .lines()
        .next()
        .ok_or(anyhow!("Input must contain at least one line"))?
        .len();
    let height = input.lines().filter(|e| !e.is_empty()).count();
    let mut grid = Grid::new(width, height);
    let mut frequencies: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (row, str) in input.lines().filter(|e| !e.is_empty()).enumerate() {
        for (col, char) in str.chars().enumerate() {
            let cell: &mut ACell = grid.get_mut(row, col).unwrap();
            cell.antinode_present = false;
            if '.' == char {
                cell.antenna = None;
            } else {
                frequencies.entry(char).or_default().push((row, col));
                cell.antenna = Some(char);
            }
        }
    }
    Ok(Map::new(grid, frequencies))
}

fn solve_part_one(mut map: Map) -> Result<Output> {
    map.calculate_antinodes(false);
    Ok(map.cells.iter().filter(|e| e.antinode_present).count())
}

fn solve_part_two(mut map: Map) -> Result<Output> {
    map.calculate_antinodes(true);
    Ok(map.cells.iter().filter(|e| e.antinode_present).count())
}
