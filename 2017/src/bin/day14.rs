use advent_lib::{
  grid::Infinite2dSet,
  runner::{Day, PartId},
};
use anyhow::Result;
use rayon::prelude::*;

type P1Out = u32;
type P2Out = usize;
type Parsed = Vec<u128>;

fn knot_rounds(buffer: &mut [u8], key: &[u8], rounds: usize) {
  let len = buffer.len();
  let mut cursor: usize = 0;
  for r in 0..rounds {
    for (i, &window_u8) in key.iter().enumerate() {
      let window = window_u8 as usize;

      // Reverse the target section by walking outside-in
      let mut left = cursor;
      let mut right = cursor + window - 1;
      while left < right {
        // Doing this conditionally cuts runtime down quite a lot!
        let l2 = if left >= len { left % len } else { left };
        let r2 = if right >= len { right % len } else { right };
        buffer.swap(l2, r2);
        left += 1;
        right -= 1;
      }

      // Move the cursor
      cursor = (cursor + window + i + (key.len() * r)) % len;
    }
  }
}

fn knot_hash(key: &[u8]) -> u128 {
  let mut knot: Vec<u8> = (0_u8..=255).collect();
  knot_rounds(&mut knot, key, 64);

  knot.chunks_exact(16).fold(0_u128, |mut out, bytes| {
    out <<= 8;
    for byte in bytes {
      out ^= (*byte as u128) & 0xff;
    }
    out
  })
}

fn make_grid(in_key: &[u8]) -> Vec<u128> {
  // Same procedure as 2017 Day 10 Part 2, with a different key.
  // Doing it 128 times is slow, but par_iter helps a ton here.
  (0_u8..=127)
    .into_par_iter()
    .map(|n| {
      let mut key: Vec<u8> = Vec::with_capacity(in_key.len() + 9);
      key.extend(in_key.iter());
      key.push(45); // ASCII '-'
      key.extend(n.to_string().bytes());
      key.extend([17, 31, 73, 47, 23].iter());
      knot_hash(&key)
    })
    .collect()
}

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    let key: Vec<u8> = lines[0].bytes().collect();
    Ok(make_grid(&key))
  }

  fn part1(&self, grid: &Parsed, _: Option<String>) -> Result<P1Out> {
    Ok(grid.iter().fold(0, |a, b| a + b.count_ones()))
  }

  /// A group finder similar to 2017 Day 12 Part 2.
  /// This time, we don't need to remember the groups, only discover them.
  fn part2(&self, grid: &Parsed, _: Option<String>) -> Result<P2Out> {
    // Roughly half of the nodes are set, faster to allocate only what we use
    let mut visited = Infinite2dSet::new(128 * 64);
    let mut groups = 0;

    let is_set = |x: i32, y: i32| {
      if x < 0 || y < 0 || x > 127 || y > 127 {
        return false;
      }
      let row = grid[y as usize];
      row & (1 << (127 - x)) != 0
    };

    let mut visit = |x: i32, y: i32| {
      if !is_set(x, y) || visited.contains(x, y) {
        return;
      }
      let mut frontier: Vec<(i32, i32)> = vec![(x, y)];
      while let Some((fx, fy)) = frontier.pop() {
        for (nx, ny) in [(fx - 1, fy), (fx, fy - 1), (fx + 1, fy), (fx, fy + 1)] {
          if is_set(nx, ny) && !visited.contains(nx, ny) {
            visited.insert(nx, ny);
            frontier.push((nx, ny));
          }
        }
      }
      groups += 1;
    };

    for y in 0..=127 {
      for x in 0..=127 {
        visit(x, y);
      }
    }

    Ok(groups)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2017, 14)
}
