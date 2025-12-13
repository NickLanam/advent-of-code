use std::collections::VecDeque;

use advent_lib::runner::{Day, PartId};
use anyhow::{Context, Result};
use fnv::{FnvBuildHasher, FnvHashMap, FnvHashSet};

type P1Out = usize;
type P2Out = usize;
type Parsed = FnvHashMap<String, FnvHashSet<String>>;

fn bfs(graph: &Parsed, start: &str, goal: &str) -> Result<usize> {
  let mut found = 0;

  // BFS, finding all paths that reach the end and counting them.
  //  This is comically fast for the input, because the spread of paths from "you" -> "out" is fairly thin.
  let mut edges = VecDeque::from([start]);
  while let Some(node) = edges.pop_front() {
    if node == goal {
      found += 1;
    } else if let Some(neighbors) = graph.get(node) {
      for next in neighbors {
        edges.push_back(next);
      }
    }
  }

  Ok(found)
}

/// DFS instead of BFS, recursive because I felt too lazy to keep it iterative.
/// Memoized on the number of paths that reach a given node, to avoid repeat work.
fn memoized_dfs(
  node: String,
  dac: bool,
  fft: bool,
  graph: &Parsed,
  memo: &mut FnvHashMap<(String, bool, bool), usize>,
) -> Result<usize> {
  if node == "out" {
    return if dac && fft { Ok(1) } else { Ok(0) };
  }

  let mut found = 0;

  let next_dac = dac || node == "dac";
  let next_fft = fft || node == "fft";

  if let Some(neighbors) = graph.get(&node) {
    for next_node in neighbors {
      let next_key = (next_node.to_string(), next_dac, next_fft);
      if let Some(&child_paths) = memo.get(&next_key) {
        found += child_paths;
      } else {
        let child_paths = memoized_dfs(next_node.to_string(), next_dac, next_fft, graph, memo)?;
        memo.insert(next_key, child_paths);
        found += child_paths;
      }
    }
  }

  Ok(found)
}

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    let mut map: FnvHashMap<String, FnvHashSet<String>> =
      FnvHashMap::with_capacity_and_hasher(lines.len(), FnvBuildHasher::default());
    for line in lines {
      let (input, rest) = line
        .split_once(": ")
        .context("Line does not contain ': '")?;
      let outputs = rest.split_whitespace().map(|s| s.to_owned());
      map.insert(input.to_owned(), FnvHashSet::from_iter(outputs));
    }
    Ok(map)
  }

  fn part1(&self, graph: &Parsed, _: Option<String>) -> Result<P1Out> {
    bfs(graph, "you", "out")
  }

  fn part2(&self, graph: &Parsed, _: Option<String>) -> Result<P2Out> {
    if !graph.contains_key(&"svr".to_owned()) {
      return Ok(0);
    }

    let mut memo: FnvHashMap<(String, bool, bool), usize> =
      FnvHashMap::with_capacity_and_hasher(graph.len(), FnvBuildHasher::default());
    memoized_dfs("svr".to_owned(), false, false, graph, &mut memo)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2025, 11)
}
