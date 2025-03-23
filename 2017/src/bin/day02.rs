use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = u32;
type P2Out = u32;
type Parsed = Vec<Vec<u32>>;

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    let mut out: Vec<Vec<u32>> = Vec::with_capacity(lines.len());
    for line in lines {
      let mut row: Vec<u32> = Vec::new();
      for n in line.split_whitespace() {
        row.push(n.parse()?);
      }
      out.push(row);
    }
    Ok(out)
  }

  fn part1(&self, rows: &Parsed, _: Option<String>) -> Result<P1Out> {
    let mut sum = 0;
    for row in rows {
      let mut min = u32::MAX;
      let mut max = u32::MIN;
      for &n in row {
        min = min.min(n);
        max = max.max(n);
      }
      sum += max - min;
    }
    Ok(sum)
  }

  fn part2(&self, rows: &Parsed, _: Option<String>) -> Result<P2Out> {
    let mut sum = 0;
    'row_loop: for row in rows {
      for i in 0..row.len() - 1 {
        let a = row[i];
        for &b in row.iter().skip(i + 1) {
          let c = a.max(b);
          let d = a.min(b);
          let e = c / d;
          if e * d == c {
            sum += e;
            continue 'row_loop;
          }
        }
      }
    }
    Ok(sum)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2017, 2)
}
