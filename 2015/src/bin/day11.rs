use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = String;
type P2Out = String;
type Parsed = u64;

fn find_next_pass(old: &Parsed) -> u64 {
  let inc = |p: u64| -> u64 {
    let mut next = p + 1;
    // This is an absurd bit-twiddling approach, but the compiler (try looking at it in Godbolt)
    // recognizes that it can unroll this loop into something pretty darned efficient
    for ii in 0..=7 {
      let i: u16 = ii << 3;
      let c = (next >> i) & 0xFF;
      match c {
        // i, l, and o are excluded. When encountering one,
        // increment until that character is legal.
        // This is done with a shortcut for performance sake.
        0x69 | 0x6C | 0x6F => {
          next += 1 << i;
          // Set all lower bytes to 0, then set then to 'a' (0x61)
          // The compiler is smart enough to turn this into instructions purpose-built for the task!
          if i > 0 {
            next = (next >> i) << i;
            for j in 0..=(ii - 1) {
              next += 0x61 << (j << 3);
            }
          }
        }
        // '}' is what's after 'z', wrap it back around to 'a'
        0x7B => {
          next -= 26 << i;
          next += 1 << (i + 8);
        }
        // Otherwise, no change needed for this digit
        _ => {}
      }
    }
    next
  };

  let is_valid = |p: u64| -> bool {
    // Check first requirement: there must be a run of three consecutive letters
    let mut seq = false;
    for ii in 0..=4 {
      let i: u16 = ii << 3;
      let c = (p >> i) & 0xFF;
      let d = (p >> (i + 8)) & 0xFF;
      let e = (p >> (i + 16)) & 0xFF;
      if d == c - 1 && e == d - 1 {
        seq = true;
        break;
      }
    }
    if !seq {
      return false;
    }

    // Check second requirement: there must be two DIFFERENT double letters
    let mut first: u8 = 0;
    let mut second: u8 = 0;
    for ii in 0..=6 {
      let i: u16 = ii << 3;
      let c: u8 = ((p >> i) & 0xFF) as u8;
      let d: u8 = ((p >> (i + 8)) & 0xFF) as u8;
      if first == 0 && c == d {
        // Note here: we theoretically should also ii += 1 here.
        // However, since the first and second sequences aren't allowed
        // to be the same, we happen to skip the next one anyway!
        first = c;
      } else if first != 0 && c == d {
        second = c;
      }
    }
    if first == 0 || second == 0 || first == second {
      return false;
    }
    true
  };

  // The password might have been valid before, so increment it once.
  // This also immediately fixes the i, l, o characters so that is_valid doesn't need to look for them.
  let mut password = inc(*old);
  while !is_valid(password) {
    password = inc(password);
  }
  password
}

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  /// 8 characters in ASCII just so happens to fit perfectly into a u64.
  /// That means we can do bit twiddling instead of string manipulation,
  /// which will run _much_ faster!
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    let bytes = lines[0].as_bytes();
    assert_eq!(bytes.len(), 8);
    let mut out: u64 = 0;

    // We can't force-cast bytes to a [u8; 8] to do u64::from_be_bytes,
    // so we have to iterate. The compiler is smart enough to fix it.
    for (i, c) in bytes.iter().enumerate() {
      out += (*c as u64) << (8 * (7 - i));
    }
    Ok(out)
  }

  fn part1(&self, old: &Parsed, _sample_name: Option<String>) -> Result<P1Out> {
    let password = find_next_pass(old);
    let out = String::from_iter(
      password
        .to_be_bytes()
        .into_iter()
        .map(|b| char::from_u32(b as u32).unwrap()),
    );
    Ok(out)
  }

  fn part2(&self, old: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    let password = find_next_pass(&find_next_pass(old));
    let out = String::from_iter(
      password
        .to_be_bytes()
        .into_iter()
        .map(|b| char::from_u32(b as u32).unwrap()),
    );
    Ok(out)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2015, 11)
}
