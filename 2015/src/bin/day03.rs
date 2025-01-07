use advent_lib::grid::Infinite2dGrid;
use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type Parsed = Vec<char>;

fn solve(dirs: &Parsed, with_robo_santa: bool) -> usize {
  let mut houses = Infinite2dGrid::<u64>::new(dirs.len());
  let mut visit = |x: i32, y: i32| {
    houses.set_action(x, y, |prev| Some(prev.unwrap_or(&0) + 1));
  };

  let mut pos = [0, 0, 0, 0];
  let mut shift = 0; // Toggles between Santa and Robo_Santa
  visit(0, 0); // Santa delivers a present at his starting location, too
  for dir in dirs {
    match dir {
      '^' => {
        pos[shift + 1] -= 1;
      }
      '>' => {
        pos[shift] += 1;
      }
      'v' => {
        pos[shift + 1] += 1;
      }
      '<' => {
        pos[shift] -= 1;
      }
      _ => {
        panic!("Unexpected direction {dir}");
      }
    }
    visit(pos[shift], pos[shift + 1]);
    if with_robo_santa {
      shift = if shift == 0 { 2 } else { 0 };
    }
  }

  houses.values().filter(|v| **v > 0).count()
}

struct Solver {}
impl Day<Parsed, usize, usize> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    Ok(lines[0].chars().collect::<Vec<char>>())
  }

  fn part1(&self, dirs: &Parsed, _sample_name: Option<String>) -> Result<usize> {
    Ok(solve(dirs, false))
  }

  fn part2(&self, dirs: &Parsed, _sample_name: Option<String>) -> Result<usize> {
    Ok(solve(dirs, true))
  }
}

fn main() -> Result<()> {
  Solver {}.run(2015, 3)
}
