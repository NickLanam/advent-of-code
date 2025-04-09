use advent_lib::runner::{Day, PartId};
use anyhow::{Context, Result, bail};
use itertools::Itertools;

type P1Out = i64;
type P2Out = i64;
type Parsed = Vec<(i64, i64, i64)>;

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    let mut layers = Vec::with_capacity(lines.len());
    for line in lines {
      let (left, right) = line.split_once(": ").context("")?;
      let offset: i64 = left.parse()?;
      let size = right.parse()?;
      // Pre-computing this saves a ton of time in part 2.
      let period = (size - 1) * 2;
      layers.push((offset, size, period));
    }
    Ok(layers)
  }

  fn part1(&self, layers: &Parsed, _: Option<String>) -> Result<P1Out> {
    Ok(
      layers
        .iter()
        .map(|&(offset, size, period)| {
          if offset % period == 0 {
            offset * size
          } else {
            0
          }
        })
        .sum(),
    )
  }

  fn part2(&self, initial_layers: &Parsed, _: Option<String>) -> Result<P2Out> {
    // 2016 Day 15's part 2 is nearly the same puzzle.
    // Instead of only one safe moment per layer, only one is UNSAFE.
    // That difference makes the original solution not make
    // any sense, and instead we end up doing brute force checks.

    // The trick, then, is to reduce the search space and make
    // the checks as cheap as we can.

    // Sorting so that the chunk_by below can do its thing
    let layers: Vec<(i64, i64)> = initial_layers
      .iter()
      .map(|&(a, _, b)| (a, b))
      .sorted_by(|(_, ap), (_, bp)| ap.cmp(bp))
      .collect();

    // Group offsets by period, giving us a list of periods with lists
    // of "banned" remainders. Each item is transformed to the congruence
    // that would cause it to trigger.
    let mut banned_congruences: Vec<(i64, Vec<i64>)> = layers
      .iter()
      .chunk_by(|&(_, p)| p)
      .into_iter()
      .map(|(period, chunk)| {
        (
          *period,
          chunk
            .map(|(offset, _)| (period - (offset % period)) % period)
            .sorted()
            .collect(),
        )
      })
      .collect();

    // Combine banned congruences whose divisors divide each other.
    // This absorbs smaller periods into larger ones and simplifies the
    // search space by another 50%.
    let periods: Vec<i64> = banned_congruences.iter().map(|&(p, _)| p).collect();
    for (i, &p1) in periods.iter().enumerate() {
      let expand_into: Vec<i64> = periods[(i + 1)..]
        .iter()
        .filter(|&&x| x % p1 == 0)
        .copied()
        .collect();
      if !expand_into.is_empty() {
        let (j, _) = &banned_congruences
          .iter()
          .enumerate()
          .find(|(_, (p2, _))| *p2 == p1)
          .unwrap();
        let (_, banned) = banned_congruences.remove(*j);
        for p3 in expand_into {
          let (_, insert_into) = banned_congruences
            .iter_mut()
            .find(|(p4, _)| *p4 == p3)
            .unwrap();
          for p5 in (0_i64..=p3).filter(|&p6| banned.contains(&(p6 % p1))) {
            insert_into.push(p5);
          }
        }
      }
    }

    // Now sort the banned remainder lists and make them unique for the next bit
    for (_, remainders) in banned_congruences.iter_mut() {
      *remainders = remainders.iter().unique().sorted().copied().collect();
    }

    // Now for the real magic trick: look for groups that only have one hole.
    // Those can use what I did in 2016 Day 15 Part 2 to create one huge
    // congruence offset + period that must be met. Using that, the brute
    // force search space becomes only a handful of values instead of millions.
    // Runtime without doing this: about 23 milliseconds.
    // Runtime WITH doing this: ???

    let (simple, complex): (Vec<_>, Vec<_>) = banned_congruences
      .iter()
      .partition(|(p, r)| (r.len() as i64) == *p - 1);

    // Using the Chinese Remainder Theorem, combine the simple congruences
    // into one huge one
    let (base, period) = simple.iter().sorted_by(|(p1, _), (p2, _)| p2.cmp(p1)).fold(
      (0, 1),
      |(acc_base, acc_period), (p, r)| {
        let allowed_remainder = (0..=*p).find(|s| !r.contains(s)).unwrap();
        let mut new_base = acc_base;
        while new_base % p != allowed_remainder {
          new_base += acc_period;
        }
        (new_base, num::integer::lcm(acc_period, *p))
      },
    );

    // All of the work above is just to make this check fewer values.
    // The inner loop would be the same if we just ... didn't do any of it.
    // The difference, however, is a 50ms runtime vs a 23µs runtime.
    // Worth it!
    for delay in (base..i64::MAX).step_by(period as usize) {
      if complex
        .iter()
        .all(|(period, bans)| !bans.contains(&(delay % *period)))
      {
        return Ok(delay);
      }
    }
    bail!("No solution found");
  }
}

fn main() -> Result<()> {
  Solver {}.run(2017, 13)
}
