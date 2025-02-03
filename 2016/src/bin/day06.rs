use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = String;
type P2Out = String;
type Parsed = Vec<String>;

fn solve(lines: &Vec<String>, invert: bool) -> Result<String> {
  let w = lines[0].len();

  // The number of times each letter, in alphabetical order, appears.
  // Hand-initialized once, then copied, to speed up the loop a little.
  let initial_freq: [u8; 26] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  ];

  let mut freq: Vec<[u8; 26]> = Vec::with_capacity(w);
  for _ in 0..w {
    freq.push(initial_freq);
  }

  for line in lines {
    for (pos, col) in freq.iter_mut().enumerate() {
      let i: u8 = line.chars().nth(pos).unwrap().try_into().unwrap();
      col[i as usize - 97] += 1;
    }
  }

  let mut out = String::with_capacity(w);
  for col in freq.iter() {
    let mut best: u8 = if invert { 255 } else { 0 };
    let mut c: char = '_';
    for (l, letter) in col.iter().enumerate() {
      if (invert && *letter > 0 && *letter < best) || (!invert && *letter > best) {
        best = *letter;
        c = (l as u8 + 97) as char;
      }
    }
    out.push(c);
  }
  Ok(out)
}

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    Ok(lines)
  }

  fn part1(&self, lines: &Parsed, _sample_name: Option<String>) -> Result<P1Out> {
    solve(lines, false)
  }

  fn part2(&self, lines: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    solve(lines, true)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2016, 6)
}
