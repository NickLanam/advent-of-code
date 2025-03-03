use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = String;
type P2Out = String;
type Parsed = Vec<bool>;

// Naively generating the curve and iteratively reducing the checksum:
//  3 microseconds part 1 ; 144 milliseconds part 2. Took me ten minutes.
// Computing bits directly, and using parity to compute the checksum (this solution):
//  700 nanoseconds part 1 ; 25 milliseconds part 2. Took me six hours.
fn solve(pattern: &Parsed, target_len: usize) -> Result<String> {
  let pattern_len = pattern.len();
  let cycle_len = (pattern_len + 1) * 2;
  let inverse: Vec<bool> = pattern.iter().rev().map(|b| !*b).collect();

  // Tracking variables to avoid slow division in get_next_bit avoids a +400% time cost
  let mut cycle_index: usize = 0; // bit_index % full_cycle
  let mut dragon_cycle: usize = 0; // bit_index % half_cycle
  let mut dragon_index: usize = 0; // (bit_index - pattern_len) / (pattern_len) + 1

  let mut get_next_bit = || -> bool {
    // Seems branch prediction performs best if we check dragons first.
    // This plus no math in the conditions cuts time in half again.
    let out: bool = if dragon_cycle == pattern_len {
      // The recursive relationship to find a given bit from its index
      // simplifies to this bit-bashing trick. Took me an hour to notice.
      (dragon_index >> dragon_index.trailing_ones()) & 0b11 == 0b10
    } else if cycle_index < pattern_len {
      pattern[dragon_cycle]
    } else {
      inverse[dragon_cycle]
    };

    // Update tracking variables (instead of doing any division in this function)
    cycle_index += 1;
    if cycle_index == cycle_len {
      cycle_index = 0;
    }
    dragon_cycle += 1;
    if dragon_cycle == pattern_len + 1 {
      dragon_cycle = 0;
      dragon_index += 1;
    }
    out
  };

  // Checksum computation is done in chunks, updating parity as it goes.
  let divisions = target_len.trailing_zeros();
  let chunk_size = 2_usize.pow(divisions);
  let num_chunks = target_len / chunk_size;

  // Slightly faster to pre-allocate the string than to map/collect.
  // Also very slightly faster than bit-bashing into a u64 then stringifying.
  let mut checksum = String::with_capacity(num_chunks);

  // As above: pairwise parity in chunks, faster than iterative reduction.
  for _ in 0..num_chunks {
    let mut parity = true;
    for _ in (0..chunk_size).step_by(2) {
      // If the next two bits match, flip parity
      parity ^= get_next_bit() == get_next_bit();
    }
    checksum.push(if parity { '1' } else { '0' });
  }

  Ok(checksum)
}

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(lines[0].chars().map(|c| c != '0').collect())
  }

  fn part1(&self, pattern: &Parsed, s: Option<String>) -> Result<P1Out> {
    let target_len = if s.is_some() { 20 } else { 272 };
    solve(pattern, target_len)
  }

  fn part2(&self, pattern: &Parsed, s: Option<String>) -> Result<P2Out> {
    let target_len = if s.is_some() { 20 } else { 35_651_584 };
    solve(pattern, target_len)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2016, 16)
}
