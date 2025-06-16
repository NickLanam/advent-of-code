use advent_lib::runner::{Day, PartId};
use anyhow::{Result, bail};

type P1Out = u64;
type P2Out = u64;
type Parsed = String;

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(lines[0].to_string())
  }

  fn part1(&self, input: &Parsed, _: Option<String>) -> Result<P1Out> {
    let w = 25;
    let h = 6;
    let lim = w * h;
    let mut fewest_zero = usize::MAX;
    let mut score = 0;
    let mut l = 0;

    let mut zero = 0;
    let mut one = 0;
    let mut two = 0;
    for ch in input.chars() {
      if l >= lim {
        if zero < fewest_zero {
          fewest_zero = zero;
          score = one * two;
        }
        zero = 0;
        one = 0;
        two = 0;
        l = 0;
      }
      l += 1;
      match ch {
        '0' => {
          zero += 1;
        }
        '1' => {
          one += 1;
        }
        '2' => {
          two += 1;
        }
        _ => bail!("Unreachable"),
      }
    }
    Ok(score)
  }

  fn part2(&self, input: &Parsed, _: Option<String>) -> Result<P2Out> {
    let w = 25;
    let h = 6;

    let mut image: Vec<Vec<char>> = vec![vec!['2'; 25]; 6];

    let mut x = 0;
    let mut y = 0;
    for ch in input.chars() {
      if x >= w {
        y += 1;
        x = 0;
      }
      if y >= h {
        y = 0;
      }
      match ch {
        '0' | '1' => {
          image[y][x] = if image[y][x] == '2' { ch } else { image[y][x] };
        }
        '2' => {
          // Transparent, so nothing happens
        }
        _ => bail!("Unreachable"),
      }
      x += 1;
    }
    for row in image {
      for ch in row {
        print!(
          "{}",
          match ch {
            '0' => ' ',
            '1' => '#',
            _ => '!',
          }
        );
      }
      println!();
    }
    Ok(0)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2019, 8)
}
