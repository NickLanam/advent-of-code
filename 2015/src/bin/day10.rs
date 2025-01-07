use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = usize;
type P2Out = usize;
type Parsed = String;

// This is a lot larger than the naive approach, but also significantly faster - far fewer allocations!
fn expand(s: String) -> String {
  // The degenerate case, where there are no runs of the same character, doubles the string.
  // Every other case uses less than double (and can sometimes even shrink the string).
  // In practice, pre-allocating a string for this worst case saves a few microseconds per iteration!
  let mut expanded = String::with_capacity(2 * s.len());
  let mut run = 0;
  let mut run_of = '0';
  for c in s.chars() {
    if run == 0 {
      run_of = c;
      run = 1;
    } else if c == run_of {
      run += 1;
    } else {
      // Faster than constructing the character from the length in any of the ways I tried
      match run {
        1 => {
          expanded.push('1');
        }
        2 => {
          expanded.push('2');
        }
        3 => {
          expanded.push('3');
        }
        _ => {
          panic!("Only way this can happen is if it was in the initial string");
        }
      }
      expanded.push(run_of);
      run_of = c;
      run = 1;
    }
  }

  // Close up with the final run
  match run {
    1 => {
      expanded.push('1');
    }
    2 => {
      expanded.push('2');
    }
    3 => {
      expanded.push('3');
    }
    _ => {
      panic!("Only way this can happen is if it was in the initial string");
    }
  }
  expanded.push(run_of);

  expanded
}

fn solve(s: String, iterations: u64) -> usize {
  let mut out = s.clone();
  for _ in 0..iterations {
    out = expand(out);
  }
  out.len()
}

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

  fn part1(&self, line: &Parsed, sample_name: Option<String>) -> Result<P1Out> {
    Ok(solve(
      line.to_string(),
      if sample_name.is_some() { 5 } else { 40 },
    ))
  }

  fn part2(&self, line: &Parsed, sample_name: Option<String>) -> Result<P2Out> {
    Ok(solve(
      line.to_string(),
      if sample_name.is_some() { 5 } else { 50 },
    ))
  }
}

fn main() -> Result<()> {
  Solver {}.run(2015, 10)
}
