use advent_lib::runner::{Day, PartId};
use anyhow::{Context, Result};
use fnv::FnvBuildHasher;
use regex::Regex;
use std::collections::{HashMap, VecDeque};

type P1Out = u16;
type P2Out = u16;

#[derive(Debug, PartialEq)]
enum Op {
  Literal,
  And,
  Or,
  Not,
  Lshift,
  Rshift,
}

#[derive(Debug)]
struct Command {
  op: Op,
  a: Option<String>,
  b: Option<String>,
  c: String,
}

type Parsed = Vec<Command>;

type Registers = HashMap<String, u16, FnvBuildHasher>;
fn simulate<'a>(commands: &Parsed, preset_b: Option<u16>) -> Result<Registers> {
  let mut registers = Registers::with_hasher(FnvBuildHasher::default());
  if preset_b.is_some() {
    registers.insert("b".to_string(), preset_b.unwrap());
  }
  let get_reg_val = |key: String, reg: &mut Registers| -> Option<u16> {
    let lit = key.parse::<u16>();
    if lit.is_ok() {
      return lit.ok();
    }
    if reg.contains_key(&key) {
      return Some(*reg.get(&key).unwrap_or(&0));
    }
    return None;
  };
  let mut queue = VecDeque::from_iter(commands.into_iter());
  while !queue.is_empty() {
    let command = queue.remove(0).unwrap();
    if preset_b.is_some() && command.op == Op::Literal && command.c == "b" {
      continue;
    }
    let need_b = command.op != Op::Literal && command.op != Op::Not;
    if !command.a.is_some() || (need_b && !command.b.is_some()) {
      queue.push_back(command);
      continue;
    }
    let av = get_reg_val(command.a.clone().unwrap(), &mut registers);
    let bv = match command.b.clone() {
      Some(bk) => get_reg_val(bk, &mut registers),
      None => Some(0),
    };
    if !av.is_some() || (need_b && !bv.is_some()) {
      queue.push_back(command);
      continue;
    }
    let val: u16 = match command.op {
      Op::Literal => av.unwrap(),
      Op::Not => !av.unwrap(),
      Op::And => av.unwrap() & bv.unwrap(),
      Op::Or => av.unwrap() | bv.unwrap(),
      Op::Lshift => av.unwrap() << bv.unwrap(),
      Op::Rshift => av.unwrap() >> bv.unwrap(),
    };
    registers.insert(command.c.to_string(), val);
  }
  return Ok(registers);
}

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    let re = Regex::new(r"^((?<a>[a-z\d]+) )?((?<op>[A-Z]+) )?(?<b>[a-z\d]+) -> (?<c>[a-z]+)$")
      .context("Bad parsing regex")?;

    Ok(
      lines
        .iter()
        .map(|line| {
          if let Some(caps) = re.captures(line) {
            let mut op: Op = Op::Literal;
            if let Some(m) = caps.name("op") {
              op = match m.as_str() {
                "AND" => Op::And,
                "OR" => Op::Or,
                "NOT" => Op::Not,
                "LSHIFT" => Op::Lshift,
                "RSHIFT" => Op::Rshift,
                other => {
                  panic!("Unrecognized op {other} in line {line}");
                }
              }
            }

            let a: Option<String>;
            let b: Option<String>;
            let c = caps.name("c").unwrap().as_str().to_string();
            match op {
              Op::And | Op::Or | Op::Lshift | Op::Rshift => {
                a = Some(caps.name("a").unwrap().as_str().to_string());
                b = Some(caps.name("b").unwrap().as_str().to_string());
              }
              Op::Not | Op::Literal => {
                a = Some(caps.name("b").unwrap().as_str().to_string());
                b = None;
              }
            }
            Command { op, a, b, c }
          } else {
            panic!("Failed to parse line {line}");
          }
        })
        .collect(),
    )
  }

  fn part1(&self, commands: &Parsed, sample_name: Option<String>) -> Result<P1Out> {
    let target_gate = if sample_name.is_some() { "i" } else { "a" };
    let registers = simulate(commands, None)?;
    Ok(*registers.get(target_gate).unwrap())
  }

  fn part2(&self, commands: &Parsed, sample_name: Option<String>) -> Result<P2Out> {
    // Sample doesn't really have a meaningful thing to do for this one
    if sample_name.is_some() {
      return Ok(1);
    }
    let first_pass = simulate(commands, None)?;
    let av = *first_pass.get("a").unwrap();
    let second_pass = simulate(commands, Some(av))?;
    Ok(*second_pass.get("a").unwrap())
  }
}

fn main() -> Result<()> {
  Solver {}.run(2015, 7)
}
