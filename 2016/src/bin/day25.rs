use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = u64;
type P2Out = u64;
type Parsed = (u64, u64);

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    if sample_name.is_some() {
      return Ok((0, 0));
    }
    let secret_1 = lines[1]
      .split_once(' ')
      .unwrap()
      .1
      .split_once(' ')
      .unwrap()
      .0
      .parse()?;
    let secret_2 = lines[2]
      .split_once(' ')
      .unwrap()
      .1
      .split_once(' ')
      .unwrap()
      .0
      .parse()?;
    Ok((secret_1, secret_2))
  }

  fn part1(&self, (secret_1, secret_2): &Parsed, sample_name: Option<String>) -> Result<P1Out> {
    if sample_name.is_some() {
      return Ok(0);
    }

    // The input gives two secret values, on the second and third line.
    // It multiplies those together, adds the value from register `a`, and
    // prints the bits of this salt (least significant to most), on repeat, forever.
    // We want the printout to go 0, 1, 0, 1... ad infinitum.

    // Solving this is a matter of addition, then:
    // - Compute the salt (multiply the input numbers together)
    // - Starting from the LSB of its binary value, add 2**position if needed (and add that to our solution)
    // - If the bit is even-indexed from the end (LSB is 0, next LSB is 1, etc), then value should be 0. Add if it isn't.
    // - If the bit is odd-indexed from the end, then value should be 1. Add if it isn't.

    let salt = secret_1 * secret_2;
    let mut result: u64 = salt;
    let mut answer: u64 = 0;

    let mut should_be_1 = false;
    for i in 0_u32..64 {
      let relevant_bits = 64 - result.leading_zeros();
      if i + 1 >= relevant_bits {
        break;
      }

      let bit = ((result >> i) & 1) == 1;
      if bit != should_be_1 {
        let adjust: u64 = 2_u64.pow(i);
        result += adjust;
        answer += adjust;
      }
      should_be_1 = !should_be_1;
    }

    Ok(answer)
  }

  fn part2(&self, _lines: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    // On day 25, there is no part 2. This star is granted by earning the other 49.
    Ok(0)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2016, 25)
}
