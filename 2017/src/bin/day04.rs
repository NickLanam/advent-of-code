use advent_lib::runner::{Day, PartId};
use anyhow::Result;
use fnv::{FnvBuildHasher, FnvHashSet};

type P1Out = u64;
type P2Out = u64;
type Parsed = Vec<Vec<String>>;

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    Ok(
      lines
        .iter()
        .map(|line| line.split_whitespace().map(|s| s.to_string()).collect())
        .collect(),
    )
  }

  fn part1(&self, lines: &Parsed, _sample_name: Option<String>) -> Result<P1Out> {
    let mut valid = 0;
    'lines_test: for line in lines {
      let mut seen =
        FnvHashSet::<String>::with_capacity_and_hasher(line.len(), FnvBuildHasher::default());
      for word in line {
        if seen.contains(word) {
          continue 'lines_test;
        } else {
          seen.insert(word.to_owned());
        }
      }
      valid += 1;
    }
    Ok(valid)
  }

  fn part2(&self, lines: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    let mut valid = 0;
    'lines_test: for line in lines {
      let mut seen =
        FnvHashSet::<String>::with_capacity_and_hasher(line.len(), FnvBuildHasher::default());
      for word in line {
        let mut letters = word.chars().collect::<Vec<char>>();
        letters.sort();
        let sorted = letters.iter().collect::<String>();
        if seen.contains(&sorted) {
          continue 'lines_test;
        } else {
          seen.insert(sorted);
        }
      }
      valid += 1;
    }
    Ok(valid)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2017, 4)
}
