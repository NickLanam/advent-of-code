use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = u32;
type P2Out = u32;
type Parsed = Vec<u32>;

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    Ok(lines[0].chars().map(|c| c.to_digit(10).unwrap()).collect())
  }

  fn part1(&self, digits: &Parsed, _sample_name: Option<String>) -> Result<P1Out> {
    let mut sum = 0;
    for i in 0..digits.len() {
      if digits[i] == digits[(i + 1) % digits.len()] {
        sum += digits[i];
      }
    }
    Ok(sum)
  }

  fn part2(&self, digits: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    let mut sum = 0;
    for i in 0..digits.len() {
      if digits[i] == digits[(i + (digits.len() / 2)) % digits.len()] {
        sum += digits[i];
      }
    }
    Ok(sum)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2017, 1)
}
