use advent_lib::{
  direction::{CardinalDirection, Rotation},
  grid::Infinite2dGrid,
  runner::{Day, PartId},
};
use anyhow::Result;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Cell {
  Clean,
  Weakened,
  Infected,
  Flagged,
}

type P1Out = u64;
type P2Out = u64;
type Parsed = (usize, Infinite2dGrid<Cell>);

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    let dim = lines.len();
    let mut grid: Infinite2dGrid<Cell> = Infinite2dGrid::new(dim * dim);
    for (y, line) in lines.iter().enumerate() {
      for (x, ch) in line.chars().enumerate() {
        if ch == '#' {
          grid.insert(x as i32, y as i32, Cell::Infected);
        }
      }
    }
    Ok((dim, grid))
  }

  fn part1(&self, (dim, init_grid): &Parsed, _: Option<String>) -> Result<P1Out> {
    let mut grid: Infinite2dGrid<Cell> = init_grid.clone();
    let mut x = (dim / 2) as i32;
    let mut y = (dim / 2) as i32;
    let mut d: CardinalDirection = CardinalDirection::N;
    let mut score = 0;

    for _ in 0..10_000 {
      if let Some(Cell::Infected) = grid.get(x, y) {
        d += Rotation::R;
        grid.insert(x, y, Cell::Clean);
      } else {
        d += Rotation::L;
        grid.insert(x, y, Cell::Infected);
        score += 1;
      }
      (x, y) = d.apply(x, y, 1);
    }

    Ok(score)
  }

  fn part2(&self, (dim, init_grid): &Parsed, _: Option<String>) -> Result<P2Out> {
    let mut grid: Infinite2dGrid<Cell> = init_grid.clone();
    let mut x = (dim / 2) as i32;
    let mut y = (dim / 2) as i32;
    let mut d: CardinalDirection = CardinalDirection::N;
    let mut score = 0;

    // TODO: This many iterations takes 600 milliseconds.
    // Part 1 takes 433 microseconds.
    // There's likely a state cycle to skip most iterations. Find it.
    // If that cycle just so happens to be in the first 10k, likely that
    // part 1 can benefit from the same optimization.
    for _ in 0..10_000_000 {
      match grid.get(x, y) {
        None | Some(Cell::Clean) => {
          d += Rotation::L;
          grid.insert(x, y, Cell::Weakened);
        }
        Some(Cell::Weakened) => {
          grid.insert(x, y, Cell::Infected);
          score += 1;
        }
        Some(Cell::Infected) => {
          d += Rotation::R;
          grid.insert(x, y, Cell::Flagged);
        }
        Some(Cell::Flagged) => {
          // Reverse by turning twice
          d += Rotation::R;
          d += Rotation::R;
          grid.remove(x, y);
        }
      }
      (x, y) = d.apply(x, y, 1);
    }

    Ok(score)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2017, 22)
}
