use fnv::{FnvBuildHasher, FnvHashSet};

use advent_lib::runner::{Day, PartId};
use anyhow::Result;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Point {
  x: i64,
  y: i64,
  z: i64,
}
impl std::fmt::Debug for Point {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}, {}, {})", self.x, self.y, self.z)
  }
}

type P1Out = usize;
type P2Out = usize;
type Parsed = Vec<Point>;

fn solve(points: &[Point], stop_after: usize) -> (Vec<FnvHashSet<&Point>>, &Point, &Point) {
  // First, compute the distance from every point to every other point.
  // Sort by distance, accept that this is slow and naive, and use it to
  // more easily solve the puzzle afterward.
  let mut distances: Vec<(u64, &Point, &Point)> = vec![];
  for (i, p1) in points.iter().enumerate() {
    for p2 in points[(i + 1)..].iter() {
      let dx = p1.x.abs_diff(p2.x).pow(2);
      let dy = p1.y.abs_diff(p2.y).pow(2);
      let dz = p1.z.abs_diff(p2.z).pow(2);
      // No need to sqrt, we're only comparing. This also avoids float math.
      let d = dx + dy + dz;
      distances.push((d, p1, p2));
    }
  }
  distances.sort_by(|(d1, ..), (d2, ..)| d1.partial_cmp(d2).unwrap());

  // Now form the groups, similar to how we did in 2018 Day 25
  // Note: if a pair results in a connection that already exists,
  // it does not count towards the taken number so we must do more.
  let mut groups: Vec<FnvHashSet<&Point>> = vec![];
  let mut latest_p1: &Point = &points[0];
  let mut latest_p2: &Point = &points[0];
  for &(_, p1, p2) in distances.iter().take(stop_after) {
    latest_p1 = p1;
    latest_p2 = p2;
    let existing: Vec<usize> = groups
      .iter()
      .enumerate()
      .filter(|(_, g)| g.contains(p1) || g.contains(p2))
      .map(|(i, _)| i)
      .collect();

    match existing.len() {
      0 => {
        let mut s = FnvHashSet::with_capacity_and_hasher(2, FnvBuildHasher::default());
        s.insert(p1);
        s.insert(p2);
        groups.push(s);
      }
      1 => {
        // Note: when a connection is a no-op, it's still counted against the total.
        groups[existing[0]].insert(p1);
        groups[existing[0]].insert(p2);
      }
      2.. => {
        // When we find more than one group, merge them together
        groups[existing[0]].insert(p1);
        groups[existing[0]].insert(p2);
        for &i in existing[1..].iter() {
          let other_as_immutable = groups[i].to_owned();
          for p3 in other_as_immutable {
            groups[existing[0]].insert(p3);
          }
        }
        for &j in existing[1..].iter().rev() {
          groups.remove(j);
        }
      }
    }

    if let Some(g0) = groups.first()
      && g0.len() == points.len()
    {
      // Stop when we have a complete graph
      break;
    }
  }

  (groups, latest_p1, latest_p2)
}

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    let mut points: Vec<Point> = Vec::with_capacity(lines.len());
    for line in lines {
      let mut parts = line.split(',');
      points.push(Point {
        x: parts.next().unwrap().parse()?,
        y: parts.next().unwrap().parse()?,
        z: parts.next().unwrap().parse()?,
      })
    }
    Ok(points)
  }

  fn part1(&self, points: &Parsed, sample_name: Option<String>) -> Result<P1Out> {
    let num_connections = if sample_name.is_some() { 10 } else { 1_000 };
    let (mut groups, ..) = solve(points, num_connections);

    // Finally, get the three largest groups' sizes, and multiply them together
    groups.sort_by(|g1, g2| g1.len().cmp(&g2.len()).reverse());
    assert!(groups.len() >= 3);

    Ok(groups[0].len() * groups[1].len() * groups[2].len())
  }

  fn part2(&self, points: &Parsed, _: Option<String>) -> Result<P2Out> {
    let (_, latest_p1, latest_p2) = solve(points, usize::MAX);
    Ok((latest_p1.x.abs() * latest_p2.x.abs()) as usize)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2025, 8)
}
