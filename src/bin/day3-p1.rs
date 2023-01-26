use std::collections::{hash_map::RandomState, HashMap, HashSet};

use anyhow::Result;
use aoc_2022::load_data_file;
use itertools::Itertools;

fn main() -> Result<()> {
    let data = load_data_file()?;

    let score: i32 = data
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(l, r)| {
            (
                HashSet::<_, RandomState>::from_iter(l.chars()),
                HashSet::<_, RandomState>::from_iter(r.chars()),
            )
        })
        .map(|(l, r)| l.intersection(&r).at_most_one().unwrap().unwrap().clone())
        .map(|c| score(&c))
        .sum();

    println!("{score}");
    return Ok(());
}
fn score(c: &char) -> i32 {
    let a = 'a'..='z';
    let b = 'A'..='Z';
    let scoring: HashMap<char, usize, RandomState> =
        HashMap::from_iter(a.chain(b).enumerate().map(|(s, c)| (c, s + 1)));

    *scoring.get(&c).unwrap() as i32
}
