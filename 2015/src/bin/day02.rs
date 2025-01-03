use advent_lib::runner::{run, PartId, RunDetails};

type Part1Solution = u64;
type Part2Solution = u64;

type Parsed = Vec<(u64, u64, u64)>;

fn parse(lines: Vec<String>, _sample_name: Option<String>, _for_part: PartId) -> Parsed {
  lines
    .iter()
    .map(|line| {
      let [l, w, h] = line
        .split('x')
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()[..]
      else {
        panic!("Line was not in the expected format");
      };
      (l, w, h)
    })
    .collect()
}

fn part1(presents: Parsed, _sample_name: Option<String>) -> Part1Solution {
  presents.into_iter().fold(0, |a, (l, w, h)| {
    let x = l * w;
    let y = w * h;
    let z = h * l;
    let m = *vec![x, y, z].iter().min().unwrap();
    a + 2 * (x + y + z) + m
  })
}

fn part2(presents: Parsed, _sample_name: Option<String>) -> Part2Solution {
  presents.into_iter().fold(0, |a, (l, w, h)| {
    let mut sides = vec![l, w, h];
    sides.sort();
    let r = 2 * (sides[0] + sides[1]);
    a + (l * w * h) + r
  })
}

fn main() {
  const DETAILS: RunDetails<Parsed, Part1Solution, Part2Solution> = RunDetails {
    year: 2015,
    day: 2,
    parse,
    part1,
    part2,
  };
  run(DETAILS);
}
