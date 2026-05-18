use advent_lib::runner::{Day, PartId};
use anyhow::{Result, bail};

type P1Out = u128;
type P2Out = u128;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Action {
  ERROR(String),
  REVERSE(),
  CUT(i32),
  INC(i32),
}
// Size of deck, list of techniques
type Parsed = (u128, Vec<Action>);

// Computes (a*x+b)**m.div_euclid(n), but with the pow2 exponentiation trick so it all fits
fn modular_exponentiate(a: i128, b: i128, m: i128, n: i128) -> (i128, i128) {
  if m == 0 {
    (1, 0)
  } else if m % 2 == 0 {
    modular_exponentiate((a * a) % n, ((a * b) + b).rem_euclid(n), m / 2, n)
  } else {
    let (c, d) = modular_exponentiate(a, b, m - 1, n);
    ((a * c).rem_euclid(n), ((a * d) + b).rem_euclid(n))
  }
}

fn modular_inverse(a0: i128, m0: i128) -> i128 {
  if m0 == 1 {
    return 1;
  }

  let (mut a, mut m, mut x0, mut inv) = (a0, m0, 0, 1);

  while a > 1 {
    inv -= (a / m) * x0;
    a = a % m;
    std::mem::swap(&mut a, &mut m);
    std::mem::swap(&mut x0, &mut inv);
  }

  if inv < 0 {
    inv += m0
  }
  inv
}

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    sample_name: Option<String>,
    part_id: PartId,
  ) -> Result<Parsed> {
    Ok((
      match (sample_name, part_id) {
        (Some(_), _) => 10,
        (None, PartId::P1) => 10_007,
        (None, PartId::P2) => 119_315_717_514_047,
      },
      lines
        .iter()
        .map(|line| {
          if line.starts_with("deal into new stack") {
            Action::REVERSE()
          } else if line.starts_with("cut ") {
            Action::CUT(line.split_at("cut ".len()).1.parse::<i32>().unwrap())
          } else if line.starts_with("deal with increment ") {
            Action::INC(
              line
                .split_at("deal with increment ".len())
                .1
                .parse::<i32>()
                .unwrap(),
            )
          } else {
            Action::ERROR(line.to_string())
          }
        })
        .collect(),
    ))
  }

  fn part1(&self, (deck_size, actions): &Parsed, sample_name: Option<String>) -> Result<P1Out> {
    // We only need to track one card's position, so we don't need to generate (let alone shuffle) the deck.
    let mut pos = if sample_name.is_some() { 7 } else { 2019 };
    for action in actions.iter() {
      match action {
        Action::ERROR(cause) => bail!("{cause}"),
        Action::REVERSE() => {
          pos = deck_size - 1 - pos;
        }
        &Action::CUT(v) => {
          let n: u128 = if v > 0 {
            v as u128
          } else {
            deck_size - (v.abs() as u128)
          };

          if pos < n {
            pos += deck_size - n;
          } else {
            pos -= n;
          }
        }
        &Action::INC(n) => {
          // We deal the entire deck to positions that jump by n at a time, wrapping.
          // That's just a simple multiplication and then modulo operation.
          // Even with this division operation, part 1 runs in about 600 nanoseconds on a Ryzen 9 3900X.
          pos = ((n as u128) * pos) % *deck_size;
        }
      }
    }
    Ok(pos)
  }

  fn part2(&self, (deck_size, actions): &Parsed, _: Option<String>) -> Result<P2Out> {
    // We now track a position rather than a card, and do the full shuffle chain ~101 trillion times.
    // We do this on a deck with ~119 trillion cards, as well.
    // Part 1 runs in about 600ns, but with that many iterations it'd still run in O(heat_death) time.
    // This isn't a coding problem so much as a math problem, and involves undergraduate linear algebra.

    // Step 1: Turn the actions list into a single `y = Ax + B` polynomial.
    // We work backwards, since we want the card that ended up in a given position rather than the opposite.
    let mut a: i128 = 1;
    let mut b: i128 = 0;
    let len = *deck_size as i128;
    for action in actions.iter().rev() {
      match action {
        Action::ERROR(cause) => bail!("{cause}"),
        Action::REVERSE() => {
          a = -a;
          b = len - b - 1;
        }
        &Action::CUT(n) => {
          b = (b + (n as i128)).rem_euclid(len);
        }
        &Action::INC(n) => {
          let z = modular_inverse(n as i128, len);
          a = (a * z).rem_euclid(len);
          b = (b * z).rem_euclid(len);
        }
      }
    }

    // Step 2: Modular exponentiation on that polynomial to compute which card lands at the target position.
    let iterations: i128 = 101_741_582_076_661;
    (a, b) = modular_exponentiate(a, b, iterations, len);
    Ok(((2020_i128 * a) + b).rem_euclid(len) as u128)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2019, 22)
}
