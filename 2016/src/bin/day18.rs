use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = usize;
type P2Out = usize;
type Parsed = Vec<bool>;

/// Solves part 1 in under 300ns, and part 2 in 1.4ms, on a Ryzen 9 3900X.
fn solve(initial: &[bool], steps: usize) -> Result<usize> {
  // Using a u128 works because the input is less than 128 columns wide.
  // If it were wider than that, we'd likely just track more than one u128
  // at the same time to mostly keep the performance of using bitwise ops.
  let w = initial.len() as u32;
  let mask = u128::MAX >> (128 - w);
  let mut row: u128 = initial
    .iter()
    .fold(0, |a, b| if *b { (a << 1) + 1 } else { a << 1 });
  let mut total: u32 = w - row.count_ones();
  for _ in 0..steps {
    row = ((row << 1) ^ (row >> 1)) & mask;
    total += w - row.count_ones();
  }
  Ok(total as usize)
}

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    Ok(lines[0].chars().map(|c| c == '^').collect())
  }

  fn part1(&self, initial: &Parsed, sample_name: Option<String>) -> Result<P1Out> {
    let steps: usize = if sample_name.is_some() { 10 } else { 40 } - 1;
    solve(initial, steps)
  }

  fn part2(&self, initial: &Parsed, sample_name: Option<String>) -> Result<P2Out> {
    let steps: usize = if sample_name.is_some() { 10 } else { 400_000 } - 1;
    solve(initial, steps)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2016, 18)
}
