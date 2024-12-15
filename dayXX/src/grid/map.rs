use std::fmt::Display;

use grid::Grid;

use super::{coordinate::Coordinate, direction::Direction};

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
            if let Some(c) = self.get_relative_cell(pos, dir) {
                cells.push(c);
            }
        }
        cells
    }

    pub fn get(&self, coordinate: Coordinate) -> Option<&T> {
        self.cells.get(coordinate.row, coordinate.col)
    }

    pub fn rows(&self) -> usize {
        self.cells.rows()
    }
    pub fn cols(&self) -> usize {
        self.cells.cols()
    }
}

impl<T: Default> Map<T> {
    pub fn clone_size(&self) -> Map<T> {
        let grid = Grid::new(self.rows(), self.cols());
        Map::new(grid)
    }
}

impl<'a, T> IntoIterator for &'a Map<T> {
    type Item = (Coordinate, &'a T);
    type IntoIter = std::iter::Map<
        std::vec::IntoIter<((usize, usize), &'a T)>,
        fn(((usize, usize), &'a T)) -> (Coordinate, &'a T),
    >;

    fn into_iter(self) -> Self::IntoIter {
        fn map_cell<T>(((r, c), t): ((usize, usize), &T)) -> (Coordinate, &T) {
            (Coordinate::new(r, c), t)
        }
        self.cells
            .indexed_iter()
            .collect::<Vec<_>>()
            .into_iter()
            .map(map_cell)
    }
}
