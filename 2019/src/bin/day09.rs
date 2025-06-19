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
    let inputs = [1];
    let res = execute(init, &inputs, None)?;
    Ok(res.outputs[0])
  }

  fn part2(&self, init: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    let inputs = [2];
    let res = execute(init, &inputs, None)?;
    Ok(res.outputs[0])
  }
}

fn main() -> Result<()> {
  Solver {}.run(2019, 9)
}
