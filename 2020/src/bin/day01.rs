use advent_lib::runner::{Day, PartId};
use anyhow::{Result, bail};

type P1Out = usize;
type P2Out = usize;
type Parsed = Vec<usize>;

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(lines.iter().map(|line| line.parse().unwrap()).collect())
  }

  fn part1(&self, nums: &Parsed, _sample_name: Option<String>) -> Result<P1Out> {
    for i in 0..(nums.len() - 1) {
      for j in (i + 1)..nums.len() {
        if nums[i] + nums[j] == 2020 {
          return Ok(nums[i] * nums[j]);
        }
      }
    }
    bail!("No answer found");
  }

  fn part2(&self, nums: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    for i in 0..(nums.len() - 2) {
      for j in (i + 1)..(nums.len() - 1) {
        for k in (j + 1)..nums.len() {
          if nums[i] + nums[j] + nums[k] == 2020 {
            return Ok(nums[i] * nums[j] * nums[k]);
          }
        }
      }
    }
    bail!("No answer found");
  }
}

fn main() -> Result<()> {
  Solver {}.run(2020, 1)
}
