use anyhow::Result;
use aoc_2022::load_data_file;
use itertools::Itertools;

fn main() -> Result<()> {
    let data = load_data_file()?;

    let score = data
        .lines()
        .map(|x| x.split(' ').collect_tuple::<(&str, &str)>().unwrap())
        .map(|(l, r)| score(parse(r), parse(l)))
        .sum::<u64>();

    println!("{score}");
    return Ok(());
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum RPS {
    R,
    P,
    S,
}

fn parse(s: &str) -> RPS {
    use RPS::*;
    match s {
        "A" | "X" => R,
        "B" | "Y" => P,
        "C" | "Z" => S,
        _ => panic!(),
    }
}

fn score(l: RPS, r: RPS) -> u64 {
    use RPS::*;
    let a = match (l, r) {
        (P, R) | (S, P) | (R, S) => 6,
        (a, b) if a == b => 3,
        _ => 0,
    };
    let b = match l {
        R => 1,
        P => 2,
        S => 3,
    };
    println!("{l:?} {r:?} -> {a} {b} {}", a + b);
    a + b
}
