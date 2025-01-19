use advent_lib::direction::{CardinalDirection, Rotation};
use advent_lib::grid::Infinite2dSet;
use advent_lib::runner::{Day, PartId};
use anyhow::{bail, Result};

type P1Out = u64;
type P2Out = u64;
type Parsed = Vec<(Rotation, u8)>;

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    Ok(
      lines[0]
        .split(", ")
        .map(|line| {
          let d: u8 = line[1..].parse().unwrap();
          if line.starts_with('L') {
            (Rotation::L, d)
          } else {
            (Rotation::R, d)
          }
        })
        .collect(),
    )
  }

  fn part1(&self, steps: &Parsed, _sample_name: Option<String>) -> Result<P1Out> {
    let mut dir = CardinalDirection::N;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for (r, dist) in steps {
      dir += r;
      (x, y) = dir.apply(x, y, *dist as i32);
    }
    Ok((x.abs() + y.abs()) as u64)
  }

  fn part2(&self, steps: &Parsed, sample_name: Option<String>) -> Result<P2Out> {
    if sample_name.is_some() && sample_name.unwrap() == "test01" {
      // The first test only makes sense for part 1 - it won't cross the same point twice.
      return Ok(0);
    }
    // Same thing, but stop when we see the same location twice
    // and throw an error if we run out of instructions before that happens
    let mut seen = Infinite2dSet::new(1_000_000);
    let mut dir = CardinalDirection::N;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for (r, dist) in steps {
      dir += r;
      let (tx, ty) = dir.apply(x, y, *dist as i32);
      let dx = (tx - x).signum();
      let dy = (ty - y).signum();

      while x != tx || y != ty {
        x += dx;
        y += dy;
        if seen.has(x, y) {
          return Ok((x.abs() + y.abs()) as u64);
        } else {
          seen.add(x, y);
        }
      }
    }
    bail!("Ran out of instructions without seeing the same location twice")
  }
}

fn main() -> Result<()> {
  Solver {}.run(2016, 1)
}
