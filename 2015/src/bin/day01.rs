use advent_lib::runner::{run, PartId, RunDetails};

type Part1Solution = i64;
type Part2Solution = usize;

type Parsed = String;

fn parse(lines: Vec<String>, _sample_name: Option<String>, _for_part: PartId) -> Parsed {
  lines[0].to_string()
}

fn part1(line: Parsed, _sample_name: Option<String>) -> Part1Solution {
  return line
    .chars()
    .fold(0_i64, |a, c| a + (if c == '(' { 1 } else { -1 }));
}

fn part2(line: Parsed, _sample_name: Option<String>) -> Part2Solution {
  let mut floor: i64 = 0;
  for (i, c) in line.chars().enumerate() {
    match c {
      '(' => {
        floor += 1;
      }
      ')' => {
        floor -= 1;
        if floor < 0 {
          return i + 1;
        }
      }
      _ => {
        panic!("Unexpected character {c}");
      }
    }
  }
  return 0; // Means "never went into the basement" - most of the samples don't.
}

fn main() {
  const DETAILS: RunDetails<Parsed, Part1Solution, Part2Solution> = RunDetails {
    year: 2015,
    day: 1,
    parse,
    part1,
    part2,
  };
  run(DETAILS);
}
