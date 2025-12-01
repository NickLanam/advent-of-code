use advent_lib::grid::Infinite2dSet;
use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = String;
type P2Out = String;
type Parsed = (usize, usize, Infinite2dSet);

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

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    let h = lines.len();
    let w = lines[0].len();
    Ok((w, h, Infinite2dSet::from_input_lines(&lines, |c| c == '#')))
  }

  fn part1(&self, (.., asteroids): &Parsed, _: Option<String>) -> Result<P1Out> {
    let mut best_x: usize = usize::MAX;
    let mut best_y: usize = usize::MAX;
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
        best_x = x as usize;
        best_y = y as usize;
      }
    }

    Ok(format!("{best_x},{best_y} ({best_score})").to_owned())
  }

  fn part2(&self, _lines: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    Ok("0,0".to_owned())
  }
}

fn main() -> Result<()> {
  Solver {}.run(2019, 10)
}
