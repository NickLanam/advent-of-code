use advent_lib::runner::{Day, PartId};
use anyhow::{Context, Result};
use regex::Regex;

type P1Out = i64;
type P2Out = i64;
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
    let re = Regex::new(r"\\(\\|x..|[^\\])").context("Bad regex")?;
    Ok(
      lines
        .into_iter()
        .map(|line| -> i64 {
          let code = line.len();
          let glyphs = re.replace_all(line, "_").len() - 2;
          return (code as i64) - (glyphs as i64);
        })
        .fold(0, |a, c| a + c),
    )
  }

  fn part2(&self, lines: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    let re = Regex::new(r#"(\\|")"#).context("Bad regex")?;
    Ok(
      lines
        .into_iter()
        .map(|line| -> i64 {
          let plain = line.len();
          let encoded = re.replace_all(line, "__").len() + 2;
          return (encoded as i64) - (plain as i64);
        })
        .fold(0, |a, c| a + c),
    )
  }
}

fn main() -> Result<()> {
  Solver {}.run(2015, 8)
}
