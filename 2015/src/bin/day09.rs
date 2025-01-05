use advent_lib::runner::{Day, PartId};
use anyhow::{Context, Result};
use petgraph::graph::{NodeIndex, UnGraph};
use regex::Regex;
use std::{cmp::Ordering, collections::HashMap};

type P1Out = u64;
type P2Out = u64;
type Parsed = UnGraph<String, u64>;

struct StackEntry {
  cost: u64,
  path: Vec<NodeIndex>,
  remain: Vec<NodeIndex>,
}

fn solve(graph: &Parsed, max_instead: bool) -> Result<u64> {
  // First, each starting node becomes a path of one, with the "untaken" nodes as the remaining items to pick, and no cost yet.
  let all: Vec<NodeIndex> = graph.node_indices().collect();
  let mut stack: Vec<StackEntry> = vec![];
  for ni in all.as_slice() {
    let path = vec![ni.to_owned()];
    let remain = all
      .as_slice()
      .into_iter()
      .filter(|other| ni.cmp(other) != Ordering::Equal)
      .map(|o| *o)
      .collect();
    stack.push(StackEntry {
      path,
      remain,
      cost: 0,
    });
  }

  // Brute force every ordering, keeping track of the best cost we see on completed paths.
  let mut best_cost: u64 = if max_instead { u64::MIN } else { u64::MAX };
  while let Some(entry) = stack.pop() {
    if entry.path.len() == all.len() {
      best_cost = if max_instead {
        best_cost.max(entry.cost)
      } else {
        best_cost.min(entry.cost)
      };
      continue;
    }
    for next in entry.remain.as_slice() {
      // In the sample, the graph isn't fully connected - just throw those paths away
      if let Some(edge) = graph.find_edge(*entry.path.last().unwrap(), *next) {
        let mut next_path: Vec<NodeIndex> = entry.path.clone();
        next_path.push(next.to_owned());

        let next_remain: Vec<NodeIndex> = entry
          .remain
          .as_slice()
          .into_iter()
          .filter(|r| next.cmp(r) != Ordering::Equal)
          .map(|o| *o)
          .collect();

        let edge_cost = graph
          .edge_weight(edge)
          .context("Found edge but did not find cost")?;

        stack.push(StackEntry {
          path: next_path,
          remain: next_remain,
          cost: entry.cost + edge_cost,
        });
      }
    }
  }

  Ok(best_cost)
}

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    let re = Regex::new(r"(?<a>[^\s]+) to (?<b>[^\s]+) = (?<cost>\d+)")?;
    let mut graph = UnGraph::<String, u64>::new_undirected();
    let mut node_keys = HashMap::<String, NodeIndex<u32>>::new();
    for line in lines.into_iter() {
      let caps = re.captures(line.as_str()).context("Bad parser regex")?;
      let a = caps.name("a").unwrap().as_str().to_string();
      let an: NodeIndex<u32>;
      if node_keys.contains_key(&a) {
        an = *node_keys.get(&a).to_owned().unwrap();
      } else {
        an = graph.add_node(a.clone());
        node_keys.insert(a.clone(), an);
      }

      let b = caps.name("b").unwrap().as_str().to_string();
      let bn: NodeIndex<u32>;
      if node_keys.contains_key(&b) {
        bn = *node_keys.get(&b).to_owned().unwrap();
      } else {
        bn = graph.add_node(b.clone());
        node_keys.insert(b.clone(), bn);
      }

      let cost = caps.name("cost").unwrap().as_str().parse::<u64>().unwrap();

      graph.add_edge(an, bn, cost);
    }
    Ok(graph)
  }

  fn part1(&self, graph: &Parsed, _sample_name: Option<String>) -> Result<P1Out> {
    solve(graph, false)
  }

  fn part2(&self, graph: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    solve(graph, true)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2015, 9)
}
