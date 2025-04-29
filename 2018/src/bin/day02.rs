use advent_lib::runner::{Day, PartId};
use anyhow::{Result, bail};
use fnv::{FnvBuildHasher, FnvHashMap};

type P1Out = u64;
type P2Out = String;
type Parsed = Vec<String>;

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(lines)
  }

  fn part1(&self, lines: &Parsed, _: Option<String>) -> Result<P1Out> {
    let mut num_twos = 0;
    let mut num_threes = 0;
    for line in lines {
      let mut m: FnvHashMap<char, usize> = FnvHashMap::with_hasher(FnvBuildHasher::default());
      for c in line.chars() {
        *(m.entry(c).or_insert(0)) += 1;
      }
      if m.values().any(|v| *v == 2) {
        num_twos += 1;
      }
      if m.values().any(|v| *v == 3) {
        num_threes += 1;
      }
    }
    Ok(num_twos * num_threes)
  }

  fn part2(&self, lines: &Parsed, _: Option<String>) -> Result<P2Out> {
    // This method may look sloppy, but it runs in 350µs.
    // The more readable methods I tried also constructed the output
    // string on every comparison, which made the runtime 10-100x worse.
    for (i, a) in lines.iter().enumerate().take(lines.len() - 1) {
      'inner: for b in lines.iter().skip(i + 1) {
        let mut remove_index = usize::MAX;
        let mut b_chars = b.chars();
        for (j, c) in a.chars().enumerate() {
          let d = b_chars.next().unwrap();
          if c != d {
            if remove_index == usize::MAX {
              remove_index = j;
            } else {
              continue 'inner;
            }
          }
        }
        return Ok(
          a.chars()
            .take(remove_index)
            .chain(a.chars().skip(remove_index + 1))
            .collect(),
        );
      }
    }
    bail!("Failed to find");
  }
}

fn main() -> Result<()> {
  Solver {}.run(2018, 2)
}
