use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = usize;
type P2Out = usize;

#[derive(Debug)]
struct PolicyAndPassword {
  lo: usize,
  hi: usize,
  ch: char,
  password: String,
}

impl PolicyAndPassword {
  fn valid_p1(&self) -> bool {
    let count = self
      .password
      .as_bytes()
      .iter()
      .filter(|&c| (*c as char) == self.ch)
      .count();
    count >= self.lo && count <= self.hi
  }

  fn valid_p2(&self) -> bool {
    // Note: it's 1-indexed in the puzzle description, so the math is unusual.
    let lc = self.password.as_bytes()[self.lo - 1] as char;
    let hc = self.password.as_bytes()[self.hi - 1] as char;
    lc != hc && (lc == self.ch || hc == self.ch)
  }
}

type Parsed = Vec<PolicyAndPassword>;

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    // This works great and is concise, but it's way more performant to skip the regex entirely.
    // let parse_re = Regex::new(r"(?<lo>\d+)-(?<hi>\d+) (?<ch>[a-z]): (?<password>.+)")?;
    Ok(
      lines
        .iter()
        .map(|line| {
          let dash_index = line.find('-').unwrap();
          let space_index = line.find(' ').unwrap();
          let lo: usize = line[0..dash_index].parse().unwrap();
          let hi: usize = line[(dash_index + 1)..space_index].parse().unwrap();
          let ch: char = line[(space_index + 1)..(space_index + 2)]
            .chars()
            .take(1)
            .last()
            .unwrap();
          let password = line[(space_index + 4)..].to_owned();
          PolicyAndPassword {
            lo,
            hi,
            ch,
            password,
          }
        })
        .collect(),
    )
  }

  fn part1(&self, policies: &Parsed, _: Option<String>) -> Result<P1Out> {
    Ok(policies.iter().filter(|p| p.valid_p1()).count())
  }

  fn part2(&self, policies: &Parsed, _: Option<String>) -> Result<P2Out> {
    Ok(policies.iter().filter(|p| p.valid_p2()).count())
  }
}

fn main() -> Result<()> {
  Solver {}.run(2020, 2)
}
