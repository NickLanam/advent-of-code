use advent_lib::runner::{Day, PartId};
use anyhow::Result;
use fnv::{FnvBuildHasher, FnvHashMap};

type P1Out = usize;
type P2Out = usize;
type Parsed = FnvHashMap<(i64, i64, i64), u64>;

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    let mut out = FnvHashMap::with_capacity_and_hasher(lines.len(), FnvBuildHasher::default());
    for line in lines {
      let (left, r) = line[5..].split_once(">, r=").expect("Bad line");
      let (x, yz) = left.split_once(",").unwrap();
      let (y, z) = yz.split_once(",").unwrap();
      out.insert((x.parse()?, y.parse()?, z.parse()?), r.parse()?);
    }
    Ok(out)
  }

  fn part1(&self, bots: &Parsed, _sample_name: Option<String>) -> Result<P1Out> {
    let mut best_strength: usize = 0;
    for ((x, y, z), r) in bots.iter() {
      let mut strength = 0;
      for ((x2, y2, z2), _r2) in bots.iter() {
        if x.abs_diff(*x2) + y.abs_diff(*y2) + z.abs_diff(*z2) <= *r {
          strength += 1;
        }
      }
      best_strength = best_strength.max(strength);
    }
    Ok(best_strength)
  }

  fn part2(&self, _lines: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    // TODO: Brute forcing this would be catastrophically slow.
    // Bust out the old algorithms textbook.
    // Likely using an octree (splitting a box up) will help substantially.
    // It also looks like many of the bots areas are nested within each other,
    // maybe I can take advantage of that? Some of these puzzles have inputs
    // that restrict a complex case to a simpler one that can be solved efficiently...

    // REMEMBER: The goal is thus...
    // - Find positions which are in range of the maximum number of bots
    // - Among those points, return the lowest manhattan distance from there to 0,0,0
    Ok(0)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2018, 23)
}
