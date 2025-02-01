use advent_lib::runner::{Day, PartId};
use anyhow::{bail, Result};

#[derive(Debug)]
struct Room {
  enc: String,
  /// Happens to always be exactly 3 digits (so 100-999 inclusive)
  /// in both the sample and the real input. Fits in a u8!
  sector_id: u16,
  /// Note: Puzzle description says these must be 5 characters exactly,
  /// and they are for both the sample and real input.
  checksum: String,
}

type P1Out = u64;
type P2Out = u64;
type Parsed = Vec<Room>;

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    Ok(
      lines
        .iter()
        .map(|line| {
          let (left, right) = line.split_at(line.len() - 10);
          let (sector_id_raw, checksum_raw) = right.split_once('[').unwrap();
          let sector_id: u16 = sector_id_raw.parse().unwrap();
          let checksum: String = checksum_raw[..checksum_raw.len() - 1].to_string();
          Room {
            enc: left[..left.len() - 1].to_string(),
            sector_id,
            checksum,
          }
        })
        .collect(),
    )
  }

  fn part1(&self, rooms: &Parsed, _sample_name: Option<String>) -> Result<P1Out> {
    let mut out: u64 = 0;
    for room in rooms {
      // The number of times each letter, in alphabetical order, appears
      // Hand-coding this initializer makes it a tiny bit faster at runtime.
      let mut freq: [(char, usize); 26] = [
        ('a', 0),
        ('b', 0),
        ('c', 0),
        ('d', 0),
        ('e', 0),
        ('f', 0),
        ('g', 0),
        ('h', 0),
        ('i', 0),
        ('j', 0),
        ('k', 0),
        ('l', 0),
        ('m', 0),
        ('n', 0),
        ('o', 0),
        ('p', 0),
        ('q', 0),
        ('r', 0),
        ('s', 0),
        ('t', 0),
        ('u', 0),
        ('v', 0),
        ('w', 0),
        ('x', 0),
        ('y', 0),
        ('z', 0),
      ];
      for c in room.enc.chars() {
        if c == '-' {
          continue;
        }
        let i: u8 = c.try_into().unwrap();
        freq[i as usize - 97].1 += 1; // 97 is ASCII 'a', and we will only see letters
      }

      freq.sort_by(|(_, a), (_, b)| b.cmp(a));

      let correct_checksum = freq.iter().take(5).map(|(c, _)| c).collect::<String>();
      if correct_checksum == room.checksum {
        out += room.sector_id as u64;
      }
    }
    Ok(out)
  }

  fn part2(&self, rooms: &Parsed, sample_name: Option<String>) -> Result<P2Out> {
    if sample_name.is_some() {
      return Ok(1);
    }

    for room in rooms {
      // The string we're looking for, after decrypting, is "northpole object storage".
      // This means the answer will have a length of 24 characters. Ignore the rest.
      if room.enc.len() != 24 {
        continue;
      }

      let decrypted: String = room
        .enc
        .chars()
        .map(|c| {
          if c == '-' {
            return ' ';
          }
          let i: u8 = c.try_into().unwrap();
          let j: u8 = (((i as u64 - 97) + (room.sector_id as u64)) % 26)
            .try_into()
            .unwrap();
          let c2: char = (j + 97).into();
          c2
        })
        .collect::<String>();
      if decrypted == "northpole object storage" {
        return Ok(room.sector_id as u64);
      }
    }
    bail!("Did not find the workshop")
  }
}

fn main() -> Result<()> {
  Solver {}.run(2016, 4)
}
