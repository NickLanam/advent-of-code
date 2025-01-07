use advent_lib::runner::{Day, PartId};
use anyhow::Result;
use crypto::digest::Digest;
use crypto::md5::Md5;

type Parsed = String;

fn solve(input: String, start_pattern: String) -> u64 {
  let mut hasher = Md5::new();
  for i in 0_u64..u64::MAX {
    hasher.reset();
    hasher.input_str(&format!("{}{}", input.clone(), i));
    let digest = hasher.result_str();
    if digest.starts_with(&start_pattern) {
      return i;
    }
  }
  0
}

struct Solver {}
impl Day<Parsed, u64, u64> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    Ok(lines[0].to_string())
  }

  fn part1(&self, input: &Parsed, _sample_name: Option<String>) -> Result<u64> {
    Ok(solve(input.to_string(), "00000".to_string()))
  }

  fn part2(&self, input: &Parsed, _sample_name: Option<String>) -> Result<u64> {
    Ok(solve(input.to_string(), "000000".to_string()))
  }
}

fn main() -> Result<()> {
  Solver {}.run(2015, 4)
}
