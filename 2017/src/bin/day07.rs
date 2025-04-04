use std::{collections::VecDeque, rc::Rc};

use advent_lib::runner::{Day, PartId};
use anyhow::{Context, Result, bail};
use fnv::{FnvBuildHasher, FnvHashMap, FnvHashSet};

struct Node {
  id: String,
  node_weight: i32,
  total_weight: i32,
  children: Vec<Rc<Node>>,
}

type P1Out = String;
type P2Out = i32;
type Parsed = Rc<Node>;

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    // Part 1's answer is needed to construct the graph for part 2, so figure it out in the parse.
    let mut all_children: FnvHashSet<&str> =
      FnvHashSet::with_capacity_and_hasher(lines.len() - 1, FnvBuildHasher::default());
    let mut explore: VecDeque<&str> = VecDeque::with_capacity(lines.len());

    // To build the graph, we first must know the initial weights and edges
    let mut initial_nodes: FnvHashMap<&str, (i32, Vec<&str>)> =
      FnvHashMap::with_capacity_and_hasher(lines.len(), FnvBuildHasher::default());

    for line in lines.iter() {
      let (id, r0) = line.split_once(" (").context("")?;
      explore.push_back(id);
      let (weight_raw, r1) = r0.split_once(')').context("")?;
      let mut child_names = vec![];
      if !r1.is_empty() {
        let (_, r2) = r1.split_once(" -> ").context("")?;
        for child_name in r2.split(", ") {
          child_names.push(child_name);
          all_children.insert(child_name);
        }
      }
      initial_nodes.insert(id, (weight_raw.parse().context("")?, child_names));
    }

    // This is the part 1 answer, and the root of the graph for part 2
    let &root_id = initial_nodes
      .keys()
      .find(|&id| !all_children.contains(id))
      .context("")?;

    // Storing RefCounts in the backing HashMap allows returning references
    // to the caller, as well as avoiding any cloning of actual nodes, and in
    // theory lets the tree have cycles or nodes with multiple parents (though
    // this puzzle doesn't do that - it's a directed acyclic graph).
    let mut hydrated_nodes = FnvHashMap::<&str, Rc<Node>>::with_capacity_and_hasher(
      lines.len(),
      FnvBuildHasher::default(),
    );

    // Using a deque, checks elements at the front of the line for
    // whether all of their children have already been processed.
    // Return to the back of the line if not.
    // After one full pass, all leaf nodes are processed and removed
    // from the deque this way, then further passes walk their way
    // back towards the root.
    'outer: while !explore.is_empty() {
      let top = explore.pop_front().context("")?;
      let (own_weight, child_names) = initial_nodes.get(top).context("")?;
      let mut children: Vec<Rc<Node>> = vec![];
      let mut child_sum = 0;
      for &child_key in child_names {
        if let Some(child) = hydrated_nodes.get(child_key) {
          child_sum += child.total_weight;
          // Only cloning the RefCount, much cheaper than cloning the node!
          children.push(child.clone());
        } else {
          explore.push_back(top);
          continue 'outer;
        }
      }
      let new_node = Rc::new(Node {
        id: top.to_string(),
        node_weight: *own_weight,
        total_weight: *own_weight + child_sum,
        children,
      });
      hydrated_nodes.insert(top, new_node);
    }

    // Finally, return the root node
    // The clone is on an Rc<Node>, so it's cheap!
    Ok(hydrated_nodes.get(root_id).context("")?.clone())
  }

  fn part1(&self, tree: &Parsed, _: Option<String>) -> Result<P1Out> {
    Ok(tree.id.to_string())
  }

  fn part2(&self, tree: &Parsed, _: Option<String>) -> Result<P2Out> {
    let mut cursor = tree;
    let mut imbalance = 0;
    loop {
      let Node {
        node_weight,
        children,
        ..
      } = &**cursor;

      // Check how many unique weights this level has (it'll be either 0, 1, or 2).
      let mut weight_incidence = FnvHashMap::<i32, u8>::with_hasher(FnvBuildHasher::default());
      for child in children {
        weight_incidence
          .entry(child.total_weight)
          .and_modify(|cw| *cw += 1)
          .or_insert(1);
      }

      if weight_incidence.len() > 1 {
        // The imbalance is in a child of this node,
        // write down the gap and scan from there.
        let (&good, _) = weight_incidence
          .iter()
          .find(|&(_, count)| *count > 1)
          .context("")?;
        let (&bad, _) = weight_incidence
          .iter()
          .find(|&(_, count)| *count == 1)
          .context("")?;
        imbalance = good - bad;
        cursor = children
          .iter()
          .find(|c| c.total_weight == bad)
          .context("")?;
        continue;
      } else {
        // The imbalance either doesn't exist (an error), or this node is it
        if imbalance == 0 {
          bail!("Tree appears to be balanced; problem description promised that would not happen");
        } else {
          return Ok(node_weight + imbalance);
        }
      }
    }
  }
}

fn main() -> Result<()> {
  Solver {}.run(2017, 7)
}
