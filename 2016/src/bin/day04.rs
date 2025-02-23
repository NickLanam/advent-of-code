use advent_lib::runner::{Day, PartId};
use anyhow::{Result, bail};

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
          // Some guarantees about the line that make it faster to parse:
          // - The sector ID is always 3 digits (100-999 inclusive)
          // - The checksum is always exactly 5 characters wrapped in []
          // - The rest of the string won't need to be broken down further for either part.
          let l = line.len();
          Room {
            enc: line[0..(l - 11)].to_string(),
            sector_id: line[(l - 10)..(l - 7)].parse().unwrap(),
            checksum: line[(l - 6)..(l - 1)].to_string(),
          }
        })
        .collect(),
    )
  }

  fn part1(&self, rooms: &Parsed, _sample_name: Option<String>) -> Result<P1Out> {
    let mut out: u64 = 0;

    // The number of times each letter, in alphabetical order, appears.
    // Hand-initialized once, then copied, to speed up the loop a little.
    let initial_freq: [(char, u8); 26] = [
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

    'room_check: for room in rooms {
      // Copied, rather than re-initialized, to shave off a few microseconds.
      let mut freq = initial_freq;
      for c in room.enc.chars() {
        if c == '-' {
          continue;
        }
        let i: u8 = c.try_into().unwrap();
        freq[i as usize - 97].1 += 1; // 97 is ASCII 'a', and we will only see letters
      }

      // This is a tiny bit faster than `freq.sort_by(|(_, a), (_, b)| b.cmp(a))`
      freq.sort_by_key(|(_, k)| std::cmp::Reverse(*k));

      // If the entire checksum matches (stop early if not), it counts
      for (check, c) in room.checksum.chars().enumerate() {
        if freq[check].0 != c {
          continue 'room_check;
        }
      }
      out += room.sector_id as u64;
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
