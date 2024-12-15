use std::fmt::Display;
use std::fmt::Write;

use anyhow::anyhow;
use anyhow::bail;
use anyhow::Result;

use crate::grid::map::Map;
pub type ProcessedInput = Map<SCell>;

type Visited = bool;
#[derive(Clone, Default)]
pub enum SCell {
    Empty(Visited),
    #[default]
    Wall,
    Player,
}

impl Display for SCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            SCell::Empty(false) => '.',
            SCell::Empty(true) => 'O',
            SCell::Wall => '#',
            SCell::Player => 'X',
        };
        f.write_char(c)
    }
}

pub fn process_input(input: String) -> Result<ProcessedInput> {
    let lines = input
        .lines()
        .filter(|x| !x.is_empty())
        .collect::<Vec<&str>>()
        .join("\n");
    Map::from_str(&lines, str_to_cell)
}

fn str_to_cell(c: char) -> Result<SCell> {
    let cell = match c {
        '#' => SCell::Wall,
        '.' => SCell::Empty(false),
        'X' => SCell::Player,
        other => bail!("Invalid Map Character {other}"),
    };
    Ok(cell)
}

type NumMoves = u32;
pub fn solve_part_one(mut data: ProcessedInput) -> Result<NumMoves> {
    println!("{}", data);
    let end_states = get_next_possibilities(data);
    end_states
        .iter()
        .filter(|(m, _)| is_completely_filled(m))
        .map(|(_, a)| a)
        .max()
        .ok_or(anyhow!("No solutions possible"))
        .copied()
}

fn is_completely_filled(m: &Map<SCell>) -> bool {
    for (_, cell) in m {
        let valid = matches!(cell, SCell::Empty(true) | SCell::Player);
        if !valid {
            return false;
        }
    }
    true
}

fn get_next_possibilities(data: Map<SCell>) -> Vec<(Map<SCell>, NumMoves)> {
    //Given a map, make valid moves.
    let player_coord = 

}

pub fn solve_part_two(mut data: ProcessedInput) -> Result<()> {
    bail!("Unimplemented");
}
