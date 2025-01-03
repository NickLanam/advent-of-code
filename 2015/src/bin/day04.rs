use std::u64::MAX;

use advent_lib::runner::{run, RunDetails};
use crypto::digest::Digest;
use crypto::md5::Md5;

type Parsed = String;

fn solve(input: String, start_pattern: String) -> u64 {
  let mut hasher = Md5::new();
  for i in 0_u64..MAX {
    hasher.reset();
    hasher.input_str(&format!("{}{}", input.clone(), i));
    let digest = hasher.result_str();
    if digest.starts_with(&start_pattern) {
      return i;
    }
  }
  return 0;
}

fn main() {
  const DETAILS: RunDetails<Parsed, u64, u64> = RunDetails {
    year: 2015,
    day: 4,
    parse: |lines, _, _| lines[0].clone(),
    part1: |input, _| solve(input, "00000".to_string()),
    part2: |input, _| solve(input, "000000".to_string()),
  };
  run(DETAILS);
}
