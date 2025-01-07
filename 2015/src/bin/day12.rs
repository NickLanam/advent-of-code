use advent_lib::runner::{Day, PartId};
use anyhow::Result;
use regex::Regex;
use serde_json::Value;

type P1Out = i64;
type P2Out = i64;
type Parsed = String;

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    Ok(lines[0].to_string())
  }

  fn part1(&self, line: &Parsed, _sample_name: Option<String>) -> Result<P1Out> {
    // The promised restrictions on the input from the problem description make it
    // so that we don't need to parse the JSON at all - just need to look for integers.
    // There aren't even any floats in there.
    let re = Regex::new(r"-?\d+").unwrap();
    let mut out: i64 = 0;
    for m in re.find_iter(line) {
      out += m.as_str().parse::<i64>().unwrap();
    }
    Ok(out)
  }

  fn part2(&self, line: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    // This time, we actually have to parse it and dive into it to get the answer.
    // Nowhere near as messy of an object as what I regularly have to parse through for work...
    let mut out: i64 = 0;
    let mut stack: Vec<Value> = vec![serde_json::from_str(line)?];
    'iter: while let Some(item) = stack.pop() {
      if item.is_object() {
        let mut sub_stack: Vec<Value> = Vec::new();
        for sub in item.as_object().unwrap().into_iter() {
          if sub.1.is_string() && sub.1.as_str().unwrap() == "red" {
            continue 'iter;
          } else {
            sub_stack.push(sub.1.to_owned())
          }
        }
        for sub in sub_stack.into_iter() {
          stack.push(sub);
        }
      } else if item.is_array() {
        for sub in item.as_array().unwrap().iter() {
          stack.push(sub.to_owned());
        }
      } else if item.is_number() {
        out += item.as_number().unwrap().as_i64().unwrap();
      }
    }
    Ok(out)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2015, 12)
}
