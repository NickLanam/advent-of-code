use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = String;
type P2Out = String;
type Parsed = Vec<String>;

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
    let mut x: i8 = 0;
    let mut y: i8 = 0;
    let mut code: Vec<i8> = vec![];
    for line in lines {
      for c in line.chars() {
        match c {
          'U' => {
            y = (y - 1).max(0);
          }
          'R' => {
            x = (x + 1).min(2);
          }
          'D' => {
            y = (y + 1).min(2);
          }
          'L' => {
            x = (x - 1).max(0);
          }
          _ => {
            panic!("Bad parse");
          }
        }
      }
      code.push((x + 1) + (3 * y));
    }
    Ok(code.iter().map(|i| i.to_string()).collect::<String>())
  }

  fn part2(&self, lines: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    let pad = [
      ['0', '0', '1', '0', '0'],
      ['0', '2', '3', '4', '0'],
      ['5', '6', '7', '8', '9'],
      ['0', 'A', 'B', 'C', '0'],
      ['0', '0', 'D', '0', '0'],
    ];

    let mut x: i8 = 0;
    let mut y: i8 = 2;
    let mut code = String::new();
    for line in lines {
      for c in line.chars() {
        match c {
          'U' => {
            if y > 0 && pad[(y - 1) as usize][x as usize] != '0' {
              y -= 1;
            }
          }
          'R' => {
            if x <= 3 && pad[y as usize][(x + 1) as usize] != '0' {
              x += 1;
            }
          }
          'D' => {
            if y <= 3 && pad[(y + 1) as usize][x as usize] != '0' {
              y += 1;
            }
          }
          'L' => {
            if x > 0 && pad[y as usize][(x - 1) as usize] != '0' {
              x -= 1;
            }
          }
          _ => {
            panic!("Bad parse");
          }
        }
      }
      code.push(pad[y as usize][x as usize]);
    }
    Ok(code)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2016, 2)
}
