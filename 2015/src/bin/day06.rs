use advent_lib::grid::{Infinite2dGrid, Infinite2dSet};
use regex::Regex;

use advent_lib::runner::{Day, PartId};
use anyhow::{Context, Result};

type P1Out = u64;
type P2Out = u64;

#[derive(Debug)]
enum CommandKind {
  On,
  Off,
  Toggle,
}

#[derive(Debug)]
struct Command {
  kind: CommandKind,
  x1: i32,
  y1: i32,
  x2: i32,
  y2: i32,
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
    let parse_re = Regex::new(
      r"^(?P<kind>turn on|turn off|toggle) (?P<x1>\d+),(?P<y1>\d+) through (?P<x2>\d+),(?P<y2>\d+)$",
    )
    .context("Bad regex for parsing")?;
    Ok(
      lines
        .iter()
        .map(|line| {
          let caps_opt = parse_re.captures(line);
          let caps = caps_opt.unwrap_or_else(|| panic!("{line}"));
          let kind: CommandKind = match &caps["kind"] {
            "turn off" => CommandKind::Off,
            "turn on" => CommandKind::On,
            "toggle" => CommandKind::Toggle,
            _ => panic!("Bad kind"),
          };
          let x1: i32 = String::from(&caps["x1"]).parse().unwrap();
          let y1: i32 = String::from(&caps["y1"]).parse().unwrap();
          let x2: i32 = String::from(&caps["x2"]).parse().unwrap();
          let y2: i32 = String::from(&caps["y2"]).parse().unwrap();
          Command {
            kind,
            x1,
            y1,
            x2,
            y2,
          }
        })
        .collect::<Vec<Command>>(),
    )
  }

  fn part1(&self, commands: &Parsed, _sample_name: Option<String>) -> Result<P1Out> {
    let mut grid = Infinite2dSet::new(1_000_000);
    for command in commands {
      let Command {
        kind,
        x1,
        y1,
        x2,
        y2,
      } = command;
      for x in *x1..=*x2 {
        for y in *y1..=*y2 {
          match kind {
            CommandKind::Off => {
              grid.remove(x, y);
            }
            CommandKind::On => {
              grid.add(x, y);
            }
            CommandKind::Toggle => {
              grid.toggle(x, y);
            }
          }
        }
      }
    }
    Ok(grid.len() as u64)
  }

  fn part2(&self, commands: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    let mut grid = Infinite2dGrid::<u64>::new(1_000_000);
    for command in commands {
      let Command {
        kind,
        x1,
        y1,
        x2,
        y2,
      } = command;
      for x in *x1..=*x2 {
        for y in *y1..=*y2 {
          match kind {
            CommandKind::Off => {
              let prev = *grid.get(x, y).unwrap_or(&0);
              if prev <= 1 {
                grid.remove(x, y);
              } else {
                grid.set(x, y, prev - 1);
              }
            }
            CommandKind::On => {
              grid.set_action(x, y, |prev| prev.map_or(Some(1), |x| Some(x + 1)));
            }
            CommandKind::Toggle => {
              grid.set_action(x, y, |prev| prev.map_or(Some(2), |x| Some(x + 2)));
            }
          }
        }
      }
    }
    Ok(grid.values().fold(0, |a, v| a + *v))
  }
}

fn main() -> Result<()> {
  Solver {}.run(2015, 6)
}
