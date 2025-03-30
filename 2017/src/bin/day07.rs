use std::collections::VecDeque;

use advent_lib::runner::{Day, PartId};
use anyhow::{Context, Result, bail};
use fnv::{FnvBuildHasher, FnvHashMap, FnvHashSet};

#[derive(Debug)]
struct Node {
  id: String,
  weight: u32,
  children: Vec<String>,
}

type P1Out = String;
type P2Out = i32;
type Parsed = Vec<Node>;

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    let mut nodes: Vec<Node> = Vec::with_capacity(lines.len());
    for line in lines {
      let (id, r0) = line.split_once(" (").context("")?;
      let (weight_raw, r1) = r0.split_once(')').context("")?;
      let mut children = vec![];
      if !r1.is_empty() {
        let (_, r2) = r1.split_once(" -> ").context("")?;
        for child in r2.split(", ") {
          children.push(child.to_string());
        }
      }
      nodes.push(Node {
        id: id.to_string(),
        weight: weight_raw
          .parse()
          .with_context(|| "Cannot unwrap {weight_raw}")?,
        children,
      });
    }
    Ok(nodes)
  }

  fn part1(&self, nodes: &Parsed, _: Option<String>) -> Result<P1Out> {
    let mut all_children: FnvHashSet<&str> =
      FnvHashSet::with_capacity_and_hasher(nodes.len() - 1, FnvBuildHasher::default());
    for n0 in nodes {
      for child in n0.children.iter() {
        all_children.insert(child.as_str());
      }
    }
    for n1 in nodes {
      if !all_children.contains(&n1.id.as_str()) {
        return Ok(n1.id.to_string());
      }
    }
    bail!("Failed to find");
  }

  fn part2(&self, nodes: &Parsed, _: Option<String>) -> Result<P2Out> {
    // Step 1: Map out the parent/child relationships
    let mut tree: FnvHashMap<&str, Vec<&str>> =
      FnvHashMap::with_capacity_and_hasher(nodes.len(), FnvBuildHasher::default());
    for node in nodes {
      tree.insert(
        node.id.as_str(),
        node.children.iter().map(|c| c.as_str()).collect(),
      );
    }

    // Step 2: Map the node names to their weights and the sums of their child towers' weights
    let mut weights: FnvHashMap<&str, (u32, Vec<u32>)> =
      FnvHashMap::with_capacity_and_hasher(nodes.len(), FnvBuildHasher::default());
    let mut explore: VecDeque<&str> = nodes.iter().map(|n| n.id.as_str()).collect();
    'outer: while !explore.is_empty() {
      let top = explore.pop_front().context("")?;
      let node = nodes.iter().find(|n| n.id == top).context("")?;
      let mut child_weights: Vec<u32> = vec![];
      for child_key in node.children.iter() {
        if let Some((child_own, grandchildren)) = weights.get(&child_key.as_str()) {
          child_weights.push(child_own + grandchildren.iter().sum::<u32>());
        } else {
          explore.push_back(top);
          continue 'outer;
        }
      }
      weights.insert(top, (node.weight, child_weights));
    }

    // Step 3: Find the imbalance.
    let root_node = self.part1(nodes, None)?.to_string();
    let mut cursor = root_node.as_str();
    let mut imbalance = 0;
    println!("{weights:?}");
    loop {
      let (own_weight, child_weights) = weights.get(cursor).context("")?;

      // Figure out if we're imbalanced at this level
      let mut weight_map = FnvHashMap::<u32, u8>::with_hasher(FnvBuildHasher::default());
      for w in child_weights {
        weight_map.entry(*w).and_modify(|ww| *ww += 1).or_insert(1);
      }
      if weight_map.len() > 1 {
        let (&good, _) = weight_map.iter().find(|&(_, count)| *count > 1).unwrap();
        let (&bad, _) = weight_map.iter().find(|&(_, count)| *count == 1).unwrap();
        // TODO: Set cursor to the new child, and compute imbalance
      } else {
        // The imbalance is not here.
        // If it's already 0, then bail, the tree is in fact balanced.
        // Otherwise, do the math here to get the answer!
        if imbalance == 0 {
          bail!("Tree seemed to be balanced");
        } else {
          // TODO: Math time, might be wrong?
          return Ok((*own_weight as i32) - imbalance);
        }
      }
    }
  }
}

fn main() -> Result<()> {
  Solver {}.run(2017, 7)
}
