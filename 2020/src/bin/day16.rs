use advent_lib::runner::{Day, PartId};
use anyhow::{Result, bail};
use fnv::{FnvBuildHasher, FnvHashMap};
use itertools::Itertools;

type P1Out = usize;
type P2Out = usize;

struct Parsed {
  ranges: FnvHashMap<String, Vec<(usize, usize)>>,
  your_ticket: Vec<usize>,
  nearby_tickets: Vec<Vec<usize>>,
}

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    let mut iter = lines.iter();

    let mut ranges = FnvHashMap::with_capacity_and_hasher(100, FnvBuildHasher::default());
    for line in iter.by_ref() {
      if line.is_empty() {
        // Done processing ranges, on to your_ticket
        break;
      }
      let (name_raw, r0) = line.split_once(": ").unwrap();
      let name = name_raw.to_owned();
      let (range_l, range_r) = r0.split_once(" or ").unwrap();
      let (ll_raw, lr_raw) = range_l.split_once('-').unwrap();
      let (rl_raw, rr_raw) = range_r.split_once('-').unwrap();
      let entry = ranges.entry(name).or_insert(vec![]);
      entry.push((ll_raw.parse().unwrap(), lr_raw.parse().unwrap()));
      entry.push((rl_raw.parse().unwrap(), rr_raw.parse().unwrap()));
    }

    let your_ticket = if let Some(line) = iter.next()
      && line.starts_with("your ticket:")
      && let Some(line) = iter.next()
    {
      line.split(',').map(|n| n.parse().unwrap()).collect()
    } else {
      bail!("Could not find your_ticket")
    };

    let mut nearby_tickets = vec![];
    for line in iter {
      if line.is_empty() || line.starts_with("nearby tickets:") {
        continue;
      }
      nearby_tickets.push(line.split(',').map(|n| n.parse().unwrap()).collect());
    }

    Ok(Parsed {
      ranges,
      your_ticket,
      nearby_tickets,
    })
  }

  fn part1(
    &self,
    Parsed {
      ranges,
      nearby_tickets,
      ..
    }: &Parsed,
    _: Option<String>,
  ) -> Result<P1Out> {
    let mut error_sum = 0;
    for ticket in nearby_tickets.iter() {
      for v in ticket.iter() {
        if ranges
          .iter()
          .all(|(_name, opts)| opts.iter().all(|&(lo, hi)| !(lo..=hi).contains(v)))
        {
          error_sum += v;
        }
      }
    }
    Ok(error_sum)
  }

  fn part2(
    &self,
    Parsed {
      ranges,
      your_ticket,
      nearby_tickets,
    }: &Parsed,
    _: Option<String>,
  ) -> Result<P2Out> {
    // First, filter out tickets that can't match (part 1), or else we'll fail to match at all.
    let valid_tickets: Vec<Vec<usize>> = nearby_tickets
      .iter()
      .chain([your_ticket])
      .filter(|t| {
        t.iter().all(|v| {
          ranges
            .iter()
            .any(|(_name, opts)| opts.iter().any(|&(lo, hi)| (lo..=hi).contains(v)))
        })
      })
      .cloned()
      .collect();

    // For each field, find each index at which every ticket has a value that fits the rules of that field
    let mut candidates: FnvHashMap<String, Vec<usize>> =
      FnvHashMap::with_capacity_and_hasher(20, FnvBuildHasher::default());
    for (name, opts) in ranges.iter() {
      let entry = candidates.entry(name.to_owned()).or_insert(vec![]);
      for i in 0..your_ticket.len() {
        let values: Vec<usize> = valid_tickets.iter().map(|t| t[i]).collect();

        if values
          .iter()
          .all(|v| opts.iter().any(|&(lo, hi)| (lo..=hi).contains(v)))
        {
          entry.push(i);
        }
      }
    }

    // Next, figure out which IS which, starting with those for which there's only one possibility.
    let mut assignments: FnvHashMap<String, usize> =
      FnvHashMap::with_capacity_and_hasher(20, FnvBuildHasher::default());
    let mut found: Vec<usize> = vec![];
    for (name, opts) in candidates.iter().sorted_by_key(|(_name, opts)| opts.len()) {
      let idx = *opts.iter().find(|&o| !found.contains(o)).unwrap();
      found.push(idx);
      assignments.insert(name.to_owned(), idx);
    }

    // Finally, the actual answer: knowing which values in your ticket go to the departure fields,
    // multiply those values together.
    let mut out = 1;
    for (name, &idx) in assignments.iter() {
      if name.starts_with("departure") {
        out *= your_ticket[idx];
      }
    }
    Ok(out)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2020, 16)
}
