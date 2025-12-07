use std::collections::{HashMap, HashSet};

use advent_lib::runner::{Day, PartId};
use anyhow::{Context, Result};

type P1Out = usize;
type P2Out = usize;
type Parsed = Vec<String>;

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(lines)
  }

  fn part1(&self, lines: &Parsed, _: Option<String>) -> Result<P1Out> {
    let mut times_split = 0;
    let start = lines[0]
      .chars()
      .position(|ch| ch == 'S')
      .with_context(|| "No start position in first line")?;

    let mut active_positions = HashSet::<usize>::with_capacity(lines[0].len());
    active_positions.insert(start);

    for line in lines[1..].iter() {
      let reached_splits: Vec<usize> = line
        .chars()
        .enumerate()
        .filter(|(i, ch)| *ch == '^' && active_positions.contains(i))
        .map(|(i, _ch)| i)
        .collect();
      if !reached_splits.is_empty() {
        times_split += reached_splits.len();
        let mut next_active = HashSet::<usize>::with_capacity(lines[0].len());
        for r in reached_splits.iter() {
          // For those that hit a split, separate them
          next_active.insert(r - 1);
          next_active.insert(r + 1);
        }
        for &r in active_positions
          .iter()
          .filter(|i| !reached_splits.contains(i))
        {
          // Keep the ones that didn't split here
          next_active.insert(r);
        }
        active_positions = next_active;
      }
    }

    Ok(times_split)
  }

  fn part2(&self, lines: &Parsed, _: Option<String>) -> Result<P2Out> {
    let start = lines[0]
      .chars()
      .position(|ch| ch == 'S')
      .with_context(|| "No start position in first line")?;

    let mut active_positions = HashMap::<usize, usize>::with_capacity(lines[0].len());
    active_positions.insert(start, 1);

    for line in lines[1..].iter() {
      let reached_splits: Vec<(usize, usize)> = line
        .chars()
        .enumerate()
        .filter(|(i, ch)| *ch == '^' && active_positions.contains_key(i))
        .map(|(i, _ch)| (i, *active_positions.get(&i).unwrap()))
        .collect();
      if !reached_splits.is_empty() {
        let mut next_active = HashMap::<usize, usize>::with_capacity(lines[0].len());
        for &(r, count) in reached_splits.iter() {
          // For those that hit a split, separate them
          next_active.insert(r - 1, count + next_active.get(&(r - 1)).unwrap_or(&0));
          next_active.insert(r + 1, count + next_active.get(&(r + 1)).unwrap_or(&0));
        }
        for (r, count) in active_positions
          .iter()
          .filter(|(i, ..)| !reached_splits.iter().any(|(j, ..)| *j == **i))
        {
          // Keep the ones that didn't split here
          next_active.insert(*r, *count + next_active.get(r).unwrap_or(&0));
        }
        active_positions = next_active;
      }
    }

    Ok(active_positions.iter().map(|(.., count)| count).sum())
  }
}

fn main() -> Result<()> {
  Solver {}.run(2025, 7)
}
