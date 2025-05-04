use advent_lib::runner::{Day, PartId};
use anyhow::{Result, bail};
use fnv::{FnvBuildHasher, FnvHashMap};
use itertools::Itertools;

type P1Out = String;
type P2Out = usize;
type Parsed = FnvHashMap<char, Vec<char>>;

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    let mut map: Parsed =
      FnvHashMap::with_capacity_and_hasher(lines.len(), FnvBuildHasher::default());
    for line in lines {
      let req = line.chars().nth(5).unwrap();
      let res = line.chars().nth(36).unwrap();
      if !req.is_ascii_uppercase() || !res.is_ascii_uppercase() {
        bail!("Input is malformed!");
      }
      map.entry(res).or_default().push(req);
      // Also insert an empty entry for the req, in case that one has no dependencies
      map.entry(req).or_default();
    }
    Ok(map)
  }

  fn part1(&self, in_map: &Parsed, _: Option<String>) -> Result<P1Out> {
    let mut map = in_map.clone();
    let mut out: String = String::with_capacity(map.len());
    while !map.is_empty() {
      let next = map
        .iter()
        .filter(|(_k, v)| v.is_empty())
        .map(|(&k, _)| k)
        .sorted()
        .next();
      if let Some(next) = next {
        map.remove(&next);
        out.push(next);
        for (_, deps) in map.iter_mut() {
          deps.retain(|&dep| dep != next);
        }
      } else {
        bail!("Map is not empty, but no chars were available:\n{map:#?}");
      }
    }
    Ok(out)
  }

  fn part2(&self, in_map: &Parsed, sample_name: Option<String>) -> Result<P2Out> {
    let base_delay = if sample_name.is_some() { 0 } else { 60 };
    let num_workers = if sample_name.is_some() { 2 } else { 5 };
    let delay_of = |c: char| -> usize {
      let mut buf = [0_u8];
      c.encode_utf8(&mut buf);
      // 'A' gets a delay of 1, and 'A' is ASCII 65.
      base_delay + (buf[0] as usize - 64)
    };

    let mut incomplete = in_map.clone();
    let mut workers: Vec<Option<(char, usize)>> = vec![None; num_workers];
    let mut tick = 0;
    loop {
      // Jump time ahead to complete whichever task is closest to done,
      // then complete that task and any others that were ready at the same time.
      // If no tasks are active (such as at the start), this step is skipped.
      if let Some(&(_, jump_by)) = workers.iter().flatten().min_by_key(|(_, r)| r) {
        tick += jump_by;
        for worker in workers.iter_mut() {
          if let Some((task, remain)) = worker {
            if *remain == 0 {
              bail!("Tried to tick down a task that was already at 0. Did it not get removed?");
            } else if *remain < jump_by {
              bail!("Tried to jump a task past its end point, did the min_by_key go wrong?");
            } else if *remain == jump_by {
              // Task complete. Remove the task from dependency lists,
              // then release the worker.
              incomplete.remove(task);
              for (_, deps) in incomplete.iter_mut() {
                deps.retain(|&dep| dep != *task);
              }
              *worker = None;
            } else {
              *remain -= jump_by;
            }
          }
        }
      }

      // If map is now empty, finish remaining tasks and complete
      if incomplete.is_empty() {
        if let Some((_, remain)) = workers.iter().flatten().max_by_key(|(_, r)| r) {
          tick += remain;
        }
        break;
      }

      // Idle workers pick up available chars and start their timers
      let mut available_tasks: Vec<char> = incomplete
        .iter()
        .filter(|(_k, v)| v.is_empty())
        .map(|(&k, _)| k)
        .sorted()
        .rev()
        .collect();
      for worker in workers.iter_mut() {
        if worker.is_some() {
          continue;
        }
        if let Some(task) = available_tasks.pop() {
          *worker = Some((task, delay_of(task)));
          // Remove the task from the map so it doesn't get picked up again
          incomplete.remove(&task);
        }
      }
    }

    // Complete!
    Ok(tick)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2018, 7)
}
