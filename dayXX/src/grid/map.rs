use anyhow::{anyhow, Result};
use grid::Grid;
use std::fmt::Display;

use super::{
    coordinate::{self, Coordinate},
    direction::Direction,
};

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
        cell: &Coordinate,
        direction: Direction,
    ) -> Option<(Coordinate, &T)> {
        let (delta_row, delta_col) = direction.to_delta();
        let new_pos = cell.checked_add_signed(delta_row, delta_col)?;
        let cell = self.get(&new_pos)?;
        Some((new_pos, cell))
    }

    pub fn get_cardinal_cells(&self, pos: &Coordinate) -> Vec<(Coordinate, &T)> {
        let mut cells = vec![];
        for dir in Direction::cardinals() {
            if let Some(c) = self.get_relative_cell(pos, dir) {
                cells.push(c);
            }
        }
        cells
    }

    pub fn get(&self, coordinate: &Coordinate) -> Option<&T> {
        self.cells.get(coordinate.row, coordinate.col)
    }

    pub fn get_mut(&mut self, coordinate: &Coordinate) -> Option<&mut T> {
        self.cells.get_mut(coordinate.row, coordinate.col)
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

    pub fn from_str_with_coords(
        input: &str,
        cell_fn: impl Fn(char, Coordinate) -> Result<T>,
    ) -> Result<Self> {
        let iter = input.lines();
        let width = iter
            .clone()
            .next()
            .ok_or(anyhow!("Input must contain at least one line"))?
            .len();
        let height = iter.clone().count();
        let mut cells = Grid::new(width, height);
        for (row, line) in iter.enumerate() {
            for (col, c) in line.chars().enumerate() {
                let coord = Coordinate::new(row, col);
                *cells.get_mut(row, col).unwrap() = cell_fn(c, coord)?;
            }
        }
        Ok(Self::new(cells))
    }

    pub fn from_str(input: &str, cell_fn: impl Fn(char) -> Result<T>) -> Result<Self> {
        let useless = |c, _: Coordinate| cell_fn(c);
        Map::from_str_with_coords(input, useless)
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
