use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = usize;
type P2Out = usize;
type Parsed = Vec<usize>;

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(lines.iter().map(|n| n.parse().unwrap()).collect())
  }

  fn part1(&self, masses: &Parsed, _: Option<String>) -> Result<P1Out> {
    Ok(masses.iter().map(|m| (m / 3).saturating_sub(2)).sum())
  }

  fn part2(&self, masses: &Parsed, _: Option<String>) -> Result<P2Out> {
    Ok(
      masses
        .iter()
        .map(|&m| {
          let mut f = (m / 3).saturating_sub(2);
          let mut total = 0;
          while f > 0 {
            total += f;
            f = (f / 3).saturating_sub(2);
          }
          total
        })
        .sum(),
    )
  }
}

fn main() -> Result<()> {
  Solver {}.run(2019, 1)
}
