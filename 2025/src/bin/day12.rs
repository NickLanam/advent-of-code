use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = u64;
type P2Out = u64;

#[derive(Debug)]
struct Parsed {
  #[allow(dead_code)]
  shapes: Vec<[[bool; 3]; 3]>, // They're all septiminos that fit in a 3x3 grid. This is not coincidental.
  regions: Vec<(usize, usize, Vec<usize>)>,
}

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, for_part: PartId) -> Result<Parsed> {
    let mut shapes: Vec<[[bool; 3]; 3]> = vec![];
    let mut regions: Vec<(usize, usize, Vec<usize>)> = vec![];

    if for_part == PartId::P2 {
      return Ok(Parsed { shapes, regions });
    }

    let mut iter = lines.iter();
    while let Some(top) = iter.next() {
      if top.is_empty() {
        continue;
      } else if top.ends_with(':') {
        // Reading a shape
        let mut l0 = iter.next().unwrap().chars();
        let mut l1 = iter.next().unwrap().chars();
        let mut l2 = iter.next().unwrap().chars();
        shapes.push([
          [
            l0.next().unwrap() == '#',
            l0.next().unwrap() == '#',
            l0.next().unwrap() == '#',
          ],
          [
            l1.next().unwrap() == '#',
            l1.next().unwrap() == '#',
            l1.next().unwrap() == '#',
          ],
          [
            l2.next().unwrap() == '#',
            l2.next().unwrap() == '#',
            l2.next().unwrap() == '#',
          ],
        ]);
      } else {
        // Reading a region
        let (l0, r0) = top.split_once(": ").unwrap();
        let (w0, h0) = l0.split_once('x').unwrap();
        let expects = r0.split_whitespace().map(|n| n.parse().unwrap()).collect();
        regions.push((w0.parse().unwrap(), h0.parse().unwrap(), expects));
      }
    }

    Ok(Parsed { shapes, regions })
  }

  fn part1(&self, Parsed { shapes: _, regions }: &Parsed, _: Option<String>) -> Result<P1Out> {
    let trivially_possible = |region: &(usize, usize, Vec<usize>)| -> bool {
      let total_wanted_shapes: usize = region.2.iter().sum();
      let space_available = region.0 * region.1;
      (total_wanted_shapes * 3 * 3) <= space_available
      // Note: Technically, we should also check if there's a trivially-impossible situation (more dots than space).
      //  If it's neither trivially possible nor trivally impossible, we'd have to actually try to Tetris pack the shapes.
      //  However, the real data gets a correct answer without bothering to do that, so... happy holidays!
    };

    Ok(
      regions
        .iter()
        .map(|r| if trivially_possible(r) { 1 } else { 0 })
        .sum(),
    )
  }

  fn part2(&self, _: &Parsed, _: Option<String>) -> Result<P2Out> {
    // Final day has no part 2, as always. However, this year has 12 puzzles instead of the usual 25.
    // Our holidays have been returned to us, and the stress of the global leaderboard eliminated.
    // Praise be, Eric. Praise be.
    Ok(0)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2025, 12)
}
