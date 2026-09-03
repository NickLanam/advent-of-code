use advent_lib::{
  grid::{Infinite3dSet, Infinite4dSet},
  runner::{Day, PartId},
};
use anyhow::Result;

type P1Out = usize;
type P2Out = usize;
type Parsed = Vec<String>;

fn count_active_neighbors_3d(x: i32, y: i32, z: i32, state: &Infinite3dSet) -> usize {
  let mut out = 0;
  for xx in (x - 1)..=(x + 1) {
    for yy in (y - 1)..=(y + 1) {
      for zz in (z - 1)..=(z + 1) {
        if xx == x && yy == y && zz == z {
          continue;
        }
        if state.contains(xx, yy, zz) {
          out += 1;
        }
      }
    }
  }
  out
}

fn count_active_neighbors_4d(x: i32, y: i32, z: i32, w: i32, state: &Infinite4dSet) -> usize {
  let mut out = 0;
  for xx in (x - 1)..=(x + 1) {
    for yy in (y - 1)..=(y + 1) {
      for zz in (z - 1)..=(z + 1) {
        for ww in (w - 1)..=(w + 1) {
          if xx == x && yy == y && zz == z && ww == w {
            continue;
          }
          if state.contains(xx, yy, zz, ww) {
            out += 1;
          }
        }
      }
    }
  }
  out
}

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(lines)
  }

  fn part1(&self, lines: &Parsed, _: Option<String>) -> Result<P1Out> {
    let init_width = lines[0].len();
    let init_height = lines.len();
    let mut min_x = 0;
    let mut max_x = init_width as i32;
    let mut min_y = 0;
    let mut max_y = init_height as i32;
    let mut min_z = 0;
    let mut max_z = 0;
    let mut state = Infinite3dSet::from_input_layer0(lines, |ch| ch == '#');
    for _ in 0..6 {
      let mut next_state = Infinite3dSet::new(state.len());
      min_x -= 1;
      min_y -= 1;
      min_z -= 1;
      max_x += 1;
      max_y += 1;
      max_z += 1;
      for x in min_x..=max_x {
        for y in min_y..=max_y {
          for z in min_z..=max_z {
            let n = count_active_neighbors_3d(x, y, z, &state);
            let active = state.contains(x, y, z);
            if (active && (2..=3).contains(&n)) || (!active && n == 3) {
              next_state.insert(x, y, z);
            }
          }
        }
      }
      state = next_state;
    }
    Ok(state.len())
  }

  fn part2(&self, lines: &Parsed, _: Option<String>) -> Result<P2Out> {
    let init_width = lines[0].len();
    let init_height = lines.len();
    let mut min_x = 0;
    let mut max_x = init_width as i32;
    let mut min_y = 0;
    let mut max_y = init_height as i32;
    let mut min_z = 0;
    let mut max_z = 0;
    let mut min_w = 0;
    let mut max_w = 0;
    let mut state = Infinite4dSet::from_input_layer0(lines, |ch| ch == '#');
    for _ in 0..6 {
      let mut next_state = Infinite4dSet::new(state.len());
      min_x -= 1;
      min_y -= 1;
      min_z -= 1;
      max_x += 1;
      max_y += 1;
      max_z += 1;
      min_w -= 1;
      max_w += 1;
      for x in min_x..=max_x {
        for y in min_y..=max_y {
          for z in min_z..=max_z {
            for w in min_w..=max_w {
              let n = count_active_neighbors_4d(x, y, z, w, &state);
              let active = state.contains(x, y, z, w);
              if (active && (2..=3).contains(&n)) || (!active && n == 3) {
                next_state.insert(x, y, z, w);
              }
            }
          }
        }
      }
      state = next_state;
    }
    Ok(state.len())
  }
}

fn main() -> Result<()> {
  Solver {}.run(2020, 17)
}
