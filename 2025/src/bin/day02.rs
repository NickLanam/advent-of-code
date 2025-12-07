use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = usize;
type P2Out = usize;
type Parsed = Vec<(usize, usize)>;

fn is_bad(i: usize) -> bool {
  let s = i.to_string();
  for l in 1..=(s.len() / 2) {
    if l == 1 || s.len().is_multiple_of(l) {
      let parts: Vec<String> = s
        .chars()
        .collect::<Vec<char>>()
        .chunks_exact(l)
        .map(|c| c.iter().collect::<String>())
        .collect();

      let mut iter = parts.iter();
      let first: &String = iter.next().unwrap();
      if iter.all(|n| n.eq(first)) {
        return true;
      }
    }
  }
  false
}

fn sum_bads(a: usize, b: usize, multiple: bool) -> usize {
  let mut sum = 0;
  for i in a..=b {
    if (multiple && i > 0 && is_bad(i))
      || (i >= 10 && i / 10 == i % 10)
      || (i >= 1_000 && i / 1_00 == i % 1_00)
      || (i >= 100_000 && i / 1_000 == i % 1_000)
      || (i >= 10_000_000 && i / 10_000 == i % 10_000)
      || (i >= 1_000_000_000 && i / 100_000 == i % 100_000)
    {
      sum += i;
    }
  }
  sum
}

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    Ok(
      lines[0]
        .split(',')
        .collect::<Vec<&str>>()
        .iter()
        .map(|group| {
          let split: Vec<usize> = group.split('-').map(|n| n.parse().unwrap()).collect();
          (split[0], split[1])
        })
        .collect(),
    )
  }

  fn part1(&self, ranges: &Parsed, _sample_name: Option<String>) -> Result<P1Out> {
    let mut sum_bad = 0;
    for &(l, r) in ranges {
      sum_bad += sum_bads(l, r, false);
    }
    Ok(sum_bad)
  }

  fn part2(&self, ranges: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    let mut sum_bad = 0;
    for &(l, r) in ranges {
      sum_bad += sum_bads(l, r, true);
    }
    Ok(sum_bad)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2025, 2)
}
