use std::fmt::Display;

use grid::Grid;

#[derive(Clone, Debug)]
pub struct Map<T> {
    pub cells: Grid<T>,
}

impl<T: Display> Display for Map<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        for row in self.cells.iter_rows() {
            for col in row.into_iter() {
                let digit_str = &col.to_string();
                string.push_str(digit_str);
            }
            string.push('\n');
        }
        string = string[0..string.len() - 1].to_string();
        f.write_str(&string)
    }
}

pub enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl Direction {
    pub fn cardinals() -> [Direction; 4] {
        [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
    }

    pub fn principles() -> [Direction; 8] {
        [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
            Direction::NorthEast,
            Direction::NorthWest,
            Direction::SouthEast,
            Direction::SouthWest,
        ]
    }

    /// Given a Direction, will return a tuple of the direction.
    /// Note that North returns (-1, 0)
    pub fn to_delta(&self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
            Direction::NorthEast => (-1, 1),
            Direction::NorthWest => (-1, -1),
            Direction::SouthEast => (1, 1),
            Direction::SouthWest => (1, -1),
        }
    }
}

impl<T> Map<T> {
    pub fn new(cells: Grid<T>) -> Self {
        Self { cells }
    }

    pub fn get_relative_cell(
        &self,
        cell: &(usize, usize),
        direction: Direction,
    ) -> Option<((usize, usize), &T)> {
        let (delta_row, delta_col) = direction.to_delta();
        let new_row = cell.0.checked_add_signed(delta_row)?;
        let new_col: usize = cell.1.checked_add_signed(delta_col)?;
        let new_pos = (new_row, new_col);
        let cell = self.cells.get(new_row, new_col)?;
        Some((new_pos, cell))
    }

    pub fn get_cardinal_cells(&self, pos: &(usize, usize)) -> Vec<((usize, usize), &T)> {
        let mut cells = vec![];
        for dir in Direction::cardinals() {
            if let Some(c) = self.get_relative_cell(&pos, dir) {
                cells.push(c);
            }
        }
        cells
    }
}
