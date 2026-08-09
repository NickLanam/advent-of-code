use advent_lib::{
  grid::Infinite2dSet,
  runner::{Day, PartId},
};
use anyhow::Result;

type P1Out = usize;
type P2Out = usize;
struct Parsed {
  width: i32,
  height: i32,
  grid: Infinite2dSet,
}
impl Parsed {
  fn tree_check(&self, row_increment: i32, col_increment: i32) -> usize {
    let mut trees = 0;
    let mut x = 0;
    let mut y = 0;
    while y < self.height {
      if self.grid.contains(x, y) {
        trees += 1;
      }
      x = (x + col_increment) % self.width;
      y += row_increment;
    }
    trees
  }
}

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    // Typecasting these now avoids doing it on every iteration in tree_check.
    let width = lines[0].len() as i32;
    let height = lines.len() as i32;
    let grid = Infinite2dSet::from_input_lines(&lines, |c| c == '#');
    Ok(Parsed {
      width,
      height,
      grid,
    })
  }

  fn part1(&self, parsed: &Parsed, _: Option<String>) -> Result<P1Out> {
    Ok(parsed.tree_check(1, 3))
  }

  fn part2(&self, parsed: &Parsed, _: Option<String>) -> Result<P2Out> {
    Ok(
      parsed.tree_check(1, 1)
        * parsed.tree_check(1, 3)
        * parsed.tree_check(1, 5)
        * parsed.tree_check(1, 7)
        * parsed.tree_check(2, 1),
    )
  }
}

fn main() -> Result<()> {
  Solver {}.run(2020, 3)
}
