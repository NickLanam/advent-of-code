use advent_lib::runner::{Day, PartId};
use advent_of_code_2019::intcode::{Execution, execute};
use anyhow::Result;
use itertools::Itertools;

type P1Out = i64;
type P2Out = i64;
type Parsed = Vec<i64>;

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(lines[0].split(',').map(|n| n.parse().unwrap()).collect())
  }

  fn part1(&self, init: &Parsed, _: Option<String>) -> Result<P1Out> {
    let mut best = i64::MIN;
    for permutation in (0_i64..=4).permutations(5) {
      let mut last_out = 0;
      for id in permutation {
        let input = [id, last_out];
        let res = execute(init, &input, None)?;
        last_out = res.outputs[0];
      }
      best = best.max(last_out);
    }
    Ok(best)
  }

  fn part2(&self, init: &Parsed, _: Option<String>) -> Result<P2Out> {
    let mut best = i64::MIN;
    for permutation in (5_i64..=9).permutations(5) {
      // First, get the runners started
      let mut runners: Vec<Execution> = vec![];
      for id in permutation {
        let input = [id];
        let res = execute(init, &input, Some(0))?;
        runners.push(res);
      }

      // Now, run them in their feedback loop until the last one halts.
      let mut last_out = 0;
      while !runners.last().unwrap().halted {
        for runner in runners.iter_mut() {
          let input = [last_out];
          *runner = execute(&runner.final_tape, &input, Some(runner.pc))?;
          last_out = runner.outputs[0];
        }
      }
      best = best.max(last_out);
    }
    Ok(best)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2019, 7)
}
