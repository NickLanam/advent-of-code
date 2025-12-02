use advent_lib::grid::Infinite2dSet;
use advent_lib::runner::{Day, PartId};
use anyhow::{Context, Result, bail};
use std::f64::consts::PI;

type P1Out = String;
type P2Out = usize;
type Parsed = Infinite2dSet;

fn integer_steps_between(x1: i32, y1: i32, x2: i32, y2: i32) -> Vec<(i32, i32)> {
  let dx = x2 - x1;
  let dxa = dx.abs();
  let dy = y2 - y1;
  let dya = dy.abs();

  if dxa == 1 || dya == 1 {
    // In this case, it's impossible to have a collision in between
    vec![]
  } else if dxa == dya || dxa == 0 || dya == 0 {
    // Stepping by one on one or both axes breaks the logic below, so it's a special case.
    let mut out = vec![];
    let mut x = x1 + dx.signum();
    let mut y = y1 + dy.signum();
    while x != x2 || y != y2 {
      out.push((x, y));
      x += dx.signum();
      y += dy.signum();
    }
    out
  } else {
    // This is the most common case, but it's still worth it to eliminate the simple cases first.
    // Finding common factors of the two components of the slope helps us figure out where to look.
    // Thankfully, the board is pretty small, so we can do a slightly expensive check here.
    let mut out = vec![];
    for factor in 2..=(dxa.max(dya) / 2) {
      if factor > dxa || factor > dya {
        break;
      }
      if dx % factor == 0 && dy % factor == 0 {
        let step_x = dx / factor;
        let step_y = dy / factor;
        for step in 1..factor {
          let nx = x1 + step_x * step;
          let ny = y1 + step_y * step;
          out.push((nx, ny));
        }
      }
    }
    out
  }
}

fn find_best_position(asteroids: &Infinite2dSet) -> Result<(i32, i32, usize)> {
  let mut best_x: i32 = i32::MAX;
  let mut best_y: i32 = i32::MAX;
  let mut best_score: usize = 0;

  // The approach: for each pair of asteroids, find integer coordinates along the line segment between them.
  // If any of those coordinates contain another asteroid, then the two being checked cannot see each other.
  // Doing it this way (with a set) avoids doing an n^3 loop over all asteroids when there are hundreds.
  for (x, y) in asteroids.keys() {
    let mut score = 0;
    'pairs: for (sx, sy) in asteroids.keys().filter(|&(x2, y2)| x2 != x || y2 != y) {
      let intersectors = integer_steps_between(x, y, sx, sy);
      for &(ix, iy) in intersectors.iter() {
        if asteroids.contains(ix, iy) {
          continue 'pairs;
        }
      }
      score += 1;
    }
    if score > best_score {
      best_score = score;
      best_x = x;
      best_y = y;
    }
  }

  Ok((best_x, best_y, best_score))
}

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(Infinite2dSet::from_input_lines(&lines, |c| c == '#'))
  }

  fn part1(&self, asteroids: &Parsed, _: Option<String>) -> Result<P1Out> {
    let (x, y, score) =
      find_best_position(asteroids).with_context(|| "Failed to find best position")?;
    Ok(format!("{x},{y} ({score})").to_owned())
  }

  fn part2(&self, asteroids: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    if asteroids.len() < 200 {
      bail!("Problem statement requires a cluster with at least 200 asteroids")
    }

    // The asteroid chosen in part 1 is the basis of part 2
    let (station_x, station_y, ..) =
      find_best_position(asteroids).with_context(|| "Failed to find best position")?;

    // Compute the relative vector from the station to each other asteroid
    // [theta, [100*x + y, hypotenuse_to_station]]
    let mut vectors_by_angle: Vec<(f64, Vec<(usize, f64)>)> = vec![];
    for (ax, ay) in asteroids
      .keys()
      .filter(|&(x, y)| x != station_x || y != station_y)
    {
      let key = (100 * ax + ay) as usize;
      let dx = (ax - station_x) as f64;
      let dy = (ay - station_y) as f64;
      let distance = (dx * dx + dy * dy).sqrt();

      // atan2 ranges [-PI, PI] with 0 being east, PI/2 being south.
      // We need [0, 2*PI) with 0 being north and PI/2 being east, for sorting later.
      let mut theta = dy.atan2(dx);
      theta = (PI * 2.5 + theta) % (PI * 2.0);

      let pos = vectors_by_angle
        .iter()
        .position(|(other_theta, ..)| *other_theta == theta);
      match pos {
        Some(pos) => {
          vectors_by_angle[pos].1.push((key, distance));
          vectors_by_angle[pos]
            .1
            .sort_by(|&(_, d1), (_, d2)| d1.total_cmp(d2));
        }
        None => {
          vectors_by_angle.push((theta, vec![(key, distance)]));
        }
      }
    }

    // Sort groups by angle, starting north and going clockwise (their contents are already sorted by lowest distance)
    vectors_by_angle.sort_by(|(t1, _), (t2, _)| t1.total_cmp(t2));

    // Now that the hard part is done, cycle through the group list, removing an item if possible at each
    // and stopping when we've removed 200 items this way.
    let mut latest_removed_key = usize::MAX;
    let mut pos = 0;
    for _removal in 1..=200 {
      while vectors_by_angle[pos].1.is_empty() {
        pos = (pos + 1) % vectors_by_angle.len();
      }
      latest_removed_key = vectors_by_angle[pos].1.remove(0).0;
      pos += 1;
    }

    Ok(latest_removed_key)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2019, 10)
}
