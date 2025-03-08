use advent_lib::runner::{Day, PartId};
use anyhow::{Context, Result};

type P1Out = u32;
type P2Out = u32;
type Parsed = Vec<(u32, u32)>;

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    let mut out: Vec<(u32, u32)> = Vec::with_capacity(lines.len());
    for line in lines {
      let (a, b) = line.split_once('-').context("Can't split that line")?;
      out.push((a.parse()?, b.parse()?))
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(out)
  }

  fn part1(&self, block_ranges: &Parsed, _: Option<String>) -> Result<P1Out> {
    // Note: not bothering to check for overflow or the case where there are
    // no open ranges, because neither the sample nor the real input do that.
    let mut min = 0;
    for (lo, hi) in block_ranges {
      if *lo <= min && *hi >= min {
        min = hi + 1;
      } else if *lo > min {
        // Exit as soon as we see an opening
        return Ok(min);
      }
    }
    // In case every range pushed us forward
    Ok(min)
  }

  fn part2(&self, block_ranges: &Parsed, sample_name: Option<String>) -> Result<P2Out> {
    let max = if sample_name.is_some() { 9 } else { u32::MAX };
    let (mut count, mut cursor) = (0, 0);
    for (lo, hi) in block_ranges {
      // Count unblocked values
      if *lo > cursor {
        count += *lo - cursor;
      }
      // Progress the cursor if we need to, stop early if
      // we've already blocked the end of the range.
      if *hi == max {
        return Ok(count);
      } else if *hi >= cursor {
        cursor = hi + 1;
      }
    }

    Ok(count + (max - cursor + 1))
  }
}

fn main() -> Result<()> {
  Solver {}.run(2016, 20)
}
