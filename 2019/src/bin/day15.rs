use std::collections::VecDeque;

use advent_lib::{
  grid::Infinite2dGrid,
  runner::{Day, PartId},
};
use advent_of_code_2019::intcode::execute;
use anyhow::{Context, Result, bail};

type P1Out = usize;
type P2Out = usize;
type Parsed = Infinite2dGrid<Tile>;

#[derive(Copy, Clone)]
enum Day15Dir {
  #[allow(unused)]
  Invalid, // Not allowed
  North, // 1
  South,
  West,
  East, // 3
}
impl Into<i64> for Day15Dir {
  fn into(self) -> i64 {
    match &self {
      Day15Dir::Invalid => panic!("0 is not a valid direction to turn"),
      Day15Dir::North => 1,
      Day15Dir::South => 2,
      Day15Dir::West => 3,
      Day15Dir::East => 4,
    }
  }
}
impl Day15Dir {
  fn apply(&self, x: i32, y: i32) -> (i32, i32) {
    match self {
      Day15Dir::Invalid => panic!("Invalid direction to move"),
      Day15Dir::North => (x, y - 1),
      Day15Dir::South => (x, y + 1),
      Day15Dir::West => (x - 1, y),
      Day15Dir::East => (x + 1, y),
    }
  }
  fn rotate_left(&self) -> Day15Dir {
    match self {
      Day15Dir::Invalid => panic!("Can't rotate from invalid direction"),
      Day15Dir::North => Day15Dir::West,
      Day15Dir::South => Day15Dir::East,
      Day15Dir::West => Day15Dir::South,
      Day15Dir::East => Day15Dir::North,
    }
  }
  fn rotate_right(&self) -> Day15Dir {
    match self {
      Day15Dir::Invalid => panic!("Can't rotate from invalid direction"),
      Day15Dir::North => Day15Dir::East,
      Day15Dir::South => Day15Dir::West,
      Day15Dir::West => Day15Dir::North,
      Day15Dir::East => Day15Dir::South,
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Tile {
  Wall,
  Empty,
  Goal,
}
impl From<i64> for Tile {
  fn from(value: i64) -> Self {
    match value {
      0 => Tile::Wall,
      1 => Tile::Empty,
      2 => Tile::Goal,
      _ => panic!("Not a tile: {value}"),
    }
  }
}

/// Thankfully, this maze does not have any island walls - we can just follow along the edge
/// to discover the entire maze, then run a standard BFS on the result to answer both questions.
fn discover_maze(initial_tape: &[i64]) -> Result<Infinite2dGrid<Tile>> {
  let mut known: Infinite2dGrid<Tile> = Infinite2dGrid::new(1_000);

  // We follow the right wall, then start over and follow the left wall.
  // This handles an edge case that actually comes up for my input.
  for flip in [false, true] {
    let mut bot_x = 0;
    let mut bot_y = 0;
    let mut goal_found = false;

    let mut dir: Day15Dir = Day15Dir::North;
    let mut result = execute(initial_tape, &[dir.into()], Some(0), Some(0))?;
    while !goal_found {
      // Follow the wall to find every position in the grid.
      let outcome = Tile::from(*result.outputs.get(0).context("No output happened")?);
      match outcome {
        Tile::Wall => {
          let (wall_x, wall_y) = dir.apply(bot_x, bot_y);
          dir = if flip {
            dir.rotate_right()
          } else {
            dir.rotate_left()
          };
          known.entry(wall_x, wall_y).or_insert(outcome);
        }
        Tile::Empty => {
          (bot_x, bot_y) = dir.apply(bot_x, bot_y);
          dir = if flip {
            dir.rotate_left()
          } else {
            dir.rotate_right()
          };
          known.entry(bot_x, bot_y).or_insert(outcome);
        }
        Tile::Goal => {
          goal_found = true;
          (bot_x, bot_y) = dir.apply(bot_x, bot_y);
          known.entry(bot_x, bot_y).or_insert(outcome);
        }
      }
      result = execute(
        &result.final_tape,
        &[dir.into()],
        Some(result.pc),
        Some(result.ro),
      )?;
    }
  }

  Ok(known)
}

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    let instructions: Vec<i64> = lines[0].split(',').map(|n| n.parse().unwrap()).collect();
    discover_maze(&instructions)
  }

  fn part1(&self, grid: &Parsed, _: Option<String>) -> Result<P1Out> {
    let mut costs: Infinite2dGrid<usize> = Infinite2dGrid::new(1_000);
    costs.insert(0, 0, 0);
    let mut frontier: VecDeque<(i32, i32, usize)> = VecDeque::from([(0, 0, 0)]);
    while let Some((fx, fy, cost)) = frontier.pop_front() {
      match grid.get(fx, fy) {
        None => {}
        Some(Tile::Empty) => {}
        Some(Tile::Wall) => {
          continue;
        }
        Some(Tile::Goal) => {
          return Ok(cost);
        }
      }
      for (nx, ny) in [(fx, fy - 1), (fx, fy + 1), (fx - 1, fy), (fx + 1, fy)] {
        let seen = costs.contains_key(nx, ny);
        let entry = costs.entry(nx, ny).or_insert(1_000);
        *entry = (cost + 1).min(*entry);
        if !seen {
          frontier.push_back((nx, ny, *entry));
        }
      }
    }

    bail!("Failed to find the goal");
  }

  fn part2(&self, grid: &Parsed, _: Option<String>) -> Result<P2Out> {
    let (start_x, start_y, _) = grid
      .entries()
      .find(|(_x, _y, t)| **t == Tile::Goal)
      .context("Goal was not found")?;

    let mut max_depth = 0;

    let mut costs: Infinite2dGrid<usize> = Infinite2dGrid::new(1_000);
    costs.insert(start_x, start_y, 0);
    let mut frontier: VecDeque<(i32, i32, usize)> = VecDeque::from([(start_x, start_y, 0)]);
    while let Some((fx, fy, cost)) = frontier.pop_front() {
      if let Some(Tile::Wall) = grid.get(fx, fy) {
        continue;
      }
      max_depth = max_depth.max(cost);
      for (nx, ny) in [(fx, fy - 1), (fx, fy + 1), (fx - 1, fy), (fx + 1, fy)] {
        let seen = costs.contains_key(nx, ny);
        let entry = costs.entry(nx, ny).or_insert(1_000);
        *entry = (cost + 1).min(*entry);
        if !seen {
          frontier.push_back((nx, ny, *entry));
        }
      }
    }
    Ok(max_depth)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2019, 15)
}
