use advent_lib::runner::{Day, PartId};
use anyhow::{Context, Result};

type P1Out = usize;
type P2Out = i64;
type Parsed = Vec<(i64, i64, i64, i64)>;

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    let mut out = Vec::with_capacity(lines.len());
    for line in lines {
      let (left, r) = line[5..].split_once(">, r=").context("Bad line")?;
      let (x, yz) = left.split_once(",").context("Can't extract coordinates")?;
      let (y, z) = yz.split_once(",").context("Can't extract coordinates")?;
      out.push((x.parse()?, y.parse()?, z.parse()?, r.parse()?));
    }
    Ok(out)
  }

  fn part1(&self, bots: &Parsed, _sample_name: Option<String>) -> Result<P1Out> {
    let &(lx, ly, lz, lr) = bots.iter().max_by_key(|&&(_, _, _, r)| r).unwrap();
    let mut in_range: usize = 0;
    for &(x, y, z, _r) in bots.iter() {
      let d = (x.abs_diff(lx) + y.abs_diff(ly) + z.abs_diff(lz)) as i64;
      if d <= lr {
        in_range += 1;
      }
    }
    Ok(in_range)
  }

  fn part2(&self, bots: &Parsed, sample_name: Option<String>) -> Result<P2Out> {
    // Massive assumption: we can reduce the solution from three dimensions to
    // one, because the structure of the input and the use of manhattan distance
    // happens to make it work out such that we can just walk through a sorted
    // sequence of
    println!("\nRUN: {sample_name:?}");

    let mut segments: Vec<(i64, i64)> = vec![];
    for &(x, y, z, r) in bots {
      let d = x.abs() + y.abs() + z.abs();
      segments.push((0.max(d - r), 1));
      segments.push((d + r + 1, -1));
    }
    segments.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    let mut count = 0;
    let mut max_count = 0;
    let mut result = 0;
    for (d, e) in segments {
      println!("Popped {d}, {e}");
      count += e;
      if count > max_count {
        result = d;
        max_count = count;
        println!("  result={result}, max_count={max_count}, count={count}");
      }
    }
    // 84_087_794 is too low, so that isn't it...
    // 84_087_817 is too high, which was the next node that was popped after the too low answer
    // Meaning the correct answer is in a range of 23 values. I get the correct
    // answer on the samples, so there's something slightly off with my input
    // that makes the trick _almost_ but not quite work...
    // 84_087_805 is "not right" (the midpoint), was hoping I'd get one more chance there.
    println!("FINAL  result={result}, max_count={max_count}, count={count}");
    Ok(result)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2018, 23)
}
