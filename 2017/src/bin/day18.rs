use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = u64;
type P2Out = u64;
type Parsed = [u64; 4];

const RNG_POOL: usize = 127;
const DIVISOR: u64 = 0x7FFFFFFF;

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, sample_name: Option<String>, _: PartId) -> Result<Parsed> {
    if sample_name.is_some() {
      return Ok([0; 4]);
    }
    // The values we care about are at the end of specific lines.
    // Everything else appears to be the same for everyone's input.
    let read = |n: usize| lines[n].split_whitespace().last().unwrap().parse().unwrap();
    Ok([read(9), read(10), read(12), read(13)])
  }

  fn part1(&self, secrets: &Parsed, sample_name: Option<String>) -> Result<P1Out> {
    if sample_name.is_some() {
      return Ok(1);
    }

    // The program starts by using this method to generate RNG_POOL random integers, and queue
    // them up those values modulo 10_000.
    // The first thing it calls `rcv` on is the last of those values generated, so all
    // part 1 has to do is generate them and return the final one modulo 10_000.
    let &[seed, factor_1, factor_2, constant] = secrets;

    let mut state = seed;
    for _ in 0..RNG_POOL {
      state = (state * factor_1) % DIVISOR;
      state = ((state * factor_2) + constant) % DIVISOR;
    }

    Ok(state % 10_000)
  }

  fn part2(&self, secrets: &Parsed, sample_name: Option<String>) -> Result<P2Out> {
    // The input program is secretly the first version of bubble sort CS students learn:
    // - Generate RNG_POOL pseudo-random numbers in 0..=9999
    // - Loop through its indices, swapping an element with
    //   the next one if the next one is greater
    // - Repeat this procedure until no swaps happen.
    //   Each repetition is done alternating between the two programs,
    //   but they're just passing control back and forth.
    // - The number of times we repeat this process, multiplied
    //   by the length of the list and divided by two, is the answer
    //   (divide by two because it only wants what program B sent).

    if sample_name.is_some() {
      return Ok(1);
    }
    let &[seed, factor_1, factor_2, constant] = secrets;

    let mut list: Vec<u64> = Vec::with_capacity(RNG_POOL);

    let mut state = seed;
    for _ in 0..RNG_POOL {
      state = (state * factor_1) % DIVISOR;
      state = ((state * factor_2) + constant) % DIVISOR;
      list.push(state % 10_000);
    }

    let mut sorting_iterations = 1;

    loop {
      let mut changed = false;
      for i in 0..(RNG_POOL - 1) {
        if list[i + 1] > list[i] {
          changed = true;
          list.swap(i, i + 1);
        }
      }
      sorting_iterations += 1;
      if !changed {
        break;
      }
    }

    Ok(sorting_iterations * (RNG_POOL as u64) / 2)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2017, 18)
}
