use advent_lib::runner::{Day, PartId};
use anyhow::{Context, Result};

type P1Out = u64;
type P2Out = u64;
type Parsed = (u64, u64);

// Turns out, this is a famous pseudo-random number generator:
// https://en.wikipedia.org/wiki/Lehmer_random_number_generator

// Unfortunately, since it's a PRNG by design, there's no shortcut
// to compute the answers without actually generating the numbers.

// These two values are common choices of factor for Lehmer RNG:
const FACTOR_A: u64 = 16_807;
const FACTOR_B: u64 = 48_271;
const DIVISOR: u64 = 0x7F_FF_FF_FF; // 2**31 - 1

// Behavior directly from the Wiki article to avoid a division.
#[inline(always)]
fn step(seed: &mut u64, factor: u64) {
  *seed *= factor;
  while *seed >= DIVISOR {
    *seed = (*seed & DIVISOR) + (*seed >> 31);
  }
}

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    let a = lines[0].split_whitespace().last().context("")?.parse()?;
    let b = lines[1].split_whitespace().last().context("")?.parse()?;
    Ok((a, b))
  }

  fn part1(&self, &(seed_a, seed_b): &Parsed, _sample_name: Option<String>) -> Result<P1Out> {
    let (mut a, mut b) = (seed_a, seed_b);
    let mut sum = 0;
    for _ in 0..40_000_000 {
      step(&mut a, FACTOR_A);
      step(&mut b, FACTOR_B);
      if (a & 0xFFFF) == (b & 0xFFFF) {
        sum += 1;
      }
    }
    Ok(sum)
  }

  fn part2(&self, &(seed_a, seed_b): &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    let (mut a, mut b) = (seed_a, seed_b);
    let mut sum = 0;
    for _ in 0..5_000_000 {
      step(&mut a, FACTOR_A);
      while a & 0b11 != 0 {
        step(&mut a, FACTOR_A);
      }
      step(&mut b, FACTOR_B);
      while b & 0b111 != 0 {
        step(&mut b, FACTOR_B);
      }
      if (a & 0xFFFF) == (b & 0xFFFF) {
        sum += 1;
      }
    }
    Ok(sum)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2017, 15)
}
