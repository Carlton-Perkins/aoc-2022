use anyhow::Result;
use aoc_2022::load_data_file;
use itertools::Itertools;

fn main() -> Result<()> {
    let data = load_data_file()?;

    let score = data
        .lines()
        .map(|x| x.split(' ').collect_tuple::<(&str, &str)>().unwrap())
        .map(|(l, r)| {
            let (other, outcome) = (parse_rps(l), parse_outcome(r));
            score(solve(other, outcome), other)
        })
        .sum::<u64>();

    println!("{score}");
    return Ok(());
}

fn solve(other: RPS, outcome: Outcome) -> RPS {
    use Outcome::*;
    use RPS::*;
    match (other, outcome) {
        (R, W) => P,
        (R, L) => S,
        (x, T) => x,
        (P, W) => S,
        (P, L) => R,
        (S, W) => R,
        (S, L) => P,
        _ => panic!(),
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum RPS {
    R,
    P,
    S,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Outcome {
    W,
    L,
    T,
}

fn parse_rps(s: &str) -> RPS {
    use RPS::*;
    match s {
        "A" => R,
        "B" => P,
        "C" => S,
        _ => panic!("{}", s),
    }
}
fn parse_outcome(s: &str) -> Outcome {
    use Outcome::*;
    match s {
        "X" => L,
        "Y" => T,
        "Z" => W,
        _ => panic!("{}", s),
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
