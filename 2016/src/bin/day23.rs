use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = u64;
type P2Out = u64;
type Parsed = (u64, u64);

// Reading the input, we find that the program really just does this,
// with two numbers in the input that actually matter.
// I did write out the entire virtual machine code to verify part 1,
// but that was a waste of time in retrospect.
fn solve(input: u64, secret_a: u64, secret_b: u64) -> u64 {
  let mut a = input;
  let mut b = input - 1;
  while b > 1 {
    a *= b;
    b -= 1;
  }
  a + (secret_a * secret_b)
}

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    if sample_name.is_some() {
      // Sample is a different program.
      return Ok((0, 0));
    }
    let secret_a: u64 = lines[19]
      .split_once(' ')
      .unwrap()
      .1
      .split_once(' ')
      .unwrap()
      .0
      .parse()
      .unwrap();
    let secret_b: u64 = lines[20]
      .split_once(' ')
      .unwrap()
      .1
      .split_once(' ')
      .unwrap()
      .0
      .parse()
      .unwrap();
    Ok((secret_a, secret_b))
  }

  fn part1(&self, (secret_a, secret_b): &Parsed, _: Option<String>) -> Result<P1Out> {
    Ok(solve(7, *secret_a, *secret_b))
  }

  fn part2(&self, (secret_a, secret_b): &Parsed, _: Option<String>) -> Result<P2Out> {
    Ok(solve(12, *secret_a, *secret_b))
  }
}

fn main() -> Result<()> {
  Solver {}.run(2016, 23)
}
