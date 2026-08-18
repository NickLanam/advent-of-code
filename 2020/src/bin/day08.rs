use advent_lib::runner::{Day, PartId};
use anyhow::{Result, bail};

type P1Out = i64;
type P2Out = i64;

#[derive(Clone, Copy, Debug)]
enum Instruction {
  Nop(i64),
  Acc(i64),
  Jmp(i64),
}
type Parsed = Vec<Instruction>;

fn run(instructions: &Parsed) -> Result<(bool, i64)> {
  let mut pc: usize = 0;
  let mut acc: i64 = 0;
  let mut seen = vec![false; instructions.len()];
  loop {
    if pc > instructions.len() {
      bail!("Instruction went out of bounds!");
    } else if pc == instructions.len() {
      return Ok((true, acc));
    } else if seen[pc] {
      return Ok((false, acc));
    }

    seen[pc] = true;
    match instructions[pc] {
      Instruction::Nop(_) => {
        pc += 1;
      }
      Instruction::Acc(n) => {
        acc += n;
        pc += 1;
      }
      Instruction::Jmp(n) => {
        if n < 0 {
          pc -= n.unsigned_abs() as usize;
        } else {
          pc += n as usize;
        }
      }
    }
  }
}

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(
      lines
        .iter()
        .map(|line| {
          let (op_raw, n_raw) = line.split_at(5);
          match op_raw {
            "nop +" => Instruction::Nop(n_raw.parse().unwrap()),
            "nop -" => Instruction::Nop(-n_raw.parse::<i64>().unwrap()),
            "acc +" => Instruction::Acc(n_raw.parse().unwrap()),
            "acc -" => Instruction::Acc(-n_raw.parse::<i64>().unwrap()),
            "jmp +" => Instruction::Jmp(n_raw.parse().unwrap()),
            "jmp -" => Instruction::Jmp(-n_raw.parse::<i64>().unwrap()),
            _ => panic!("Not a valid instruction"),
          }
        })
        .collect(),
    )
  }

  fn part1(&self, instructions: &Parsed, _: Option<String>) -> Result<P1Out> {
    Ok(run(instructions)?.1)
  }

  fn part2(&self, instructions: &Parsed, _: Option<String>) -> Result<P2Out> {
    for (i, inst) in instructions.iter().enumerate() {
      match inst {
        Instruction::Nop(n) => {
          let mut next = instructions.clone();
          next[i] = Instruction::Jmp(*n);
          let (halts, acc) = run(&next)?;
          if halts {
            return Ok(acc);
          }
        }
        Instruction::Jmp(n) => {
          let mut next = instructions.clone();
          next[i] = Instruction::Nop(*n);
          let (halts, acc) = run(&next)?;
          if halts {
            return Ok(acc);
          }
        }
        _ => { /* Ignore */ }
      }
    }
    bail!("None of the possible subtitutions halted")
  }
}

fn main() -> Result<()> {
  Solver {}.run(2020, 8)
}
