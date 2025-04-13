use advent_lib::runner::{Day, PartId};
use anyhow::{Context, Result};

type P1Out = u64;
type P2Out = u64;
type Parsed = (u64, u64);

const MUL_A: u64 = 16_807;
const MUL_B: u64 = 48_271;
const DIVISOR: u64 = 2_147_483_647; // 2**31 - 1

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
    // TODO: There's something fishy with the input values.
    // Likely we don't need to iterate at all.
    for _ in 0..40_000_000 {
      a = (a * MUL_A) % DIVISOR;
      b = (b * MUL_B) % DIVISOR;
      if (a & 0xFFFF) == (b & 0xFFFF) {
        sum += 1;
      }
    }
    Ok(sum)
  }

  fn part2(&self, &(seed_a, seed_b): &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    let (mut a, mut b) = (seed_a, seed_b);
    let mut sum = 0;
    // TODO: There's something fishy with the input values.
    // Likely we don't need to iterate at all.
    for _ in 0..5_000_000 {
      a = (a * MUL_A) % DIVISOR;
      while a & 0b11 != 0 {
        a = (a * MUL_A) % DIVISOR;
      }
      b = (b * MUL_B) % DIVISOR;
      while b & 0b111 != 0 {
        b = (b * MUL_B) % DIVISOR;
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
