use advent_lib::runner::{Day, PartId};
use anyhow::{Result, bail};

type P1Out = u16;
type P2Out = u16;
type Parsed = Vec<u16>;

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(
      lines[0]
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect(),
    )
  }

  fn part1(&self, input: &Parsed, _: Option<String>) -> Result<P1Out> {
    let mut stream = input.iter();
    let mut metadata_sum = 0;
    // children_remaining, count_metadata
    let mut stack: Vec<(u16, u16)> = vec![];
    loop {
      let top = stack.last_mut();
      if let Some((children_remaining, count_metadata)) = top {
        if *children_remaining == 0 {
          // Pop metadata then remove this item from the stack entirely
          for _ in 0..*count_metadata {
            metadata_sum += *stream.next().unwrap();
          }
          stack.pop();
        } else {
          *children_remaining -= 1;
          let c = stream.next().unwrap();
          let m = stream.next().unwrap();
          stack.push((*c, *m));
        }
      } else {
        // Stack empty, try popping from stream to get the next one
        // If stream is finished, we're already done.
        if let Some(c) = stream.next() {
          if let Some(m) = stream.next() {
            stack.push((*c, *m));
          } else {
            bail!("Imbalanced?");
          }
        } else {
          break;
        }
      }
    }
    Ok(metadata_sum)
  }

  fn part2(&self, input: &Parsed, _: Option<String>) -> Result<P2Out> {
    // Similar idea to part 1, but keeping track of scores along the way.
    // Takes about 3.5x as long to run as part 1 does. Not as bad as expected!
    let mut stream = input.iter();
    // children_remaining, count_metadata, child_scores
    let mut stack: Vec<(u16, u16, Vec<u16>)> = vec![];
    loop {
      let top = stack.last_mut();
      if let Some((children_remaining, count_metadata, child_scores)) = top {
        if *children_remaining == 0 {
          // Pop metadata then remove this item from the stack entirely
          let metadata: Vec<u16> = (0..*count_metadata)
            .map(|_| *stream.next().unwrap())
            .collect();

          // Rules of the puzzle decree scores are computed exactly like this
          let score = if child_scores.is_empty() {
            // Score is the metadata sum
            metadata.iter().sum()
          } else {
            // Score is built from the scores of child entries
            let mut sum = 0;
            for m in metadata.iter() {
              if let Some(s) = child_scores.get((*m as usize) - 1) {
                sum += *s;
              }
            }
            sum
          };

          // If that was the last node, its score is the final result.
          // Otherwise, bubble its score up to its parent.
          // Either way, remove this item since we're done with it.
          stack.pop();
          if stack.is_empty() {
            return Ok(score);
          } else {
            stack.last_mut().unwrap().2.push(score);
          }
        } else {
          // children_remaining is at least 1, so dig deeper
          *children_remaining -= 1;
          let c = stream.next().unwrap();
          let m = stream.next().unwrap();
          stack.push((*c, *m, Vec::with_capacity(*c as usize)));
        }
      } else {
        // Stack empty, try popping from stream to get the next one
        // If stream is finished, we're already done.
        if let Some(c) = stream.next() {
          if let Some(m) = stream.next() {
            stack.push((*c, *m, Vec::with_capacity(*c as usize)));
          } else {
            bail!("Imbalanced?");
          }
        } else {
          bail!("Ran out of elements. We should have returned the final score by now!");
        }
      }
    }
  }
}

fn main() -> Result<()> {
  Solver {}.run(2018, 8)
}
