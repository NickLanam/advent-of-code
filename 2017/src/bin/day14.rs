use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = u32;
type P2Out = u64;
type Parsed = Vec<u8>;

fn knot_rounds(buffer: &mut [u8], in_bytes: &[u8], rounds: usize) {
  let len = buffer.len();
  let mut cursor: usize = 0;
  for r in 0..rounds {
    for (i, &window_u8) in in_bytes.iter().enumerate() {
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
      cursor = (cursor + window + i + (in_bytes.len() * r)) % len;
    }
  }
}

fn make_knot(key: &[u8]) -> u128 {
  let mut knot: Vec<u8> = (0_u8..=255).collect();
  knot_rounds(&mut knot, key, 64);
  let hash = knot.chunks_exact(16).fold(0_u128, |mut out, bytes| {
    out <<= 8;
    for byte in bytes {
      out ^= (*byte as u128) & 0xff;
    }
    out
  });

  hash
}

fn make_grid(in_key: &[u8]) -> Vec<u128> {
  (0_u8..128)
    .map(|n| {
      let mut key: Vec<u8> = in_key.to_vec();
      let mut rest: Vec<u8> = format!("-{n}").bytes().collect();
      let mut always: Vec<u8> = vec![17, 31, 73, 47, 23];
      key.append(&mut rest);
      key.append(&mut always);
      make_knot(&key)
    })
    .collect()
}

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    Ok(lines[0].bytes().collect())
  }

  fn part1(&self, in_bytes: &Parsed, _sample_name: Option<String>) -> Result<P1Out> {
    Ok(
      make_grid(in_bytes)
        .iter()
        .fold(0, |a, b| a + b.count_ones()),
    )
  }

  fn part2(&self, _lines: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    Ok(0)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2017, 14)
}
