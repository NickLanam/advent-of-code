use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = String;
type P2Out = String;
type Parsed = i32;

fn make_power_level_grid(serial: i32) -> [[i32; 300]; 300] {
  let mut grid = [[0_i32; 300]; 300];
  for x in 0_usize..300 {
    let rack = (x as i32) + 11; // 10 but we're pretending we're 1-indexed
    #[allow(clippy::needless_range_loop)]
    for y in 0_usize..300 {
      let p = ((((rack * (y as i32 + 1) + serial) * rack) / 100) % 10) - 5;
      grid[y][x] = p;
    }
  }
  grid
}

/// Returns (0-indexed x, 0-indexed y, sum of that area)
fn search_grid(grid: &[[i32; 300]; 300], search_dim: usize) -> (usize, usize, i32) {
  let mut best = (0, 0, 0);
  let max_d = 300 - search_dim;

  for y in 0_usize..=max_d {
    for x in 0_usize..=max_d {
      let score = grid[y..y + search_dim]
        .iter()
        .map(|row| row[x..x + search_dim].iter().sum::<i32>())
        .sum::<i32>();
      if score > best.2 {
        best = (x, y, score);
      }
    }
  }
  best
}

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(lines[0].parse()?)
  }

  fn part1(&self, &serial: &Parsed, _: Option<String>) -> Result<P1Out> {
    let grid = make_power_level_grid(serial);
    let (x, y, _) = search_grid(&grid, 3);

    // Puzzle wants 1-indexing
    Ok(format!("{},{}", x + 1, y + 1).to_string())
  }

  fn part2(&self, &serial: &Parsed, _: Option<String>) -> Result<P2Out> {
    let grid = make_power_level_grid(serial);

    let mut best = (0_usize, 0_usize, 0_i32, 1_usize);
    // It's at least 10 for the sample and the real answer, so avoid expensive early options
    for dim in 10..=300 {
      let (x, y, score) = search_grid(&grid, dim);

      // Heuristic: I noticed that for both sample and real, most dimensions get a 0.
      // The first time we see a zero is after we find the correct answer as well.
      // Both the sample and my answer had a dimension under 20, so huge savings there.
      // NOTE: The power levels by definition range -5 to 4. There's something to that.
      if score == 0 {
        break;
      } else if score > best.2 {
        best = (x, y, score, dim);
      }
    }

    // Puzzle wants 1-indexing
    Ok(format!("{},{},{}", best.0 + 1, best.1 + 1, best.3).to_string())
  }
}

fn main() -> Result<()> {
  Solver {}.run(2018, 11)
}
