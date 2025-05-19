use advent_lib::{
  grid::Infinite2dSet,
  runner::{Day, PartId},
};
use anyhow::Result;

type P1Out = usize;
type P2Out = usize;
type Parsed = (usize, usize, usize, usize, Infinite2dSet);

fn solve(&(_x_min, _y_min, y_min, y_max, ref walls): &Parsed) -> Result<(usize, usize)> {
  let mut touched = Infinite2dSet::new(100);
  let mut settled = Infinite2dSet::new(100);
  touched.insert(500, 1);

  let mut cursors: Vec<(i32, i32)> = vec![(500, 1)];
  while let Some((cx, cy)) = cursors.pop() {
    if cy as usize > y_max {
      // Stop when we reach the bottom of the map
      continue;
    }
    let wall_below = walls.contains(cx, cy + 1);
    let settled_below = settled.contains(cx, cy + 1);

    if !wall_below && !settled_below {
      // If the space below is empty, mark it as touched and move this cursor there
      touched.insert(cx, cy + 1);
      cursors.push((cx, cy + 1));
    } else {
      // If the space below is a wall or is settled, more complex tests apply:
      // - Flood | outward, dropping down if possible
      // - If we don't drop down and hit two walls, settle that range and move cursor back up
      let (mut left_wall, mut right_wall) = (false, false);
      let (mut lx, mut rx) = (cx - 1, cx + 1);
      // Scan left...
      'left_scan: loop {
        if walls.contains(lx, cy) {
          left_wall = true;
          lx += 1;
          break 'left_scan;
        } else if !walls.contains(lx, cy + 1) && !settled.contains(lx, cy + 1) {
          // Drop down
          if !touched.contains(lx, cy) {
            touched.insert(lx, cy);
            touched.insert(lx, cy + 1);
            cursors.push((lx, cy + 1));
          }
          break 'left_scan;
        } else {
          touched.insert(lx, cy);
          lx -= 1;
        }
      }
      // Scan right...
      'right_scan: loop {
        if walls.contains(rx, cy) {
          right_wall = true;
          rx -= 1;
          break 'right_scan;
        } else if !walls.contains(rx, cy + 1) && !settled.contains(rx, cy + 1) {
          // Drop down
          if !touched.contains(rx, cy) {
            touched.insert(rx, cy);
            touched.insert(rx, cy + 1);
            cursors.push((rx, cy + 1));
          }
          break 'right_scan;
        } else {
          touched.insert(rx, cy);
          rx += 1;
        }
      }
      // If we hit two walls, fill the space and the cursor moves back up
      if left_wall && right_wall {
        for sx in lx..=rx {
          settled.insert(sx, cy);
        }
        cursors.push((cx, cy - 1));
        touched.insert(cx, cy - 1); // Doesn't happen in a nested buckets case otherwise
      }
    }
  }

  // Only count touched nodes that are in-bounds and not also settled
  // Cheaper to do this at the end than to clean it up as we go!
  let mut num_unsettled = 0;
  for (ux, uy) in touched.keys() {
    if uy >= y_min as i32 && uy <= y_max as i32 && !settled.contains(ux, uy) {
      num_unsettled += 1;
    }
  }

  Ok((num_unsettled, settled.len()))
}

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    let mut x_min = 500;
    let mut x_max = 500;
    let mut y_min = i32::MAX;
    let mut y_max = 1;
    let mut walls = Infinite2dSet::new(1_000);
    for line in lines {
      let (l, r) = line.split_once(", ").unwrap();
      let (mode, d0n) = l.split_once('=').unwrap();
      let (_, range) = r.split_once('=').unwrap();
      let (r0n, r1n) = range.split_once("..").unwrap();

      let d0: i32 = d0n.parse()?;
      let d1_min: i32 = r0n.parse()?;
      let d1_max: i32 = r1n.parse()?;
      if mode == "x" {
        x_min = x_min.min(d0);
        x_max = x_max.max(d0);
        y_min = y_min.min(d1_min);
        y_max = y_max.max(d1_max);
        for y in d1_min..=d1_max {
          walls.insert(d0, y);
        }
      } else {
        x_min = x_min.min(d1_min);
        x_max = x_max.max(d1_max);
        y_min = y_min.min(d0);
        y_max = y_max.max(d0);
        for x in d1_min..=d1_max {
          walls.insert(x, d0);
        }
      }
    }
    Ok((
      x_min as usize,
      x_max as usize,
      y_min as usize,
      y_max as usize,
      walls,
    ))
  }

  fn part1(&self, parsed: &Parsed, _: Option<String>) -> Result<P1Out> {
    let solution = solve(parsed);
    if let Ok((unsettled, settled)) = solution {
      Ok(unsettled + settled)
    } else {
      Err(solution.unwrap_err())
    }
  }

  fn part2(&self, parsed: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    let solution = solve(parsed);
    if let Ok((_unsettled, settled)) = solution {
      Ok(settled)
    } else {
      Err(solution.unwrap_err())
    }
  }
}

fn main() -> Result<()> {
  Solver {}.run(2018, 17)
}
