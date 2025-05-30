use advent_lib::runner::{Day, PartId};
use anyhow::Result;
use fnv::{FnvBuildHasher, FnvHashMap};

type P1Out = usize;
type P2Out = i64;
type Parsed = FnvHashMap<(i64, i64, i64), i64>;

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
        if x.abs_diff(*x2) + y.abs_diff(*y2) + z.abs_diff(*z2) <= (*r as u64) {
          strength += 1;
        }
      }
      best_strength = best_strength.max(strength);
    }
    Ok(best_strength)
  }

  fn part2(&self, bots: &Parsed, sample_name: Option<String>) -> Result<P2Out> {
    // Massive assumption: we can reduce the solution from three dimensions to
    // one, because the structure of the input and the use of manhattan distance
    // happens to make it work out such that we can just walk through a sorted
    // sequence of
    println!("\nRUN: {sample_name:?}");

    let mut segments: Vec<(i64, i64)> = vec![];
    for (&(x, y, z), &r) in bots.iter() {
      let d = x.abs() + y.abs() + z.abs();
      segments.push((0.max(d - r), 1));
      segments.push((d + r + 1, -1));
    }
    segments.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    let mut count = 0;
    let mut max_count = 0;
    let mut result = 0;
    for (d, e) in segments {
      if sample_name.is_some() {
        println!("Popped {d}, {e}");
      }
      count += e;
      if count > max_count {
        result = d;
        max_count = count;
      }
      if sample_name.is_some() {
        println!("  result={result}, max_count={max_count}, count={count}");
      }
    }
    // 84_087_794 is too low, so that isn't it...
    // Yet, getting the correct 36 for the given sample.
    // Implication: reducing to one dimension isn't quite right after all?
    println!("FINAL  result={result}, max_count={max_count}, count={count}");
    Ok(result)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2018, 23)
}
