use advent_lib::runner::{Day, PartId};
use anyhow::{Context, Result};
use petgraph::graph::{DiGraph, NodeIndex};
use regex::Regex;
use std::{cmp::Ordering, collections::HashMap};

type P1Out = i32;
type P2Out = i32;
type Parsed = DiGraph<String, i32>;

struct StackEntry {
  cost: i32,
  path: Vec<NodeIndex>,
  remain: Vec<NodeIndex>,
}

fn solve(graph: &Parsed) -> Result<i32> {
  // First, each starting node becomes a path of one, with the "untaken" nodes as the remaining items to pick, and no cost yet.
  let all: Vec<NodeIndex> = graph.node_indices().collect();
  let mut stack: Vec<StackEntry> = vec![];
  for ni in all.as_slice() {
    let path = vec![ni.to_owned()];
    let remain = all
      .as_slice()
      .iter()
      .filter(|other| ni.cmp(other) != Ordering::Equal).copied()
      .collect();
    stack.push(StackEntry {
      path,
      remain,
      cost: 0,
    });
  }

  // Brute force every ordering, keeping track of the best cost we see on completed paths.
  let mut best_cost = i32::MIN;
  while let Some(entry) = stack.pop() {
    if entry.path.len() > all.len() {
      best_cost = best_cost.max(entry.cost);
      continue;
    }

    for next in entry.remain.as_slice() {
      let last = *entry.path.last().unwrap();
      let ab = graph.find_edge(last, *next).unwrap();
      let ba = graph.find_edge(*next, last).unwrap();

      let edge_cost_a = graph.edge_weight(ab).unwrap();
      let edge_cost_b = graph.edge_weight(ba).unwrap();

      let mut next_path: Vec<NodeIndex> = entry.path.clone();
      next_path.push(next.to_owned());

      let mut next_remain: Vec<NodeIndex> = entry
        .remain
        .as_slice()
        .iter()
        .filter(|r| next.cmp(r) != Ordering::Equal).copied()
        .collect();

      // A little different from day 9: we have to close the loop when done.
      if next_remain.is_empty() && next_path.first().cmp(&next_path.last()) != Ordering::Equal {
        next_remain.push(*next_path.first().unwrap());
      }

      stack.push(StackEntry {
        path: next_path,
        remain: next_remain,
        cost: entry.cost + edge_cost_a + edge_cost_b,
      });
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
    for_part: PartId,
  ) -> Result<Parsed> {
    let re = Regex::new(
      r"^(?<a>[^\s]+) would (?<mode>gain|lose) (?<amount>\d+) happiness units by sitting next to (?<b>.+)\.$",
    )?;
    let mut graph = DiGraph::<String, i32>::new();
    let mut node_keys = HashMap::<String, NodeIndex<u32>>::new();

    let mut key_for = |n: &str, g: &mut DiGraph<String, i32>| -> NodeIndex<u32> {
      if node_keys.contains_key(n) {
        *node_keys.get(n).unwrap()
      } else {
        let k = g.add_node(n.to_string());
        node_keys.insert(n.to_string(), k);
        k
      }
    };

    for line in lines.into_iter() {
      let caps = re.captures(line.as_str()).context("Bad parser regex")?;
      let a = caps.name("a").unwrap().as_str();
      let an = key_for(a, &mut graph);

      let b = caps.name("b").unwrap().as_str();
      let bn = key_for(b, &mut graph);

      let mode = caps.name("mode").unwrap().as_str().to_string();
      let amount = caps
        .name("amount")
        .unwrap()
        .as_str()
        .parse::<i32>()
        .unwrap();
      let diff: i32 = if mode == "gain" { amount } else { -amount };

      graph.add_edge(an, bn, diff);
    }

    if for_part == PartId::P2 {
      let you_ni = graph.add_node("You".to_string());
      for ni in graph.node_indices() {
        if ni.cmp(&you_ni) == Ordering::Equal {
          continue;
        }
        graph.add_edge(you_ni, ni, 0);
        graph.add_edge(ni, you_ni, 0);
      }
    }

    Ok(graph)
  }

  fn part1(&self, graph: &Parsed, _sample_name: Option<String>) -> Result<P1Out> {
    solve(graph)
  }

  fn part2(&self, graph: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    solve(graph)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2015, 13)
}
