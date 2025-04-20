use advent_lib::runner::{Day, PartId};
use anyhow::{Context, Result};
use fnv::{FnvBuildHasher, FnvHashMap};

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum CellKind {
  Two,
  Three,
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
    match self.kind {
      CellKind::Two => {
        let (a, b, c, d) = (
          (self.value >> 3) & 0x1,
          (self.value >> 2) & 0x1,
          (self.value >> 1) & 0x1,
          self.value & 0x1,
        );
        // Rotate 90, 180, and 270
        out.push(Cell {
          kind: CellKind::Two,
          value: (c << 3) + (a << 2) + (d << 1) + b,
        });
        out.push(Cell {
          kind: CellKind::Two,
          value: (d << 3) + (c << 2) + (b << 1) + a,
        });
        out.push(Cell {
          kind: CellKind::Two,
          value: (b << 3) + (d << 2) + (a << 1) + c,
        });
        // Horizontal mirror and its rotations
        out.push(Cell {
          kind: CellKind::Two,
          value: (b << 3) + (a << 2) + (d << 1) + c,
        });
        out.push(Cell {
          kind: CellKind::Two,
          value: (d << 3) + (b << 2) + (c << 1) + a,
        });
        out.push(Cell {
          kind: CellKind::Two,
          value: (c << 3) + (d << 2) + (a << 1) + b,
        });
        out.push(Cell {
          kind: CellKind::Two,
          value: (a << 3) + (c << 2) + (b << 1) + d,
        });
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
        // Rotate 90, 180, and 270
        out.push(Cell {
          kind: CellKind::Three,
          value: (g << 8)
            + (d << 7)
            + (a << 6)
            + (h << 5)
            + (e << 4)
            + (b << 3)
            + (i << 2)
            + (f << 1)
            + c,
        });
        out.push(Cell {
          kind: CellKind::Three,
          value: (i << 8)
            + (h << 7)
            + (g << 6)
            + (f << 5)
            + (e << 4)
            + (d << 3)
            + (c << 2)
            + (b << 1)
            + a,
        });
        out.push(Cell {
          kind: CellKind::Three,
          value: (c << 8)
            + (f << 7)
            + (i << 6)
            + (b << 5)
            + (e << 4)
            + (h << 3)
            + (a << 2)
            + (d << 1)
            + g,
        });
        // Horizontal mirror and its rotations
        out.push(Cell {
          kind: CellKind::Three,
          value: (c << 8)
            + (b << 7)
            + (a << 6)
            + (f << 5)
            + (e << 4)
            + (d << 3)
            + (i << 2)
            + (h << 1)
            + g,
        });
        out.push(Cell {
          kind: CellKind::Three,
          value: (i << 8)
            + (f << 7)
            + (c << 6)
            + (h << 5)
            + (e << 4)
            + (b << 3)
            + (g << 2)
            + (d << 1)
            + a,
        });
        out.push(Cell {
          kind: CellKind::Three,
          value: (g << 8)
            + (h << 7)
            + (i << 6)
            + (d << 5)
            + (e << 4)
            + (f << 3)
            + (a << 2)
            + (b << 1)
            + c,
        });
        out.push(Cell {
          kind: CellKind::Three,
          value: (a << 8)
            + (d << 7)
            + (g << 6)
            + (b << 5)
            + (e << 4)
            + (h << 3)
            + (c << 2)
            + (f << 1)
            + i,
        });
      }
    }
    out
  }
}

// TODO: Remove when done testing
impl std::fmt::Debug for Cell {
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self.kind {
      CellKind::Two => {
        let (a, b, c, d) = (
          (self.value >> 3) & 0x1 != 0,
          (self.value >> 2) & 0x1 != 0,
          (self.value >> 1) & 0x1 != 0,
          self.value & 0x1 != 0,
        );
        write!(
          formatter,
          "{}{} / {}{}",
          if a { '#' } else { '.' },
          if b { '#' } else { '.' },
          if c { '#' } else { '.' },
          if d { '#' } else { '.' }
        )
      }
      CellKind::Three => {
        let (a, b, c, d, e, f, g, h, i) = (
          (self.value >> 8) & 0x1 != 0,
          (self.value >> 7) & 0x1 != 0,
          (self.value >> 6) & 0x1 != 0,
          (self.value >> 5) & 0x1 != 0,
          (self.value >> 4) & 0x1 != 0,
          (self.value >> 3) & 0x1 != 0,
          (self.value >> 2) & 0x1 != 0,
          (self.value >> 1) & 0x1 != 0,
          self.value & 0x1 != 0,
        );
        write!(
          formatter,
          "{}{}{} / {}{}{} / {}{}{}",
          if a { '#' } else { '.' },
          if b { '#' } else { '.' },
          if c { '#' } else { '.' },
          if d { '#' } else { '.' },
          if e { '#' } else { '.' },
          if f { '#' } else { '.' },
          if g { '#' } else { '.' },
          if h { '#' } else { '.' },
          if i { '#' } else { '.' }
        )
      }
    }
  }
}

type P1Out = usize;
type P2Out = usize;
type Parsed = FnvHashMap<Cell, Vec<Cell>>;

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
      for a_c in a.chars() {
        match a_c {
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
      match a.len() {
        5 => {
          // It's a 2x2 -> 3x3 rule
          let a_cell = Cell {
            kind: CellKind::Two,
            value: a_val,
          };
          let mut b_val: u16 = 0;
          for b_c in b.chars() {
            match b_c {
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
          let b_cell = Cell {
            kind: CellKind::Three,
            value: b_val,
          };
          for t in a_cell.transformations() {
            rules.insert(t, vec![b_cell]);
          }
        }
        11 => {
          // It's a 3x3 -> 4x4 rule, which we turn into four 2x2's.
          // Note for state transform code later: some of the 2x2's might be
          // identical, so be careful with count increments!
          let a_cell = Cell {
            kind: CellKind::Three,
            value: a_val,
          };
          // Break the 4x4 into four 2x2's
          let (mut b_cell, mut c_cell, mut d_cell, mut e_cell) = (
            Cell {
              kind: CellKind::Two,
              value: 0,
            },
            Cell {
              kind: CellKind::Two,
              value: 0,
            },
            Cell {
              kind: CellKind::Two,
              value: 0,
            },
            Cell {
              kind: CellKind::Two,
              value: 0,
            },
          );
          for (i, ch) in b.chars().filter(|&it| it != '/').enumerate() {
            let cell = match i {
              0 | 1 | 4 | 5 => &mut b_cell,
              2 | 3 | 6 | 7 => &mut c_cell,
              8 | 9 | 12 | 13 => &mut d_cell,
              10 | 11 | 14 | 15 => &mut e_cell,
              _ => {
                panic!("That isn't 4x4, {line}");
              }
            };
            match ch {
              '.' => {
                cell.value <<= 1;
              }
              '#' => {
                cell.value <<= 1;
                cell.value += 1;
              }
              _ => {
                panic!("Line has bad chars in it: {line}");
              }
            }
          }
          for t in a_cell.transformations() {
            rules.insert(t, vec![b_cell, c_cell, d_cell, e_cell]);
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
    let mut state: FnvHashMap<Cell, usize> = FnvHashMap::with_hasher(FnvBuildHasher::default());
    state.insert(
      Cell {
        kind: CellKind::Three,
        value: 0b_010_001_111,
      },
      1,
    );

    // TODO: Logic is wrong here! The 2x2 rules are checked before the 3x3
    // rules, in the puzzle description, and sets of 3 iterations turn a 3x3 into
    // a 9x9 grid. 3x3 -> 4x4 -> 6x6 -> 9x9, NOT 8x8 as my code does now.

    // The optimization I did to turn 4x4 into a wad of 2x2 was, thus, not right.
    // Have to actually keep the 4x4 together and find a rule that turns it into
    // nine 2x2s rather than four 3x3s. That's creating rules out of thin air.
    // And matching on them!

    println!("START:\n\n{state:#?}\n");
    // Apply the transformations, updating the map each time.
    let iterations = if sample_name.is_some() { 2 } else { 5 };
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
      println!("{state:#?}\n");
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

  fn part2(&self, _lines: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    Ok(0)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2017, 21)
}
