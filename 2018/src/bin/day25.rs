use advent_lib::runner::{Day, PartId};
use anyhow::Result;

#[derive(Clone, PartialEq, Debug)]
struct Point {
  x: i8,
  y: i8,
  z: i8,
  t: i8,
}
impl Point {
  fn manhattan(&self, other: &Point) -> u8 {
    self.x.abs_diff(other.x)
      + self.y.abs_diff(other.y)
      + self.z.abs_diff(other.z)
      + self.t.abs_diff(other.t)
  }
  fn close_enough(&self, other: &Point) -> bool {
    self.manhattan(other) <= 3
  }
}

type P1Out = usize;
type P2Out = usize;
type Parsed = Vec<Point>;

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
        t: parts.next().unwrap().parse()?,
      })
    }
    Ok(points)
  }

  fn part1(&self, points: &Parsed, _sample_name: Option<String>) -> Result<P1Out> {
    let mut constellations: Vec<Vec<Point>> = vec![];
    for point in points {
      let mut merge_ids: Vec<usize> = vec![];
      for (i, constellation) in constellations.iter_mut().enumerate() {
        if constellation.iter().any(|p| p.close_enough(point)) {
          if !merge_ids.is_empty() {
            merge_ids.push(i);
          } else {
            merge_ids.push(i);
            constellation.push(point.clone());
          }
        }
      }
      if merge_ids.is_empty() {
        constellations.push(vec![point.clone()]);
      } else if merge_ids.len() > 1 {
        let mut merged_group: Vec<Point> = vec![];
        for &id in merge_ids.iter() {
          merged_group.append(&mut constellations[id]);
        }
        constellations.retain(|g| !g.is_empty());
        constellations.push(merged_group);
      }
    }
    Ok(constellations.len())
  }

  fn part2(&self, _lines: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    Ok(0)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2018, 25)
}
