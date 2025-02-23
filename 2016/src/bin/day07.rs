use advent_lib::runner::{Day, PartId};
use anyhow::{Result, bail};

type P1Out = u64;
type P2Out = u64;
type Parsed = Vec<String>;

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    // Observation: every line has at least one [bracket set], many have at least two.
    // None of these sections are close to the end of that line.
    // There are no nested brackets, no mismatched pairs.
    // That is to say: it's always well-formed and doesn't have many edge cases.
    Ok(lines)
  }

  fn part1(&self, lines: &Parsed, _sample_name: Option<String>) -> Result<P1Out> {
    let mut sum = 0;
    'line_check: for line in lines {
      let chars: Vec<char> = line.chars().collect();
      let mut have_abba = false;
      let mut in_brackets = false;
      'char_loop: for i in 0..=(chars.len() - 4) {
        let [a, b, c, d] = chars[i..(i + 4)] else {
          bail!("Bad loop conditions");
        };
        if a == '[' {
          in_brackets = true;
          continue 'char_loop;
        } else if a == ']' {
          in_brackets = false;
          continue 'char_loop;
        }
        if a != b && b != '[' && b != ']' && c == b && d == a {
          if in_brackets {
            continue 'line_check;
          } else {
            have_abba = true;
          }
        }
      }
      if have_abba {
        sum += 1;
      }
    }
    Ok(sum)
  }

  fn part2(&self, lines: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    let mut sum = 0;
    'line_check: for line in lines {
      let mut inners: Vec<String> = vec![];
      let mut expected_inners: Vec<String> = vec![];
      let mut in_brackets = false;
      let chars: Vec<char> = line.chars().collect();
      'char_loop: for i in 0..=(chars.len() - 3) {
        let [a, b, c] = chars[i..(i + 3)] else {
          bail!("Bad loop conditions");
        };
        if a == '[' {
          in_brackets = true;
          continue 'char_loop;
        } else if a == ']' {
          in_brackets = false;
          continue 'char_loop;
        }
        if a == c && a != b && b != '[' && b != ']' {
          if in_brackets {
            inners.push(String::from_iter([a, b, a]));
          } else {
            expected_inners.push(String::from_iter([b, a, b]));
          }
        }
      }
      if !expected_inners.is_empty() {
        // Check if _any_ of expected_inners exist in inners.
        // Only need one for it to count!
        for ei in expected_inners {
          if inners.contains(&ei) {
            sum += 1;
            continue 'line_check;
          }
        }
      }
    }
    Ok(sum)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2016, 7)
}
