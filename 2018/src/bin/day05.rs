use advent_lib::runner::{Day, PartId};
use anyhow::Result;

fn reduce_and_score<'a, T: IntoIterator<Item = &'a u8>>(source: T) -> usize {
  // Using a stack to do the reduction avoids expensive removals from
  // a Vec or the complexities of an intrusive linked list cursor.
  // Working off an iterator directly also avoids a second copy of the full string.
  let mut stack: Vec<u8> = vec![];

  for &v in source.into_iter() {
    if let Some(&last) = stack.last() {
      // 'A' is 65, 'a' is 97.
      // Since the input is ASCII, using u8 is nice and cheap here.
      if last.abs_diff(v) == 32 {
        stack.pop();
        continue;
      }
    }
    stack.push(v);
  }
  stack.len()
}

type P1Out = usize;
type P2Out = usize;
type Parsed = Vec<u8>;

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(
      lines[0]
        .chars()
        .map(|c| {
          // The input is ASCII; mapping char to u8 allows
          // simpler (and faster) comparisons in the reducer
          let mut dst = [0_u8];
          c.encode_utf8(&mut dst);
          dst[0]
        })
        .collect(),
    )
  }

  fn part1(&self, line: &Parsed, _: Option<String>) -> Result<P1Out> {
    Ok(reduce_and_score(line.iter()))
  }

  fn part2(&self, line: &Parsed, _: Option<String>) -> Result<P2Out> {
    let mut best = usize::MAX;
    // 'A' through 'Z'
    for v in 65..=90 {
      let next = reduce_and_score(line.iter().filter(|&&v2| v2 != v && v2 - 32 != v));
      if next < best {
        best = next;
      }
    }
    Ok(best)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2018, 5)
}
