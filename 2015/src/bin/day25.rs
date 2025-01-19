use advent_lib::runner::{Day, PartId};
use anyhow::Result;
use regex::Regex;

type P1Out = u64;
type P2Out = u64;
type Parsed = (u64, u64);

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    let re = Regex::new(r"Enter the code at row ([\d]+), column ([\d]+).$").unwrap();
    let caps = re.captures(lines[0].as_str()).unwrap();
    let row = caps[1].parse().unwrap();
    let column = caps[2].parse().unwrap();
    Ok((row, column))
  }

  fn part1(&self, (row, column): &Parsed, _sample_name: Option<String>) -> Result<P1Out> {
    let n = row + column - 1;
    let t = (n * (n + 1)) / 2;
    let num_iterations = t - (row - 1) - 1;
    let mut val: u64 = 20_151_125;
    for _ in 0..num_iterations {
      val = (val * 252_533) % 33_554_393;
    }
    Ok(val)
  }

  fn part2(&self, _lines: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    // On day 25, there is no part 2. This star is granted by earning the other 49.
    Ok(0)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2015, 25)
}
