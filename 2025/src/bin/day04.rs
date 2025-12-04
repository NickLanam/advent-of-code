use advent_lib::{
  grid::Infinite2dSet,
  runner::{Day, PartId},
};
use anyhow::Result;

type P1Out = u64;
type P2Out = u64;
type Parsed = Infinite2dSet;

fn is_accessible(grid: &Infinite2dSet, x: i32, y: i32) -> bool {
  ((grid.contains(x - 1, y - 1) as u8)
    + (grid.contains(x, y - 1) as u8)
    + (grid.contains(x + 1, y - 1) as u8)
    + (grid.contains(x - 1, y) as u8)
    + (grid.contains(x + 1, y) as u8)
    + (grid.contains(x - 1, y + 1) as u8)
    + (grid.contains(x, y + 1) as u8)
    + (grid.contains(x + 1, y + 1) as u8))
    < 4
}

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(Infinite2dSet::from_input_lines(&lines, |c| c == '@'))
  }

  fn part1(&self, grid: &Parsed, _: Option<String>) -> Result<P1Out> {
    Ok(grid.keys().fold(0, |acc, (x, y)| {
      if is_accessible(grid, x, y) {
        acc + 1
      } else {
        acc
      }
    }))
  }

  fn part2(&self, init_grid: &Parsed, _: Option<String>) -> Result<P2Out> {
    let mut total = 0;

    let mut grid = init_grid.clone();
    loop {
      let mut removes: Vec<(i32, i32)> = vec![];
      for (x, y) in grid.keys() {
        if is_accessible(&grid, x, y) {
          removes.push((x, y));
        }
      }
      if removes.is_empty() {
        break;
      } else {
        for (rx, ry) in removes {
          grid.remove(rx, ry);
          total += 1;
        }
      }
    }

    Ok(total)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2025, 4)
}
