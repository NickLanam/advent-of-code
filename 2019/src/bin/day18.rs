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
use itertools::Itertools;

type P1Out = usize;
type P2Out = usize;

// To make later code legible. These are bitmaps.
type Keys = u32;
type Doors = u32;

#[derive(Clone)]
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

  paths
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

  let all_keys = input.keys.values().fold(0_u32, |acc, &key_id| acc | key_id);

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

  /// Split into four regions, each of which ignores doors whose keys are in other regions.
  /// Solve each independently, sum their answers, and that's what we're looking for.
  fn part2(&self, base_input: &Parsed, _: Option<String>) -> Result<P2Out> {
    let (base_x, base_y) = base_input.start;

    // Each sub-section can use the same patched set of walls. Faster to set up that way.
    let mut patched_walls = base_input.walls.clone();
    for (wx, wy) in [
      (base_x, base_y),
      (base_x, base_y - 1),
      (base_x + 1, base_y),
      (base_x, base_y + 1),
      (base_x - 1, base_y),
    ] {
      patched_walls.insert(wx, wy);
    }

    let mut answer = 0;
    for (start_x, start_y, is_west, is_north) in [
      (base_x - 1, base_y - 1, true, true),
      (base_x + 1, base_y - 1, false, true),
      (base_x - 1, base_y + 1, true, false),
      (base_x + 1, base_y + 1, false, false),
    ] {
      let mut input = base_input.clone();
      input.walls = patched_walls.clone();
      input.start = (start_x, start_y);

      // Keep only the keys and doors that are within this region's boundaries.
      // Additionally, drop doors for which the keys aren't in this region.
      let kept_keys: Infinite2dGrid<Keys> = base_input
        .keys
        .entries()
        .filter(|&(x, y, _key_id)| {
          let x_ok = if is_west { x <= start_x } else { x >= start_x };
          let y_ok = if is_north { y <= start_y } else { y >= start_y };
          x_ok && y_ok
        })
        .collect();

      let kept_doors: Infinite2dGrid<Doors> = base_input
        .doors
        .entries()
        .filter(|&(x, y, door_id)| {
          let x_ok = if is_west { x <= start_x } else { x >= start_x };
          let y_ok = if is_north { y <= start_y } else { y >= start_y };
          x_ok && y_ok && kept_keys.values().contains(door_id)
        })
        .collect();

      input.keys = kept_keys;
      input.doors = kept_doors;

      answer += solve_area(&input)?;
    }

    Ok(answer)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2019, 18)
}
