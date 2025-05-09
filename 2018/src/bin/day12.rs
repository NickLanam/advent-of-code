use std::collections::VecDeque;

use advent_lib::runner::{Day, PartId};
use anyhow::Result;
use fnv::FnvHashSet;

type P1Out = i64;
type P2Out = i64;
type Parsed = (Vec<bool>, Vec<([bool; 5], bool)>);

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    let initial_state: Vec<bool> = lines[0]
      .split_once(": ")
      .unwrap()
      .1
      .chars()
      .map(|c| c == '#')
      .collect();
    let mut rules: Vec<([bool; 5], bool)> = Vec::with_capacity(lines.len() - 2);
    for line in lines[2..].iter() {
      let mut chars = line.chars();
      let pattern = [
        chars.next().unwrap() == '#',
        chars.next().unwrap() == '#',
        chars.next().unwrap() == '#',
        chars.next().unwrap() == '#',
        chars.next().unwrap() == '#',
      ];
      let result = chars.last().unwrap() == '#';
      rules.push((pattern, result));
    }
    Ok((initial_state, rules))
  }

  fn part1(&self, (initial_state, rules): &Parsed, _: Option<String>) -> Result<P1Out> {
    let generations = 20;
    let mut state = FnvHashSet::<i64>::default();
    let mut min_x: i64 = 0;
    let mut max_x: i64 = initial_state.len() as i64;
    for (i, v) in initial_state.iter().enumerate() {
      if *v {
        state.insert(i as i64);
      }
    }

    for _ in 0..generations {
      let mut next_state = FnvHashSet::<i64>::default();
      for i in (min_x - 3)..(max_x + 3) {
        let test = [
          state.contains(&(i - 2)),
          state.contains(&(i - 1)),
          state.contains(&(i)),
          state.contains(&(i + 1)),
          state.contains(&(i + 2)),
        ];
        for (pattern, result) in rules {
          if pattern == &test {
            if *result {
              next_state.insert(i);
              min_x = min_x.min(i);
              max_x = max_x.max(i);
            }
            break;
          }
        }
      }
      state = next_state;
    }

    Ok(state.iter().sum())
  }

  fn part2(&self, (initial_state, rules): &Parsed, sample_name: Option<String>) -> Result<P2Out> {
    if sample_name.is_some() {
      return Ok(1);
    }
    let generations: usize = 50_000_000_000;
    let mut state = FnvHashSet::<i64>::default();
    let mut min_x: i64 = 0;
    let mut max_x: i64 = initial_state.len() as i64;
    for (i, v) in initial_state.iter().enumerate() {
      if *v {
        state.insert(i as i64);
      }
    }

    // NOTE: If you get a wrong answer, try increasing this number. I originally hardcoded
    // a tuple checking the last two diffs, and it took a while to realize I needed to
    // wait longer for stability.
    const STABLE_LEN: usize = 4;
    let mut score_history: VecDeque<i64> = VecDeque::from([state.iter().sum::<i64>()]);
    'generate: for g in 0..generations {
      let mut next_state = FnvHashSet::<i64>::default();
      for i in (min_x - 3)..(max_x + 3) {
        let test = [
          state.contains(&(i - 2)),
          state.contains(&(i - 1)),
          state.contains(&(i)),
          state.contains(&(i + 1)),
          state.contains(&(i + 2)),
        ];
        for (pattern, result) in rules {
          if pattern == &test {
            if *result {
              next_state.insert(i);
              min_x = min_x.min(i);
              max_x = max_x.max(i);
            }
            break;
          }
        }
      }
      state = next_state;

      // The optimization: after less than 200 generations, the score starts
      // incrementing by a fixed amount each cycle. Detect when that happens,
      // and skip to the end immediately instead of simulating 50bil steps.
      while score_history.len() >= STABLE_LEN {
        score_history.pop_front();
      }
      score_history.push_back(state.iter().sum::<i64>());
      if score_history.len() == STABLE_LEN {
        let diff = score_history[0].abs_diff(score_history[1]);
        for (i, v) in score_history.iter().enumerate().skip(1) {
          if v.abs_diff(score_history[i - 1]) != diff {
            continue 'generate;
          }
        }
        let mut s = score_history.pop_back().unwrap();
        let cycles_left = (generations - g) as i64;
        s += (cycles_left - 1) * diff as i64;
        return Ok(s);
      }
    }

    Ok(state.iter().sum())
  }
}

fn main() -> Result<()> {
  Solver {}.run(2018, 12)
}
