use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = usize;
type P2Out = usize;
type Parsed = Vec<i64>;

const ROUNDS: usize = 100;

fn round(line: &[i64]) -> Vec<i64> {
  let len = line.len();
  let mid = len / 2;
  let mut out = vec![0; len];
  let mut sums = vec![0; len];
  let mut working_sum = 0;

  for i in (0..len).rev() {
    working_sum += line[i];
    sums[i] = working_sum;
    if i >= mid {
      out[i] = working_sum.abs() % 10
    };
  }

  for i in 0..mid {
    out[i] = (i..len)
      .step_by((i + 1) << 1)
      .enumerate()
      .fold(0, |sum, (idx, start)| {
        let mut sum_of_range = sums[start];
        if start + i + 1 < len {
          sum_of_range -= sums[start + i + 1];
        }
        sum
          + if idx & 1 == 0 {
            sum_of_range
          } else {
            -sum_of_range
          }
      })
      .abs()
      % 10;
  }

  out
}

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(
      lines[0]
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect(),
    )
  }

  fn part1(&self, init: &Parsed, _: Option<String>) -> Result<P1Out> {
    let mut line = init.clone();
    for _round in 1..=ROUNDS {
      line = round(&line);
    }

    Ok(
      line
        .iter()
        .take(8)
        .fold(0, |acc, n| (acc * 10) + ((n.abs() % 10) as usize)),
    )
  }

  fn part2(&self, init: &Parsed, _: Option<String>) -> Result<P2Out> {
    // There are much faster ways to do this which don't have to actually stretch the array,
    // by noticing the pattern in triangular->tetrahedral numbers and only ever computing
    // the 8 digits we're using (no need to calculate the rest)... but this is the first
    // method that worked for me before looking up solutions so I'm committing it.
    let offset = init[0..7].iter().fold(0, |acc, n| (acc * 10) + n) as usize;
    let mut line = init
      .iter()
      .cycle()
      .take(init.len() * 10_000)
      .map(|&n| n)
      .collect::<Vec<i64>>();
    for _round in 1..=ROUNDS {
      line = round(&line);
    }
    Ok(
      line
        .iter()
        .skip(offset)
        .take(8)
        .fold(0, |acc, n| (acc * 10) + ((n.abs() % 10) as usize)),
    )
  }
}

fn main() -> Result<()> {
  Solver {}.run(2019, 16)
}
