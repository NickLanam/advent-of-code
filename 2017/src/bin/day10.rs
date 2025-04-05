use advent_lib::runner::{Day, PartId};
use anyhow::Result;
use std::fmt::Write;

type P1Out = u64;
type P2Out = String;
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

  fn part1(&self, in_bytes: &Parsed, sample_name: Option<String>) -> Result<P1Out> {
    let len = if sample_name.is_some() { 4 } else { 255 };
    let mut buffer: Vec<u8> = (0..=len).collect();
    knot_rounds(&mut buffer, in_bytes, 1);

    Ok((buffer[0] as u64) * (buffer[1] as u64))
  }

  fn part2(&self, in_bytes: &Parsed, _: Option<String>) -> Result<P2Out> {
    let mut buffer: Vec<u8> = (0..=255).collect();
    knot_rounds(&mut buffer, in_bytes, 64);

    Ok(
      buffer
        .chunks_exact(16)
        .fold(String::with_capacity(32), |mut out, chunk| {
          let v: u8 = chunk.iter().fold(0x00, |a, b| a ^ b);
          let _ = write!(out, "{v:02x}");
          out
        }),
    )
  }
}

fn main() -> Result<()> {
  Solver {}.run(2017, 10)
}
