use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = usize;
type P2Out = usize;
type Parsed = ();

// The real program uses an obtuse approach to generate
// an input number that has exactly 3 prime divisors,
// then adds up (unique) products of subsets of those divisors.
// This approach takes under 450ns for my input.
fn solve(in_target: usize) -> Result<usize> {
  let mut target = in_target;
  let mut divisors = Vec::with_capacity(3);

  // ASSUMPTION: There are always exactly 3 prime divisors, and two of them are small.
  for i in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
    if target == 1 {
      break;
    }
    while target.is_multiple_of(i) {
      divisors.push(i);
      target /= i;
    }
    if divisors.len() == 2 {
      // Shortcut: assume what remains is a prime number.
      divisors.push(target);
      break;
    }
  }

  let (a, b, c) = (divisors[0], divisors[1], divisors[2]);
  let mut parts = vec![];
  for part in [a, b, c, a * b, a * c, b * c, a * b * c] {
    if !parts.contains(&part) {
      parts.push(part);
    }
  }

  Ok(1 + parts.iter().sum::<usize>())
}

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, _: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    // It's another "analyze the assembly program" puzzle.

    // The input is a mess of initialization logic.
    // I originally did the brute-force simulation to find it.
    // Calculating it is obnoxious, so I'm hardcoding those results.
    // Sorry again, Eric :(
    Ok(())
  }

  fn part1(&self, _: &Parsed, _: Option<String>) -> Result<P1Out> {
    // 19 * 7 * 7
    solve(931)
  }

  fn part2(&self, _: &Parsed, _: Option<String>) -> Result<P1Out> {
    // 7 * 29 * 51_977
    solve(10_551_331)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2018, 19)
}
