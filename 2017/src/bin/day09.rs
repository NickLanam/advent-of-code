use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = u64;
type P2Out = u64;
type Parsed = String;

fn solve(line: &str) -> (u64, u64) {
  let mut level_sum = 0;
  let mut garbage_sum = 0;
  let mut nest_level = 0;
  let mut in_garbage = false;
  let mut negating = false;
  for c in line.chars() {
    if negating {
      negating = false;
    } else if c == '!' {
      negating = true;
    } else if c == '<' && !in_garbage {
      in_garbage = true;
    } else if c == '>' && in_garbage {
      in_garbage = false;
    } else if in_garbage {
      garbage_sum += 1;
    } else if c == '{' {
      nest_level += 1;
    } else if c == '}' {
      if nest_level == 0 {
        panic!("Unbalanced");
      } else {
        level_sum += nest_level;
        nest_level -= 1;
      }
    } else if c == ',' {
      // Comma between groups
      continue;
    } else {
      panic!(
        "Found garbage outside of garbage markers: {c}, level_sum={level_sum}, garbage_sum={garbage_sum}, nest_level={nest_level}, in_garbage={in_garbage}, negating={negating}"
      );
    }
  }
  (level_sum, garbage_sum)
}

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(lines[0].to_string())
  }

  fn part1(&self, line: &Parsed, _: Option<String>) -> Result<P1Out> {
    let (level_sum, _) = solve(line);
    Ok(level_sum)
  }

  fn part2(&self, line: &Parsed, _: Option<String>) -> Result<P2Out> {
    let (_, garbage_sum) = solve(line);
    Ok(garbage_sum)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2017, 9)
}
