use advent_lib::runner::{Day, PartId};
use anyhow::{Result, bail};

#[derive(Debug)]
enum Dir {
  N,
  NE,
  SE,
  S,
  SW,
  NW,
}

type P1Out = u64;
type P2Out = u64;
type Parsed = Vec<Dir>;

fn solve(dirs: &[Dir]) -> (u64, u64) {
  let [
    mut n,
    mut ne,
    mut se,
    mut s,
    mut sw,
    mut nw,
    mut total,
    mut max,
  ] = [0_u64; 8];

  for dir in dirs {
    total += 1;
    match dir {
      Dir::N => {
        n += 1;
      }
      Dir::S => {
        s += 1;
      }
      Dir::NE => {
        ne += 1;
      }
      Dir::NW => {
        nw += 1;
      }
      Dir::SE => {
        se += 1;
      }
      Dir::SW => {
        sw += 1;
      }
    }

    // Moves that cancel out
    while n > 0 && s > 0 {
      total -= 2;
      n -= 1;
      s -= 1;
    }
    while ne > 0 && sw > 0 {
      total -= 2;
      ne -= 1;
      sw -= 1;
    }
    while nw > 0 && se > 0 {
      total -= 2;
      nw -= 1;
      se -= 1;
    }

    // Moves that reduce to a different one
    while ne > 0 && nw > 0 {
      total -= 1;
      ne -= 1;
      nw -= 1;
      n += 1;
    }
    while se > 0 && sw > 0 {
      total -= 1;
      se -= 1;
      sw -= 1;
      s += 1;
    }
    while n > 0 && se > 0 {
      total -= 1;
      n -= 1;
      se -= 1;
      ne += 1;
    }
    while n > 0 && sw > 0 {
      total -= 1;
      n -= 1;
      sw -= 1;
      nw += 1;
    }
    while s > 0 && ne > 0 {
      total -= 1;
      s -= 1;
      ne -= 1;
      se += 1;
    }
    while s > 0 && nw > 0 {
      total -= 1;
      s -= 1;
      nw -= 1;
      sw += 1;
    }

    max = max.max(total);
  }

  (total, max)
}

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    let mut dirs = vec![];
    for d in lines[0].split(',') {
      dirs.push(match d {
        "n" => Dir::N,
        "ne" => Dir::NE,
        "se" => Dir::SE,
        "s" => Dir::S,
        "sw" => Dir::SW,
        "nw" => Dir::NW,
        _ => bail!("Unknown dir {d}"),
      })
    }
    Ok(dirs)
  }

  fn part1(&self, dirs: &Parsed, _: Option<String>) -> Result<P1Out> {
    Ok(solve(dirs).0)
  }

  fn part2(&self, dirs: &Parsed, _: Option<String>) -> Result<P2Out> {
    Ok(solve(dirs).1)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2017, 11)
}
