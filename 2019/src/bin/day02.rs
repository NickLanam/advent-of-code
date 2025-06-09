use advent_lib::runner::{Day, PartId};
use advent_of_code_2019::intcode::execute;
use anyhow::Result;

type P1Out = i64;
type P2Out = i64;
type Parsed = Vec<i64>;

fn run(init: &[i64], reg1: i64, reg2: i64) -> Result<i64> {
  let mut program = init.to_owned();
  program[1] = reg1;
  program[2] = reg2;

  let out = execute(&program, &[])?;
  Ok(out.final_tape[0])
}

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(lines[0].split(',').map(|n| n.parse().unwrap()).collect())
  }

  fn part1(&self, init: &Parsed, _: Option<String>) -> Result<P1Out> {
    run(init, 12, 2)
  }

  fn part2(&self, init: &Parsed, _: Option<String>) -> Result<P2Out> {
    // The program does a roundabout way of computing `a * factor + constant + b`.
    // We can run the program with a=0,b=0 to learn `constant`, then a=1,b=0 to learn `factor`.
    // Then, we can solve `a * factor + constant + b = 19_690_720` for `a` and `b`.
    let constant = run(init, 0, 0)?;
    let factor = run(init, 1, 0)? - constant;
    let target = 19_690_720 - constant;

    // a*x + b = target; we know x; we know a and b are integers... so this is easy!
    let a = target / factor;
    let b = target % factor;
    Ok(a * 100 + b)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2019, 2)
}
