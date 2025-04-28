use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = i32;
type P2Out = i32;
type Parsed = Vec<i32>;

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(lines.iter().map(|l| l.parse().unwrap()).collect())
  }

  fn part1(&self, diffs: &Parsed, _: Option<String>) -> Result<P1Out> {
    Ok(diffs.iter().sum())
  }

  fn part2(&self, diffs: &Parsed, _: Option<String>) -> Result<P2Out> {
    // We're looking for a recurrence relation. We only need to
    // store the result of the first loop to do this.
    let mut base: Vec<i32> = Vec::with_capacity(diffs.len());

    let mut state = 0;
    for d in diffs {
      state += d;
      base.push(state);
    }

    let cycle_diff = *base.last().unwrap();

    let mut best_score = usize::MAX;
    let mut best_target = 0;

    // Check when each result would recur, and return the one
    // that would recur the soonest. Still O(n**2) and does division
    // in the inner loop, maybe there's an even faster approach?
    for (ti, &target) in base.iter().enumerate() {
      for (ci, &check) in base.iter().enumerate() {
        if ti == ci {
          continue;
        }

        let d = target - check;
        if d % cycle_diff == 0 {
          let score = (diffs.len() * (d / cycle_diff) as usize) + ti.abs_diff(ci) + ti;
          if score < best_score {
            best_score = score;
            best_target = target;
          }
        }
      }
    }

    Ok(best_target)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2018, 1)
}
