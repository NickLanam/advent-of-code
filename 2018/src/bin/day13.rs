use advent_lib::{
  direction::{CardinalDirection, Rotation},
  grid::Infinite2dGrid,
  runner::{Day, PartId},
};
use anyhow::{Result, bail};
use fnv::FnvHashSet;

#[derive(Clone, PartialEq, Eq)]
enum RailKind {
  Horizontal,
  Vertical,
  Slash,
  Backslash,
  Intersection,
}

#[derive(Clone, PartialEq, Eq)]
enum TurnCycle {
  Left,
  Straight,
  Right,
}

#[derive(Clone, PartialEq, Eq)]
struct Cart {
  id: usize,
  x: i32,
  y: i32,
  d: CardinalDirection,
  m: TurnCycle,
}

type P1Out = String;
type P2Out = String;
type Parsed = (Vec<Cart>, Infinite2dGrid<RailKind>);

fn step(cart: &mut Cart, grid: &Infinite2dGrid<RailKind>) -> Result<()> {
  (cart.x, cart.y) = cart.d.apply(cart.x, cart.y, 1);
  match grid.get(cart.x, cart.y) {
    Some(RailKind::Horizontal) | Some(RailKind::Vertical) => {
      // Nothing to do here, we already moved forward.
    }
    Some(RailKind::Slash) => {
      // This points either south-east or north-west, depending on how we entered it
      match cart.d {
        CardinalDirection::N | CardinalDirection::S => {
          cart.d += Rotation::R;
        }
        _ => {
          cart.d += Rotation::L;
        }
      }
    }
    Some(RailKind::Backslash) => match cart.d {
      // Opposite behavior from Slash
      CardinalDirection::N | CardinalDirection::S => {
        cart.d += Rotation::L;
      }
      _ => {
        cart.d += Rotation::R;
      }
    },
    Some(RailKind::Intersection) => {
      // Rules directly from puzzle description
      match cart.m {
        TurnCycle::Left => {
          cart.d += Rotation::L;
          cart.m = TurnCycle::Straight;
        }
        TurnCycle::Straight => {
          cart.m = TurnCycle::Right;
        }
        TurnCycle::Right => {
          cart.d += Rotation::R;
          cart.m = TurnCycle::Left;
        }
      }
    }
    None => {
      bail!(
        "Cart moved onto a blank space at {},{}. Why?",
        cart.x,
        cart.y
      );
    }
  }
  Ok(())
}

fn solve(
  carts: &mut [Cart],
  grid: &Infinite2dGrid<RailKind>,
  stop_after_first_collision: bool,
) -> Result<(i32, i32)> {
  let mut removed_ids: FnvHashSet<usize> = FnvHashSet::default();
  // If it takes more iterations than this, I made a mistake somewhere.
  for _ in 0..1_000_000 {
    // Ensure we iterate left-to-right, then top-to-bottom, per the puzzle description
    carts.sort_by_key(|cart| (cart.x, cart.y));
    // Not using iter_mut() because we also loop over it a second time immutably within
    for i in 0..carts.len() {
      let to_modify = &mut carts[i];
      if removed_ids.contains(&to_modify.id) {
        continue;
      }
      step(to_modify, grid)?;

      // If we directly reference these in the inner loop,
      // we're borrowing `carts` both mutably and immutably.
      // What we're doing is safe, but we have to tell the Rust
      // compiler by dereferencing the components outside the loop.
      let cid = to_modify.id;
      let cx = to_modify.x;
      let cy = to_modify.y;

      for other in carts.iter() {
        if cid == other.id || removed_ids.contains(&other.id) {
          continue;
        }
        if cx == other.x && cy == other.y {
          if stop_after_first_collision {
            return Ok((cx, cy));
          } else {
            removed_ids.insert(cid);
            removed_ids.insert(other.id);
          }
        }
      }
    }

    if removed_ids.len() >= carts.len() - 1 {
      for Cart { id, x, y, .. } in carts.iter() {
        if !removed_ids.contains(id) {
          return Ok((*x, *y));
        }
      }
      bail!("No carts remain. Did we start with an even number of them?");
    }
  }
  bail!("Failed to find a collision in a reasonable timeframe");
}

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    let mut carts: Vec<Cart> = vec![];
    let mut grid = Infinite2dGrid::<RailKind>::new(lines.len() * lines.len());
    for (yy, line) in lines.iter().enumerate() {
      let y = yy as i32;
      for (xx, ch) in line.chars().enumerate() {
        let x = xx as i32;
        match ch {
          '-' => {
            grid.insert(x, y, RailKind::Horizontal);
          }
          '|' => {
            grid.insert(x, y, RailKind::Vertical);
          }
          '+' => {
            grid.insert(x, y, RailKind::Intersection);
          }
          '/' => {
            grid.insert(x, y, RailKind::Slash);
          }
          '\\' => {
            grid.insert(x, y, RailKind::Backslash);
          }
          '>' => {
            grid.insert(x, y, RailKind::Horizontal);
            carts.push(Cart {
              id: carts.len(),
              x,
              y,
              d: CardinalDirection::E,
              m: TurnCycle::Left,
            });
          }
          '<' => {
            grid.insert(x, y, RailKind::Horizontal);
            carts.push(Cart {
              id: carts.len(),
              x,
              y,
              d: CardinalDirection::W,
              m: TurnCycle::Left,
            });
          }
          '^' => {
            grid.insert(x, y, RailKind::Vertical);
            carts.push(Cart {
              id: carts.len(),
              x,
              y,
              d: CardinalDirection::N,
              m: TurnCycle::Left,
            });
          }
          'v' => {
            grid.insert(x, y, RailKind::Vertical);
            carts.push(Cart {
              id: carts.len(),
              x,
              y,
              d: CardinalDirection::S,
              m: TurnCycle::Left,
            });
          }
          ' ' => {
            // Don't even store empty spaces, there's no point
          }
          _ => {
            bail!("Unrecognized char: {ch}");
          }
        }
      }
    }
    Ok((carts, grid))
  }

  fn part1(&self, (in_carts, grid): &Parsed, _: Option<String>) -> Result<P1Out> {
    let mut carts = in_carts.clone();
    let (x, y) = solve(&mut carts, grid, true)?;
    Ok(format!("{x},{y}"))
  }

  fn part2(&self, (in_carts, grid): &Parsed, _: Option<String>) -> Result<P2Out> {
    if in_carts.len() % 2 == 0 {
      // The first sample has an even number of carts, so part 2 doesn't apply.
      return Ok("N/A: Starting with an even number of carts".to_string());
    }
    let mut carts = in_carts.clone();
    let (x, y) = solve(&mut carts, grid, false)?;
    Ok(format!("{x},{y}"))
  }
}

fn main() -> Result<()> {
  Solver {}.run(2018, 13)
}
