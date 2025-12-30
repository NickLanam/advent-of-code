use std::collections::VecDeque;

use advent_lib::{
  grid::{Infinite2dGrid, Infinite2dSet},
  runner::{Day, PartId},
};
use anyhow::Result;
use fnv::{FnvBuildHasher, FnvHashMap};

type P1Out = usize;
type P2Out = usize;

struct Parsed {
  start: (i32, i32),
  walls: Infinite2dSet,
  key_pos: FnvHashMap<u8, (i32, i32)>,
  pos_key: Infinite2dGrid<u8>,
  #[allow(unused)]
  door_pos: FnvHashMap<u8, (i32, i32)>,
  pos_door: Infinite2dGrid<u8>,
}

// Computes reachable positions with a given position and set of held keys, including
// how many steps it takes to reach each one. Used by search_outer to find valid moves.
struct KeyMove {
  key_id: u8,
  key_x: i32,
  key_y: i32,
  cost: usize,
}
fn search_inner(
  input: &Parsed,
  held_keys: &Vec<u8>,
  (start_x, start_y): (i32, i32),
) -> Vec<KeyMove> {
  let mut moves: Vec<KeyMove> = vec![];
  let mut touched = Infinite2dSet::new(10);
  let mut frontier: VecDeque<(i32, i32, usize)> = VecDeque::new();
  frontier.push_back((start_x, start_y, 0));

  while let Some((fx, fy, cost)) = frontier.pop_front() {
    if touched.contains(fx, fy) || input.walls.contains(fx, fy) {
      touched.insert(fx, fy);
      continue;
    }
    touched.insert(fx, fy);

    // Found a key we don't have yet -> remember this for outer BFS
    if let Some(&key_id) = input.pos_key.get(fx, fy)
      && !held_keys.contains(&key_id)
    {
      moves.push(KeyMove {
        key_id,
        key_x: fx,
        key_y: fy,
        cost,
      });
      // Skip adding neighbors from here - we'll get keys after this one later
      // (otherwise we'd have to account for moves that pick up one key on the way to another)
      continue;
    }

    // Don't try to go through doors that we can't open
    if let Some(&door_id) = input.pos_door.get(fx, fy)
      && !held_keys.contains(&door_id)
    {
      continue;
    }

    // Explore neighbors
    for (nx, ny) in [(fx, fy - 1), (fx + 1, fy), (fx, fy + 1), (fx - 1, fy)] {
      frontier.push_back((nx, ny, cost + 1));
    }
  }

  moves
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct SearchNode {
  held_keys: Vec<u8>,
  x: i32,
  y: i32,
}
/// Dijkstra's to find a meta-path, discovering nodes along the way.
/// Similar to about a dozen other Advent of Code puzzles, really.
/// TODO: Still very slow to solve (minute or so), I need to cache
/// the hot path and avoid duplicate explorations better
fn search_outer(input: &Parsed) -> usize {
  let mut frontier: Vec<SearchNode> = vec![SearchNode {
    held_keys: Vec::with_capacity(26),
    x: input.start.0,
    y: input.start.1,
  }];

  let mut dist: FnvHashMap<SearchNode, usize> =
    FnvHashMap::with_capacity_and_hasher(100, FnvBuildHasher::default());
  let mut prev: FnvHashMap<SearchNode, SearchNode> =
    FnvHashMap::with_capacity_and_hasher(100, FnvBuildHasher::default());

  dist.insert(frontier[0].clone(), 0);

  let mut best_goal = usize::MAX;

  while !frontier.is_empty() {
    let (min_index, _) = frontier
      .iter()
      .enumerate()
      .min_by_key(|(_i, node)| dist.get(node).unwrap_or(&usize::MAX))
      .unwrap();
    let current = frontier.remove(min_index);

    let current_cost = dist.get(&current).map(|&o| o);
    // println!("search_outer: {current:?} costs {current_cost:?}");

    for KeyMove {
      key_id,
      key_x,
      key_y,
      cost,
    } in search_inner(input, &current.held_keys, (current.x, current.y))
    {
      let mut next_keys = current.held_keys.clone();
      next_keys.push(key_id);
      next_keys.sort(); // For stable checking - we don't ultimate care about the order

      // Part 1 answer while we're here
      if let Some(current_cost) = current_cost
        && next_keys.len() == input.key_pos.len()
      {
        best_goal = best_goal.min(current_cost + cost);
        continue; // No need to grab neighbors now
      }

      let next_node = SearchNode {
        held_keys: next_keys,
        x: key_x,
        y: key_y,
      };
      if !frontier.contains(&next_node) {
        frontier.push(next_node.clone()); // Discovering options along the way
      }
      let alt = dist
        .get(&current)
        .unwrap_or(&usize::MAX)
        .saturating_add(cost);
      // println!("  Next {next_node:?}, alt={alt}");
      if alt < *dist.get(&next_node).unwrap_or(&usize::MAX) {
        dist.insert(next_node.clone(), alt);
        prev.insert(next_node.clone(), current.clone());
      }
    }
  }

  /* Reconstructing, if part 2 asks for it:
  S ← empty sequence
  u ← target
  if prev[u] is defined or u = source: // Proceed if the vertex is reachable
      while u is defined:              // Construct the shortest path with a stack S
          S.push(u)                    // Push the vertex onto the stack
          u ← prev[u]                  // Traverse from target to source
  */

  // println!("\x1B[33mSolution: {best_goal}\x1B[0m");
  best_goal
}

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    let mut walls = Infinite2dSet::new(lines.len() * lines[0].len() / 2);
    let mut start = (0, 0);
    let mut key_pos = FnvHashMap::with_capacity_and_hasher(10, FnvBuildHasher::default());
    let mut pos_key = Infinite2dGrid::new(10);
    let mut door_pos = FnvHashMap::with_capacity_and_hasher(10, FnvBuildHasher::default());
    let mut pos_door = Infinite2dGrid::new(10);

    for (y, line) in lines.iter().enumerate() {
      for (x, c) in line.chars().enumerate() {
        if c.is_ascii_lowercase() {
          key_pos.insert((c as u8) - ('a' as u8), (x as i32, y as i32));
          pos_key.insert(x as i32, y as i32, (c as u8) - ('a' as u8));
        } else if c.is_ascii_uppercase() {
          door_pos.insert((c as u8) - ('A' as u8), (x as i32, y as i32));
          pos_door.insert(x as i32, y as i32, (c as u8) - ('A' as u8));
        } else if c == '#' {
          walls.insert(x as i32, y as i32);
        } else if c == '@' {
          start = (x as i32, y as i32);
        }
      }
    }

    Ok(Parsed {
      start,
      walls,
      key_pos,
      pos_key,
      door_pos,
      pos_door,
    })
  }

  // Two search layers: inner layer finds reachable keys and the number of steps to each.
  // Outer layer tries each of those outcomes to discover new outcomes, summing steps
  // Goal state is when all keys are collected (doors do not actually matter for this)
  fn part1(&self, input: &Parsed, _: Option<String>) -> Result<P1Out> {
    Ok(search_outer(input))
  }

  fn part2(&self, _input: &Parsed, _: Option<String>) -> Result<P2Out> {
    Ok(0)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2019, 18)
}
