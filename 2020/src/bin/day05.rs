use advent_lib::runner::{Day, PartId};
use anyhow::{Result, bail};
use itertools::Itertools;

type P1Out = u32;
type P2Out = u32;
type Parsed = Vec<u32>;

/// Conveniently, the input maps to binary numbers directly.\
/// It's just a little roundabout in how it defines the fact.
fn get_seat_id(line: &str) -> u32 {
  let mut row: u32 = 0;
  let mut col: u32 = 0;
  for (i, ch) in line.chars().enumerate() {
    if ch == 'B' {
      row += 1 << (6 - i);
    } else if ch == 'R' {
      col += 1 << (2 - (i - 7));
    }
  }
  (row * 8) + col
}

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(
      lines
        .iter()
        .map(|line| get_seat_id(line.as_str()))
        .collect(),
    )
  }

  fn part1(&self, seat_ids: &Parsed, _sample_name: Option<String>) -> Result<P1Out> {
    Ok(*seat_ids.iter().max().unwrap())
  }

  fn part2(&self, seat_ids: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    let mut sorted = seat_ids.clone();
    sorted.sort();
    for (v1, v2) in sorted.iter().tuple_windows() {
      if *v2 != *v1 + 1 {
        return Ok(v1 + 1);
      }
    }
    bail!("No hole found!");
  }
}

fn main() -> Result<()> {
  Solver {}.run(2020, 5)
}
