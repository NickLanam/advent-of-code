use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = usize;
type P2Out = usize;
type Parsed = (usize, Vec<usize>);

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    let start = lines[0].parse().unwrap();
    let times = lines[1]
      .split(',')
      .map(|n| {
        if n.starts_with('x') {
          0
        } else {
          n.parse().unwrap()
        }
      })
      .collect();
    Ok((start, times))
  }

  fn part1(&self, (start, times): &Parsed, _sample_name: Option<String>) -> Result<P1Out> {
    let ids: Vec<usize> = times.iter().filter(|n| **n > 0).cloned().collect();
    let mut shortest_wait = usize::MAX;
    let mut soonest_id = 0;
    for &id in ids.iter() {
      let wait = id - (*start % id);
      if wait < shortest_wait {
        shortest_wait = wait;
        soonest_id = id;
      }
    }
    Ok(shortest_wait * soonest_id)
  }

  fn part2(&self, (_start, ids): &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    // Pretty straightforward: we're finding common mulitples, with a twist.
    // Part 1 takes 250ns on my hardware; part 2 takes 3µs, so it's plenty fast.
    let mut r = 1;
    let mut earliest = 0;
    for (i, &s) in ids.iter().enumerate() {
      if s == 0 {
        continue;
      }
      while (earliest + i) % s != 0 {
        earliest += r;
      }
      r *= s;
    }
    Ok(earliest)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2020, 13)
}
