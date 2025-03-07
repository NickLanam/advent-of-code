use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = u64;
type P2Out = u64;
type Parsed = u64;

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    Ok(lines[0].parse().unwrap())
  }

  fn part1(&self, initial_size: &Parsed, _sample_name: Option<String>) -> Result<P1Out> {
    // First, I noticed this pattern worked.
    // let mut lead: u64 = 1;
    // let mut step: u64 = 1;
    // let mut size: u64 = *initial_size;
    // while size > 1 {
    //   step <<= 1;
    //   if size & 0b1 == 0b1 {
    //     lead += step;
    //   }
    //   size >>= 1;
    // }
    // Ok(lead)

    // Which simplifies to this, which is 5 instructions:
    let z = initial_size.leading_zeros();
    Ok((initial_size << z << 1 >> z) + 0b1)
  }

  fn part2(&self, target: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    // Write out how it goes for the first 100 numbers and the pattern becomes surprisingly clear:
    // We take the starting number and subtract the largest power of 3 that's less than it.
    // Note: this doesn't work for targets less than 4. 1 -> 1, 2 -> 1, 3 -> 3, but after that the pattern works.
    let mut i = 1;
    while i * 3 < *target {
      i *= 3
    }
    Ok(*target - i)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2016, 19)
}
