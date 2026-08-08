use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = u32;
type P2Out = u32;

// The input is a 5x5 grid, and part 1's output strongly hints that
// it'd be smart to cram that into a u32, with the top-left cell
// being bit 0 (so it's backwards).
#[derive(Copy, Clone)]
struct State(u32);
impl State {
  fn is_bug(&self, x: usize, y: usize) -> bool {
    assert!(x < 5 && y < 5, "is_bug only works in the grid spaces");
    let addr = (y * 5) + x;
    (0b1 & (self.0 >> addr)) == 1
  }

  fn adjacent_bugs(&self, x: usize, y: usize) -> usize {
    let mut adj = 0;
    if x > 0 && self.is_bug(x - 1, y) {
      adj += 1;
    }
    if x < 4 && self.is_bug(x + 1, y) {
      adj += 1;
    }
    if y > 0 && self.is_bug(x, y - 1) {
      adj += 1;
    }
    if y < 4 && self.is_bug(x, y + 1) {
      adj += 1;
    }
    adj
  }

  fn step(&self) -> State {
    let mut out = 0;
    for y in 0..5 {
      for x in 0..5 {
        let adj = self.adjacent_bugs(x, y);
        let offset = (y * 5) + x;
        if self.is_bug(x, y) {
          out |= if adj == 1 { 0b1 << offset } else { 0 };
        } else {
          out |= if adj == 1 || adj == 2 {
            0b1 << offset
          } else {
            0
          };
        }
      }
    }
    State(out)
  }
}

// Somwhat trivial: each # is a 1, . is a 0, the binary representation is backwards.
// That is: top-left cell is bit 0, bottom-right is bit 24. Cram it all into a u32.
impl From<&str> for State {
  fn from(value: &str) -> Self {
    State(
      value
        .chars()
        .enumerate()
        .fold(0_u32, |a, (i, c)| if c == '#' { a | (0b1 << i) } else { a }),
    )
  }
}

// Turning it back into a display value is the same deal, just backwards.
impl std::fmt::Display for State {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let out = (0..5)
      .map(|y| {
        (0..5)
          .map(|x| {
            if self.is_bug(x, y) {
              "#".to_string()
            } else {
              ".".to_string()
            }
          })
          .collect::<Vec<String>>()
          .join("")
      })
      .collect::<Vec<String>>()
      .join("\n");

    write!(f, "{}", out)
  }
}

struct Solver;
impl Day<State, P1Out, P2Out> for Solver {
  fn parse(&self, raw: Vec<String>, _: Option<String>, _: PartId) -> Result<State> {
    Ok(State::from(raw.join("").as_str()))
  }

  fn part1(&self, init: &State, _: Option<String>) -> Result<P1Out> {
    let mut seen: Vec<u32> = vec![];
    let mut state = init.to_owned();
    while !seen.contains(&state.0) {
      seen.push(state.0);
      state = state.step();
    }
    Ok(state.0)
  }

  fn part2(&self, _state: &State, _: Option<String>) -> Result<P2Out> {
    Ok(0)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2019, 24)
}
