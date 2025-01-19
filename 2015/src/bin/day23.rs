use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = u64;
type P2Out = u64;

#[derive(Debug, PartialEq)]
enum Operation {
  Half,
  Triple,
  Increment,
  Jump,
  JumpIfEven,
  JumpIfOne,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Register {
  A,
  B,
}

struct RunResult {
  pc: usize,
  a: u64,
  b: u64,
  halt: bool,
}

#[derive(Debug)]
struct Instruction {
  op: Operation,
  reg: Option<Register>,
  offset: Option<i64>,
}

#[derive(Debug)]
struct State<'a> {
  pc: usize,
  instructions: &'a Vec<Instruction>,
  a: u64,
  b: u64,
  halted: bool,
}

fn step(state: &State) -> RunResult {
  if state.pc >= state.instructions.len() {
    return RunResult {
      pc: state.pc,
      a: state.a,
      b: state.b,
      halt: true,
    };
  }

  let inst: &Instruction = &state.instructions[state.pc];
  match inst.op {
    Operation::Half => RunResult {
      pc: state.pc + 1,
      a: match inst.reg {
        Some(Register::A) => state.a / 2,
        _ => state.a,
      },
      b: match inst.reg {
        Some(Register::B) => state.b / 2,
        _ => state.b,
      },
      halt: false,
    },
    Operation::Triple => RunResult {
      pc: state.pc + 1,
      a: match inst.reg {
        Some(Register::A) => state.a * 3,
        _ => state.a,
      },
      b: match inst.reg {
        Some(Register::B) => state.b * 3,
        _ => state.b,
      },
      halt: false,
    },
    Operation::Increment => RunResult {
      pc: state.pc + 1,
      a: match inst.reg {
        Some(Register::A) => state.a + 1,
        _ => state.a,
      },
      b: match inst.reg {
        Some(Register::B) => state.b + 1,
        _ => state.b,
      },
      halt: false,
    },
    Operation::Jump => RunResult {
      pc: ((state.pc as i64) + inst.offset.unwrap()) as usize,
      a: state.a,
      b: state.b,
      halt: false,
    },
    Operation::JumpIfEven => {
      let v = if inst.reg == Some(Register::A) {
        state.a
      } else {
        state.b
      };
      let jump = v % 2 == 0;
      RunResult {
        pc: if jump {
          ((state.pc as i64) + inst.offset.unwrap()) as usize
        } else {
          state.pc + 1
        },
        a: state.a,
        b: state.b,
        halt: false,
      }
    }
    Operation::JumpIfOne => {
      let v = if inst.reg == Some(Register::A) {
        state.a
      } else {
        state.b
      };
      let jump = v == 1;
      RunResult {
        pc: if jump {
          ((state.pc as i64) + inst.offset.unwrap()) as usize
        } else {
          state.pc + 1
        },
        a: state.a,
        b: state.b,
        halt: false,
      }
    }
  }
}

type Parsed = Vec<Instruction>;

fn solve(instructions: &Parsed, init: u64, sample: bool) -> Result<u64> {
  // After writing all this code to do a simulation, I looked at the input.
  // What we're really doing is thus:
  // - Longwinded way to set an initial value of A
  // - Perform the Collatz Conjecture on it until A == 1
  // - Return the number of iterations it took to get there.
  // - Part 2 changes the initial value of A.

  // I'm keeping the code to run the simulation anyway, since it works on
  // anyone's input and takes under 6 microseconds to run on any given value.
  // This also avoids hard-coding my input.

  // A faster solve would involve looking at a bunch of people's inputs to see
  // if there's a consistent way to tell which instructions are the initializers,
  // run only those, then directly run the Collatz Conjecture on that instead
  // of running all of the assembly loops. That'd get it down to sub-microsecond!

  let mut state = State {
    pc: 0,
    a: init,
    b: 0,
    instructions,
    halted: false,
  };
  while !state.halted {
    let next = step(&state);
    state = State {
      pc: next.pc,
      instructions,
      a: next.a,
      b: next.b,
      halted: next.halt,
    };
  }
  Ok(if sample { state.a } else { state.b })
}

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    Ok(
      lines
        .iter()
        .map(|line| {
          let (op, rest) = line.split_once(' ').unwrap();
          let splat = if rest.contains(", ") {
            rest.split_once(", ").unwrap()
          } else {
            (rest, "")
          };
          let mut reg: Option<Register> = None;
          let mut offset: Option<i64> = None;
          match splat.0 {
            "a" => {
              reg = Some(Register::A);
            }
            "b" => {
              reg = Some(Register::B);
            }
            _ => {
              offset = Some(splat.0.replace("+", "").parse().unwrap());
            }
          }
          if !splat.1.is_empty() && offset.is_none() {
            offset = Some(splat.1.replace("+", "").parse().unwrap());
          }
          Instruction {
            op: match op {
              "hlf" => Operation::Half,
              "tpl" => Operation::Triple,
              "inc" => Operation::Increment,
              "jmp" => Operation::Jump,
              "jie" => Operation::JumpIfEven,
              "jio" => Operation::JumpIfOne,
              _ => panic!("Unknown instruction"),
            },
            reg,
            offset,
          }
        })
        .collect::<Vec<Instruction>>(),
    )
  }

  fn part1(&self, instructions: &Parsed, sample_name: Option<String>) -> Result<P1Out> {
    solve(instructions, 0, sample_name.is_some())
  }

  fn part2(&self, instructions: &Parsed, sample_name: Option<String>) -> Result<P2Out> {
    solve(instructions, 1, sample_name.is_some())
  }
}

fn main() -> Result<()> {
  Solver {}.run(2015, 23)
}
