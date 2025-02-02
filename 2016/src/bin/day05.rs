use advent_lib::runner::{Day, PartId};
use anyhow::Result;
use crypto::digest::Digest;
use crypto::md5::Md5;

type Parsed = String;

struct Solver {}
impl Day<Parsed, String, String> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    Ok(lines[0].to_string())
  }

  fn part1(&self, input: &Parsed, _sample_name: Option<String>) -> Result<String> {
    // 2015 Day 4 was very nearly the same puzzle as this, so reusing mostly the same code.
    let mut hasher = Md5::new();
    let mut out = String::with_capacity(8);
    for i in 0_u64..u64::MAX {
      if out.len() == 8 {
        break;
      }
      hasher.reset();
      hasher.input_str(&format!("{}{}", input.clone(), i));
      let digest = hasher.result_str();
      if digest.starts_with("00000") {
        out.push(digest.chars().nth(5).unwrap());
      }
    }
    Ok(out)
  }

  fn part2(&self, input: &Parsed, _sample_name: Option<String>) -> Result<String> {
    let mut hasher = Md5::new();
    let mut out: [char; 8] = ['_', '_', '_', '_', '_', '_', '_', '_'];
    let mut found = 0_u8;
    for i in 0_u64..u64::MAX {
      if found == 8 {
        break;
      }
      hasher.reset();
      hasher.input_str(&format!("{}{}", input.clone(), i));
      let digest = hasher.result_str();
      if digest.starts_with("00000") {
        let pos: u8 = digest.chars().nth(5).unwrap().try_into().unwrap();
        if (pos - 48) < 8 && out[pos as usize - 48] == '_' {
          let c = digest.chars().nth(6).unwrap();
          out[pos as usize - 48] = c; // 48 is ASCII '0'
          found += 1;
        }
      }
    }
    Ok(String::from_iter(out.iter()))
  }
}

fn main() -> Result<()> {
  Solver {}.run(2016, 5)
}
