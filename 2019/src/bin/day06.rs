use std::collections::VecDeque;

use advent_lib::runner::{Day, PartId};
use anyhow::{Result, bail};
use fnv::{FnvBuildHasher, FnvHashMap, FnvHashSet};

type P1Out = usize;
type P2Out = usize;

// orbitee -> Vec<orbiter>
type Parsed = FnvHashMap<String, Vec<String>>;

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    let mut out: FnvHashMap<String, Vec<String>> =
      FnvHashMap::with_capacity_and_hasher(lines.len(), FnvBuildHasher::default());
    for line in lines {
      let (orbitee, orbiter) = line.split_once(')').unwrap();
      out
        .entry(orbitee.to_string())
        .or_default()
        .push(orbiter.to_string());
    }
    Ok(out)
  }

  fn part1(&self, map: &Parsed, _: Option<String>) -> Result<P1Out> {
    // Breadth-first exploration, summing depths of connections.
    let mut sum = 0;
    let mut frontier: VecDeque<(String, usize)> = VecDeque::new();
    frontier.push_front(("COM".to_string(), 0));
    while let Some((orbitee, depth)) = frontier.pop_front() {
      sum += depth;
      if let Some(children) = map.get(&orbitee) {
        for child in children {
          frontier.push_back((child.to_owned(), depth + 1));
        }
      }
    }
    Ok(sum)
  }

  fn part2(&self, forward_map: &Parsed, _: Option<String>) -> Result<P2Out> {
    let mut backward_map: FnvHashMap<String, Vec<String>> =
      FnvHashMap::with_capacity_and_hasher(forward_map.len(), FnvBuildHasher::default());
    for (orbitee, orbiters) in forward_map.iter() {
      for orbiter in orbiters.iter() {
        backward_map
          .entry(orbiter.to_string())
          .or_default()
          .push(orbitee.to_string());
      }
    }

    let mut visited: FnvHashSet<String> =
      FnvHashSet::with_capacity_and_hasher(forward_map.len(), FnvBuildHasher::default());

    let start = backward_map.get("YOU").unwrap()[0].to_string();
    let end = backward_map.get("SAN").unwrap()[0].to_string();

    let mut frontier: VecDeque<(String, usize)> = VecDeque::new();
    frontier.push_front((start, 0));

    // Relatively simple BFS
    while let Some((node, depth)) = frontier.pop_front() {
      if visited.contains(node.as_str()) {
        continue;
      } else {
        visited.insert(node.to_string());
      }
      if node == end {
        return Ok(depth);
      }
      if let Some(forward_targets) = forward_map.get(node.as_str()) {
        for target in forward_targets {
          frontier.push_back((target.to_string(), depth + 1));
        }
      }
      if let Some(backward_targets) = backward_map.get(node.as_str()) {
        for target in backward_targets {
          frontier.push_back((target.to_string(), depth + 1));
        }
      }
    }
    bail!("Failed to find a route!");
  }
}

fn main() -> Result<()> {
  Solver {}.run(2019, 6)
}
