use std::fmt::Display;
use std::fmt::Write;
use std::io::Read;

use anyhow::anyhow;
use anyhow::bail;
use anyhow::Result;
use grid::Grid;

use crate::grid::coordinate;
use crate::grid::coordinate::Coordinate;
use crate::grid::direction::Direction;
use crate::grid::map::Map;
pub type ProcessedInput = (Warehouse, Vec<Direction>);

#[derive(Clone)]
pub struct Warehouse {
    map: Map<WCell>,
    robot_pos: Coordinate,
}

impl Warehouse {
    pub fn new(map: Map<WCell>) -> Result<Self> {
        let mut robot_pos = None;
        for (pos, cell) in &map {
            if matches!(cell, WCell::Robot) {
                robot_pos = Some(pos)
            }
        }
        let robot_pos = robot_pos.ok_or(anyhow!("No Robot Found in Map"))?;

        Ok(Self { map, robot_pos })
    }
}

#[derive(Debug, Default, Clone)]
enum WCell {
    Wall,
    #[default]
    Empty,
    Box(BoxType),
    Robot,
}

#[derive(Debug, Default, Clone)]

enum BoxType {
    #[default]
    Normal,
    Left,
    Right,
}

impl Display for WCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            WCell::Wall => '#',
            WCell::Empty => '.',
            WCell::Box(BoxType::Normal) => 'O',
            WCell::Box(BoxType::Left) => '[',
            WCell::Box(BoxType::Right) => ']',
            WCell::Robot => '@',
        };
        f.write_char(c)
    }
}

pub fn process_input(input: String) -> Result<ProcessedInput> {
    let mut input_iter = input.lines();
    let mut string = String::new();
    for map_line in &mut input_iter {
        if map_line.is_empty() {
            break;
        }
        string.push_str(map_line);
        string.push('\n');
    }
    let map = Map::from_str(&string, cell_fn)?;
    let warehouse = Warehouse::new(map)?;
    let mut direction = vec![];
    for direction_str in input_iter {
        for c in direction_str.chars() {
            let res = match c {
                '^' => Direction::North,
                '>' => Direction::East,
                'v' => Direction::South,
                '<' => Direction::West,
                other => bail!("Invalid Direction Code {other}"),
            };
            direction.push(res);
        }
    }
    Ok((warehouse, direction))
}

fn cell_fn(c: char) -> Result<WCell> {
    let res = match c {
        '#' => WCell::Wall,
        '.' => WCell::Empty,
        'O' => WCell::Box(BoxType::Normal),
        '@' => WCell::Robot,
        other => bail!("Invalid Char: {other}"),
    };
    Ok(res)
}

fn tick(warehouse: Warehouse, dir: Direction) -> Warehouse {
    // println!("Ticking");
    let mut clone = warehouse.clone();
    let original_pos = warehouse.robot_pos;
    let move_blocked = check_move(&clone, original_pos.clone(), &dir);
    if !move_blocked {
        make_move(&mut clone, original_pos.clone(), &dir);
        clone.robot_pos = original_pos.translate(dir).unwrap();
    }
    clone
}

/// Assumes that a cell is a valid movement option! Will Overwrite the movement Square!
/// Will Panic is assumption is violated. Call `check_move` first.
fn make_move(warehouse: &mut Warehouse, pos: Coordinate, dir: &Direction) {
    let curr_cell = warehouse.map.get(&pos).unwrap().clone();
    let (new_pos, new_cell) = warehouse.map.get_relative_cell(&pos, dir).unwrap();
    if let WCell::Box(l) = new_cell.clone() {
        move_box(warehouse, new_pos.clone(), dir, &l);
    }
    let new_cell = warehouse.map.get_mut(&new_pos).unwrap();
    *new_cell = curr_cell;
    let old_cell = warehouse.map.get_mut(&pos).unwrap();
    *old_cell = WCell::Empty;
}

fn move_box(warehouse: &mut Warehouse, pos: Coordinate, dir: &Direction, l: &BoxType) {
    if matches!(dir, Direction::East | Direction::West) {
        return make_move(warehouse, pos, dir);
    }
    let other_pos = match l {
        BoxType::Normal => return make_move(warehouse, pos, dir),
        BoxType::Left => pos.translate(Direction::East),
        BoxType::Right => pos.translate(Direction::West),
    }
    .unwrap();
    make_move(warehouse, pos, dir);
    make_move(warehouse, other_pos, dir);
}

fn check_move(warehouse: &Warehouse, pos: Coordinate, dir: &Direction) -> bool {
    // println!("Checking move: {:?} in dir: {:?}", pos, dir);
    let move_cell = warehouse.map.get_relative_cell(&pos, dir);
    let (new_pos, new_cell) = match move_cell {
        Some(t) => t,
        None => return false,
    };
    // println!("Move cell: {:#?}", move_cell);
    // There is something in the new cell. Check whether it would be blocking.
    match new_cell.clone() {
        WCell::Wall => true,
        WCell::Empty => false,
        WCell::Box(l) => check_box_move(warehouse, new_pos.clone(), dir, l),
        WCell::Robot => true,
    }
}

fn check_box_move(warehouse: &Warehouse, pos: Coordinate, dir: &Direction, l: BoxType) -> bool {
    // Given a wide box, handle its movement.
    // First, if we are coming from the east or west, treat this as a regular move.
    // println!("Checking box, {:?} at pos {:?}", dir, pos);
    if matches!(dir, Direction::East | Direction::West) {
        return check_move(warehouse, pos, dir);
    }
    // We now know that we are moving either North or South, and that this will recurse.
    // Find the other position
    let other_pos = match l {
        BoxType::Normal => return check_move(warehouse, pos, dir),
        BoxType::Left => pos.translate(Direction::East),
        BoxType::Right => pos.translate(Direction::West),
    }
    .unwrap();
    let a = check_move(warehouse, pos.clone(), dir);
    let b = check_move(warehouse, other_pos, dir);
    a || b // Both must be unobstructed
}

fn calculate_gps(warehouse: Warehouse) -> usize {
    let mut sum = 0;
    for (pos, cell) in &warehouse.map {
        if !matches!(cell, WCell::Box(BoxType::Left | BoxType::Normal)) {
            continue;
        }
        sum += (100 * pos.row) + pos.col;
    }
    sum
}

pub fn solve_part_one(mut data: ProcessedInput) -> Result<usize> {
    let mut warehouse = data.0;
    let directions = data.1;
    for dir in directions {
        // println!("R: {:?}, M:{}", warehouse.robot_pos, warehouse.map);
        // std::io::stdin().read_exact(&mut [1; 1]);
        warehouse = tick(warehouse, dir);
    }
    println!("Result: {}", warehouse.map);
    Ok(calculate_gps(warehouse))
}

pub fn solve_part_two(mut data: ProcessedInput) -> Result<usize> {
    let mut warehouse = widen_warehouse(data.0)?;
    let directions = data.1;
    for dir in directions {
        // println!("{}", warehouse.map);
        // std::io::stdin().read_exact(&mut [1; 1]);
        warehouse = tick(warehouse, dir);
    }
    println!("Result: {}", warehouse.map);
    Ok(calculate_gps(warehouse))
}

fn widen_warehouse(warehouse: Warehouse) -> Result<Warehouse> {
    let mut wide_cells = Grid::new(warehouse.map.rows(), warehouse.map.cols() * 2);
    for (coordinate, cell) in &warehouse.map {
        let wide_col = coordinate.col * 2;
        let (new_left, new_right) = match cell {
            WCell::Wall => (WCell::Wall, WCell::Wall),
            WCell::Empty => (WCell::Empty, WCell::Empty),
            WCell::Box(BoxType::Normal) => (WCell::Box(BoxType::Left), WCell::Box(BoxType::Right)),
            WCell::Box(other) => bail!("Cannot expand {:?}", other),
            WCell::Robot => (WCell::Robot, WCell::Empty),
        };
        let left_cell = wide_cells
            .get_mut(coordinate.row, wide_col)
            .ok_or(anyhow!("Invalid widened access"))?;
        *left_cell = new_left;
        let right_cell = wide_cells
            .get_mut(coordinate.row, wide_col + 1)
            .ok_or(anyhow!("Invalid widened access"))?;
        *right_cell = new_right;
    }
    let map = Map::new(wide_cells);
    Warehouse::new(map)
}
