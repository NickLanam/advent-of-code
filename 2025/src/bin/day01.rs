use advent_lib::runner::{Day, PartId};
use anyhow::Result;

#[derive(Clone, Debug, PartialEq)]
enum Dir {
  L,
  R,
}

type P1Out = usize;
type P2Out = usize;
type Parsed = Vec<(Dir, usize)>;

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(
      lines
        .iter()
        .map(|s| s.split_at(1))
        .map(|(l, r)| {
          let d = match l.chars().next().unwrap() {
            'L' => Dir::L,
            'R' => Dir::R,
            _ => panic!("What? {l}"),
          };
          let n = r.parse().unwrap();
          (d, n)
        })
        .collect(),
    )
  }

  fn part1(&self, steps: &Parsed, _: Option<String>) -> Result<P1Out> {
    let mut times = 0;
    let mut pos = 50;
    for (d, n) in steps.iter() {
      match d {
        Dir::L => {
          pos += 100 - (n % 100);
        }
        Dir::R => {
          pos += n % 100;
        }
      }
      pos %= 100;
      if pos == 0 {
        times += 1;
      }
    }
    Ok(times)
  }

  fn part2(&self, steps: &Parsed, _: Option<String>) -> Result<P2Out> {
    let mut times = 0;
    let mut pos = 50;
    for (d, n0) in steps.iter() {
      let mut n = *n0;
      times += n / 100; // Integer division. Cycling through 0 counts as reaching 0.
      n %= 100;
      match d {
        Dir::L => {
          if pos != 0 && n >= pos {
            times += 1;
          }
          pos += 100 - n;
        }
        Dir::R => {
          if pos + n >= 100 {
            times += 1;
          }
          pos += n;
        }
      }
      pos %= 100;
    }
    Ok(times)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2025, 1)
}
