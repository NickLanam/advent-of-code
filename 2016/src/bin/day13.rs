use std::collections::VecDeque;

use advent_lib::grid::{Infinite2dGrid, Infinite2dSet};
use advent_lib::runner::{Day, PartId};
use anyhow::{Result, bail};

type P1Out = u64;
type P2Out = u64;
type Parsed = u64;

fn wall_test(seed: u64, walls: &mut Infinite2dGrid<bool>, x: i32, y: i32) -> bool {
  *walls.get_or_insert_with(x, y, || {
    let xx = x as u64;
    let yy = y as u64;
    let cmp: u64 = (3 * xx) + (2 * xx * yy) + (xx * xx) + (yy * yy) + yy + seed;
    let num_ones = cmp.count_ones();
    !num_ones.is_multiple_of(2)
  })
}

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    Ok(lines[0].parse().unwrap())
  }

  fn part1(&self, seed: &Parsed, sample_name: Option<String>) -> Result<P1Out> {
    let goal = if sample_name.is_some() {
      (7_i32, 4_i32)
    } else {
      (31, 39)
    };
    // Saves a few microseconds
    let capacity = ((goal.0 + 1) * (goal.1 + 1)) as usize;

    let mut walls = Infinite2dGrid::<bool>::new(capacity);

    let mut seen = Infinite2dSet::new(capacity);

    let mut edges = VecDeque::from([(0_u64, 1_i32, 1_i32)]);

    // BFS, tracking position to seek the goal.
    // Takes less than 100 microseconds for both parts, so no need to implement a better pathfinder.
    while let Some((depth, x, y)) = edges.pop_front() {
      if x == goal.0 && y == goal.1 {
        return Ok(depth);
      }
      seen.insert(x, y);
      for (nx, ny) in [(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)] {
        if nx >= 0 && ny >= 0 && !seen.contains(nx, ny) {
          let is_wall = wall_test(*seed, &mut walls, x, y);
          if !is_wall {
            edges.push_back((depth + 1, nx, ny));
          }
        }
      }
    }
    bail!("Solution not found, saw {} nodes", seen.len())
  }

  fn part2(&self, seed: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    let capacity = 2_500;

    let mut walls = Infinite2dGrid::<bool>::new(capacity);

    let mut seen = Infinite2dSet::new(capacity);
    seen.insert(1, 1);

    let mut edges = VecDeque::from([(0_u64, 1_i32, 1_i32)]);

    // Doesn't check as many nodes as part 1, so it's even faster
    while let Some((depth, x, y)) = edges.pop_front() {
      if depth > 50 {
        continue;
      }
      for (nx, ny) in [(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)] {
        if nx >= 0 && ny >= 0 && !seen.contains(nx, ny) {
          let is_wall = wall_test(*seed, &mut walls, x, y);
          if !is_wall {
            seen.insert(x, y);
            edges.push_back((depth + 1, nx, ny));
          }
        }
      }
    }
    Ok(seen.len() as u64)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2016, 13)
}
