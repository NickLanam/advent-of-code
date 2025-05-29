use std::collections::VecDeque;

use advent_lib::{
  grid::{Infinite2dGrid, Infinite2dSet},
  runner::{Day, PartId},
};
use anyhow::{Result, bail};

#[derive(Clone, Default, PartialEq, Hash)]
struct Room {
  n: bool,
  e: bool,
  s: bool,
  w: bool,
}

type P1Out = usize;
type P2Out = usize;
type Parsed = String;

// Convenience function to update the map without duplicate match branches in build_map
fn traverse(map: &mut Infinite2dGrid<Room>, x: i32, y: i32, dir: char) -> Result<(i32, i32)> {
  match dir {
    'N' => {
      map.entry(x, y).or_default().n = true;
      map.entry(x, y - 1).or_default().s = true;
      Ok((x, y - 1))
    }
    'E' => {
      map.entry(x, y).or_default().e = true;
      map.entry(x + 1, y).or_default().w = true;
      Ok((x + 1, y))
    }
    'S' => {
      map.entry(x, y).or_default().s = true;
      map.entry(x, y + 1).or_default().n = true;
      Ok((x, y + 1))
    }
    'W' => {
      map.entry(x, y).or_default().w = true;
      map.entry(x - 1, y).or_default().e = true;
      Ok((x - 1, y))
    }
    _ => {
      bail!("Tried to traverse {dir}?");
    }
  }
}

fn build_map(path_pattern: &str) -> Result<Infinite2dGrid<Room>> {
  let mut map: Infinite2dGrid<Room> = Infinite2dGrid::new(path_pattern.len());
  map.insert(0, 0, Default::default());

  let mut frontier: Vec<(i32, i32)> = vec![(0, 0)];
  let mut stack: Vec<(Vec<(i32, i32)>, Infinite2dSet)> = vec![];
  let mut heads: Vec<(i32, i32)> = vec![(0, 0)];
  let mut tails = Infinite2dSet::new(path_pattern.len());

  for ch in path_pattern.chars().skip(1).take(path_pattern.len() - 2) {
    match ch {
      'N' | 'E' | 'S' | 'W' => {
        for (x, y) in frontier.iter_mut() {
          (*x, *y) = traverse(&mut map, *x, *y, ch)?;
        }
      }
      '(' => {
        stack.push((heads.clone(), tails.clone()));
        heads = frontier.clone();
        tails = Infinite2dSet::new(path_pattern.len());
      }
      ')' => {
        frontier.clear();
        frontier.extend(tails.keys());
        (heads, tails) = stack.pop().unwrap();
      }
      '|' => {
        for (x, y) in frontier.iter() {
          tails.insert(*x, *y);
        }
        frontier = heads.clone();
      }
      _ => {
        bail!("Unrecognized char: {ch}");
      }
    }
  }
  Ok(map)
}

// Part 1 answer: how far away is the furthest door (by optimal path)?
// Part 2 answer: how many doors have optimal paths at least 1_000 steps long?
fn build_paths(map: &Infinite2dGrid<Room>) -> Result<(usize, usize)> {
  let mut longest_path = 0;
  let mut long_paths = 0;

  // For both parts, exhaustive BFS is the right tool for the job.
  let mut frontier: VecDeque<(usize, i32, i32)> = VecDeque::new();
  let mut seen = Infinite2dSet::new(map.len());
  frontier.push_back((0, 0, 0));
  while let Some((d, x, y)) = frontier.pop_front() {
    if seen.contains(x, y) {
      continue;
    }
    seen.insert(x, y);
    longest_path = longest_path.max(d);
    if d >= 1_000 {
      long_paths += 1;
    }
    if let Some(room) = map.get(x, y) {
      if room.n {
        frontier.push_back((d + 1, x, y - 1));
      }
      if room.e {
        frontier.push_back((d + 1, x + 1, y));
      }
      if room.s {
        frontier.push_back((d + 1, x, y + 1));
      }
      if room.w {
        frontier.push_back((d + 1, x - 1, y));
      }
    }
  }

  Ok((longest_path, long_paths))
}

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(lines[0].to_string())
  }

  fn part1(&self, path_pattern: &Parsed, _: Option<String>) -> Result<P1Out> {
    let map = build_map(path_pattern)?;
    let (farthest_path, _long_paths) = build_paths(&map)?;
    Ok(farthest_path)
  }

  fn part2(&self, path_pattern: &Parsed, _: Option<String>) -> Result<P2Out> {
    let map = build_map(path_pattern)?;
    let (_farthest_path, long_paths) = build_paths(&map)?;
    Ok(long_paths)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2018, 20)
}
