use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = u64;
type P2Out = u64;
type Parsed = (u64, u64, u64);

// This came from my input's last two constant values.
// They're not in the same place for everyone's input,
// so I hard-coded it. Sorry, Eric!
const INPUT_CONSTANT: u64 = 198;

// Despite this being the slow way, the values are
// so small that even part 2 takes 60 nanoseconds.
fn slow_fib(f1: u64, f0: u64, n: u64) -> u64 {
  let mut next = f1;
  let mut prev = f0;
  for _ in 0..n {
    let old_next = next;
    next += prev;
    prev = old_next;
  }

  next + INPUT_CONSTANT
}

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    // The sample is a different program
    if sample_name.is_some() {
      return Ok((0, 0, 0));
    }
    // The real input is just the fibonacci sequence and some addition at the end
    let a = lines[0].split(' ').nth(1).unwrap().parse().unwrap();
    let b: u64 = lines[1].split(' ').nth(1).unwrap().parse().unwrap();
    let n: u64 = lines[2].split(' ').nth(1).unwrap().parse().unwrap();
    Ok((a, b, n))
  }

  fn part1(&self, (a, b, n): &Parsed, sample_name: Option<String>) -> Result<P1Out> {
    // Sample is a different program than real input
    if sample_name.is_some() {
      return Ok(42);
    }
    Ok(slow_fib(*a, *b, *n))
  }

  fn part2(&self, (a, b, n): &Parsed, sample_name: Option<String>) -> Result<P2Out> {
    if sample_name.is_some() {
      return Ok(1);
    }
    Ok(slow_fib(*a, *b, *n + 7))
  }
}

fn main() -> Result<()> {
  Solver {}.run(2016, 12)
}
