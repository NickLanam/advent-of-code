use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = usize;
type P2Out = usize;
type Parsed = Vec<i64>;

const ROUNDS: usize = 100;

// Only part 1 bothers to do this; part 2 skips it!
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

fn collect_result(line: &[i64]) -> usize {
  line
    .iter()
    .take(8)
    .fold(0, |acc, n| (acc * 10) + (n.abs() % 10) as usize)
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

    Ok(collect_result(&line))
  }

  fn part2(&self, init: &Parsed, _: Option<String>) -> Result<P2Out> {
    // Two observations that take this from 60sec town to 2.5sec then to 0.150sec:
    // - The pattern works out that we're doing partial sums each round, and don't need
    //   to run the provided round algorithm at all in part 2. That gets 60sec -> 2.5sec.
    // - There's a triangle of 0s in the math, and a 7-digit offset... so we can skip
    //   computing anything at all for #offset digits (2.5sec -> 0.150sec)
    let offset = init[0..7].iter().fold(0, |acc, n| (acc * 10) + n) as usize;
    let mut line: Vec<i64> = init
      .iter()
      .cycle()
      .take(init.len() * 10_000)
      .skip(offset) // Order matters, here!
      .copied()
      .collect();

    for _round in 1..=ROUNDS {
      let mut sums = vec![0; line.len() + 1];
      let mut total = 0;
      for i in 0..line.len() {
        total += line[i];
        sums[i + 1] = total;
      }

      for i in 0..line.len() {
        let value = total - sums[i];
        line[i] = value % 10;
      }
    }
    Ok(collect_result(&line))
  }
}

fn main() -> Result<()> {
  Solver {}.run(2019, 16)
}
