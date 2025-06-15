use advent_lib::runner::{Day, PartId};
use anyhow::{Result, bail};

type P1Out = u64;
type P2Out = u64;
type Parsed = String;

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(lines[0].to_string())
  }

  fn part1(&self, input: &Parsed, _sample_name: Option<String>) -> Result<P1Out> {
    let w = 25;
    let h = 6;
    let lim = w * h;
    let mut fewest_zero = usize::MAX;
    let mut score = 0;
    let mut l = 0;

    let mut zero = 0;
    let mut one = 0;
    let mut two = 0;
    for ch in input.chars() {
      if l >= lim {
        if zero < fewest_zero {
          fewest_zero = zero;
          score = one * two;
        }
        zero = 0;
        one = 0;
        two = 0;
        l = 0;
      }
      l += 1;
      match ch {
        '0' => {
          zero += 1;
        }
        '1' => {
          one += 1;
        }
        '2' => {
          two += 1;
        }
        _ => bail!("Unreachable"),
      }
    }
    Ok(score)
  }

  fn part2(&self, _lines: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    /*
    let w = 25;
    let h = 6;
    let mut layers: Vec<Vec<Vec<u8>>> = vec![];
    layers.push(Vec::with_capacity(h));
    for ch in lines[0].chars() {
      let layer = layers.last().unwrap();
    }
    let mut out = Vec::with_capacity(h);
    out.push(Vec::with_capacity(w));
    let mut y = 0;
    let mut x = 0;
    for ch in lines[0].chars() {
      let v: u8 = match ch {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        _ => bail!("Unrecognized char '{ch}' in input stream"),
      };
      if x == w {
        out.push(Vec::with_capacity(w));
        x = 0;
        y += 1;
      }
      out[y].push(v);
      x += 1;
    }
    Ok(out)
    */
    Ok(0)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2019, 8)
}
