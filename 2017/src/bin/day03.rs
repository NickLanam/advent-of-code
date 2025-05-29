use advent_lib::{
  grid::Infinite2dGrid,
  runner::{Day, PartId},
};
use anyhow::{Result, bail};

type P1Out = u32;
type P2Out = u32;
type Parsed = u32;

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    Ok(lines[0].parse().unwrap())
  }

  fn part1(&self, &target: &Parsed, _: Option<String>) -> Result<P1Out> {
    // First, find the floor of the target's square root.
    // For even roots, we know that they're (root / 2) - 1 steps
    // left of 1, and (root / 2) steps above it.
    // For odd roots, we know they're floor(root / 2) steps
    // right and below 1.
    // In both cases, it's (root - 1) manhattan distance away.

    let root = (target as f32).sqrt().floor() as u32;

    let mut distance = root - 1;

    // From there, we move like this until reaching
    // the next square number:
    // - One step further away horizontally
    // - floor(root / 2) steps closer vertically
    // - ceil(root / 2) steps farther vertically
    // - ceil(root / 2) steps closer horizonally
    // - floor(root / 2) steps farther horizontally
    // - Then we're at the next square number
    // So, where we are in the above sequence
    // determines the answer!

    let mut rem = target - (root * root);

    // One step horizontally further away
    if rem > 0 {
      rem -= 1;
      distance += 1;
    }

    let mut f = root / 2;
    if rem > root {
      // Skip the vertical steps and go to the
      // horizontal ones as described above
      // This also shifts when we change from
      // getting closer to getting further.
      rem -= root;
      f += root % 2;
    }

    if rem <= f {
      distance -= rem;
    } else {
      distance += rem - f;
    }

    Ok(distance)
  }

  fn part2(&self, &target: &Parsed, _: Option<String>) -> Result<P2Out> {
    // This sequence is described by https://oeis.org/A141481
    // We COULD just look at that list to get the correct answer
    // (and doing so verifies this code).

    // Unfortunately, that doesn't tell us much, and we still do a brute
    // force solution with the same knowledge as part 1 about how the
    // spiral moves.

    // This sequence grows so fast that we compute only a tiny number of terms
    let mut memory = Infinite2dGrid::<u32>::new(50);
    memory.insert(0, 0, 1);

    let mut i = 0;
    let mut j = 0;

    for cell in (1..u32::MAX).step_by(2) {
      for (d_cell, d_i, d_j) in [(0, 1, 0), (0, 0, -1), (1, -1, 0), (1, 0, 1)] {
        for _ in 0..(cell + d_cell) {
          i += d_i;
          j += d_j;
          let mut next = 0;
          for k in (i - 1)..(i + 2) {
            for l in (j - 1)..(j + 2) {
              next += memory.get_or_default(k, l, &0);
              if next > target {
                return Ok(next);
              }
            }
          }
          memory.insert(i, j, next);
        }
      }
    }
    bail!("Failed to find an answer, generator code must be wrong");
  }
}

fn main() -> Result<()> {
  Solver {}.run(2017, 3)
}
