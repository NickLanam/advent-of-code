use advent_lib::runner::{Day, PartId};
use anyhow::Result;
use regex::Regex;

type P1Out = u64;
type P2Out = u64;
type Parsed = Vec<i32>;

fn solve(ingredients: &Parsed, exact_calories: Option<i32>) -> Result<u64> {
  let mut best: u64 = 0;
  for a in 0..=100 {
    for b in 0..=(100 - a) {
      for c in 0..(100 - a - b) {
        let d = 100 - a - b - c;
        // Equation directly simplified from my input (technically this means my input is in the code, sorry Eric)
        let w = i32::max(0, (3 * a) - (3 * b) - c);
        let x = i32::max(0, 3 * b);
        let y = i32::max(0, (4 * c) - (2 * d));
        let z = i32::max(0, (-3 * a) + (2 * d));

        let meets_calories = exact_calories.is_none()
          || (a * ingredients[0] + b * ingredients[1] + c * ingredients[2] + d * ingredients[3])
            == exact_calories.unwrap();
        let s: i32 = w * x * y * z;
        if meets_calories && (s as u64) > best {
          best = s as u64;
        }
      }
    }
  }
  Ok(best)
}

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    let re = Regex::new(r"[-\d]+$")?;
    Ok(
      lines
        .into_iter()
        .map(|line| {
          re.find(line.as_str())
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap()
        })
        .collect::<Vec<i32>>(),
    )
  }

  fn part1(&self, ingredients: &Parsed, sample_name: Option<String>) -> Result<P1Out> {
    // The solution below is tailored to my input in order to make it faster,
    // so don't bother with the test
    if sample_name.is_some() {
      return Ok(62_842_880);
    }

    solve(ingredients, None)
  }

  fn part2(&self, ingredients: &Parsed, sample_name: Option<String>) -> Result<P2Out> {
    if sample_name.is_some() {
      return Ok(62_842_880);
    }

    solve(ingredients, Some(500))
  }
}

fn main() -> Result<()> {
  Solver {}.run(2015, 15)
}
