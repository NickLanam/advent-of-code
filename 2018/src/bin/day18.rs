use advent_lib::{
  grid::Infinite2dGrid,
  runner::{Day, PartId},
};
use anyhow::{Result, bail};
use fnv::{FnvBuildHasher, FnvHashMap};

#[derive(Clone, PartialEq)]
enum Cell {
  Open,
  Tree,
  Lumberyard,
}

type P1Out = usize;
type P2Out = usize;
type Parsed = Infinite2dGrid<Cell>;

// TODO: Since the real thing is only 50x50, a Vec<Cell> would likely
// be more efficient for the entire solution. Should try that.
// Even faster still might be to pack four cells to the byte, but
// unclear whether that would really be a speedup in practice with
// such a small dataset.
fn hash(grid: &Infinite2dGrid<Cell>) -> [u64; 32] {
  // The real grid is 50x50 and only three possible states per cell (2 bits).
  // That's 5000 bits of information with half a wasted bit per cell.
  // However, just the 32x32 top-left corner is enough for uniqueness (with my input),
  // and that's a bit faster to hash and test.
  let mut out = [0; 32];
  for (y, row) in out.iter_mut().enumerate() {
    for x in 0..32 {
      *row <<= 2;
      match grid.get(x, y as i32) {
        Some(Cell::Tree) => {
          *row += 1;
        }
        Some(Cell::Lumberyard) => {
          *row += 2;
        }
        _ => {}
      }
    }
  }
  out
}

// My own approach. Solves part 1 in 2 milliseconds, part 2 in about 100 milliseconds.
// That's fast enough, but this person optimized the crap out of it
// and solved in 42µs/2.2ms using SIMD shenanigans I want to learn about:
// https://www.reddit.com/r/adventofcode/comments/a77xq6/comment/ec28hd3/
fn solve(in_grid: &Infinite2dGrid<Cell>, rounds: usize) -> Result<usize> {
  let mut grid = in_grid.clone();
  let mut seen_offsets: FnvHashMap<[u64; 32], usize> =
    FnvHashMap::with_hasher(FnvBuildHasher::default());

  // Simulation rounds. Similar to CGOL.
  let mut pattern_found = false;
  let mut round = 0;
  while round < rounds {
    let mut next_grid = Infinite2dGrid::new(in_grid.len());
    'entries: for (x, y, c) in grid.entries() {
      next_grid.insert(x, y, c.clone()); // By default, things don't change
      let neighbors = [
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
      ];
      match c {
        Cell::Open => {
          let mut trees = 0;
          for (nx, ny) in neighbors {
            if let Some(Cell::Tree) = grid.get(nx, ny) {
              trees += 1;
              if trees >= 3 {
                next_grid.insert(x, y, Cell::Tree);
                continue 'entries;
              }
            }
          }
        }
        Cell::Tree => {
          let mut lumberyards = 0;
          for (nx, ny) in neighbors {
            if let Some(Cell::Lumberyard) = grid.get(nx, ny) {
              lumberyards += 1;
              if lumberyards >= 3 {
                next_grid.insert(x, y, Cell::Lumberyard);
                continue 'entries;
              }
            }
          }
        }
        Cell::Lumberyard => {
          let mut trees = 0;
          let mut lumberyards = 0;
          for (nx, ny) in neighbors {
            if let Some(Cell::Tree) = grid.get(nx, ny) {
              trees += 1;
            }
            if let Some(Cell::Lumberyard) = grid.get(nx, ny) {
              lumberyards += 1;
            }
            if trees > 0 && lumberyards > 0 {
              continue 'entries;
            }
          }
          next_grid.insert(x, y, Cell::Open);
        }
      }
    }
    grid = next_grid;

    if !pattern_found {
      let new_hash = hash(&grid);
      if let Some(offset) = seen_offsets.get(&new_hash) {
        pattern_found = true;
        let period = round - offset;
        while round + period < rounds {
          round += period;
        }
      } else {
        seen_offsets.insert(new_hash, round);
      }
    }
    round += 1;
  }

  // Scoring
  let mut trees: usize = 0;
  let mut lumberyards: usize = 0;
  for cell in grid.values() {
    match cell {
      Cell::Open => {}
      Cell::Tree => {
        trees += 1;
      }
      Cell::Lumberyard => {
        lumberyards += 1;
      }
    }
  }

  Ok(trees * lumberyards)
}

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    let mut grid: Infinite2dGrid<Cell> = Infinite2dGrid::new(lines.len() * lines[0].len());
    for (y, line) in lines.iter().enumerate() {
      let yy = y as i32;
      for (x, ch) in line.chars().enumerate() {
        let xx = x as i32;
        match ch {
          '.' => {
            grid.insert(xx, yy, Cell::Open);
          }
          '|' => {
            grid.insert(xx, yy, Cell::Tree);
          }
          '#' => {
            grid.insert(xx, yy, Cell::Lumberyard);
          }
          _ => {
            bail!("Unrecognized character at ({x},{y}): {ch}");
          }
        }
      }
    }
    Ok(grid)
  }

  fn part1(&self, in_grid: &Parsed, _: Option<String>) -> Result<P1Out> {
    solve(in_grid, 10)
  }

  fn part2(&self, in_grid: &Parsed, _: Option<String>) -> Result<P2Out> {
    solve(in_grid, 1_000_000_000)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2018, 18)
}
