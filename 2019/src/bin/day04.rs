use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = usize;
type P2Out = usize;
type Parsed = (u32, u32);

fn to_digits(i: u32) -> [u8; 6] {
  let mut out = [0_u8; 6];
  out[0] = ((i / 100000) % 10) as u8;
  out[1] = ((i / 10000) % 10) as u8;
  out[2] = ((i / 1000) % 10) as u8;
  out[3] = ((i / 100) % 10) as u8;
  out[4] = ((i / 10) % 10) as u8;
  out[5] = (i % 10) as u8;
  out
}

fn from_digits(digits: &[u8; 6]) -> u32 {
  digits[0] as u32 * 100000
    + digits[1] as u32 * 10000
    + digits[2] as u32 * 1000
    + digits[3] as u32 * 100
    + digits[4] as u32 * 10
    + digits[5] as u32
}

// Slightly faster than just checking every possibility
// for the never-decreasing mode: directly jump ahead.
fn increment(digits: &mut [u8; 6]) {
  // First, increase by one.
  digits[5] += 1;
  for i in [5, 4, 3, 2, 1] {
    if digits[i] > 9 {
      digits[i] -= 10;
      digits[i - 1] += 1;
    }
  }
  if digits[0] > 9 {
    // Main code will need to notice this
    return;
  }

  // Second, skip ahead to the next number that meets the "never decrease" rule.
  for i in [0, 1, 2, 3, 4] {
    digits[i + 1] = digits[i + 1].max(digits[i]);
  }
}

fn is_valid(digits: &[u8; 6], part2: bool) -> bool {
  let mut run_length = 1;
  let mut last = digits[0];
  for &next in &digits[1..] {
    match next.cmp(&last) {
      std::cmp::Ordering::Less => {
        return false;
      }
      std::cmp::Ordering::Equal => {
        run_length += 1;
      }
      std::cmp::Ordering::Greater => {
        if (part2 && run_length == 2) || (!part2 && run_length >= 2) {
          return true;
        }
        run_length = 1;
      }
    }
    last = next;
  }
  (part2 && run_length == 2) || (!part2 && run_length >= 2)
}

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    let (lo, hi) = lines[0].split_once('-').unwrap();
    Ok((lo.parse()?, hi.parse()?))
  }

  fn part1(&self, &(lo, hi): &Parsed, _: Option<String>) -> Result<P1Out> {
    let mut count = 0;
    let mut digits = to_digits(lo);
    while from_digits(&digits) <= hi {
      if is_valid(&digits, false) {
        count += 1;
      }
      increment(&mut digits);
    }
    Ok(count)
  }

  fn part2(&self, &(lo, hi): &Parsed, _: Option<String>) -> Result<P2Out> {
    let mut count = 0;
    let mut digits = to_digits(lo);
    while from_digits(&digits) <= hi {
      if is_valid(&digits, true) {
        count += 1;
      }
      increment(&mut digits);
    }
    Ok(count)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2019, 4)
}
