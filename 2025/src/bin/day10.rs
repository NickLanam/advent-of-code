use advent_lib::runner::{Day, PartId};
use anyhow::Result;

#[derive(Debug, Clone)]
struct Line {
  lights: Vec<bool>,
  wirings: Vec<Vec<usize>>,
  joltage: Vec<usize>,
}

type P1Out = usize;
type P2Out = usize;
type Parsed = Vec<Line>;

fn brute_force_part_1(goal: &Vec<bool>, buttons: Vec<Vec<usize>>, state: Vec<bool>) -> usize {
  if state.iter().eq(goal.iter()) {
    0
  } else if buttons.is_empty() {
    usize::MAX
  } else {
    let mut best = usize::MAX;
    for (i, button) in buttons.iter().enumerate() {
      let next_buttons: Vec<Vec<usize>> = buttons[(i + 1)..].to_vec();
      let mut next_state = state.clone();
      for &j in button {
        next_state[j] = !next_state[j];
      }
      best = best.min(1_usize.saturating_add(brute_force_part_1(goal, next_buttons, next_state)));
    }
    best
  }
}

fn brute_force_part_2(goal: &Vec<usize>, buttons: Vec<Vec<usize>>, state: Vec<usize>) -> usize {
  if state.iter().eq(goal.iter()) {
    0
  } else if buttons.is_empty() {
    usize::MAX
  } else {
    let mut best = usize::MAX;
    for (i, button) in buttons.iter().enumerate() {
      // TODO: Nope, in part 2 a button can be pressed more than once, so we actually do have to do the linear algebra solution
      // which would also work for part 1, to solve this efficiently.
      let next_buttons: Vec<Vec<usize>> = buttons[(i + 1)..].to_vec();
      let mut next_state = state.clone();
      for &j in button {
        next_state[j] += 1;
      }
      best = best.min(1_usize.saturating_add(brute_force_part_2(goal, next_buttons, next_state)));
    }
    best
  }
}

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(
      lines
        .iter()
        .map(|line| {
          let mut lights: Vec<bool> = vec![];
          let mut wirings: Vec<Vec<usize>> = vec![];
          let mut joltage: Vec<usize> = vec![];
          for chunk in line.split_whitespace() {
            if chunk.starts_with('[') {
              lights = chunk[1..(chunk.len() - 1)]
                .chars()
                .map(|c| c == '#')
                .collect();
            } else if chunk.starts_with('(') {
              wirings.push(
                chunk[1..(chunk.len() - 1)]
                  .split(',')
                  .map(|n| n.parse().unwrap())
                  .collect(),
              );
            } else {
              joltage = chunk[1..(chunk.len() - 1)]
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect();
            }
          }
          Line {
            lights,
            wirings,
            joltage,
          }
        })
        .collect(),
    )
  }

  fn part1(&self, lines: &Parsed, _: Option<String>) -> Result<P1Out> {
    Ok(
      lines
        .iter()
        .map(|line| {
          // TODO: This is a modified version of Lights Out, see if a generic matrix solver for that works here. It should.
          brute_force_part_1(
            &line.lights,
            line.wirings.clone(),
            (0..line.lights.len()).map(|_| false).collect(),
          )
        })
        .sum(),
    )
  }

  fn part2(&self, lines: &Parsed, _: Option<String>) -> Result<P2Out> {
    Ok(
      lines
        .iter()
        .map(|line| {
          brute_force_part_2(
            &line.joltage,
            line.wirings.clone(),
            (0..line.joltage.len()).map(|_| 0).collect(),
          )
        })
        .sum(),
    )
  }
}

fn main() -> Result<()> {
  Solver {}.run(2025, 10)
}
