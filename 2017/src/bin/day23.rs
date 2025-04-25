use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = u64;
type P2Out = u64;
type Parsed = (u64, u64, usize);

/// This one is another "hand-decompile and optimize some assembly" puzzle.
/// The parse just finds the probably-different-for-everyone numbers, hoping
/// that they'll be on the same lines for everyone. If they aren't, oops.
struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    for_part: PartId,
  ) -> Result<Parsed> {
    let p = |line: &str| {
      line
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<i32>()
        .unwrap()
        .unsigned_abs()
    };
    // Seems there's only one number that might differ between inputs
    let mut b = p(&lines[0]) as u64;
    let mut c = b;
    let step = p(&lines[lines.len() - 2]) as usize;

    if for_part == PartId::P2 {
      let b_mul: u64 = p(&lines[4]) as u64;
      let b_add: u64 = p(&lines[5]) as u64;
      let c_add: u64 = p(&lines[7]) as u64;
      b = b * b_mul + b_add;
      c = b + c_add;
    }

    Ok((b, c, step))
  }

  fn part1(&self, (b, _c, _step): &Parsed, _sample_name: Option<String>) -> Result<P1Out> {
    // Because it asks how many times the naive primality test in the assembly calls `mul`,
    // we can avoid running the code entirely and do this formula on the only input that
    // affects the result.
    Ok((b - 2) * (b - 2))
  }

  fn part2(&self, &(b, c, step): &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    // The input assembly runs a primality test on a stepped range of numbers,
    // counting how many are NOT prime, a returns that.
    // By using a slightly more efficient primality test directly, we can
    // solve in under 200 microseconds here.

    // Naive brute force primality test. Fast enough for Advent of Code.
    let mut out = 0;
    for i in (b..=c).step_by(step) {
      if (2..=i.isqrt()).any(|j| i % j == 0) {
        out += 1;
      }
    }
    Ok(out)
  }

  // Slightly more efficient prime number test...
}

fn main() -> Result<()> {
  Solver {}.run(2017, 23)
}
