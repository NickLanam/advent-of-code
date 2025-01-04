use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = u64;
type P2Out = u64;
type Parsed = Vec<(u64, u64, u64)>;

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    Ok(
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
        .collect(),
    )
  }

  fn part1(&self, presents: &Parsed, _sample_name: Option<String>) -> Result<P1Out> {
    Ok(presents.into_iter().fold(0, |a, (l, w, h)| {
      let x = l * w;
      let y = w * h;
      let z = h * l;
      let m = *vec![x, y, z].iter().min().unwrap();
      a + 2 * (x + y + z) + m
    }))
  }

  fn part2(&self, presents: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    Ok(presents.into_iter().fold(0, |a, (l, w, h)| {
      let mut sides = vec![l, w, h];
      sides.sort();
      let r = 2 * (sides[0] + sides[1]);
      a + (l * w * h) + r
    }))
  }
}

fn main() -> Result<()> {
  Solver {}.run(2015, 2)
}
