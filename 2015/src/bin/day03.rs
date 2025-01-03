use advent_lib::grid::Infinite2dGrid;

use advent_lib::runner::{run, PartId, RunDetails};

type Parsed = Vec<char>;

fn parse(lines: Vec<String>, _sample_name: Option<String>, _for_part: PartId) -> Parsed {
  lines[0].chars().collect::<Vec<char>>()
}

fn solve(dirs: Parsed, with_robo_santa: bool) -> usize {
  let mut houses = Infinite2dGrid::<u64>::new(dirs.len());
  let mut visit = |x: i32, y: i32| {
    houses.set_action(x, y, |prev| Some(prev.unwrap_or(&0) + 1));
  };

  let mut pos = vec![0, 0, 0, 0];
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

fn main() {
  const DETAILS: RunDetails<Parsed, usize, usize> = RunDetails {
    year: 2015,
    day: 3,
    parse,
    part1: |dirs, _| solve(dirs, false),
    part2: |dirs, _| solve(dirs, true),
  };
  run(DETAILS);
}
