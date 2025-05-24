use advent_lib::runner::{Day, PartId};
use anyhow::{Result, bail};

#[derive(Debug, Default, PartialEq, Hash)]
struct Room {
  n: bool,
  e: bool,
  s: bool,
  w: bool,
}

type P1Out = usize;
type P2Out = usize;
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
    // TODO: This first attempt assumed there'd be no loops. It could be adjusted to
    // remove easily-detected (shorter) loops, but there are sample inputs (and my real one)
    // that have harder-to-detect ones.
    // Best to actually build the map and pathfind it.
    let mut stack: Vec<Vec<usize>> = vec![vec![]];
    for ch in line.chars().skip(1).take(line.len() - 2) {
      match ch {
        'N' | 'E' | 'S' | 'W' => {
          if let Some(top_group) = stack.last_mut() {
            if let Some(top_size) = top_group.last_mut() {
              *top_size += 1;
            } else {
              top_group.push(1);
            }
          } else {
            stack.push(vec![1]);
          }
        }
        '(' => {
          stack.push(vec![0]);
        }
        ')' => {
          let max = *stack.pop().unwrap().iter().max().unwrap();
          let l = stack.len();
          *stack[l - 1].last_mut().unwrap() += max;
        }
        '|' => {
          let l = stack.len();
          stack[l - 1].push(0);
        }
        _ => {
          bail!("Unrecognized char: {ch}");
        }
      }
      // println!("After seeing '{ch}', stack is now:\n  {stack:?}");
    }
    // 4221 is too high. Because of the loops.
    // Can't just replace them either: sure, I can turn 'EW', 'WE', 'NESW', and every other
    // combination into the empty string, but for example NNNNESSSSW is a loop too.
    Ok(stack.pop().unwrap().pop().unwrap())
  }

  fn part2(&self, _lines: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    Ok(0)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2018, 20)
}
