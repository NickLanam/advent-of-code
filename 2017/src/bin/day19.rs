use advent_lib::{
  direction::{CardinalDirection, Rotation},
  runner::{Day, PartId},
};
use anyhow::{Context, Result};

type P1Out = String;
type P2Out = u32;
type Parsed = Vec<Vec<char>>;

fn trace(maze: &[Vec<char>]) -> Result<(String, u32)> {
  let (w, h) = (maze[0].len() as i32, maze.len() as i32);
  let mut seen_letters: Vec<char> = vec![];
  let mut steps_taken = 1; // Includes the initial space, per puzzle description

  let mut d: CardinalDirection = CardinalDirection::S;
  let mut cy: i32 = 0;
  let mut cx: i32 = maze[0]
    .iter()
    .position(|&c| c == '|')
    .context("Failed to find start point")? as i32;

  // Right from the puzzle description: move forward if we can.
  // If we can't, try left and try right. Only one of those two
  // will exist in that case (I checked my input to prove it).
  // Whenever we move, count that as a step taken, and remember
  // any letters we see along the way. That's the whole puzzle.
  'movement: loop {
    for d2 in [d, d + Rotation::L, d + Rotation::R] {
      let (nx, ny) = d2.apply(cx, cy, 1);
      if nx >= 0 && ny >= 0 && nx < w && ny < h {
        let c = maze[ny as usize][nx as usize];
        match c {
          '|' | '+' | '-' => {
            steps_taken += 1;
            d = d2;
            cx = nx;
            cy = ny;
            continue 'movement;
          }
          ' ' => {}
          _ => {
            steps_taken += 1;
            d = d2;
            cx = nx;
            cy = ny;
            seen_letters.push(c);
            continue 'movement;
          }
        }
      }
    }
    break;
  }

  Ok((seen_letters.iter().collect(), steps_taken))
}

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(lines.iter().map(|line| line.chars().collect()).collect())
  }

  fn part1(&self, maze: &Parsed, _: Option<String>) -> Result<P1Out> {
    let (seen_letters, _) = trace(maze)?;
    Ok(seen_letters)
  }

  fn part2(&self, maze: &Parsed, _: Option<String>) -> Result<P2Out> {
    let (_, steps_taken) = trace(maze)?;
    Ok(steps_taken)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2017, 19)
}
