use advent_lib::runner::{Day, PartId};
use anyhow::Result;
use fnv::{FnvBuildHasher, FnvHashMap};

type P1Out = u32;
type P2Out = u32;

// The input is a 5x5 grid, and part 1's output strongly hints that
// it'd be smart to cram that into a u32, with the top-left cell
// being bit 0 (so it's backwards).
#[derive(Copy, Clone)]
struct State {
  value: u32,
  depth: i32,
}

impl State {
  fn count_bugs(&self) -> u32 {
    self.value.count_ones()
  }

  fn is_bug(&self, x: usize, y: usize) -> bool {
    assert!(x < 5 && y < 5, "is_bug only works in the grid spaces");
    let addr = (y * 5) + x;
    (0b1 & (self.value >> addr)) == 1
  }

  fn adjacent_bugs_p1(&self, x: usize, y: usize) -> usize {
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

  fn step_p1(&self) -> State {
    let mut out = 0;
    for y in 0..5 {
      for x in 0..5 {
        let adj = self.adjacent_bugs_p1(x, y);
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
    State {
      value: out,
      depth: self.depth,
    }
  }

  fn adjacent_bugs_p2(&self, x: usize, y: usize, stack: &FnvHashMap<i32, State>) -> usize {
    assert!(
      !(x == 2 && y == 2),
      "adjacent_bugs_p2 should never be asked to look at the center"
    );
    let mut adj = 0;

    // The center cell is a layer deeper, so we count all 1's on the neighboring edge if
    // that cell is the center. Similarly, if we would look outside of this grid,
    // we consider one layer outside and the corresponding neighbor cell we're touching.
    // Outside of those cases, the rules are the same.

    // First, the old/simple rules with the special cases omitted
    if x > 0 && !(x == 3 && y == 2) && self.is_bug(x - 1, y) {
      adj += 1;
    }
    if x < 4 && !(x == 1 && y == 2) && self.is_bug(x + 1, y) {
      adj += 1;
    }
    if y > 0 && !(y == 3 && x == 2) && self.is_bug(x, y - 1) {
      adj += 1;
    }
    if y < 4 && !(y == 1 && x == 2) && self.is_bug(x, y + 1) {
      adj += 1;
    }

    // Now, the outbound rules (looking towards an outside layer)
    let default_outer = State {
      value: 0,
      depth: self.depth - 1,
    };
    let outer_layer = stack.get(&(self.depth - 1)).unwrap_or(&default_outer);
    if x == 0 && outer_layer.is_bug(1, 2) {
      adj += 1;
    }
    if x == 4 && outer_layer.is_bug(3, 2) {
      adj += 1;
    }
    if y == 0 && outer_layer.is_bug(2, 1) {
      adj += 1;
    }
    if y == 4 && outer_layer.is_bug(2, 3) {
      adj += 1;
    }

    // Finally, the inbound rules (looking towards this grid's center)
    let default_inner = State {
      value: 0,
      depth: self.depth + 1,
    };
    let inner_layer = stack.get(&(self.depth + 1)).unwrap_or(&default_inner);
    if x == 1 && y == 2 {
      // Look at the left edge of the deeper layer
      adj += (0..5).fold(0, |a, y2| a + if inner_layer.is_bug(0, y2) { 1 } else { 0 });
    }
    if x == 3 && y == 2 {
      // Right edge
      adj += (0..5).fold(0, |a, y2| a + if inner_layer.is_bug(4, y2) { 1 } else { 0 });
    }
    if x == 2 && y == 1 {
      // Top edge
      adj += (0..5).fold(0, |a, x2| a + if inner_layer.is_bug(x2, 0) { 1 } else { 0 });
    }
    if x == 2 && y == 3 {
      // Bottom edge
      adj += (0..5).fold(0, |a, x2| a + if inner_layer.is_bug(x2, 4) { 1 } else { 0 });
    }

    adj
  }

  fn step_p2(&self, stack: &FnvHashMap<i32, State>) -> State {
    let mut out = 0;
    for y in 0..5 {
      for x in 0..5 {
        if x == 2 && y == 2 {
          // This bit is always 0 in each map as it goes to a different layer.
          continue;
        }
        let adj = self.adjacent_bugs_p2(x, y, stack);
        // The rest of the rules are the same as part 1.
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
    State {
      value: out,
      depth: self.depth,
    }
  }
}

// Somwhat trivial: each # is a 1, . is a 0, the binary representation is backwards.
// That is: top-left cell is bit 0, bottom-right is bit 24. Cram it all into a u32.
impl From<&str> for State {
  fn from(value: &str) -> Self {
    State {
      value: value
        .chars()
        .enumerate()
        .fold(0_u32, |a, (i, c)| if c == '#' { a | (0b1 << i) } else { a }),
      depth: 0,
    }
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
    // Yes, scanning a retained array of unbounded size, but it only takes a microsecond
    // on the real input and even less on the sample, so it's fine.
    let mut seen: Vec<u32> = vec![];
    let mut state = init.to_owned();
    while !seen.contains(&state.value) {
      seen.push(state.value);
      state = state.step_p1();
    }
    Ok(state.value)
  }

  fn part2(&self, init: &State, sample_name: Option<String>) -> Result<P2Out> {
    let iteration_limit = if sample_name.is_some() { 10 } else { 200 };

    let mut min_depth = 0;
    let mut max_depth = 0;
    let mut stack =
      FnvHashMap::<i32, State>::with_capacity_and_hasher(11, FnvBuildHasher::default());
    stack.insert(0, init.to_owned());

    for _ in 0..iteration_limit {
      let mut next_stack = stack.clone();
      min_depth -= 1;
      max_depth += 1;
      for depth in min_depth..=max_depth {
        let default = State { value: 0, depth };
        let state = stack.get(&depth).unwrap_or(&default);
        next_stack.insert(depth, state.step_p2(&stack));
      }

      // Optimization: shrink the boundary in away from all-0 states.
      // This cuts runtime in half on the real data, so pretty important.
      while let Some(State {
        value: outer_value, ..
      }) = next_stack.get(&min_depth)
        && *outer_value == 0
      {
        next_stack.remove(&min_depth);
        min_depth += 1;
      }
      while let Some(State {
        value: inner_value, ..
      }) = next_stack.get(&max_depth)
        && *inner_value == 0
      {
        next_stack.remove(&max_depth);
        max_depth -= 1;
      }

      // Apply the changes
      stack = next_stack;
    }

    Ok(
      stack
        .iter()
        .fold(0, |acc, (_depth, layer)| acc + layer.count_bugs()),
    )
  }
}

fn main() -> Result<()> {
  Solver {}.run(2019, 24)
}
