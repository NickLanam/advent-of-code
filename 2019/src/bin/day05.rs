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
    let out = execute(init, &[1])?;
    Ok(*out.outputs.last().unwrap())
  }

  fn part2(&self, init: &Parsed, _: Option<String>) -> Result<P2Out> {
    // TODO: Part 1 is working, but part 2 is reaching pc=0 with reg[0] = 294 so something about the four new instructions isn't quite right
    //  Thankfully, day2 still works properly as well.
    let out = execute(init, &[5])?;
    Ok(*out.outputs.last().unwrap())
  }
}

fn main() -> Result<()> {
  Solver {}.run(2019, 5)
}
