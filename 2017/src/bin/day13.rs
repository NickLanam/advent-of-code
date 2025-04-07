use advent_lib::runner::{Day, PartId};
use anyhow::{Context, Result, bail};

type P1Out = i64;
type P2Out = i64;
type Parsed = Vec<(i64, i64, i64)>;

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    let mut layers = Vec::with_capacity(lines.len());
    for line in lines {
      let (left, right) = line.split_once(": ").context("")?;
      let offset = left.parse()?;
      let size = right.parse()?;
      let period = (size - 1) * 2;
      layers.push((offset, size, period));
    }
    Ok(layers)
  }

  fn part1(&self, layers: &Parsed, _: Option<String>) -> Result<P1Out> {
    Ok(
      layers
        .iter()
        .map(|&(offset, size, period)| {
          if offset % period == 0 {
            offset * size
          } else {
            0
          }
        })
        .sum(),
    )
  }

  fn part2(&self, layers: &Parsed, _: Option<String>) -> Result<P2Out> {
    // 2016 Day 15's part 2 is nearly the same puzzle.
    // Instead of only one safe moment per layer, only one is UNSAFE.
    if layers.is_empty() {
      bail!("No layers, no solution");
    } else if layers.len() == 1 {
      // One layer, trivial solution
      if layers[0].1 > 1 {
        return Ok(1);
      } else {
        bail!("There is a layer with a depth of one, no solution is possible");
      }
    }

    // Brute force isn't as clever as what I did in 2016 Day 15, Part 2.
    // However, the two problems have a difference that changes the solution:
    // the old version had only one safe spot in each cycle. The new version
    // has only one UNSAFE spot in each cycle.
    // Brute force takes 50 milliseconds.
    // The right solution will likely take less than 2 microseconds.
    'test: for delay in 0..i64::MAX {
      for &(offset, _, period) in layers {
        if (delay + offset) % period == 0 {
          continue 'test;
        }
      }
      return Ok(delay);
    }
    bail!("No solution found");
  }
}

fn main() -> Result<()> {
  Solver {}.run(2017, 13)
}
