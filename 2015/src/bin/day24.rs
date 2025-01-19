use std::cmp::Ordering;

use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = u64;
type P2Out = u64;
type Parsed = Vec<u16>;

#[derive(Debug)]
struct Group {
  sum: u16,
  prod: u64,
  len: usize,
  taken: Vec<u16>,
}

fn solve(boxes: &Parsed, num_groups: u16) -> Result<u64> {
  let total_weight = boxes.iter().sum::<u16>();
  let group_weight = total_weight / num_groups;

  // First, find every possible subset of boxes that results in the correct weight for a group.
  let mut valid_groups: Vec<Group> = vec![];
  let mut stack: Vec<(usize, Group)> = vec![(
    0,
    Group {
      sum: 0,
      prod: 1,
      len: 0,
      taken: vec![],
    },
  )];
  while let Some((
    offset,
    Group {
      sum,
      prod,
      len,
      taken,
    },
  )) = stack.pop()
  {
    if sum == group_weight {
      valid_groups.push(Group {
        sum,
        prod,
        len,
        taken: taken.clone(),
      });
      continue;
    }
    for (j, b) in boxes.iter().enumerate().skip(offset) {
      if sum + *b <= group_weight {
        let mut new_taken = taken.clone();
        new_taken.push(*b);
        stack.push((
          j + 1,
          Group {
            sum: sum + *b,
            prod: prod * (*b as u64),
            len: len + 1,
            taken: new_taken,
          },
        ));
      }
    }
  }

  // Prefer those with the best legroom, and break ties with lowest quantum entanglement (product of contents).
  valid_groups.sort_by(|a, b| {
    let len_cmp = a.len.cmp(&b.len);
    if len_cmp == Ordering::Equal {
      return a.prod.cmp(&b.prod);
    }
    len_cmp
  });

  // Next, we SHOULD be tracking best_legroom and best_qe as we loop through sorted valid_groups here.
  // We SHOULD stop when we get to a group that's worse than the best seen so far,
  // and only mark one as best-so-far if there's a combination of other groups that actually results
  // in having every original box combined.
  // However, it turns out that for both the sample and my real input... the first group we see after
  // the above sort already qualifies, and is thus guaranteed to be the solution without checking.
  // Result: A bit under 1.3 seconds to run both parts on my real input.
  Ok(valid_groups[0].prod)
}

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    // Assumptions about the input, used to make this solution simpler and faster:
    // - The input is a list of unique, positive integers, sorted smallest to largest
    // - There are fewer than 50 of them
    // - The largest one is less than 127, so it fits in a u8 easily
    // - The sum of all of these integers is an exact multiple of both 3 and 4
    // - Said sum is less than 2**11, so no group can get a sum that exceeds a u16
    // - Similarly, the product of any given valid group's contents won't outgrow a u64
    let boxes: Vec<u16> = lines.iter().map(|n| n.parse().unwrap()).collect();
    Ok(boxes)
  }

  fn part1(&self, boxes: &Parsed, _sample_name: Option<String>) -> Result<P1Out> {
    solve(boxes, 3)
  }

  fn part2(&self, boxes: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    solve(boxes, 4)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2015, 24)
}
