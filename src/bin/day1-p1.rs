use anyhow::Result;
use aoc_2022::load_data_file;
use itertools::Itertools;

fn main() -> Result<()> {
    let data = load_data_file()?;

    let max = data
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|line| line.parse::<u64>().unwrap())
                .sum::<u64>()
        })
        .max()
        .unwrap();

    println!("{max}");
    return Ok(());
}
