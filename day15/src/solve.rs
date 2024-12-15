use anyhow::bail;
use anyhow::Result;

pub type ProcessedInput = ();

pub fn process_input(input: String) -> Result<ProcessedInput> {
    let lines = input.lines().filter(|x| !x.is_empty());
    bail!("Unimplemented");
}

pub fn solve_part_one(mut map: ProcessedInput) -> Result<()> {
    bail!("Unimplemented");
}

pub fn solve_part_two(mut map: ProcessedInput) -> Result<()> {
    bail!("Unimplemented");
}
