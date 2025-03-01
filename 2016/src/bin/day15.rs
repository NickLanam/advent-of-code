use advent_lib::runner::{Day, PartId};
use anyhow::{Context, Result, bail};

#[derive(Debug, Clone)]
struct Disc {
  id: i64,
  length: i64,
  init: i64,
}

type P1Out = i64;
type P2Out = i64;
type Parsed = Vec<Disc>;

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
  let (mut old_r, mut r, mut old_s, mut s, mut old_t, mut t) = (a, b, 1, 0, 0, 1);

  while r != 0 {
    let (q, m) = (old_r / r, old_r % r);
    (old_r, r) = (r, m);
    (old_s, s) = (s, old_s - q * s);
    (old_t, t) = (t, old_t - q * t);
  }
  (old_r, old_s, old_t)
}

fn combine(a_period: i64, a_phase: i64, b_period: i64, b_phase: i64) -> Result<(i64, i64)> {
  let (gcd, s, _t) = extended_gcd(a_period, b_period);
  let advantage: i64 = a_phase - b_phase;
  let (pd_mult, pd_remainder) = (advantage / gcd, advantage % gcd);
  if pd_remainder != 0 {
    bail!("Remainder was non-zero");
  }
  let combined_period: i64 = b_period * (a_period / gcd);
  let combined_phase: i64 = (a_phase - s * pd_mult * a_period) % combined_period;
  Ok((combined_period, combined_phase))
}

fn solve(discs: &[Disc]) -> Result<i64> {
  if discs.is_empty() {
    bail!("No discs, no solution");
  } else if discs.len() == 1 {
    // The problem is trivial with just one disc
    return Ok(discs[0].length - ((discs[0].init + discs[0].id) % discs[0].length));
  }

  // Find the greatest common demoninator of the periods of the first two
  // discs, and the offset that lets the ball sync with both.
  let (mut combined_period, mut combined_phase) = combine(
    discs[0].length,
    discs[0].id + discs[0].init,
    discs[1].length,
    discs[1].id + discs[1].init,
  )?;

  // Now take that gcd and offset as if they were one disc,
  // and repeat the process for each remaining disc.
  for disc in discs.iter().skip(2) {
    (combined_period, combined_phase) = combine(
      combined_period,
      combined_phase,
      disc.length,
      disc.id + disc.init,
    )?;
  }

  // Now we know an offset that solves the problem, and the greatest
  // common denominator of all of their periods, so mod them
  // together to find when we should release the ball.
  Ok((0 - combined_phase + combined_period) % combined_period)
}

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    let mut discs: Vec<Disc> = Vec::with_capacity(lines.len());
    for line in lines {
      let (_, r0) = line
        .split_once("Disc #")
        .context("Can't split on 'Disc #'")?;
      let (id_raw, r1) = r0.split_once(" has ").context("Can't split for the id")?;
      let (length_raw, r2) = r1
        .split_once(" positions; at time=0, it is at position ")
        .context("Can't split for the length")?;
      let (init_raw, _) = r2
        .split_once(".")
        .context("Can't split for the initial position")?;

      let id: i64 = id_raw.parse()?;
      let length: i64 = length_raw.parse()?;
      let init: i64 = init_raw.parse()?;

      discs.push(Disc { id, length, init });
    }
    Ok(discs)
  }

  fn part1(&self, discs: &Parsed, _sample_name: Option<String>) -> Result<P1Out> {
    solve(discs)
  }

  fn part2(&self, discs: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    // Part 2 is just "add another disc on the end".
    // The naive approach would explode in runtime with this.
    // With the solution above, though, my Ryzen 9 3900X solves
    // part 1 in 230 nanoseconds and part 2 in 800 nanoseconds.
    let mut more_discs = discs.clone();
    more_discs.push(Disc {
      id: discs.len() as i64 + 1,
      length: 11,
      init: 0,
    });
    solve(&more_discs)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2016, 15)
}
