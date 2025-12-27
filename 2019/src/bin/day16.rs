use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = usize;
type P2Out = usize;
type Parsed = Vec<i64>;

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(
      lines[0]
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect(),
    )
  }

  fn part1(&self, init: &Parsed, _: Option<String>) -> Result<P1Out> {
    const BASE_PATTERN: [i64; 4] = [0, 1, 0, -1];
    const ROUNDS: usize = 100;

    let len = init.len();
    let mut line = init.clone();
    for _round in 1..=ROUNDS {
      let mut next = vec![0; len];
      for (i, item) in next.iter_mut().enumerate() {
        for (j, &n) in line.iter().enumerate().skip(i) {
          let m = BASE_PATTERN[((j + 1) / (i + 1)) % 4];
          *item += n * m;
        }
        *item = item.abs() % 10;
      }
      line = next;
    }

    Ok(
      line
        .iter()
        .take(8)
        .fold(0, |acc, n| (acc * 10) + ((n.abs() % 10) as usize)),
    )
  }

  fn part2(&self, _lines: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    Ok(0)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2019, 16)
}
