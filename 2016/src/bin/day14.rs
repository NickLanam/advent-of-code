use advent_lib::runner::{Day, PartId};
use anyhow::{Result, bail};
use crypto::digest::Digest;
use crypto::md5::Md5;

type P1Out = usize;
type P2Out = usize;
type Parsed = String;

fn solve(salt: &str, iterations: usize) -> Result<usize> {
  let mut hasher = Md5::new();
  let mut tracking: Vec<(usize, String, bool)> = Vec::with_capacity(1_000);
  let mut discovered: Vec<usize> = Vec::with_capacity(64);
  for i in 0..usize::MAX {
    hasher.reset();
    hasher.input_str(&format!("{salt}{i}"));
    for _ in 0..iterations {
      let prev = hasher.result_str();
      hasher.reset();
      hasher.input_str(prev.as_str());
    }
    let digest = hasher.result_str();
    if !tracking.is_empty() {
      tracking.retain(|(pos, _search, consumed)| !*consumed && i < *pos + 1_000);
      let mut to_consume: Vec<usize> = Vec::with_capacity(tracking.len());
      for (ti, (pos, search, _consumed)) in tracking.iter().enumerate() {
        if digest.contains(search) {
          to_consume.push(ti);
          discovered.push(*pos);
          if discovered.len() == 64 {
            return Ok(*pos);
          }
        }
      }
      for tci in to_consume {
        // To be removed with the retain() call on the next pass
        tracking[tci].2 = true;
      }
    }

    // Lastly, check if this one could be a key, itself
    let mut run = 0;
    let mut run_char = '_';
    for c in digest.chars() {
      if c == run_char {
        run += 1;
        if run == 3 {
          tracking.push((i, c.to_string().repeat(5), false));
          break;
        }
      } else {
        run = 1;
        run_char = c;
      }
    }
  }
  bail!("Failed to find!")
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

  fn part1(&self, salt: &Parsed, _sample_name: Option<String>) -> Result<P1Out> {
    solve(salt, 0)
  }

  fn part2(&self, salt: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    solve(salt, 2016)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2016, 14)
}
