use advent_lib::runner::{Day, PartId};
use advent_of_code_2019::intcode::execute;
use anyhow::Result;

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
    for a_id in [0_i64, 1, 2, 3, 4] {
      let a_in = [a_id, 0];
      let a_res = execute(init, &a_in)?;
      for b_id in [0_i64, 1, 2, 3, 4] {
        if b_id == a_id {
          continue;
        }
        let b_in = [b_id, a_res.outputs[0]];
        let b_res = execute(init, &b_in)?;
        for c_id in [0_i64, 1, 2, 3, 4] {
          if c_id == b_id || c_id == a_id {
            continue;
          }
          let c_in = [c_id, b_res.outputs[0]];
          let c_res = execute(init, &c_in)?;
          for d_id in [0_i64, 1, 2, 3, 4] {
            if d_id == c_id || d_id == b_id || d_id == a_id {
              continue;
            }
            let d_in = [d_id, c_res.outputs[0]];
            let d_res = execute(init, &d_in)?;
            for e_id in [0_i64, 1, 2, 3, 4] {
              if e_id == d_id || e_id == c_id || e_id == b_id || e_id == a_id {
                continue;
              }
              let e_in = [e_id, d_res.outputs[0]];
              let e_res = execute(init, &e_in)?;
              best = best.max(e_res.outputs[0]);
            }
          }
        }
      }
    }
    Ok(best)
  }

  fn part2(&self, _init: &Parsed, _: Option<String>) -> Result<P2Out> {
    Ok(0)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2019, 7)
}
