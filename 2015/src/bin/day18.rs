use advent_lib::runner::{Day, PartId};
use anyhow::Result;
use fnv::{FnvBuildHasher, FnvHashSet};

type P1Out = usize;
type P2Out = usize;

#[derive(Debug)]
struct Parsed {
  w: i32,
  h: i32,
  grid: FnvHashSet<(i32, i32)>,
}

fn count_neighbors(grid: &FnvHashSet<(i32, i32)>, x: i32, y: i32, w: i32, h: i32) -> i32 {
  let mut count: i32 = 0;
  for (sx, sy) in [
    (x - 1, y - 1),
    (x, y - 1),
    (x + 1, y - 1),
    (x - 1, y),
    (x + 1, y),
    (x - 1, y + 1),
    (x, y + 1),
    (x + 1, y + 1),
  ] {
    if sx >= 0 && sx < w && sy >= 0 && sy < h && grid.contains(&(sx, sy)) {
      count += 1;
    }
  }
  count
}

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    let h = lines.len() as i32;
    let w = lines[0].len() as i32;
    let mut grid: FnvHashSet<(i32, i32)> =
      FnvHashSet::with_capacity_and_hasher((w * h) as usize, FnvBuildHasher::default());

    for (y, line) in lines.iter().enumerate() {
      for (x, c) in line.chars().enumerate() {
        if c == '#' {
          grid.insert((x as i32, y as i32));
        }
      }
    }

    Ok(Parsed { w, h, grid })
  }

  fn part1(&self, parsed: &Parsed, _sample_name: Option<String>) -> Result<P1Out> {
    let Parsed { w, h, grid: gg } = parsed;
    let mut grid = gg.clone();
    let num_steps: i32 = if *w == 6 { 4 } else { 100 };

    for _ in 0..num_steps {
      let mut next_grid =
        FnvHashSet::with_capacity_and_hasher((w * h) as usize, FnvBuildHasher::default());
      for y in 0..*h {
        for x in 0..*w {
          let n = count_neighbors(&grid, x, y, *w, *h);
          let has = grid.contains(&(x, y));
          if (has && (2..=3).contains(&n)) || (!has && n == 3) {
            next_grid.insert((x, y));
          }
        }
      }
      grid = next_grid;
    }

    Ok(grid.len())
  }

  fn part2(&self, parsed: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    let Parsed { w, h, grid: gg } = parsed;
    let mut grid = gg.clone();
    grid.insert((0, 0));
    grid.insert((w - 1, 0));
    grid.insert((w - 1, h - 1));
    grid.insert((0, h - 1));
    let num_steps: i32 = if *w == 6 { 5 } else { 100 };

    for _ in 0..num_steps {
      let mut next_grid =
        FnvHashSet::with_capacity_and_hasher((w * h) as usize, FnvBuildHasher::default());
      next_grid.insert((0, 0));
      next_grid.insert((w - 1, 0));
      next_grid.insert((w - 1, h - 1));
      next_grid.insert((0, h - 1));
      for y in 0..*h {
        for x in 0..*w {
          let n = count_neighbors(&grid, x, y, *w, *h);
          let has = grid.contains(&(x, y));
          if (has && (2..=3).contains(&n)) || (!has && n == 3) {
            next_grid.insert((x, y));
          }
        }
      }
      grid = next_grid;
    }

    Ok(grid.len())
  }
}

fn main() -> Result<()> {
  Solver {}.run(2015, 18)
}
