use advent_lib::runner::{Day, PartId};
use anyhow::Result;

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
    Ok(lines)
  }

  fn part1(&self, lines: &Parsed, _sample_name: Option<String>) -> Result<P1Out> {
    Ok(
      lines
        .iter()
        .map::<bool, _>(|source| {
          let s = source.as_bytes();
          let mut vowels = 0;
          let mut double = false;
          for i in 0..s.len() {
            let c = s[i];
            let d = if i < s.len() - 1 { s[i + 1] } else { 0 };
            // ab, cd, pq, and xy mark the word as naughty
            if (c == 97 || c == 99 || c == 112 || c == 120) && d == c + 1 {
              return false;
            }
            // Count vowels (not including y)
            if c == 97 || c == 101 || c == 105 || c == 111 || c == 117 {
              vowels += 1;
            }
            // If we didn't already see a double, check for one
            if !double && c == d {
              double = true;
            }
          }
          return vowels >= 3 && double;
        })
        .fold(0, |a, c| a + (if c { 1 } else { 0 })),
    )
  }

  fn part2(&self, lines: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    Ok(
      lines
        .iter()
        .map::<bool, _>(|source| {
          let s = source.as_bytes();
          let mut spaced = false;
          let mut paired = false;
          for i in 0..s.len() - 2 {
            let c = s[i];
            let d = s[i + 1];
            let e = s[i + 2];
            if !spaced && c == e {
              spaced = true;
            }
            if !paired && i < s.len() - 3 {
              for j in (i + 2)..(s.len() - 1) {
                let f = s[j];
                let g = s[j + 1];
                if f == c && g == d {
                  paired = true;
                  break;
                }
              }
            }
            if spaced && paired {
              return true;
            }
          }
          return false;
        })
        .fold(0, |a, c| a + (if c { 1 } else { 0 })),
    )
  }
}

fn main() -> Result<()> {
  Solver {}.run(2015, 5)
}
