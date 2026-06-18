use std::collections::{BinaryHeap, VecDeque};

use advent_lib::{
  grid::{Infinite2dGrid, Infinite2dSet},
  runner::{Day, PartId},
};
use anyhow::{Result, bail};
use fnv::{FnvBuildHasher, FnvHashMap};

type P1Out = usize;
type P2Out = usize;

const DEPTH_LIMIT: usize = 1_000;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Portal {
  id: String,
  is_inner: bool,
  partner_x: i32,
  partner_y: i32,
}

struct Maze {
  entry: (i32, i32),
  exit: (i32, i32),
  dots: Infinite2dSet,
  portals: Infinite2dGrid<Portal>,
}

#[derive(Clone, Eq, PartialEq)]
struct State {
  cost: usize,
  depth: usize,
  x: i32,
  y: i32,
}
impl Ord for State {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    other
      .depth
      .cmp(&self.depth)
      .then(other.cost.cmp(&self.cost))
  }
}
impl PartialOrd for State {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

struct Solver;
impl Day<Maze, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Maze> {
    // Facts to consider:
    // - Each label marks the only dot it touches as a key location
    // - AA is the entrance, ZZ is the exit
    // - All others have two points with the same label - landing on one teleports to the other,
    //   skipping the initial point entirely (meaning it takes only 1 move to enter and use a portal).
    // - The empty spaces in the donut's center are unreachable. Label marks are too.
    // - There are far more walls than there are dots, so represent the dots and portals only.

    // Start by turning into a 2d array so we can peek nearby labels when seeing a dot.
    let h = lines.len();
    let w = lines[0].len();
    let array_grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    let mut entry = (0, 0);
    let mut exit = (0, 0);
    let mut dots = Infinite2dSet::new(w * h / 2);
    let mut portals = Infinite2dGrid::new(150);

    // name, is_inner, x, y (of the unmatched portal - when we create the matched Portal struct, it points to the partner)
    let mut unmatched: Vec<(String, bool, i32, i32)> = vec![];

    let mut mark_notable = |x: i32, y: i32, c0: char, c1: char| {
      let s = format!("{c0}{c1}");
      if s == "AA" {
        entry = (x, y);
      } else if s == "ZZ" {
        exit = (x, y);
      } else if let Some(match_index) = unmatched.iter().position(|(name, _, _, _)| *name == s) {
        let (_, was_inner, x2, y2) = unmatched.remove(match_index);
        portals.insert(
          x,
          y,
          Portal {
            id: s.to_owned(),
            is_inner: !was_inner,
            partner_x: x2,
            partner_y: y2,
          },
        );
        portals.insert(
          x2,
          y2,
          Portal {
            id: s.to_owned(),
            is_inner: was_inner,
            partner_x: x,
            partner_y: y,
          },
        );
      } else {
        let inner = x > 2 && (x as usize) < (w - 3) && y > 2 && (y as usize) < (h - 3);
        unmatched.push((s, inner, x, y));
      }
    };

    for y in 0..h {
      for x in 0..w {
        let c = array_grid[y][x];
        // We're not doing anything directly with any other options here.
        if c == '.' {
          dots.insert(x as i32, y as i32);
          // Look for a label - if we find its partner, we'll put both directions in the map
          // Exceptions: AA and ZZ are the entry and exit, respectively

          // North
          match (array_grid[y - 2][x], array_grid[y - 1][x]) {
            (c0, c1) if c0.is_ascii_uppercase() && c1.is_ascii_uppercase() => {
              mark_notable(x as i32, y as i32, c0, c1);
            }
            _ => {}
          }
          // East
          match (array_grid[y][x + 1], array_grid[y][x + 2]) {
            (c0, c1) if c0.is_ascii_uppercase() && c1.is_ascii_uppercase() => {
              mark_notable(x as i32, y as i32, c0, c1);
            }
            _ => {}
          }
          // South
          match (array_grid[y + 1][x], array_grid[y + 2][x]) {
            (c0, c1) if c0.is_ascii_uppercase() && c1.is_ascii_uppercase() => {
              mark_notable(x as i32, y as i32, c0, c1);
            }
            _ => {}
          }
          // West
          match (array_grid[y][x - 2], array_grid[y][x - 1]) {
            (c0, c1) if c0.is_ascii_uppercase() && c1.is_ascii_uppercase() => {
              mark_notable(x as i32, y as i32, c0, c1);
            }
            _ => {}
          }
        }
      }
    }

    if !unmatched.is_empty() {
      bail!("Failed to find matching portals for: {unmatched:?}");
    }

    Ok(Maze {
      entry,
      exit,
      dots,
      portals,
    })
  }

  /// Given the parsed maze (the hard part of this puzzle),
  /// do a basic BFS where the portal locations cause neighbors to be weird.
  /// Travel cost is always 1, so Dijkstra's is overkill - BFS already solves optimally.
  fn part1(&self, maze: &Maze, _: Option<String>) -> Result<P1Out> {
    let mut seen = Infinite2dSet::new(maze.dots.len() + maze.portals.len());
    let mut frontier: VecDeque<(i32, i32, usize)> = VecDeque::new();
    frontier.push_back((maze.entry.0, maze.entry.1, 0));
    seen.insert(maze.entry.0, maze.entry.1);

    while let Some((fx, fy, cost)) = frontier.pop_front() {
      if fx == maze.exit.0 && fy == maze.exit.1 {
        return Ok(cost);
      }

      for (nx, ny) in [(fx, fy - 1), (fx + 1, fy), (fx, fy + 1), (fx - 1, fy)] {
        if let Some(Portal {
          partner_x,
          partner_y,
          ..
        }) = maze.portals.get(nx, ny)
        {
          if seen.insert(nx, ny) {
            seen.insert(*partner_x, *partner_y);
            // Walking onto a portal space doesn't do anything, but makes the other end a neighbor.
            // Every portal location is inside a dead end, so we can skip the extra step and always
            // assume that we'll go through it.
            frontier.push_back((*partner_x, *partner_y, cost + 2));
          }
        } else if (maze.dots.contains(nx, ny) || (nx == maze.exit.0 && ny == maze.exit.1))
          && seen.insert(nx, ny)
        {
          frontier.push_back((nx, ny, cost + 1));
        }
      }
    }

    bail!("Failed to find a path");
  }

  /// Same idea as part 1, but now we track depth and reachability as well.
  /// Dijkstra's this time, so that we can prioritize low depth over low cost.
  fn part2(&self, maze: &Maze, _: Option<String>) -> Result<P2Out> {
    // (x, y, depth) -> cost
    let mut dist: FnvHashMap<(i32, i32, usize), usize> = FnvHashMap::with_capacity_and_hasher(
      maze.dots.len() + maze.portals.len(),
      FnvBuildHasher::default(),
    );
    dist.insert((maze.entry.0, maze.entry.1, 0), 0);

    let mut heap = BinaryHeap::<State>::new();
    heap.push(State {
      cost: 0,
      depth: 0,
      x: maze.entry.0,
      y: maze.entry.1,
    });

    while let Some(State { cost, depth, x, y }) = heap.pop() {
      if depth >= DEPTH_LIMIT {
        println!("Hit depth limit, something went wrong");
        break;
      }

      if x == maze.exit.0 && y == maze.exit.1 && depth == 0 {
        return Ok(cost);
      }

      if let Some(&dist_cost) = dist.get(&(x, y, depth))
        && cost > dist_cost
      {
        continue;
      }

      for (nx, ny) in [(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)] {
        if let Some(Portal {
          is_inner: was_inner,
          partner_x,
          partner_y,
          ..
        }) = maze.portals.get(nx, ny)
        {
          if !was_inner && depth == 0 {
            // Can't go out when already at the outermost layer
            continue;
          }
          let next_depth = if *was_inner { depth + 1 } else { depth - 1 };
          let dist_exit = dist
            .entry((*partner_x, *partner_y, next_depth))
            .or_insert(usize::MAX);
          if cost + 2 < *dist_exit {
            *dist_exit = cost + 2;
            heap.push(State {
              cost: cost + 2,
              depth: next_depth,
              x: *partner_x,
              y: *partner_y,
            });
          }
        } else if maze.dots.contains(nx, ny) || (nx == maze.exit.0 && ny == maze.exit.1) {
          let dist_next = dist.entry((nx, ny, depth)).or_insert(usize::MAX);
          if cost + 1 < *dist_next {
            *dist_next = cost + 1;
            heap.push(State {
              cost: cost + 1,
              depth,
              x: nx,
              y: ny,
            });
          }
        }
      }
    }

    bail!("Failed to find a path")
  }
}

fn main() -> Result<()> {
  Solver {}.run(2019, 20)
}
