use advent_lib::runner::{Day, PartId};
use anyhow::Result;
use fnv::{FnvBuildHasher, FnvHashMap};

type P1Out = usize;
type P2Out = usize;
type Parsed = Vec<usize>;

// Solves part 1 in 41µs and prat 2 in 1.8sec on a Ryzen 9 3900X, which is awful.
// A more correct solution would detect the cycle early on and exit at that point.
#[allow(clippy::ptr_arg)]
fn solve_naive(seed: &Vec<usize>, iterations: usize) -> usize {
  let mut spoken: FnvHashMap<usize, (usize, usize)> =
    FnvHashMap::with_capacity_and_hasher(1_000, FnvBuildHasher::default());
  let mut latest = *seed.iter().last().unwrap();
  for (i, s) in seed.iter().enumerate() {
    spoken.insert(*s, (0, i + 1));
    latest = *s;
  }
  for iter in seed.len()..iterations {
    latest = if let Some(&(p0, p1)) = spoken.get(&latest) {
      if p0 > 0 {
        let e = spoken.entry(p1 - p0).or_insert((0, 0));
        *e = (e.1, iter + 1);
        p1 - p0
      } else {
        let e = spoken.entry(0).or_insert((0, 0));
        *e = (e.1, iter + 1);
        0
      }
    } else {
      unreachable!();
    };
  }
  latest
}

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(lines[0].split(',').map(|n| n.parse().unwrap()).collect())
  }

  fn part1(&self, seed: &Parsed, _: Option<String>) -> Result<P1Out> {
    Ok(solve_naive(seed, 2020))
  }

  fn part2(&self, seed: &Parsed, _: Option<String>) -> Result<P2Out> {
    Ok(solve_naive(seed, 30_000_000))
  }
}

fn main() -> Result<()> {
  Solver {}.run(2020, 15)
}
