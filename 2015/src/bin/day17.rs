use std::cmp::Ordering;

use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = u64;
type P2Out = u64;
type Parsed = Vec<u32>;

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    Ok(
      lines
        .into_iter()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u32>>(),
    )
  }

  fn part1(&self, containers: &Parsed, sample_name: Option<String>) -> Result<P1Out> {
    let eggnog: u32 = if sample_name.is_some() { 25 } else { 150 };
    let l = containers.len();

    // There are few enough containers that a bitfield is an option.
    // This is fast enough that I don't care about doing the proper dynamic programming solution:
    // wall time from hitting Enter in my shell to having both answers is under 15 milliseconds.
    let mut total_combos: u64 = 0;
    for combo in 0..(2_u32.pow(l as u32)) {
      let size = containers
        .iter()
        .enumerate()
        .filter(|(i, _)| ((combo >> (l - i - 1)) & 0b1) == 1)
        .map(|(_, c)| *c)
        .sum::<u32>();
      if size == eggnog {
        total_combos += 1;
      }
    }
    Ok(total_combos)
  }

  fn part2(&self, containers: &Parsed, sample_name: Option<String>) -> Result<P2Out> {
    let eggnog: u32 = if sample_name.is_some() { 25 } else { 150 };
    let l = containers.len();

    // Same solution as above, just tracking different things from the loop
    let mut fewest_containers = usize::MAX;
    let mut total_min_combos: u64 = 0;
    for combo in 0..(2_u32.pow(l as u32)) {
      let count = containers
        .iter()
        .enumerate()
        .filter(|(i, _)| ((combo >> (l - i - 1)) & 0b1) == 1);
      let size = &count.clone().map(|(_, c)| *c).sum::<u32>();
      if *size == eggnog {
        let c = &count.count();
        match c.cmp(&fewest_containers) {
          Ordering::Equal => {
            total_min_combos += 1;
          }
          Ordering::Less => {
            fewest_containers = *c;
            total_min_combos = 1;
          }
          _ => {}
        }
      }
    }
    Ok(total_min_combos)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2015, 17)
}
