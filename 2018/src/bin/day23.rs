use advent_lib::runner::{Day, PartId};
use anyhow::{Context, Result};

type P1Out = usize;
type P2Out = usize;
type Parsed = Vec<(i64, i64, i64, i64)>;

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    let mut out = Vec::with_capacity(lines.len());
    for line in lines {
      let (left, r) = line[5..].split_once(">, r=").context("Bad line")?;
      let (x, yz) = left.split_once(",").context("Can't extract coordinates")?;
      let (y, z) = yz.split_once(",").context("Can't extract coordinates")?;
      out.push((x.parse()?, y.parse()?, z.parse()?, r.parse()?));
    }
    Ok(out)
  }

  // Trivial implementation of the spec. Takes 2µs.
  fn part1(&self, bots: &Parsed, _: Option<String>) -> Result<P1Out> {
    let &(lx, ly, lz, lr) = bots.iter().max_by_key(|&&(_, _, _, r)| r).unwrap();
    let mut in_range: usize = 0;
    for &(x, y, z, _r) in bots.iter() {
      let d = (x.abs_diff(lx) + y.abs_diff(ly) + z.abs_diff(lz)) as i64;
      if d <= lr {
        in_range += 1;
      }
    }
    Ok(in_range)
  }

  // Originally, I oversimplified this solution to use a single axis
  // number line (about 100µs). This got an answer just 22 away from
  // the correct one, which meant that assumption isn't quite right.
  // This approach does a dividing search of a bounding box that has
  // all nodes, similar to how octrees work. It takes 15ms. Sadness.
  fn part2(&self, bots: &Parsed, _: Option<String>) -> Result<P2Out> {
    let (mut x_min, mut x_max, mut y_min, mut y_max, mut z_min, mut z_max) =
      (i64::MAX, i64::MIN, i64::MAX, i64::MIN, i64::MAX, i64::MIN);
    for &(x, y, z, _r) in bots.iter() {
      x_min = x_min.min(x);
      x_max = x_max.max(x);
      y_min = y_min.min(y);
      y_max = y_max.max(y);
      z_min = z_min.min(z);
      z_max = z_max.max(z);
    }

    let mut range = 1;
    while range < x_max - x_min {
      range *= 2;
    }

    loop {
      let ru = range as usize;
      let mut best_in_range = 0;
      let mut best_dist = 0;
      let mut best_pos = (0, 0, 0);

      // Divide the current search box into smaller boxes with `range`-long
      // sides, and check each corner to see how many bots are in its range.
      // The corner that's seen by the most bots (closest to origin in a tie)
      // becomes the new center point, and the search range is halved, until
      // we've refined to a single point. That point's distance from origin
      // is the answer.
      for x in (x_min..=x_max).step_by(ru) {
        for y in (y_min..=y_max).step_by(ru) {
          for z in (z_min..=z_max).step_by(ru) {
            let mut in_range = 0;
            for &(bx, by, bz, br) in bots.iter() {
              if ((bx.abs_diff(x) + by.abs_diff(y) + bz.abs_diff(z)) as i64 - br) / range <= 0 {
                in_range += 1;
              }
            }
            let dist = x.abs() + y.abs() + z.abs();
            if in_range > best_in_range || (in_range == best_in_range && dist < best_dist) {
              best_in_range = in_range;
              best_dist = dist;
              best_pos = (x, y, z);
            }
          }
        }
      }

      if range == 1 {
        return Ok(best_dist as usize);
      }

      // Next search is centered on the best point so far, at half radius.
      x_min = best_pos.0 - range;
      x_max = best_pos.0 + range;
      y_min = best_pos.1 - range;
      y_max = best_pos.1 + range;
      z_min = best_pos.2 - range;
      z_max = best_pos.2 + range;
      range /= 2;
    }
  }
}

fn main() -> Result<()> {
  Solver {}.run(2018, 23)
}
