use advent_lib::runner::{Day, PartId};
use anyhow::{Result, bail};

type P1Out = u16;
type P2Out = usize;
type Parsed = usize;

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    Ok(lines[0].parse()?)
  }

  fn part1(&self, &step: &Parsed, _: Option<String>) -> Result<P1Out> {
    // Naive basic implementation of the puzzle description.
    let mut buffer: Vec<u16> = vec![0];
    let mut cursor: usize = 0;
    for el in 1..=2017 {
      cursor = ((cursor + step) % buffer.len()) + 1;
      buffer.insert(cursor, el);
    }
    for (i, el) in buffer.iter().enumerate() {
      if *el == 2017 {
        return Ok(buffer[(i + 1) % buffer.len()]);
      }
    }
    bail!("Failed to find");
  }

  fn part2(&self, &step: &Parsed, _: Option<String>) -> Result<P2Out> {
    // Since we're only looking for the value at buffer[1], we don't
    // need to generate the rest of the buffer at all - only track
    // the cursor and the value that would be inserted at index 1.
    // Still pretty slow when iterating to 50,000,000 though.
    let mut value_after_zero = 0;
    let mut cursor: usize = 0;
    for el in 1..=50_000_000 {
      // The below is a faster way to do this:
      // cursor = ((cursor + step) % el) + 1;
      cursor += step;
      while cursor >= el {
        cursor -= el;
      }
      cursor += 1;
      if cursor == 1 {
        value_after_zero = el;
      }
    }
    Ok(value_after_zero)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2017, 17)
}
