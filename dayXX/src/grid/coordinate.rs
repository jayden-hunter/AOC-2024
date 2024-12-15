pub struct Coordinate {
    pub row: usize,
    pub col: usize,
}

impl Coordinate {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub fn checked_add_signed(&self, delta_row: isize, delta_col: isize) -> Option<Coordinate> {
        let row = self.row.checked_add_signed(delta_row)?;
        let col = self.col.checked_add_signed(delta_col)?;
        Some(Coordinate::new(row, col))
    }

    pub fn checked_add(&self, delta_row: usize, delta_col: usize) -> Option<Coordinate> {
        let row = self.row.checked_add(delta_row)?;
        let col = self.col.checked_add(delta_col)?;
        Some(Coordinate::new(row, col))
    }
}
