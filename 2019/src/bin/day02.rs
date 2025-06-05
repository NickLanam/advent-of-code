use advent_lib::runner::{Day, PartId};
use anyhow::{Context, Result, bail};

type P1Out = usize;
type P2Out = usize;
type Parsed = Vec<usize>;

fn run(init: &[usize], reg1: usize, reg2: usize) -> Result<usize> {
  let mut program = init.to_owned();
  program[1] = reg1;
  program[2] = reg2;

  let mut pc = 0;
  loop {
    let &opcode = program
      .get(pc)
      .context("Program counter exited available memory")?;
    match opcode {
      1 => {
        let (&a, &b, &c) = (
          program
            .get(pc + 1)
            .context("opcode 1 unable to get param a")?,
          program
            .get(pc + 2)
            .context("opcode 1 unable to get param b")?,
          program
            .get(pc + 3)
            .context("opcode 1 unable to get param c")?,
        );
        if program.len() <= a.max(b).max(c) {
          // program.resize(a.max(b).max(c) + 1, 0);
          bail!(
            "Parameters reach out of bounds for opcode 1: pc={pc}, op={opcode}, a={a}, b={b}, c={c}, also program[0] is {} and program.len() is {}",
            program[0],
            program.len()
          );
        }
        program[c] = program[a] + program[b];
        pc += 4;
      }
      2 => {
        let (&a, &b, &c) = (
          program
            .get(pc + 1)
            .context("opcode 2 unable to get param a")?,
          program
            .get(pc + 2)
            .context("opcode 2 unable to get param b")?,
          program
            .get(pc + 3)
            .context("opcode 2 unable to get param c")?,
        );
        if program.len() <= a.max(b).max(c) {
          // program.resize(a.max(b).max(c) + 1, 0);
          bail!("Parameters reach out of bounds for opcode 2");
        }
        program[c] = program[a] * program[b];
        pc += 4;
      }
      99 => {
        return Ok(program[0]);
      }
      _ => {
        bail!("Unknown opcode: {opcode}");
      }
    }
  }
}

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    Ok(lines[0].split(',').map(|n| n.parse().unwrap()).collect())
  }

  fn part1(&self, init: &Parsed, sample_name: Option<String>) -> Result<P1Out> {
    if sample_name.is_some() {
      // The given sample is a different program entirely
      return Ok(0);
    }
    run(init, 12, 2)
  }

  fn part2(&self, init: &Parsed, sample_name: Option<String>) -> Result<P2Out> {
    if sample_name.is_some() {
      // The given sample is a different program entirely
      return Ok(0);
    }

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
