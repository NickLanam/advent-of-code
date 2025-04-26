use advent_lib::runner::{Day, PartId};
use anyhow::{Context, Result};

type P1Out = u16;
type P2Out = u16;
type Parsed = Vec<(u16, u16)>;

// This takes about 50 microseconds for the sample.
// The real input has enough possible bridges that it
// takes about 40 milliseconds. Would memoization help?
// Or would tracking sub-bridges cost more than it saves?
fn solve(planks: &[(u16, u16)]) -> (u16, u16) {
  let mut longest = 0;
  let mut strongest = 0;
  let mut strongest_at_longest = 0;

  // Set up paths to explore. (open_node, length, score, remaining_planks)
  #[allow(clippy::type_complexity)]
  let mut frontier: Vec<(u16, u16, u16, Vec<&(u16, u16)>)> = vec![];
  for (pi, plank) in planks.iter().enumerate() {
    // Simpler logic later to create both orientations now.
    // Still, the puzzle says the start node must be a 0.
    if plank.1 == 0 {
      let mut remain: Vec<&(u16, u16)> = planks.iter().collect();
      remain.swap_remove(pi);
      frontier.push((plank.0, 1, plank.0 + plank.1, remain));
    } else if plank.0 == 0 {
      let mut remain: Vec<&(u16, u16)> = planks.iter().collect();
      remain.swap_remove(pi);
      frontier.push((plank.1, 1, plank.0 + plank.1, remain));
    }
  }

  // Take a bridge from the frontier.
  // For each remaining plank that can attach to the end, push that bridge to the frontier.
  // If there are no pushable planks left, it's a finished bridge, push it to that.
  while let Some(tentative) = frontier.pop() {
    let (open, length, score, remain) = tentative;
    let mut is_final = true;

    for (pi, &plank) in remain.iter().enumerate() {
      if open == plank.0 || open == plank.1 {
        is_final = false;
        let mut next_remain: Vec<&(u16, u16)> = remain.clone();
        next_remain.swap_remove(pi);

        frontier.push((
          if open == plank.0 { plank.1 } else { plank.0 },
          length + 1,
          score + plank.0 + plank.1,
          next_remain,
        ));
      }
    }

    // The one-element bridges, we skip, because they could
    // be the initial values flipped the wrong way around
    if is_final && length > 1 {
      if score > strongest {
        strongest = score;
      }
      if length > longest {
        longest = length;
        strongest_at_longest = score;
      } else if length == longest && score > strongest_at_longest {
        strongest_at_longest = score;
      }
    }
  }

  (strongest, strongest_at_longest)
}

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    let mut out: Vec<(u16, u16)> = Vec::with_capacity(lines.len());
    for line in lines {
      let (l, r) = line.split_once('/').context("Unsplittable line?")?;
      out.push((l.parse()?, r.parse()?));
    }
    Ok(out)
  }

  fn part1(&self, planks: &Parsed, _: Option<String>) -> Result<P1Out> {
    Ok(solve(planks).0)
  }

  fn part2(&self, planks: &Parsed, _: Option<String>) -> Result<P2Out> {
    Ok(solve(planks).1)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2017, 24)
}
