use advent_lib::runner::{Day, PartId};
use anyhow::Result;
use microlp::{ComparisonOp, OptimizationDirection, Problem, Variable};

#[derive(Debug, Clone)]
struct Line {
  lights: Vec<bool>,
  buttons: Vec<Vec<usize>>,
  joltage: Vec<usize>,
}

type P1Out = usize;
type P2Out = usize;
type Parsed = Vec<Line>;

/// Try every combination, but not every permutation, of button presses, and recursively determine how many it takes to solve
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

/// Brute force won't work here, we need an lp solver (or to do manual Gaussian elimination, but I don't want to write that)
fn linear_solver_part_2(line: &Line) -> Result<usize> {
  let mut problem = Problem::new(OptimizationDirection::Minimize);

  let vars: Vec<Variable> = line
    .buttons
    .iter()
    .map(|_| problem.add_integer_var(1.0, (0, i32::MAX)))
    .collect();

  for (j, &joltage) in line.joltage.iter().enumerate() {
    let mut rels: Vec<(Variable, f64)> = vec![];
    for (b, button) in line.buttons.iter().enumerate() {
      if button.contains(&j) {
        rels.push((vars[b], 1.0));
      }
    }
    problem.add_constraint(rels.as_slice(), ComparisonOp::Eq, joltage as f64);
  }

  let solution = problem.solve()?;

  // Note: the rounding here is important, as we keep being one or two f64::EPSILON out from the integers (+/-).
  // Rounding here makes sure we reach the integer that was intended.
  Ok(solution.objective().round() as usize)
}

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(
      lines
        .iter()
        .map(|line| {
          let mut lights: Vec<bool> = vec![];
          let mut buttons: Vec<Vec<usize>> = vec![];
          let mut joltage: Vec<usize> = vec![];
          for chunk in line.split_whitespace() {
            if chunk.starts_with('[') {
              lights = chunk[1..(chunk.len() - 1)]
                .chars()
                .map(|c| c == '#')
                .collect();
            } else if chunk.starts_with('(') {
              buttons.push(
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
            buttons,
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
          brute_force_part_1(
            &line.lights,
            line.buttons.clone(),
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
        .map(|line| linear_solver_part_2(line).unwrap())
        .sum(),
    )
  }
}

fn main() -> Result<()> {
  Solver {}.run(2025, 10)
}
