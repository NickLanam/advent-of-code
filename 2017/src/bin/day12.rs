use advent_lib::runner::{Day, PartId};
use anyhow::{Result, bail};
use fnv::{FnvBuildHasher, FnvHashSet};

type P1Out = usize;
type P2Out = usize;
type Parsed = Vec<FnvHashSet<usize>>;

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    let links: Vec<Vec<usize>> = lines
      .iter()
      .map(|line| {
        line
          .split_once(" <-> ")
          .unwrap()
          .1
          .split(", ")
          .map(|n| n.parse().unwrap())
          .collect()
      })
      .collect();

    let mut visited: Vec<bool> = vec![false; lines.len()];

    let mut groups: Vec<FnvHashSet<usize>> = vec![];

    let mut visit = |i: usize| {
      if visited[i] {
        return;
      }
      let mut group = FnvHashSet::with_hasher(FnvBuildHasher::default());
      let mut frontier: Vec<usize> = vec![i];
      while let Some(next) = frontier.pop() {
        group.insert(next);
        for &sink in &links[next] {
          group.insert(sink);
          if !visited[sink] {
            visited[sink] = true;
            frontier.push(sink);
          }
        }
      }
      groups.push(group);
    };

    for i in 0..lines.len() {
      visit(i);
    }

    Ok(groups)
  }

  fn part1(&self, groups: &Parsed, _: Option<String>) -> Result<P1Out> {
    for group in groups {
      if group.contains(&0) {
        return Ok(group.len());
      }
    }
    bail!("No group contained 0!");
  }

  fn part2(&self, groups: &Parsed, _: Option<String>) -> Result<P2Out> {
    Ok(groups.len())
  }
}

fn main() -> Result<()> {
  Solver {}.run(2017, 12)
}
