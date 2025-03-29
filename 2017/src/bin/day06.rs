use advent_lib::runner::{Day, PartId};
use anyhow::Result;
use fnv::{FnvBuildHasher, FnvHashMap};

type P1Out = usize;
type P2Out = usize;
type Parsed = Vec<u8>;

/// max_by returns the LAST maximum value,
/// we want the first one.
#[inline(always)]
fn find_max(prev: &[u8]) -> (usize, u8) {
  let mut i = 0;
  let mut m = 0;
  for (j, &n) in prev.iter().enumerate() {
    if n > m {
      m = n;
      i = j;
    }
  }
  (i, m)
}

// It can get the lengths, but converting them once
// outside saves a litte time in the tight loop
fn iterate(prev: &[u8], lu: usize, l: u8) -> Vec<u8> {
  let (i, m) = find_max(prev);

  // The most efficient way I could figure out to do this
  // with as few memory copies as possible
  let (d, r) = (m / l, m % l);
  let ru = r as usize;
  let wraps = i + ru >= lu;
  let wrapped_ru = i + ru - lu;

  prev
    .iter()
    .enumerate()
    .map(|(k, &item)| {
      let v = if k == i { d } else { item + d };
      if (k > i && k - i <= ru) || (wraps && wrapped_ru >= k) {
        v + 1
      } else {
        v
      }
    })
    .collect()
}

/// I tried the Floyd method and several hybrid memoization
/// tactics, but it turns out that using the extra memory
/// for this method takes 2/3 as long as any other approach.
#[allow(unused)]
fn solve(initial_blocks: &[u8]) -> (usize, usize) {
  // This initial capaciy seems to get the best runtime
  // by having an optimal usage ratio by the end
  let mut seen: FnvHashMap<Vec<u8>, usize> =
    FnvHashMap::with_capacity_and_hasher(8_192, FnvBuildHasher::default());

  let lu = initial_blocks.len();
  let l = lu as u8;
  let mut prev = initial_blocks.to_owned();
  seen.insert(prev.clone(), 0);

  for step in 0..usize::MAX {
    let next = iterate(&prev, lu, l);
    if let Some(offset) = seen.get(&next) {
      return (*offset, step + 1 - offset);
    } else {
      seen.insert(next.clone(), step + 1);
      prev = next.clone();
    }
  }
  panic!("Failed to find");
}

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(
      lines[0]
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect(),
    )
  }

  fn part1(&self, initial: &Parsed, _: Option<String>) -> Result<P1Out> {
    let (offset, cycle_len) = solve(initial);
    Ok(offset + cycle_len)
  }

  fn part2(&self, initial: &Parsed, _: Option<String>) -> Result<P2Out> {
    let (_, cycle_len) = solve(initial);
    Ok(cycle_len)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2017, 6)
}
