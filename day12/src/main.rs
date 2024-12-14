use anyhow::{anyhow, bail, Result};
use grid::Grid;
use humantime::format_duration;
use map::{Direction, Map};
use std::{collections::VecDeque, fmt::Display, fs::read_to_string, time::Instant};
mod map;

type ProcessedInput = Map<char>;
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

    let time_two = Instant::now();
    let part_two = solve_part_two(processed);
    let time_two = format_duration(time_two.elapsed());

    println!("Part One: {:?} (Took: {})", part_one, time_one);
    println!("Part Two: {:?} (Took: {})", part_two, time_two);
    Ok(())
}

fn process_input(string: String) -> Result<ProcessedInput> {
    let lines = string.lines().filter(|e| !e.is_empty());
    let rows = lines.clone().count();
    let cols = lines
        .clone()
        .next()
        .ok_or(anyhow!("Input must contain at least one line"))?
        .len();
    let mut grid = Grid::new(rows, cols);
    for (row, line) in lines.enumerate() {
        for (col, c) in line.chars().enumerate() {
            *grid.get_mut(row, col).unwrap() = c;
        }
    }
    Ok(Map::new(grid))
}

fn solve_part_one(map: ProcessedInput) -> Result<Output> {
    let regions = split_map_into_regions(&map);
    let mut count = 0;
    for region in regions {
        let area = calculate_area(&region);
        let perimeter = calculate_perimeter(&region);
        let score = area * perimeter;
        count += score;
    }
    Ok(count)
}

fn split_map_into_regions(map: &Map<char>) -> Vec<Map<bool>> {
    // Create a mutable clone of the map where cells can be consumed.
    let mut grid: Grid<Option<char>> = Grid::new(map.cells.rows(), map.cells.cols());
    for (pos, &cell) in map.cells.indexed_iter() {
        *grid.get_mut(pos.0, pos.1).unwrap() = Some(cell);
    }

    let mut map_clone = Map::new(grid);
    let mut regions = vec![];

    // We will manually iterate over the positions without holding a borrow on the map.
    let positions: Vec<(usize, usize)> = map_clone
        .cells
        .indexed_iter()
        .filter_map(|(pos, &cell)| if cell.is_some() { Some(pos) } else { None })
        .collect();

    for pos in positions {
        // Only carve regions for cells that are still present.
        if map_clone.cells.get(pos.0, pos.1).and_then(|&c| c).is_some() {
            if let Some(region) = carve_contiguous_region(&mut map_clone, pos) {
                regions.push(region);
            }
        }
    }

    regions
}

fn carve_contiguous_region(map: &mut Map<Option<char>>, pos: (usize, usize)) -> Option<Map<bool>> {
    let mut region = Map::new(Grid::new(map.cells.rows(), map.cells.cols()));
    let cell_type = (*map.cells.get(pos.0, pos.1)?)?; // Clone to avoid borrowing issues
    let mut positions_to_check = VecDeque::from(vec![pos]);

    while let Some(pos) = positions_to_check.pop_front() {
        // Clone the cell's content to avoid borrowing issues
        let cell = match map.cells.get(pos.0, pos.1).cloned() {
            Some(Some(t)) => t,
            _ => continue,
        };

        if cell != cell_type {
            continue;
        }

        // We found a new cell. Add it to the region.
        if let Some(region_cell) = region.cells.get_mut(pos.0, pos.1) {
            *region_cell = true;
        }

        // Now remove it from the map.
        if let Some(map_cell) = map.cells.get_mut(pos.0, pos.1) {
            *map_cell = None;
        }

        // Add adjacent positions to check.
        for (adjacent, _) in map.get_cardinal_cells(&pos) {
            positions_to_check.push_back(adjacent);
        }
    }

    Some(region)
}

fn calculate_perimeter(region: &Map<bool>) -> Output {
    let mut perimeter = 0;
    for (pos, cell) in region.cells.indexed_iter() {
        for dir in Direction::cardinals() {
            let adjacent_cell = region.get_relative_cell(&pos, dir);
            let is_border = match adjacent_cell {
                Some((_, &other_cell)) => *cell && !other_cell, // Cell is true AND other is false
                None => *cell, // Cell is true and adjacent is border of grid
            };
            if is_border {
                perimeter += 1;
            }
        }
    }
    perimeter
}

fn calculate_area(region: &Map<bool>) -> Output {
    region.cells.iter().filter(|&&e| e).count() as Output
}

fn solve_part_two(map: ProcessedInput) -> Result<Output> {
    let regions = split_map_into_regions(&map);
    let mut count = 0;
    for region in regions {
        let area = calculate_area(&region);
        let sides = calculate_sides(region.clone());
        let score = area * sides;
        count += score;
    }
    Ok(count)
}

fn calculate_sides(mut region: Map<bool>) -> Output {
    // Given a region, find the number of sides.
    // We could scan horizontally, check the number of gaps. Then do the same vertically.
    let mut pad_col = Vec::new();
    pad_col.resize(region.cells.rows(), false);
    region.cells.push_col(pad_col);
    let mut pad_row = Vec::new();
    pad_row.resize(region.cells.cols(), false);
    region.cells.push_row(pad_row);

    let mut sides = 0;
    sides += scan_sides(&region);
    region.cells.rotate_right(); // Rotate grid clockwise lol
    sides += scan_sides(&region);
    sides as Output
}

fn scan_sides(region: &Map<bool>) -> Output {
    let mut sides = 0;
    for (row_num, row_iter) in region.cells.iter_rows().enumerate() {
        // We start a row. We are looking above in this instance.
        let mut currently_tracing_side = false;
        let mut previous_cell = false;
        for (col_num, &cell) in row_iter.enumerate() {
            let above_cell = region.get_relative_cell(&(row_num, col_num), Direction::North);
            let is_edge = match above_cell {
                Some((_, &t)) => {
                    // If the above cell is something, then we are tracing a side is the current cell is nothing.
                    // If the above cell is nothing, then we are tracing a side if the current cell is something.
                    cell ^ t
                }
                None => {
                    // If the current cell is true, and above us is an edge, then we should definitely be tracking a side.
                    cell
                }
            };
            // Edge case: Check that the previous cell is the same as this cell.
            let inverted_edge = previous_cell ^ cell;
            // If the edge was inverted, we should indicate that we are no longer tracing a side.
            if inverted_edge && is_edge && currently_tracing_side {
                sides += 1;
            }

            // We now know if we found an edge.
            if currently_tracing_side && !is_edge {
                // We were tracing an edge and it has disappeared.
                currently_tracing_side = false;
            } else if !currently_tracing_side && is_edge {
                //We found a new edge.
                currently_tracing_side = true;
                sides += 1;
            }
            previous_cell = cell;
        }
    }
    sides
}
