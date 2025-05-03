use advent_lib::{
  grid::Infinite2dSet,
  runner::{Day, PartId},
};
use anyhow::{Context, Result};
use rayon::prelude::*;

type P1Out = i32;
type P2Out = i32;
#[derive(Clone, Copy, PartialEq, Eq)]
struct Point {
  x: i32,
  y: i32,
}
struct Parsed {
  min_x: i32,
  max_x: i32,
  min_y: i32,
  max_y: i32,
  points: Vec<Point>,
}

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    let (mut min_x, mut max_x, mut min_y, mut max_y) = (i32::MAX, i32::MIN, i32::MAX, i32::MIN);
    let mut points: Vec<Point> = Vec::with_capacity(lines.len());
    for line in lines {
      let (a, b) = line.split_once(", ").context("Bad line")?;
      let (x, y) = (a.parse()?, b.parse()?);
      min_x = min_x.min(x);
      max_x = max_x.max(x);
      min_y = min_y.min(y);
      max_y = max_y.max(y);
      points.push(Point { x, y });
    }
    Ok(Parsed {
      min_x,
      max_x,
      min_y,
      max_y,
      points,
    })
  }

  fn part1(
    &self,
    Parsed {
      min_x,
      max_x,
      min_y,
      max_y,
      points,
    }: &Parsed,
    _: Option<String>,
  ) -> Result<P1Out> {
    let mut best_area = 0;
    'scan: for &Point { x, y } in points {
      // Naive method: do a flood fill from this coordinate. At each explore,
      // if no other nodes are closer to that point (expensive test), then
      // add it to the area and mark any unexplored neighbors for exploring.
      // Repeat until none are found.

      let other_points: Vec<Point> = points
        .iter()
        .filter(|&&Point { x: x2, y: y2 }| x2 != x || y2 != y)
        .copied()
        .collect();

      // Skip nodes that have infinite area: if any of the near points outside the
      // bounding box belong to this node, an infinite number of points in that
      // direction do too.
      for (tx, ty) in [
        (min_x - 1, y),
        (x, min_y - 1),
        (max_x + 1, y),
        (x, max_y + 1),
      ] {
        let d = tx.abs_diff(x) + ty.abs_diff(y);
        if !other_points
          .iter()
          .any(|&Point { x: ox, y: oy }| tx.abs_diff(ox) + ty.abs_diff(oy) <= d)
        {
          continue 'scan;
        }
      }

      // Remaining points have finite area. Naively discover it via flood fill.
      // This is very slow: 6 milliseconds for my input.
      let mut area = 1;
      let mut explored: Infinite2dSet = Infinite2dSet::new(4);
      explored.add(x, y);
      let mut frontier: Vec<(i32, i32)> = vec![(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)];
      while let Some((fx, fy)) = frontier.pop() {
        if explored.has(fx, fy) {
          continue;
        }
        explored.add(fx, fy);
        let dist_to_source = fx.abs_diff(x) + fy.abs_diff(y);
        if !other_points
          .iter()
          .any(|&Point { x: rx, y: ry }| (rx.abs_diff(fx) + ry.abs_diff(fy)) <= dist_to_source)
        {
          // It's closer to the source than elsewhere, yay.
          area += 1;
          frontier.push((fx - 1, fy));
          frontier.push((fx, fy - 1));
          frontier.push((fx + 1, fy));
          frontier.push((fx, fy + 1));
        }
        best_area = best_area.max(area);
      }
    }
    Ok(best_area)
  }

  fn part2(
    &self,
    Parsed {
      min_x,
      max_x,
      min_y,
      max_y,
      points,
    }: &Parsed,
    sample_name: Option<String>,
  ) -> Result<P2Out> {
    let limit = if sample_name.is_some() { 32 } else { 10_000 };
    // Brute force: just try 'em all, and paralellize to make the CPU fans cry instead of the wall clock.
    // This is far worse than part 1: 6 seconds if not par_iter'd, 600ms if par_iter'd.
    // The result is a messy polygon shape, but it's close-ish to a circle whereas this scans
    // a square. It might help to find the geometric center of the input points, and spiral
    // outward from there like the flood fill in part 1? Frontier would get big though.
    // TODO: Actually... check if the bounding box is fully within the solution region.
    //  If so, can start having that region pre-marked as part of it and scan out from there,
    //  which skips a huge chunk of the iteration area. But then we're doing spirals,
    //  which isn't friendly to par_iter, so it'd need to be at least a 10x improvement to be worth it.
    let safe_zone_size = ((min_y - limit)..(max_y + limit))
      .into_par_iter()
      .map(|y| {
        let mut safe_cells = 0;
        for x in (min_x - limit)..(max_x + limit) {
          let dist: i32 = points
            .iter()
            .map(|&Point { x: px, y: py }| (x.abs_diff(px) + y.abs_diff(py)) as i32)
            .sum();
          if dist < limit {
            safe_cells += 1;
          }
        }
        safe_cells
      })
      .sum();
    Ok(safe_zone_size)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2018, 6)
}
