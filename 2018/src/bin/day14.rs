use advent_lib::runner::{Day, PartId};
use anyhow::{Result, bail};

type P1Out = String;
type P2Out = usize;
type Parsed = String;

#[inline(always)]
fn step(buf: &mut Vec<u8>, e0: &mut usize, e1: &mut usize) {
  let (b0, b1) = (buf[*e0], buf[*e1]);
  let next = b0 + b1;
  if next < 10 {
    buf.push(next);
  } else {
    // The buffer is split into digits 0-9,
    // so the highest value of `next` is 18.
    // Thus, we can avoid a divmod here.
    buf.push(1);
    buf.push(next - 10);
  }
  *e0 += 1 + (b0 as usize);
  *e1 += 1 + (b1 as usize);
  while *e0 >= buf.len() {
    *e0 -= buf.len();
  }
  while *e1 >= buf.len() {
    *e1 -= buf.len();
  }
}

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(lines[0].clone())
  }

  fn part1(&self, line: &Parsed, _: Option<String>) -> Result<P1Out> {
    let limit: usize = line.parse()?;
    let mut buf: Vec<u8> = vec![3, 7];
    let mut e0: usize = 0;
    let mut e1: usize = 1;

    while buf.len() < limit + 10 {
      step(&mut buf, &mut e0, &mut e1);
    }

    Ok(
      buf[limit..]
        .iter()
        .fold(String::with_capacity(10), |mut s, n| {
          s.push(n.to_string().chars().next().unwrap());
          s
        }),
    )
  }

  fn part2(&self, line: &Parsed, sample_name: Option<String>) -> Result<P2Out> {
    let pattern: Vec<u8> = (if sample_name.is_some() {
      self.part1(line, sample_name).unwrap()
    } else {
      line.clone()
    })
    .chars()
    .map(|c| c.to_digit(10).unwrap() as u8)
    .collect();

    let mut buf: Vec<u8> = vec![3, 7];
    let mut e0: usize = 0;
    let mut e1: usize = 1;

    'indexing: for i in 0.. {
      while buf.len() <= i + pattern.len() {
        step(&mut buf, &mut e0, &mut e1);
      }
      for (j, &p) in pattern.iter().enumerate() {
        if p != buf[i + j] {
          continue 'indexing;
        }
      }
      return Ok(i);
    }

    // Technically reachable, with malformed input.
    bail!("Failed to find an answer in a reasonable range");
  }
}

fn main() -> Result<()> {
  Solver {}.run(2018, 14)
}
