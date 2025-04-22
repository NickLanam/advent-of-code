use advent_lib::runner::{Day, PartId};
use anyhow::{Context, Result};
use fnv::{FnvBuildHasher, FnvHashMap};

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum CellKind {
  Two,
  Three,
  Four,
}

/// A cell is just a bunch of true/false flags, so
/// a bitfield with a size marker seems the best
/// way to deal with it and make transforms fast.
#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Cell {
  kind: CellKind,
  value: u16,
}
impl Cell {
  /// Util for parsing, to find all possible rules
  /// Can create duplicates for many configurations,
  /// but they end up in a map that eliminates dupes.
  fn transformations(&self) -> Vec<Cell> {
    let mut out = vec![*self];

    // Manual way - fast but verbose as heck
    match self.kind {
      CellKind::Two => {
        let (a, b, c, d) = (
          (self.value >> 3) & 0x1,
          (self.value >> 2) & 0x1,
          (self.value >> 1) & 0x1,
          self.value & 0x1,
        );
        for next in [
          // Rot90, Rot180, Rot270
          (c, a, d, b),
          (d, c, b, a),
          (b, d, a, c),
          // Mirror, and its rotations
          (b, a, d, c),
          (d, b, c, a),
          (c, d, a, b),
          (a, c, b, d),
        ] {
          let next_value: u16 = (next.0 << 3) + (next.1 << 2) + (next.2 << 1) + next.3;
          out.push(Cell {
            kind: CellKind::Two,
            value: next_value,
          });
        }
      }
      CellKind::Three => {
        let (a, b, c, d, e, f, g, h, i) = (
          (self.value >> 8) & 0x1,
          (self.value >> 7) & 0x1,
          (self.value >> 6) & 0x1,
          (self.value >> 5) & 0x1,
          (self.value >> 4) & 0x1,
          (self.value >> 3) & 0x1,
          (self.value >> 2) & 0x1,
          (self.value >> 1) & 0x1,
          self.value & 0x1,
        );
        for next in [
          // Rot90, Rot180, Rot270
          (g, d, a, h, e, b, i, f, c),
          (i, h, g, f, e, d, c, b, a),
          (c, f, i, b, e, h, a, d, g),
          // Mirror, and its rotations
          (c, b, a, f, e, d, i, h, g),
          (i, f, c, h, e, b, g, d, a),
          (g, h, i, d, e, f, a, b, c),
          (a, d, g, b, e, h, c, f, i),
        ] {
          let next_value: u16 = (next.0 << 8)
            + (next.1 << 7)
            + (next.2 << 6)
            + (next.3 << 5)
            + (next.4 << 4)
            + (next.5 << 3)
            + (next.6 << 2)
            + (next.7 << 1)
            + next.8;
          out.push(Cell {
            kind: CellKind::Three,
            value: next_value,
          });
        }
      }
      CellKind::Four => {
        let (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p) = (
          (self.value >> 15) & 0x1,
          (self.value >> 14) & 0x1,
          (self.value >> 13) & 0x1,
          (self.value >> 12) & 0x1,
          (self.value >> 11) & 0x1,
          (self.value >> 10) & 0x1,
          (self.value >> 9) & 0x1,
          (self.value >> 8) & 0x1,
          (self.value >> 7) & 0x1,
          (self.value >> 6) & 0x1,
          (self.value >> 5) & 0x1,
          (self.value >> 4) & 0x1,
          (self.value >> 3) & 0x1,
          (self.value >> 2) & 0x1,
          (self.value >> 1) & 0x1,
          self.value & 0x1,
        );
        for next in [
          // Rot90, Rot180, Rot270
          (m, i, e, a, n, j, f, b, o, k, g, c, p, l, h, d),
          (p, o, n, m, l, k, j, i, h, g, f, e, d, c, b, a),
          (d, h, l, p, c, g, k, o, b, f, j, n, a, e, i, m),
          // Mirror, and its rotations
          (d, c, b, a, h, g, f, e, l, k, j, i, p, o, n, m),
          (p, l, h, d, o, k, g, c, n, j, f, b, m, i, e, a),
          (m, n, o, p, i, j, k, l, e, f, g, h, a, b, c, d),
          (a, e, i, m, b, f, j, n, c, g, k, o, d, h, l, p),
        ] {
          let next_value: u16 = (next.0 << 15)
            + (next.1 << 14)
            + (next.2 << 13)
            + (next.3 << 12)
            + (next.4 << 11)
            + (next.5 << 10)
            + (next.6 << 9)
            + (next.7 << 8)
            + (next.8 << 7)
            + (next.9 << 6)
            + (next.10 << 5)
            + (next.11 << 4)
            + (next.12 << 3)
            + (next.13 << 2)
            + (next.14 << 1)
            + next.15;
          out.push(Cell {
            kind: CellKind::Four,
            value: next_value,
          });
        }
      }
    }
    out
  }
}

type P1Out = usize;
type P2Out = usize;
type Parsed = FnvHashMap<Cell, Vec<Cell>>;

// Util for parse, when building the 4x4 -> 6x6 rules
fn four_to_twos(four: Cell) -> [Cell; 4] {
  if four.kind != CellKind::Four {
    panic!("Do not try to four_to_two a cell that isn't 4x4");
  }
  // Break the 4x4 into four 2x2's - which we'll save for later when we make the 4x4 -> 6x6 rule
  [
    Cell {
      kind: CellKind::Two,
      value: (((four.value >> 15) & 0x1) << 3)
        + (((four.value >> 14) & 0x1) << 2)
        + (((four.value >> 11) & 0x1) << 1)
        + ((four.value >> 10) & 0x1),
    },
    Cell {
      kind: CellKind::Two,
      value: (((four.value >> 13) & 0x1) << 3)
        + (((four.value >> 12) & 0x1) << 2)
        + (((four.value >> 9) & 0x1) << 1)
        + ((four.value >> 8) & 0x1),
    },
    Cell {
      kind: CellKind::Two,
      value: (((four.value >> 7) & 0x1) << 3)
        + (((four.value >> 6) & 0x1) << 2)
        + (((four.value >> 3) & 0x1) << 1)
        + ((four.value >> 2) & 0x1),
    },
    Cell {
      kind: CellKind::Two,
      value: (((four.value >> 5) & 0x1) << 3)
        + (((four.value >> 4) & 0x1) << 2)
        + (((four.value >> 1) & 0x1) << 1)
        + (four.value & 0x1),
    },
  ]
}

// Another util for parse, to create rules that turn 4x4 -> [2x2; 9] instead of 4x4 -> [3x3; 4]
fn create_four_rule(four: Cell, rules: &Parsed) -> Vec<Cell> {
  // Four 2x2 cells
  let twos = four_to_twos(four);

  // Transform them into four 3x3 cells.
  // Assumes it only sees single 2x2 -> single 3x3 rules
  let threes = twos.map(|two| rules.get(&two).unwrap()[0]);

  // Transform those four 3x3 cells into nine 2x2 cells, and return that
  // Very similar logic to four_to_twos here.
  vec![
    // Top-left
    Cell {
      kind: CellKind::Two,
      value: (((threes[0].value >> 8) & 0x1) << 3)
        + (((threes[0].value >> 7) & 0x1) << 2)
        + (((threes[0].value >> 5) & 0x1) << 1)
        + ((threes[0].value >> 4) & 0x1),
    },
    // Top-center
    Cell {
      kind: CellKind::Two,
      value: (((threes[0].value >> 6) & 0x1) << 3)
        + (((threes[1].value >> 8) & 0x1) << 2)
        + (((threes[0].value >> 3) & 0x1) << 1)
        + ((threes[1].value >> 5) & 0x1),
    },
    // Top-right
    Cell {
      kind: CellKind::Two,
      value: (((threes[1].value >> 7) & 0x1) << 3)
        + (((threes[1].value >> 6) & 0x1) << 2)
        + (((threes[1].value >> 4) & 0x1) << 1)
        + ((threes[1].value >> 3) & 0x1),
    },
    // Center-left
    Cell {
      kind: CellKind::Two,
      value: (((threes[0].value >> 2) & 0x1) << 3)
        + (((threes[0].value >> 1) & 0x1) << 2)
        + (((threes[2].value >> 8) & 0x1) << 1)
        + ((threes[2].value >> 7) & 0x1),
    },
    // Center-center
    Cell {
      kind: CellKind::Two,
      value: ((threes[0].value & 0x1) << 3)
        + (((threes[1].value >> 2) & 0x1) << 2)
        + (((threes[2].value >> 6) & 0x1) << 1)
        + ((threes[3].value >> 8) & 0x1),
    },
    // Center-right
    Cell {
      kind: CellKind::Two,
      value: (((threes[1].value >> 1) & 0x1) << 3)
        + ((threes[1].value & 0x1) << 2)
        + (((threes[3].value >> 7) & 0x1) << 1)
        + ((threes[3].value >> 6) & 0x1),
    },
    // Bottom-left
    Cell {
      kind: CellKind::Two,
      value: (((threes[2].value >> 5) & 0x1) << 3)
        + (((threes[2].value >> 4) & 0x1) << 2)
        + (((threes[2].value >> 2) & 0x1) << 1)
        + ((threes[2].value >> 1) & 0x1),
    },
    // Bottom-center
    Cell {
      kind: CellKind::Two,
      value: (((threes[2].value >> 3) & 0x1) << 3)
        + (((threes[3].value >> 5) & 0x1) << 2)
        + ((threes[2].value & 0x1) << 1)
        + ((threes[3].value >> 2) & 0x1),
    },
    // Bottom-right
    Cell {
      kind: CellKind::Two,
      value: (((threes[3].value >> 4) & 0x1) << 3)
        + (((threes[3].value >> 3) & 0x1) << 2)
        + (((threes[3].value >> 1) & 0x1) << 1)
        + (threes[3].value & 0x1),
    },
  ]
}

fn solve(rules: &Parsed, iterations: usize) -> Result<usize> {
  let mut state: FnvHashMap<Cell, usize> = FnvHashMap::with_hasher(FnvBuildHasher::default());
  state.insert(
    Cell {
      kind: CellKind::Three,
      value: 0b_010_001_111,
    },
    1,
  );

  // Apply the transformations, updating the map each time.
  for _ in 0..iterations {
    let mut next_state: FnvHashMap<Cell, usize> =
      FnvHashMap::with_hasher(FnvBuildHasher::default());
    for (cell, &count) in state.iter() {
      let next_values = rules.get(cell).context("Unable to match cell transform")?;
      for n in next_values {
        let entry = next_state.entry(*n).or_insert(0);
        *entry += count;
      }
    }
    state = next_state;
  }

  // Puzzle answer wants how many pixels are lit in the final grid.
  // Count the ones in each state, multiply by occurrences, sum, done.
  Ok(
    state
      .iter()
      .map(|(Cell { value, .. }, count)| (value.count_ones() as usize) * count)
      .sum(),
  )
}

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    let mut rules: FnvHashMap<Cell, Vec<Cell>> =
      FnvHashMap::with_capacity_and_hasher(lines.len() * 4, FnvBuildHasher::default());

    for line in lines {
      let (a, b) = line.split_once(" => ").context("")?;
      let mut a_val: u16 = 0;
      for a_ch in a.chars() {
        match a_ch {
          '.' => {
            a_val <<= 1;
          }
          '#' => {
            a_val <<= 1;
            a_val += 1;
          }
          _ => { /* Slash, do nothing */ }
        }
      }
      let mut b_val: u16 = 0;
      for b_ch in b.chars() {
        match b_ch {
          '.' => {
            b_val <<= 1;
          }
          '#' => {
            b_val <<= 1;
            b_val += 1;
          }
          _ => { /* Slash, do nothing */ }
        }
      }
      match a.len() {
        5 => {
          // It's a 2x2 -> 3x3 rule
          let a_cell = Cell {
            kind: CellKind::Two,
            value: a_val,
          };
          let b_cell = Cell {
            kind: CellKind::Three,
            value: b_val,
          };
          for t in a_cell.transformations() {
            rules.insert(t, vec![b_cell]);
          }
        }
        11 => {
          // It's a 3x3 -> 4x4 rule.
          // We'll also build the 4x4 -> 6x6 rule from here.
          let a_cell = Cell {
            kind: CellKind::Three,
            value: a_val,
          };
          // The transform rule creates this 4x4, but the 2x2 subgrids for it
          // are computed as well so we can later create a 4x4 -> 6x6 rule.
          // That 6x6 is made of 2x2, instead of by 3x3, because the puzzle says so.
          let b_cell = Cell {
            kind: CellKind::Four,
            value: b_val,
          };
          for t in a_cell.transformations() {
            rules.insert(t, vec![b_cell]);
          }

          // At this point, since the input puts all of the 2x2 rules before the 3x3 ones,
          // we already know all of the 2x2 rules. We can use this, and the 2x2 subgrids we
          // just made, to also make a 4x4 -> 6x6 rule (as nine 2x2's instead of as four 3x3's).
          for b_t in b_cell.transformations() {
            rules.insert(b_t, create_four_rule(b_cell, &rules));
          }
        }
        _ => {
          // Doesn't happen on a (does on b)
          panic!("Impossible rule line: {line}");
        }
      }
    }

    Ok(rules)
  }

  fn part1(&self, rules: &Parsed, sample_name: Option<String>) -> Result<P1Out> {
    let iterations = if sample_name.is_some() { 2 } else { 5 };
    solve(rules, iterations)
  }

  fn part2(&self, rules: &Parsed, sample_name: Option<String>) -> Result<P2Out> {
    let iterations = if sample_name.is_some() { 2 } else { 18 };
    solve(rules, iterations)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2017, 21)
}
