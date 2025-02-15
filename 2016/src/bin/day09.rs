use advent_lib::runner::{Day, PartId};
use anyhow::Result;

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
    let chars: Vec<char> = line.chars().collect();
    let mut count: usize = 0;
    let mut i = 0;
    let mut checking_a = false;
    let mut checking_b = false;
    let mut l: usize = 0;
    let mut r: usize = 0;

    // Actually decompressing the string is dumb if we only want to know its length.
    // Jumping through the initial string and doing math avoids allocating anything.
    while i < chars.len() {
      let c = chars[i];
      if c == '(' {
        checking_a = true;
        checking_b = false;
        l = 0;
        i += 1;
      } else if c == 'x' && checking_a {
        checking_a = false;
        checking_b = true;
        r = 0;
        i += 1;
      } else if c == ')' && checking_b {
        checking_a = false;
        checking_b = false;
        i += l + 1_usize;
        count += l * r;
        l = 0;
        r = 0;
      } else if checking_a {
        l *= 10;
        l += c.to_digit(10).unwrap() as usize;
        i += 1;
      } else if checking_b {
        r *= 10;
        r += c.to_digit(10).unwrap() as usize;
        i += 1;
      } else {
        count += 1;
        i += 1;
      }
    }
    Ok(count)
  }

  fn part2(&self, line: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    let chars: Vec<char> = line.chars().collect();
    let mut count: usize = 0;
    let mut i = 0;
    let mut checking_a = false;
    let mut checking_b = false;
    let mut l: usize = 0;
    let mut r: usize = 0;

    // (1x1) is 5 characters, worst case is a bunch of multipliers then a single letter.
    // Pre-allocating the worst case amount of space saves about 20% of the run time on average.
    let mut mul_stack: Vec<(usize, usize)> = Vec::with_capacity(line.len() / 5);

    // More efficient than re-computing it every time
    let mut running_mul: usize = 1;

    // As before, just go through the string - but instead of skipping ahead when
    // we finish reading a multiplier, keep a running combined multiplier for all
    // characters that aren't part of a multiplier. When we reach the end of a
    // tracked multiplier's range, remove it (multiple can end at the same index)
    while i < chars.len() {
      let c = chars[i];
      if c == '(' {
        checking_a = true;
        checking_b = false;
        l = 0;
      } else if c == 'x' && checking_a {
        checking_a = false;
        checking_b = true;
        r = 0;
      } else if c == ')' && checking_b {
        checking_a = false;
        checking_b = false;
        running_mul *= r;
        mul_stack.push((l + i + 1, r));
        l = 0;
        r = 0;
      } else if checking_a {
        l *= 10;
        l += c.to_digit(10).unwrap() as usize;
      } else if checking_b {
        r *= 10;
        r += c.to_digit(10).unwrap() as usize;
      } else {
        count += running_mul;

        // Pop any multipliers that have expired (there may be more than one)
        while !mul_stack.is_empty() {
          let (stop_index, mul_to_remove) = mul_stack.last().unwrap();
          if i + 1 >= *stop_index {
            running_mul /= *mul_to_remove;
            mul_stack.pop();
          } else {
            break;
          }
        }
      }
      i += 1;
    }
    Ok(count)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2016, 9)
}
