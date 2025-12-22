// use std::collections::VecDeque;

use advent_lib::runner::{Day, PartId};
use anyhow::Result;
use fnv::{FnvBuildHasher, FnvHashMap, FnvHashSet};

#[derive(Clone, PartialEq, Eq)]
struct Reaction {
  inputs: Vec<(String, usize)>,
  output: (String, usize),
}

fn sorted_reactions(reactions: &[Reaction]) -> Vec<Reaction> {
  let mut sorted: Vec<Reaction> = vec![];
  let mut known = FnvHashSet::<String>::with_capacity_and_hasher(100, FnvBuildHasher::default());
  known.insert("ORE".to_string());

  while sorted.len() < reactions.len() {
    'reaction: for reaction in reactions.iter() {
      if sorted.contains(reaction) {
        continue;
      }
      for (in_name, _in_count) in reaction.inputs.iter() {
        if !known.contains(in_name) {
          continue 'reaction;
        }
      }
      known.insert(reaction.output.0.to_owned());
      sorted.push(reaction.clone());
    }
  }

  sorted.reverse();
  sorted
}

fn ore_per_fuel(needed_fuel: usize, reactions: &[Reaction]) -> usize {
  let mut wanted =
    FnvHashMap::<String, usize>::with_capacity_and_hasher(100, FnvBuildHasher::default());
  wanted.insert("FUEL".to_string(), needed_fuel);

  for reaction in reactions.iter() {
    let quantity_wanted = *wanted.get(&reaction.output.0).unwrap_or(&0);
    let quantity_produced = reaction.output.1;
    let multiplier = quantity_wanted.div_ceil(quantity_produced);
    wanted.remove(&reaction.output.0);

    for input in reaction.inputs.iter() {
      *wanted.entry(input.0.to_owned()).or_insert(0) += multiplier * input.1;
    }
  }

  *wanted.get("ORE").unwrap_or(&usize::MAX)
}

struct Solver;
impl Day<Vec<Reaction>, usize, usize> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Vec<Reaction>> {
    let mut out = vec![];
    for line in lines {
      let (l0, out_raw) = line.split_once(" => ").unwrap();
      let (out_q, out_id) = out_raw.split_once(' ').unwrap();
      let mut inputs = vec![];
      for in_raw in l0.split(", ") {
        let (in_q, in_id) = in_raw.split_once(' ').unwrap();
        inputs.push((in_id.to_owned(), in_q.parse().unwrap()));
      }
      out.push(Reaction {
        inputs,
        output: (out_id.to_owned(), out_q.parse().unwrap()),
      });
    }

    Ok(sorted_reactions(&out))
  }

  fn part1(&self, reactions: &Vec<Reaction>, _: Option<String>) -> Result<usize> {
    Ok(ore_per_fuel(1, reactions))
  }

  /// Binary search to figure out how many fuel we can actually make with that much ore
  /// There's almost certainly a direct solution that's worth exploring, but binary search
  /// still takes less than 350µs on my laptop and was trivial to implement, soooo...
  fn part2(&self, reactions: &Vec<Reaction>, _: Option<String>) -> Result<usize> {
    const AVAILABLE_ORE: usize = 1_000_000_000_000;

    let mut lo = 0;
    let mut hi = usize::MAX;

    loop {
      if hi - lo <= 1 {
        return Ok(lo);
      }
      let mid = (lo + hi) / 2;
      let ore = ore_per_fuel(mid, reactions);

      if ore < AVAILABLE_ORE {
        lo = mid;
      } else if ore > AVAILABLE_ORE {
        hi = mid;
      }
    }
  }
}

fn main() -> Result<()> {
  Solver {}.run(2019, 14)
}
