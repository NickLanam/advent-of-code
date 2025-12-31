use advent_lib::runner::{Day, PartId};
use advent_of_code_2019::intcode::execute;
use anyhow::{Result, bail};

type P1Out = usize;
type P2Out = usize;
type Parsed = Vec<i64>;

fn test(instructions: &Parsed, x: i64, y: i64) -> Result<bool> {
  let result = execute(instructions, &[x, y], None, None)?;
  match result.outputs[0] {
    0 => Ok(false),
    1 => Ok(true),
    _ => {
      bail!("Output was neither 0 nor 1, it was {:?}", result.outputs);
    }
  }
}

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(lines[0].split(',').map(|n| n.parse().unwrap()).collect())
  }

  fn part1(&self, instructions: &Parsed, _: Option<String>) -> Result<P1Out> {
    // Starting at y=4 because my input has blank rows early on due to the shallow angle
    let mut min_x = 3; // This one is the first one that is set
    let mut max_x = 4; // This one is the first one that is NOT set
    let mut tractor_spots = 1;
    for y in 4..50 {
      let mut min2 = min_x;
      let mut max2 = max_x;
      while test(instructions, max2, y)? {
        max2 += 1;
      }
      while !test(instructions, min2, y)? && min2 < max2 {
        min2 += 1;
      }
      tractor_spots += (max2 - min2) as usize;
      min_x = min2;
      max_x = max2;
    }

    Ok(tractor_spots)
  }

  fn part2(&self, instructions: &Parsed, _: Option<String>) -> Result<P2Out> {
    // This time, keep track of (min_x, max_x) starting when the difference is at least 100.
    // The first time min_x+99 is in-bounds for the row at y-99, we have our answer.
    let mut seen_bounds = [(0, 0); 100];
    let mut cycle_index = 0;

    // I ran this from y=4 the first time, then grabbed higher start points
    // just to speed up the computation.
    // In reality, the ideal solution is probaby doing trigonometry on the slopes of
    // the two lines at the tractor beam's edges to get the answer almost instantly.
    let mut min_x = 731;
    let mut max_x = 902;
    for y in 1_001..100_000 {
      let mut min2 = min_x;
      let mut max2 = max_x;
      while test(instructions, max2, y)? {
        max2 += 1;
      }
      while !test(instructions, min2, y)? && min2 < max2 {
        min2 += 1;
      }
      min_x = min2;
      max_x = max2;
      if y >= 500 && max_x - min_x >= 100 {
        // We can start checking earlier lines now
        seen_bounds[cycle_index] = (min_x, max_x);
        cycle_index = (cycle_index + 1) % 100;
        let (other_min, other_max) = seen_bounds[cycle_index];
        if other_min <= min_x && other_max >= min_x + 100 {
          return Ok((min_x * 10_000 + (y - 99)) as usize);
        }
      }
    }
    Ok(0)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2019, 19)
}
