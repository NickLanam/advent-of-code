use std::{
  cmp::Reverse,
  collections::{BinaryHeap, VecDeque},
};

use advent_lib::{
  grid::{Infinite2dGrid, Infinite2dSet},
  runner::{Day, PartId},
};
use anyhow::{Result, bail};
use fnv::{FnvBuildHasher, FnvHashSet};

type P1Out = usize;
type P2Out = usize;

// To make later code legible. These are bitmaps.
type Keys = u32;
type Doors = u32;

struct Parsed {
  start: (i32, i32),
  walls: Infinite2dSet,
  keys: Infinite2dGrid<Keys>,
  doors: Infinite2dGrid<Doors>,
}

type Path = (usize, (i32, i32), Keys, Doors);
fn find_paths(input: &Parsed, start: (i32, i32)) -> Vec<Path> {
  let mut paths = vec![];
  let mut seen = Infinite2dSet::new(10);
  let mut frontier = VecDeque::new();
  frontier.push_back((0, start, 0));

  while let Some((cost, (fx, fy), doors_seen)) = frontier.pop_front() {
    if seen.insert(fx, fy) {
      if let Some(&key) = input.keys.get(fx, fy) {
        paths.push((cost, (fx, fy), key, doors_seen));
      }
      for (nx, ny) in [(fx, fy - 1), (fx + 1, fy), (fx, fy + 1), (fx - 1, fy)] {
        if !input.walls.contains(nx, ny) {
          frontier.push_back((
            cost + 1,
            (nx, ny),
            doors_seen | input.doors.get(nx, ny).unwrap_or(&0),
          ));
        }
      }
    }
  }

  return paths;
}

fn solve_area(input: &Parsed) -> Result<usize> {
  let mut path_cache: Infinite2dGrid<Vec<Path>> = Infinite2dGrid::new(10_000);
  for (px, py) in input.keys.keys().chain([input.start].iter().cloned()) {
    path_cache.insert(px, py, find_paths(input, (px, py)));
  }

  let mut frontier = BinaryHeap::new();
  frontier.push(Reverse((0, input.start, 0)));

  let mut seen =
    FnvHashSet::<(i32, i32, Keys)>::with_capacity_and_hasher(10_000, FnvBuildHasher::default());

  let all_keys = (1 << input.keys.len()) - 1;

  while let Some(Reverse((cost, (rx, ry), held_keys))) = frontier.pop() {
    if held_keys == all_keys {
      return Ok(cost);
    }
    if seen.insert((rx, ry, held_keys))
      && let Some(reachable) = path_cache.get(rx, ry)
    {
      for &(move_cost, (nx, ny), next_key, next_need) in reachable {
        if (held_keys & next_key == 0) && (next_need & !held_keys == 0) {
          frontier.push(Reverse((cost + move_cost, (nx, ny), held_keys | next_key)));
        }
      }
    }
  }

  bail!("Failed to find a path that collects every key");
}

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    let mut walls = Infinite2dSet::new(lines.len() * lines[0].len() / 2);
    let mut start = (0, 0);
    let mut keys = Infinite2dGrid::new(26);
    let mut doors = Infinite2dGrid::new(26);

    for (y, line) in lines.iter().enumerate() {
      for (x, c) in line.chars().enumerate() {
        match c {
          '#' => {
            walls.insert(x as i32, y as i32);
          }
          '@' => {
            start = (x as i32, y as i32);
          }
          '.' => {}
          c if c.is_ascii_lowercase() => {
            let key_id = 1_u32 << ((c as u8) - b'a');
            keys.insert(x as i32, y as i32, key_id);
          }
          c if c.is_ascii_uppercase() => {
            let door_id = 1_u32 << ((c as u8) - b'A');
            doors.insert(x as i32, y as i32, door_id);
          }
          _ => bail!("Can't parse character: {c}"),
        }
      }
    }

    Ok(Parsed {
      start,
      walls,
      keys,
      doors,
    })
  }

  fn part1(&self, input: &Parsed, _: Option<String>) -> Result<P1Out> {
    solve_area(input)
  }

  fn part2(&self, _input: &Parsed, _: Option<String>) -> Result<P2Out> {
    // TODO: Split the input into four sections, removing doors in each quadrant for which the key is in
    // a different quadrant. Solve each area independently this way, and add their results together.
    // Should take just 10% as long as part 1 does due to massively reduced search spaces.
    Ok(0)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2019, 18)
}
