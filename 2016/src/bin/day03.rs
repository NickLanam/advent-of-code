use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = u64;
type P2Out = u64;
type Parsed = Vec<[u16; 3]>;

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    // There are far more concise ways to do this by using trim and split and parse.
    // However, this approach is the fastest one I could figure out, and it was fun.
    // Assumptions: lines.len() % 3 == 0; a line is 15 chars of 3 right-aligned u16.
    let mut out: Parsed = Vec::with_capacity(lines.len());
    for line in lines {
      // This way is quite fast, but the below way takes half as long
      // let a = line[0..5].trim();
      // let b = line[5..10].trim();
      // let c = line[10..].trim();
      // out.push([a.parse().unwrap(), b.parse().unwrap(), c.parse().unwrap()]);

      // The fastest way I could manage. Pretty much the same thing as above,
      // but inlined into a single loop and assuming the line is simple ASCII.
      let mut abc: [u16; 3] = [0, 0, 0];
      let mut skipping = true; // Columns are right-aligned with spaces.
      let mut which: usize = 0;
      for c in line.chars() {
        if c == ' ' {
          if !skipping {
            skipping = true;
            // Assumptions: Exactly three integers; no trailing space.
            which += 1;
          }
          continue;
        }
        skipping = false;
        let digit: u8 = c.try_into().unwrap();
        abc[which] *= 10;
        abc[which] += (digit as u16) - 48; // ASCII 48 is '0'
      }
      out.push(abc);
    }
    Ok(out)
  }

  fn part1(&self, lines: &Parsed, _sample_name: Option<String>) -> Result<P1Out> {
    let mut valid: u64 = 0;
    for line in lines {
      let [a, b, c] = *line;
      if a + b > c && a + c > b && b + c > a {
        valid += 1;
      }
    }
    Ok(valid)
  }

  fn part2(&self, lines: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    let mut valid: u64 = 0;
    for line in lines.chunks_exact(3) {
      let [[a0, a1, a2], [b0, b1, b2], [c0, c1, c2]] = *line else {
        panic!(
          "Unreachable (chunks_exact implementation guarantees it, but the compiler is unable to prove it)"
        );
      };
      if a0 + b0 > c0 && a0 + c0 > b0 && b0 + c0 > a0 {
        valid += 1;
      }
      if a1 + b1 > c1 && a1 + c1 > b1 && b1 + c1 > a1 {
        valid += 1;
      }
      if a2 + b2 > c2 && a2 + c2 > b2 && b2 + c2 > a2 {
        valid += 1;
      }
    }
    Ok(valid)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2016, 3)
}
