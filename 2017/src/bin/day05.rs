use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = usize;
type P2Out = usize;
type Parsed = Vec<i64>;

/// Disappointing: I could not find any trick with the puzzle
/// to avoid a brute force solution, and it seems nobody else
/// did either. 500µs part 1, 36ms part 2 on a Ryzen 9 3900X.
fn solve(initial: &[i64], part_2: bool) -> usize {
  let mut list = initial.to_owned();
  let mut steps = 0;
  let mut pc: i64 = 0;
  while let Some(o) = list.get_mut(pc as usize) {
    pc += *o;
    steps += 1;
    if part_2 && *o >= 3 {
      *o -= 1;
    } else {
      *o += 1;
    }
  }
  steps
}

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(lines.iter().map(|line| line.parse().unwrap()).collect())
  }

  fn part1(&self, positions: &Parsed, _: Option<String>) -> Result<P1Out> {
    Ok(solve(positions, false))
  }

  fn part2(&self, positions: &Parsed, _: Option<String>) -> Result<P2Out> {
    Ok(solve(positions, true))
  }
}

fn main() -> Result<()> {
  Solver {}.run(2017, 5)
}
