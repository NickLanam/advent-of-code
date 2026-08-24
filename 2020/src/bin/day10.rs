use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = usize;
type P2Out = usize;
type Parsed = Vec<u8>;

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    let mut sorted: Vec<u8> = [0_u8]
      .iter()
      .cloned()
      .chain(lines.iter().map(|n| n.parse().unwrap()))
      .collect();
    sorted.sort();
    sorted.push(sorted[sorted.len() - 1] + 3);
    Ok(sorted)
  }

  fn part1(&self, sorted: &Parsed, _: Option<String>) -> Result<P1Out> {
    let mut diffs = [0_u8; 3];
    let mut prev = 0;
    for &next in sorted.iter() {
      let diff = (next - prev) as usize;
      if (1..=3).contains(&diff) {
        diffs[diff - 1] += 1;
      }
      prev = next;
    }
    Ok((diffs[0] as usize) * (diffs[2] as usize))
  }

  fn part2(&self, sorted: &Parsed, _: Option<String>) -> Result<P2Out> {
    // We'll be iterating backwards over the sorted values to store sub-problem solutions,
    // and then using those to compose the solution at the initial node.
    let mut known: Vec<usize> = vec![0; sorted.len()];

    for start_index in (0..sorted.len()).rev() {
      let start = sorted[start_index];
      let mut sum = 0;
      for (i, &v) in sorted.iter().enumerate().skip(start_index) {
        match v - start {
          0 => {}
          1..=3 => sum += known[i] + 1,
          _ => {
            // The list was sorted, so once we see this then there will be no more matches
            break;
          }
        }
      }
      known[start_index] = sum.saturating_sub(1);
    }

    // There's always one initial path coming in; add that.
    Ok(known[0] + 1)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2020, 10)
}
