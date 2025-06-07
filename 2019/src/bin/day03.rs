use advent_lib::{
  direction::CardinalDirection,
  runner::{Day, PartId},
};
use anyhow::{Result, bail};

struct Move {
  dir: CardinalDirection,
  by: i32,
}

// (steps, x1, y1, x2, y2)
type Line = (i32, i32, i32, i32, i32);

type P1Out = usize;
type P2Out = usize;
type Parsed = (Vec<Move>, Vec<Move>);

fn moves_to_lines(moves: &[Move]) -> Vec<Line> {
  let mut out = vec![];
  let mut x = 0;
  let mut y = 0;
  let mut steps = 0;
  for Move { dir, by } in moves {
    let (x2, y2) = dir.apply(x, y, *by);
    out.push((steps, x, y, x2, y2));
    steps += by;
    (x, y) = (x2, y2);
  }
  out
}

// (x, y, score)
fn find_intersections(a_lines: &[Line], b_lines: &[Line]) -> Vec<(i32, i32, i32)> {
  // Both lines are either vertical or horizontal, so this is straightforward.
  let mut intersections = vec![];
  for &(b_steps, bx1, by1, bx2, by2) in b_lines.iter() {
    for &(a_steps, ax1, ay1, ax2, ay2) in a_lines.iter() {
      if ax1 == ax2 && ax1 == bx1 && ax1 == bx2 {
        // Both lines vertical on same x line -> check if the y ranges overlap
        // If they do, every point along that overlap is an intersection
        let min_y = ay1.min(ay2).max(by1.min(by2));
        let max_y = ay1.max(ay2).min(by1.max(by2));
        if min_y <= max_y {
          for y in min_y..=max_y {
            let score = a_steps + (ay1.abs_diff(y) as i32) + b_steps + (by1.abs_diff(y) as i32);
            intersections.push((ax1, y, score));
          }
        }
      } else if ay1 == ay2 && ay1 == by1 && ay1 == by2 {
        // Both lines horizontal on same y line -> check if the x ranges overlap
        // If they do, every point along that overlap is an intersection
        let min_x = ax1.min(ax2).max(bx1.min(bx2));
        let max_x = ax1.max(ax2).min(bx1.max(bx2));
        if min_x <= max_x {
          for x in min_x..=max_x {
            let score = a_steps + (ax1.abs_diff(x) as i32) + b_steps + (bx1.abs_diff(x) as i32);
            intersections.push((x, ay1, score));
          }
        }
      } else if ax1 == ax2 && by1 == by2 {
        // First line is vertical, other is horizontal. Only one way for them to overlap.
        if ay1.min(ay2) <= by1 && ay1.max(ay2) >= by1 && bx1.min(bx2) <= ax1 && bx1.max(bx2) >= ax1
        {
          let (x, y) = (ax1, by1);
          let score = a_steps + (ay1.abs_diff(y) as i32) + b_steps + (bx1.abs_diff(x) as i32);
          intersections.push((x, y, score));
        }
      } else if ay1 == ay2 && bx1 == bx2 {
        // First line is horizontal, other is vertical. Only one way for them to overlap.
        if ax1.min(ax2) <= bx1 && ax1.max(ax2) >= bx1 && by1.min(by2) <= ay1 && by1.max(by2) >= ay1
        {
          let (x, y) = (bx1, ay1);
          let score = a_steps + (ax1.abs_diff(x) as i32) + b_steps + (by1.abs_diff(y) as i32);
          intersections.push((x, y, score));
        }
      }
      // Otherwise, the lines can't overlap - they're parallel but not touching
    }
  }
  intersections.retain(|&(x, y, _)| x != 0 || y != 0);
  intersections
}

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    let parse_line = |line: &str| -> Result<Vec<Move>> {
      let mut list: Vec<Move> = vec![];
      for part in line.split(',') {
        let (l, r) = part.split_at(1);
        let dir = match l {
          "U" => CardinalDirection::N,
          "R" => CardinalDirection::E,
          "L" => CardinalDirection::W,
          "D" => CardinalDirection::S,
          _ => bail!("Bad input"),
        };
        let by = r.parse()?;
        list.push(Move { dir, by });
      }
      Ok(list)
    };

    Ok((
      parse_line(lines[0].as_str())?,
      parse_line(lines[1].as_str())?,
    ))
  }

  fn part1(&self, (a_moves, b_moves): &Parsed, _: Option<String>) -> Result<P1Out> {
    let a_lines = moves_to_lines(a_moves);
    let b_lines = moves_to_lines(b_moves);

    let mut min = usize::MAX;
    for (x, y, _score) in find_intersections(&a_lines, &b_lines) {
      min = min.min((x.abs() + y.abs()) as usize)
    }
    Ok(min)
  }

  fn part2(&self, (a_moves, b_moves): &Parsed, _: Option<String>) -> Result<P2Out> {
    let a_lines = moves_to_lines(a_moves);
    let b_lines = moves_to_lines(b_moves);

    let mut min_score = usize::MAX;
    for (_x, _y, score) in find_intersections(&a_lines, &b_lines) {
      if score > 0 && (score as usize) < min_score {
        min_score = score as usize;
      }
    }
    Ok(min_score)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2019, 3)
}
