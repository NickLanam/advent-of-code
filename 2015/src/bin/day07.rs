use advent_lib::runner::{Day, PartId};
use anyhow::{Context, Result};
use fnv::FnvBuildHasher;
use regex::Regex;
use std::collections::HashMap;

type P1Out = u16;
type P2Out = u16;

#[derive(Debug)]
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
    let mut registers =
      HashMap::<String, u16, FnvBuildHasher>::with_hasher(FnvBuildHasher::default());
    for command in commands {
      let Command { op, a, b, c } = command;
      let av = match a {
        Some(ref ak) => *registers.get(ak).unwrap_or(&0),
        None => 0,
      };
      let bv = match b {
        Some(ref bk) => *registers.get(bk).unwrap_or(&0),
        None => 0,
      };

      match op {
        Op::Literal => {
          registers.insert(c.to_string(), av);
        }
        Op::Not => {
          registers.insert(c.to_string(), !av);
        }
        Op::And => {
          registers.insert(c.to_string(), av & bv);
        }
        Op::Or => {
          registers.insert(c.to_string(), av | bv);
        }
        Op::Lshift => {
          registers.insert(c.to_string(), av << bv);
        }
        Op::Rshift => {
          registers.insert(c.to_string(), av >> bv);
        }
      }
    }
    Ok(*registers.get(target_gate).unwrap())
  }

  fn part2(&self, _commands: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    Ok(0)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2015, 7)
}
