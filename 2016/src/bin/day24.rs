use std::collections::VecDeque;

use advent_lib::{
  grid::{Infinite2dGrid, Infinite2dSet},
  runner::{Day, PartId},
};
use anyhow::{Context, Result};
use fnv::{FnvBuildHasher, FnvHashMap};

type P1Out = usize;
type P2Out = usize;
type Parsed = (usize, usize, Infinite2dSet, FnvHashMap<char, (i32, i32)>);

// Create the distance matrix emanating from a start point.
// Pretty basic BFS, and the majority of the solution's runtime.
// On the real input, this takes nearly half a millisecond
// each time we do it.
fn distance_matrix(
  w: usize,
  h: usize,
  nodes: &Infinite2dSet,
  (start_x, start_y): (i32, i32),
) -> Infinite2dGrid<usize> {
  let mut dist: Infinite2dGrid<usize> = Infinite2dGrid::new(w * h);
  dist.set(start_x, start_y, 0);

  // Discover nodes we can reach and memorize the distance to each.
  let mut frontier: VecDeque<(i32, i32)> = VecDeque::from([(start_x, start_y)]);
  while !frontier.is_empty() {
    let (ux, uy) = frontier.pop_front().unwrap();
    let du = *dist.get(ux, uy).unwrap() + 1;
    for (nx, ny) in [(ux, uy - 1), (ux + 1, uy), (ux, uy + 1), (ux - 1, uy)] {
      if nodes.contains(nx, ny) {
        let dn = *dist.get(nx, ny).unwrap_or(&usize::MAX);
        if du < dn {
          dist.set(nx, ny, du);
          frontier.push_back((nx, ny));
        }
      }
    }
  }

  dist
}

fn solve(
  w: usize,
  h: usize,
  nodes: &Infinite2dSet,
  targets: &FnvHashMap<char, (i32, i32)>,
  include_return_trip: bool,
) -> Result<usize> {
  // Step 1: find the shortest path from each target to each other target.
  let mut path_lengths: FnvHashMap<(char, char), usize> =
    FnvHashMap::with_capacity_and_hasher(targets.len(), FnvBuildHasher::default());

  for (i, &a) in targets.keys().enumerate().take(targets.len() - 1) {
    // Build the distance matrix to all other reachable nodes
    let &a_pos = targets.get(&a).context("")?;
    let dist = distance_matrix(w, h, nodes, a_pos);

    // Memorize only the distances between nodes of interest, discarding the rest
    for &b in targets.keys().skip(i + 1) {
      let &(bx, by) = targets.get(&b).context("")?;
      let distance = *dist.get(bx, by).context("")?;
      path_lengths.insert((a, b), distance);
      path_lengths.insert((b, a), distance);
    }
  }

  // Step 2: check permutations of possible routes using the above distances.
  // Depth-first so that we can prune routes that can't possibly beat the
  // best one already found.
  let mut best = usize::MAX;
  let mut frontier = vec![(vec!['0'], 0_usize)];
  while !frontier.is_empty() {
    let (path, prev_len) = frontier.pop().context("")?;

    if prev_len < best {
      if path.len() == targets.len() {
        // This bit is for part 2 - we want the best cost that INCLUDES the return trip.
        let return_cost: usize = if include_return_trip {
          *path_lengths
            .get(&(*path.last().context("")?, '0'))
            .context("")?
        } else {
          0
        };
        let prev_cost = prev_len + return_cost;
        if prev_cost < best {
          best = prev_cost;
        }
      } else {
        for remain in targets.keys().filter(|&k| !path.contains(k)) {
          let mut next_path = path.clone();
          next_path.push(*remain);
          let step_len = *path_lengths
            .get(&(*path.last().context("")?, *remain))
            .context("")?;
          frontier.push((next_path, prev_len + step_len));
        }
      }
    }
  }

  Ok(best)
}

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    let h = lines.len();
    let w = lines[0].len();

    // Barely enough targets to be worth using a hash map at all, but it's slightly faster.
    let mut targets: FnvHashMap<char, (i32, i32)> =
      FnvHashMap::with_capacity_and_hasher(8, FnvBuildHasher::default());

    // Ignoring the outer wall, look for non-wall nodes.
    let mut nodes: Infinite2dSet = Infinite2dSet::new((w - 2) * (h - 2));
    for (y, row) in lines.iter().enumerate().skip(1).take(h - 2) {
      for (x, c) in row.chars().enumerate().skip(1).take(w - 2) {
        if c != '#' {
          let x2 = x as i32;
          let y2 = y as i32;
          nodes.insert(x2, y2);
          if c.is_ascii_digit() {
            targets.insert(c, (x2, y2));
          }
        }
      }
    }

    Ok((w, h, nodes, targets))
  }

  fn part1(&self, (w, h, nodes, targets): &Parsed, _: Option<String>) -> Result<P1Out> {
    solve(*w, *h, nodes, targets, false)
  }

  fn part2(&self, (w, h, nodes, targets): &Parsed, _: Option<String>) -> Result<P2Out> {
    solve(*w, *h, nodes, targets, true)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2016, 24)
}
