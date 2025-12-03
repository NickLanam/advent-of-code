use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = usize;
type P2Out = usize;
type Parsed = Vec<Vec<usize>>;

fn solve(banks: &Parsed, digits: usize) -> usize {
  let mut sum = 0;
  for bank in banks {
    let mut found_digits: Vec<usize> = Vec::with_capacity(digits);
    let mut latest_pause = 0;
    while found_digits.len() < digits {
      let mut best_digit = 0;
      let mut best_index = latest_pause;
      #[allow(clippy::needless_range_loop)]
      for i in latest_pause..(bank.len() - (digits - found_digits.len() - 1)) {
        if bank[i] > best_digit {
          best_index = i;
          best_digit = bank[i];
          if best_digit == 9 {
            break;
          }
        }
      }
      found_digits.push(best_digit);
      latest_pause = best_index + 1;
    }
    sum += found_digits.iter().fold(0, |acc, d| acc * 10 + d);
  }
  sum
}

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(
      lines
        .iter()
        .map(|line| {
          line
            .chars()
            .collect::<Vec<char>>()
            .iter()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect()
        })
        .collect(),
    )
  }

  fn part1(&self, banks: &Parsed, _: Option<String>) -> Result<P1Out> {
    Ok(solve(banks, 2))
  }

  fn part2(&self, banks: &Parsed, _: Option<String>) -> Result<P2Out> {
    Ok(solve(banks, 12))
  }
}

fn main() -> Result<()> {
  Solver {}.run(2025, 3)
}
