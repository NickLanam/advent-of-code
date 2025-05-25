use advent_lib::runner::{Day, PartId};
use anyhow::Result;
use fnv::{FnvBuildHasher, FnvHashSet};

type P1Out = u64;
type P2Out = u64;
type Parsed = (u64, u64);

fn solve(secret: u64, factor: u64, first_result: bool) -> u64 {
  let mut seen: FnvHashSet<u64> = FnvHashSet::with_hasher(FnvBuildHasher::default());
  let mut last_seen = 0;
  let mut state = 0;
  loop {
    let mut check = state | 0x10000;
    state = secret;
    loop {
      // Fancy way of pretending it's a 24-bit unsigned integer
      state = (((state + (check & 0xFF)) & 0xFFFFFF) * factor) & 0xFFFFFF;
      if 0x100 > check {
        if first_result {
          return state;
        } else {
          // Part 2 wants the last valid answer to part 1.
          // As soon as we see a duplicate, all future answers will be too.
          if seen.contains(&state) {
            return last_seen;
          } else {
            last_seen = state;
            seen.insert(state);
          }
          break;
        }
      }
      check >>= 8;
    }
  }
}

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    // The constants on lines 2 and 3 don't impact the output at all
    // All but two constants are either 2^n or (2^n)-1, so they're not secrets.
    // Those last two are probably the only meaningful difference between inputs?
    let secret: u64 = lines[8].split_whitespace().nth(1).unwrap().parse()?;
    let factor: u64 = lines[12].split_whitespace().nth(2).unwrap().parse()?;
    Ok((secret, factor))
  }

  fn part1(&self, &(secret, factor): &Parsed, _: Option<String>) -> Result<P1Out> {
    Ok(solve(secret, factor, true))
  }

  fn part2(&self, &(secret, factor): &Parsed, _: Option<String>) -> Result<P2Out> {
    Ok(solve(secret, factor, false))
  }
}

fn main() -> Result<()> {
  Solver {}.run(2018, 21)
}
