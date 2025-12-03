use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = u64;
type P2Out = String;
type Parsed = Vec<u8>;

fn knot_rounds(buffer: &mut [u8], key: &[u8], rounds: usize) {
  let len = buffer.len();
  let mut cursor: usize = 0;
  for r in 0..rounds {
    for (i, &window_u8) in key.iter().enumerate() {
      let window = window_u8 as usize;

      // Reverse the target section by walking outside-in
      let mut left = cursor;
      let mut right = cursor + window - 1;
      while left < right {
        // Doing this conditionally cuts runtime down quite a lot!
        let l2 = if left >= len { left % len } else { left };
        let r2 = if right >= len { right % len } else { right };
        buffer.swap(l2, r2);
        left += 1;
        right -= 1;
      }

      // Move the cursor
      cursor = (cursor + window + i + (key.len() * r)) % len;
    }
  }
}

fn knot_hash(key: &[u8]) -> u128 {
  let mut knot: Vec<u8> = (0_u8..=255).collect();
  knot_rounds(&mut knot, key, 64);

  knot.chunks_exact(16).fold(0_u128, |mut out, bytes| {
    out <<= 8;
    for byte in bytes {
      out ^= (*byte as u128) & 0xff;
    }
    out
  })
}

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    for_part: PartId,
  ) -> Result<Parsed> {
    match for_part {
      PartId::P1 => Ok(lines[0].split(',').map(|n| n.parse().unwrap()).collect()),
      PartId::P2 => {
        let mut parsed: Vec<u8> = lines[0].bytes().collect();
        parsed.append(&mut vec![17, 31, 73, 47, 23]);
        Ok(parsed)
      }
    }
  }

  fn part1(&self, key: &Parsed, sample_name: Option<String>) -> Result<P1Out> {
    let len = if sample_name.is_some() { 4 } else { 255 };
    let mut buffer: Vec<u8> = (0..=len).collect();
    knot_rounds(&mut buffer, key, 1);

    Ok((buffer[0] as u64) * (buffer[1] as u64))
  }

  fn part2(&self, key: &Parsed, _: Option<String>) -> Result<P2Out> {
    let hash = knot_hash(key);
    Ok(format!("{hash:032x}"))
  }
}

fn main() -> Result<()> {
  Solver {}.run(2017, 10)
}
