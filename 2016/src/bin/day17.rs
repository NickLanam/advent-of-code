use std::collections::VecDeque;

use advent_lib::runner::{Day, PartId};
use anyhow::{Result, bail};
use crypto::digest::Digest;
use crypto::md5::Md5;

type P1Out = String;
type P2Out = String;
type Parsed = String;

#[derive(Clone, Debug)]
struct Path {
  x: u8,
  y: u8,
  // Easier to keep it as a string for the hashing later
  path: String,
}

fn solve(which: PartId, passkey: &String) -> Result<String> {
  let mut hasher = Md5::new();
  let mut frontier: VecDeque<Path> = VecDeque::with_capacity(100);
  let mut hash_buf: [u8; 16] = [0; 16];
  frontier.push_back(Path {
    x: 0,
    y: 0,
    path: passkey.to_string(),
  });
  let mut longest_so_far: usize = 0;

  while let Some(Path { x, y, path }) = frontier.pop_front() {
    if x == 3 && y == 3 {
      match which {
        PartId::P1 => {
          return Ok(path[passkey.len()..].to_string());
        }
        PartId::P2 => {
          longest_so_far = path.len() - passkey.len();
          continue;
        }
      }
    }
    // Theoretically, this could be made faster by storing the hash too, and using
    // a lengthening attack to only need to partially compute the new hash from the old.
    // The library here strictly enforces you NOT doing that, so would need to implement
    // it the hard way probably.
    hasher.reset();
    hasher.input_str(&path);
    hasher.result(&mut hash_buf);
    let u = ((hash_buf[0] & 0xF0) >> 4) > 0xA;
    let d = (hash_buf[0] & 0x0F) > 0xA;
    let l = ((hash_buf[1] & 0xF0) >> 4) > 0xA;
    let r = (hash_buf[1] & 0x0F) > 0xA;
    if x > 0 && l {
      let mut west = path.to_string();
      west.push('L');
      frontier.push_back(Path {
        x: x - 1,
        y,
        path: west,
      });
    }
    if x < 3 && r {
      let mut east = path.to_string();
      east.push('R');
      frontier.push_back(Path {
        x: x + 1,
        y,
        path: east,
      });
    }
    if y > 0 && u {
      let mut north = path.to_string();
      north.push('U');
      frontier.push_back(Path {
        x,
        y: y - 1,
        path: north,
      });
    }
    if y < 3 && d {
      let mut south = path.to_string();
      south.push('D');
      frontier.push_back(Path {
        x,
        y: y + 1,
        path: south,
      });
    }
  }
  match which {
    PartId::P1 => {
      bail!("Failed to find any solutions");
    }
    PartId::P2 => Ok(longest_so_far.to_string()),
  }
}

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    Ok(lines[0].to_string())
  }

  fn part1(&self, passkey: &Parsed, _sample_name: Option<String>) -> Result<P1Out> {
    solve(PartId::P1, passkey)
  }

  fn part2(&self, passkey: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    solve(PartId::P2, passkey)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2016, 17)
}
