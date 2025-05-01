use std::cmp::Ordering;

use advent_lib::runner::{Day, PartId};
use anyhow::{Result, bail};
use fnv::{FnvBuildHasher, FnvHashMap};

#[derive(Debug, PartialEq, Eq)]
enum Action {
  Begin(u16),
  Wake,
  Sleep,
}

#[derive(Debug, PartialEq, Eq)]
struct Log {
  month: u8,
  day: u8,
  hour: u8,
  minute: u8,
  action: Action,
}
impl std::cmp::PartialOrd for Log {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}
impl std::cmp::Ord for Log {
  fn cmp(&self, other: &Self) -> Ordering {
    self
      .month
      .cmp(&other.month)
      .then(self.day.cmp(&other.day))
      .then(self.hour.cmp(&other.hour))
      .then(self.minute.cmp(&other.minute))
  }
}

/// ASSUMPTIONS (all of which held true for my input):
/// - The logs were sorted by time in `parse`.
/// - Every guard that falls asleep, wakes up before the next guard's shift
/// - The last guard wakes up before their shift ends too
/// - It's possible for a guard to not sleep during a shift (such guards aren't questioned)
/// - The year is always 1518
/// - No guard happens to sleep through midnight on a month boundary (that would be a pain)
/// - Some guards DO sleep through midnight on other days (in real input, but not in sample)
fn logs_to_schedule(logs: &[Log]) -> Result<FnvHashMap<u16, FnvHashMap<u8, usize>>> {
  // ID -> minute (across all hours) -> times_sleeping_at_that_minute
  let mut guards: FnvHashMap<u16, FnvHashMap<u8, usize>> =
    FnvHashMap::with_hasher(FnvBuildHasher::default());

  // From the (sorted) logs, build the above map
  let mut guard: Option<u16> = None;
  let mut sleeping_since: Option<(u8, u8, u8, u8)> = None;

  let mut wake = |g: u16, from: (u8, u8, u8, u8), to: (u8, u8, u8, u8)| {
    let (month, day, hour, minute) = to;

    // Mark all slept minutes as, well, slept
    let (since_month, mut since_day, mut since_hour, mut since_minute) = from;
    while since_month != month || since_day != day || since_hour != hour || since_minute != minute {
      *(guards
        .entry(g)
        .or_insert(FnvHashMap::with_hasher(FnvBuildHasher::default()))
        .entry(since_minute)
        .or_insert(0)) += 1;
      since_minute += 1;
      if since_minute == 60 {
        since_minute = 0;
        since_hour += 1;
        if since_hour == 24 {
          since_hour = 0;
          since_day += 1;
          // ASSUMPTION: No guard sleeps through midnight on the last day of a month.
          // If they did, the logic here would be more verbose, but the same idea.
        }
      }
    }
  };

  for Log {
    month,
    day,
    hour,
    minute,
    action,
  } in logs.iter()
  {
    match action {
      Action::Wake => {
        if let Some(g) = guard {
          if let Some(since) = sleeping_since {
            sleeping_since = None;
            wake(g, since, (*month, *day, *hour, *minute));
          } else {
            bail!("Wake action without sleeping first, guard known as {g}");
          }
        } else {
          bail!("Wake action without a guard, did you forget to sort the list?");
        }
      }
      Action::Sleep => {
        if let Some(g) = guard {
          if sleeping_since.is_some() {
            bail!("Guard {g} fell asleep, but they were already sleeping since {sleeping_since:?}");
          }
          sleeping_since = Some((*month, *day, *hour, *minute));
        } else {
          bail!("Sleep action without a guard, did you forget to sort the list?");
        }
      }
      Action::Begin(id) => {
        guard = Some(*id);
      }
    }
  }

  if let Some(g) = guard {
    if let Some(since) = sleeping_since {
      bail!("Guard {g} was still sleeping at the end, since {since:?}");
    }
  }
  Ok(guards)
}

type P1Out = usize;
type P2Out = usize;
type Parsed = Vec<Log>;

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    let mut out: Vec<Log> = Vec::with_capacity(lines.len());
    for line in lines {
      let month: u8 = line[6..=7].parse()?;
      let day: u8 = line[9..=10].parse()?;
      let hour: u8 = line[12..=13].parse()?;
      let minute: u8 = line[15..=16].parse()?;
      let action: Action = match line.chars().nth(19) {
        Some('f') => Action::Sleep,
        Some('w') => Action::Wake,
        Some('G') => {
          let n = line[26..].split_once(' ').unwrap().0.parse()?;
          Action::Begin(n)
        }
        _ => {
          bail!("What is this line? {line} -> {:?}", line.chars().nth(19));
        }
      };
      out.push(Log {
        month,
        day,
        hour,
        minute,
        action,
      });
    }
    out.sort();
    Ok(out)
  }

  fn part1(&self, logs: &Parsed, _sample_name: Option<String>) -> Result<P1Out> {
    let guards = logs_to_schedule(logs)?;

    // Which guard sleeps the most?
    // Multiply their ID by the minute that they sleep most often.
    let sleepiest_guard = guards
      .iter()
      .max_by_key(|(_id, times)| times.values().sum::<usize>())
      .unwrap();
    let sleepiest_minute = sleepiest_guard
      .1
      .iter()
      .max_by_key(|(_minute, count)| **count)
      .unwrap();
    Ok((*sleepiest_guard.0 as usize) * *sleepiest_minute.0 as usize)
  }

  fn part2(&self, logs: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    let guards = logs_to_schedule(logs)?;

    // There is a specific minute upon which one guard is asleep more often than any other,
    // and more often than any other minute that same guard sleeps on.
    // Find that guard and that minute, and multiply them together.

    // First, list each guard by their sleepiest minute and its frequency.
    let sleepiest_times: Vec<(u16, u8, usize)> = guards
      .iter()
      .map(|(g, times)| {
        let sleepiest_minute = times.iter().max_by_key(|(_minute, count)| **count).unwrap();
        (*g, *sleepiest_minute.0, *sleepiest_minute.1)
      })
      .collect();

    // Now it's easy to answer the question.
    let sleepiest_guard_minute = sleepiest_times.iter().max_by_key(|(_g, _m, c)| *c).unwrap();
    Ok((sleepiest_guard_minute.0 as usize) * (sleepiest_guard_minute.1 as usize))
  }
}

fn main() -> Result<()> {
  Solver {}.run(2018, 4)
}
