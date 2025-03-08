use advent_lib::runner::{Day, PartId};
use anyhow::{Result, bail};

type P1Out = u64;
type P2Out = u64;
type Parsed = u64;

/// Used this method to figure out the pattern.
///
/// After staring at it long enough... the pattern does this:
/// * Let lo = the highest (3.pow(x)) < target
/// * Let hi = lo * 3 (which will be >= target)
/// * If (hi - target) < lo, answer is (target - (hi - target))
/// * Otherwise, answer is (target - lo)
///
/// This works for all games with at least two players.
#[allow(dead_code)]
fn slow_manual_part2_scan() {
  for l in 2_usize..=(3_usize.pow(5)) {
    let mut v = Vec::with_capacity(l);
    for f in 1..=l {
      v.push(f);
    }
    let mut t = 0_usize;
    while v.len() > 1 {
      let c = (t + (v.len() / 2)) % v.len();
      v.remove(c);
      if c > t {
        t += 1;
      }
      if t >= v.len() {
        t = 0;
      }
    }
    let answer = v[0];
    println!("{l:3} elements -> {answer}");
  }
}

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    Ok(lines[0].parse().unwrap())
  }

  fn part1(&self, initial_size: &Parsed, _: Option<String>) -> Result<P1Out> {
    // First, I noticed this pattern worked.
    // let mut lead: u64 = 1;
    // let mut step: u64 = 1;
    // let mut size: u64 = *initial_size;
    // while size > 1 {
    //   step *= 2;
    //   if size % 2 == 1 {
    //     lead += step;
    //   }
    //   size /= 2; // Truncate
    // }
    // Ok(lead)

    // Simplified: "ignoring leading zeros, do a bitwise left-rotate by 1".
    // Which is only a handful of CPU instructions, no loops:
    let z = initial_size.leading_zeros();
    Ok((initial_size << z << 1 >> z) + 0b1)
  }

  fn part2(&self, target: &Parsed, _: Option<String>) -> Result<P2Out> {
    if *target < 2 {
      bail!("Need at least two players!");
    }

    // See slow_manual_part2_scan() above to verify this works.
    let mut lo = 1;
    while lo * 3 < *target {
      lo *= 3
    }
    let hi = lo * 3;
    let hi_distance = hi - *target;
    Ok(*target - lo.min(hi_distance))
  }
}

fn main() -> Result<()> {
  Solver {}.run(2016, 19)
}
