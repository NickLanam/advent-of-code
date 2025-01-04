use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = i64;
type P2Out = usize;
type Parsed = String;

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    Ok(lines[0].to_string())
  }

  fn part1(&self, line: &Parsed, _sample_name: Option<String>) -> Result<P1Out> {
    Ok(
      line
        .chars()
        .fold(0_i64, |a, c| a + (if c == '(' { 1 } else { -1 })),
    )
  }

  fn part2(&self, line: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    let mut floor: i64 = 0;
    for (i, c) in line.chars().enumerate() {
      match c {
        '(' => {
          floor += 1;
        }
        ')' => {
          floor -= 1;
          if floor < 0 {
            return Ok(i + 1);
          }
        }
        _ => {
          panic!("Unexpected character {c}");
        }
      }
    }
    // 0 Means "never went into the basement" - most of the samples don't.
    Ok(0)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2015, 1)
}
