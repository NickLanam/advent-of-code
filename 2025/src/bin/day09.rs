use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = usize;
type P2Out = usize;
type Parsed = Vec<(usize, usize)>;

fn intersects_perimeter(
  &(x1, y1): &(usize, usize),
  &(x2, y2): &(usize, usize),
  points: &[(usize, usize)],
) -> bool {
  let x_min = x1.min(x2);
  let x_max = x1.max(x2);
  let y_min = y1.min(y2);
  let y_max = y1.max(y2);

  for (i, &(p1x, p1y)) in points[0..(points.len() - 1)].iter().enumerate() {
    let (p2x, p2y) = points[i + 1];

    let pmin_x = p1x.min(p2x);
    let pmax_x = p1x.max(p2x);
    let pmin_y = p1y.min(p2y);
    let pmax_y = p1y.max(p2y);

    // Check if this line intersects (perpendicular cut) any of the rectangle edges
    if pmin_x == pmax_x {
      // Vertical line in the perimeter, check the horizontal edges of the rectangle
      let in_main_axis = x_min < pmin_x && pmin_x < x_max;
      let cross_start = pmin_y < y_min && pmax_y > y_min;
      let cross_end = pmin_y < y_max && pmax_y > y_max;
      let inside_start = pmin_y >= y_min && pmin_y <= y_max;
      let inside_end = pmax_y >= y_min && pmax_y <= y_max;
      if in_main_axis && (cross_start || cross_end || inside_start || inside_end) {
        return true;
      }
    } else if pmin_y == pmax_y {
      // Other way around
      let in_main_axis = y_min < pmin_y && pmin_y < y_max;
      let cross_start = pmin_x < x_min && pmax_x > x_min;
      let cross_end = pmin_x < x_max && pmax_x > x_max;
      let inside_start = pmin_x >= x_min && pmin_x <= x_max;
      let inside_end = pmax_x >= x_min && pmax_x <= x_max;
      if in_main_axis && (cross_start || cross_end || inside_start || inside_end) {
        return true;
      }
    } else {
      panic!("Perimeter points are not horizontal or vertical: ({p1x}, {p1y}) and ({p2x}, {p2y})");
    }
  }
  false
}

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    let mut points: Vec<(usize, usize)> = lines
      .iter()
      .map(|line| {
        let (l, r) = line.split_once(',').unwrap();
        (l.parse().unwrap(), r.parse().unwrap())
      })
      .collect();
    points.push(points[0]); // Make a cycle so that part 2 can notice the last connection
    Ok(points)
  }

  fn part1(&self, points: &Parsed, _: Option<String>) -> Result<P1Out> {
    let mut best = 0;
    for (i, p1) in points.iter().enumerate() {
      for p2 in points[(i + 1)..].iter() {
        best = best.max((p1.0.abs_diff(p2.0) + 1) * (p1.1.abs_diff(p2.1) + 1));
      }
    }
    Ok(best)
  }

  fn part2(&self, points: &Parsed, _: Option<String>) -> Result<P2Out> {
    // Similar to part 1, but we don't count rectangles that are not fully inside the polygon.
    let mut best = 0;
    for (i, p1) in points.iter().enumerate() {
      for p2 in points[(i + 1)..].iter() {
        let area = (p1.0.abs_diff(p2.0) + 1) * (p1.1.abs_diff(p2.1) + 1);
        // By checking area before intersection, we skip checks on most of the rectangles
        if area > best && !intersects_perimeter(p1, p2, points) {
          best = area;
        }
      }
    }
    Ok(best)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2025, 9)
}
