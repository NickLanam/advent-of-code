use advent_lib::runner::{Day, PartId};
use anyhow::{Context, Result};

type P1Out = usize;
type P2Out = usize;
type Parsed = (Vec<(usize, usize)>, Vec<usize>);

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    let split = lines
      .iter()
      .position(|l| l.is_empty())
      .with_context(|| "Input needs a blank line somewhere")?;

    let raw_ranges = lines[0..split].to_vec();
    let raw_available = lines[(split + 1)..].to_vec();

    let ranges = raw_ranges
      .iter()
      .map(|range| {
        let (l, r) = range.split_once('-').unwrap();
        (l.parse().unwrap(), r.parse().unwrap())
      })
      .collect();

    let available = raw_available
      .iter()
      .map(|line| line.parse().unwrap())
      .collect();

    Ok((ranges, available))
  }

  fn part1(&self, (ranges, available): &Parsed, _: Option<String>) -> Result<P1Out> {
    Ok(available.iter().fold(0, |acc, ingredient| {
      if ranges
        .iter()
        .any(|(lo, hi)| lo <= ingredient && hi >= ingredient)
      {
        acc + 1
      } else {
        acc
      }
    }))
  }

  fn part2(&self, (init_ranges, _): &Parsed, _: Option<String>) -> Result<P2Out> {
    let mut ranges = init_ranges.clone();
    ranges.sort_by(|(lo1, hi1), (lo2, hi2)| lo1.cmp(lo2).then(hi1.cmp(hi2)));

    let mut disjoints: Vec<(usize, usize)> = vec![];

    // Walk through the ranges, combining those which overlap with their (sorted) neighbors
    // so that we don't count the overlapped area twice at the end.
    for &(lo, hi) in ranges.iter() {
      if let Some(&(lo2, hi2)) = disjoints.last()
        && ((lo2 <= lo && lo <= hi2)
          || (lo2 <= hi && hi <= hi2)
          || (lo <= lo2 && lo2 <= hi)
          || (lo <= hi2 && hi2 <= hi))
      {
        disjoints.pop();
        disjoints.push((lo.min(lo2), hi.max(hi2)));
      } else {
        disjoints.push((lo, hi));
      }
    }

    Ok(disjoints.iter().fold(0, |acc, (lo, hi)| acc + hi - lo + 1))
  }
}

fn main() -> Result<()> {
  Solver {}.run(2025, 5)
}
