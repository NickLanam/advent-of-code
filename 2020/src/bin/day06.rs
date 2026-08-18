use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = usize;
type P2Out = usize;
type Parsed = Vec<Vec<Vec<u8>>>;

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(
      lines
        .split(|line| line.is_empty())
        .map(|group| {
          group
            .iter()
            // Converting to integers before solving speeds up solving considerably.
            // Notably, storing them as u8 makes both parse and solve faster than storing as usize,
            // even though they get converted to usize in the hot path for indexing during solving.
            // I suspect the compiler does an optimization to skip the cast when that happens,
            // because the array is small enough to get away with it.
            .map(|line| line.chars().map(|c| (c as u8) - 97).collect())
            .collect()
        })
        .collect(),
    )
  }

  fn part1(&self, groups: &Parsed, _: Option<String>) -> Result<P1Out> {
    let mut sum = 0;
    for group in groups.iter() {
      let mut yesses = [false; 26];
      for line in group.iter() {
        for &i in line.iter() {
          yesses[i as usize] = true;
        }
      }
      // We could do this inline in the loop above, but that makes for slow branch predictions.
      // Faster to do it as a separate loop, once, down here.
      for &y in yesses.iter() {
        if y {
          sum += 1;
        }
      }
    }
    Ok(sum)
  }

  fn part2(&self, groups: &Parsed, _: Option<String>) -> Result<P2Out> {
    // Nearly the same code, same optimizations, but keeping track of how many yesses per question per group.
    let mut sum = 0;
    for group in groups.iter() {
      let mut yesses = [0_usize; 26];
      for line in group.iter() {
        for &i in line.iter() {
          yesses[i as usize] += 1;
        }
      }
      for &y in yesses.iter() {
        if y >= group.len() {
          sum += 1;
        }
      }
    }
    Ok(sum)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2020, 6)
}
