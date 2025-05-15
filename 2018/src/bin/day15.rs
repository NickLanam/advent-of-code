use std::collections::VecDeque;

use advent_lib::{
  grid::Infinite2dSet,
  runner::{Day, PartId},
};
use anyhow::{Result, bail};
use fnv::FnvHashMap;
use itertools::Itertools;

#[derive(Clone, Debug)]
struct Unit {
  id: usize,
  is_elf: bool,
  x: i32,
  y: i32,
  hp: u8,
  ap: u8,
}

type P1Out = usize;
type P2Out = u64;
// (width, height, walls, initial_units)
type Parsed = (usize, usize, Infinite2dSet, Vec<Unit>);

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    let h = lines.len();
    let w = lines[0].len();
    let mut walls = Infinite2dSet::new(w * h / 2);
    let mut units: Vec<Unit> = vec![];
    for (y, line) in lines.iter().enumerate() {
      for (x, ch) in line.chars().enumerate() {
        match ch {
          '#' => {
            walls.insert(x as i32, y as i32);
          }
          '.' => { /* No-op */ }
          'G' => {
            units.push(Unit {
              id: units.len(),
              is_elf: false,
              x: x as i32,
              y: y as i32,
              hp: 200,
              ap: 3,
            });
          }
          'E' => {
            units.push(Unit {
              id: units.len(),
              is_elf: true,
              x: x as i32,
              y: y as i32,
              hp: 200,
              ap: 3,
            });
          }
          _ => {
            bail!("Unrecognized item in grid: '{ch}' at ({x}, {y})");
          }
        }
      }
    }
    Ok((w, h, walls, units))
  }

  fn part1(&self, (w, h, walls, in_units): &Parsed, _: Option<String>) -> Result<P1Out> {
    let mut units: FnvHashMap<usize, Unit> = FnvHashMap::default();
    for unit in in_units {
      units.insert(unit.id, unit.clone());
    }

    for round in 0.. {
      println!("BEGIN ROUND, {round} have already completed");
      let mut anything_happened = false;

      // Units take their turns in the reading order they started the round with.
      let turn_order: Vec<usize> = units
        .values()
        .filter(|u| u.hp > 0)
        .sorted_by(|a, b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)))
        .map(|u| u.id)
        .collect();

      'round: for id in turn_order {
        // Without cloning, we would borrow both mutably and immutably.
        let original_unit = units.get(&id).unwrap().clone();
        println!("Round {round}, check unit {original_unit:?}");
        if original_unit.hp == 0 {
          continue 'round;
        }
        let enemies: Vec<&Unit> = units
          .values()
          .filter(|&Unit { is_elf, hp, .. }| *is_elf != original_unit.is_elf && *hp > 0)
          .collect();
        if enemies.is_empty() {
          // Short-circuit: end the game if either team has been completely defeated.
          println!("  No surviving enemies, end the game.");
          anything_happened = false;
          break 'round;
        }

        // 1. Find positions where we could reach at least one foe
        let mut target_positions: Vec<(i32, i32)> = vec![];
        for other in enemies.iter() {
          for (tx, ty) in [
            (other.x - 1, other.y),
            (other.x, other.y - 1),
            (other.x + 1, other.y),
            (other.x, other.y + 1),
          ] {
            if !walls.contains(tx, ty)
              && !units.values().any(
                |Unit {
                   id, x: ux, y: uy, ..
                 }| *id != original_unit.id && *ux == tx && *uy == ty,
              )
            {
              // Duplicates slow us down here, but not as much
              // as using a hashset for such a small list would.
              println!("  Could target position ({tx},{ty})");
              target_positions.push((tx, ty));
            }
          }
        }

        // 2. If we are not already in an attack position, try to move towards one.
        if !target_positions
          .iter()
          .any(|(tx, ty)| *tx == original_unit.x && *ty == original_unit.y)
        {
          println!("  Not in an attack position, try moving towards one");
          // Breadth-first search, making sure to push_back coordinates in reading order.
          // In order to satisfy the oddly specific rules about target selection, we also
          // have to do a sort after each step of BFS (of items that are already mostly
          // sorted, so not a huge performance loss).

          let mut seen: Infinite2dSet = Infinite2dSet::new(w * h / 2);
          seen.insert(original_unit.x, original_unit.y);

          let mut frontier: VecDeque<Vec<(i32, i32)>> = VecDeque::new();
          frontier.push_back(vec![(original_unit.x, original_unit.y)]);

          'bfs: while let Some(path) = frontier.pop_front() {
            let &(px, py) = path.last().unwrap();
            println!(
              "  Frontier has {} items. Extend ({px},{py}) w/ len={}",
              frontier.len(),
              path.len()
            );

            if let Some(&(fx, fy)) = path.get(1) {
              if target_positions
                .iter()
                .any(|&(tx, ty)| tx == px && ty == py)
              {
                println!("  I see a path to ({px},{py}). Taking the first step to ({fx},{fy}).");
                units.entry(original_unit.id).and_modify(|u| {
                  u.x = fx;
                  u.y = fy;
                });
                anything_happened = true;
                break 'bfs;
              }
            }

            // In reading order: north, then west, then east, then south
            for (nx, ny) in [(px, py - 1), (px - 1, py), (px + 1, py), (px, py + 1)] {
              if !seen.contains(nx, ny)
                && !walls.contains(nx, ny)
                && !enemies.iter().any(|e| e.x == nx && e.y == ny)
              {
                let mut new_path = path.clone();
                seen.insert(nx, ny);
                new_path.push((nx, ny));
                frontier.push_back(new_path);
              }
            }
            frontier = VecDeque::from(
              frontier
                .iter()
                .sorted_by(|a, b| {
                  // Sort not by length, but by reading order of the edge node.
                  // This deals with a specific nuance of the puzzle's rules,
                  // and actually finds a _less_ optimal solution than not doing it!
                  // The order is already nearly correct, so this isn't as expensive
                  // as it looks.
                  let (ax, ay) = a.last().unwrap();
                  let (bx, by) = b.last().unwrap();
                  ay.cmp(by).then(ax.cmp(bx))
                })
                .cloned()
                .collect::<Vec<Vec<(i32, i32)>>>(),
            );
          }
        }

        // 3. If we're now in an attack position, figure out which enemy to attack, and do so.
        // Grabbing a fresh reference to avoid confusing the borrow checker about mutable vs immutable.
        let updated_unit = units.get(&original_unit.id).unwrap();
        if let Some(&(tx, ty)) = target_positions
          .iter()
          .find(|(tx, ty)| *tx == updated_unit.x && *ty == updated_unit.y)
        {
          println!("  Can attack from ({tx},{ty})");
          let target_unit = units
            .values()
            .filter(|e| {
              let dx = e.x.abs_diff(tx);
              let dy = e.y.abs_diff(ty);
              e.is_elf != updated_unit.is_elf
                && ((dx == 1 && dy == 0) || (dx == 0 && dy == 1))
                && e.hp > 0
            })
            .sorted_by(|a, b| a.hp.cmp(&b.hp).then(a.y.cmp(&b.y)).then(a.x.cmp(&b.x)))
            .next();
          if let Some(target_unit) = target_unit {
            let ap = updated_unit.ap;
            println!("  Deal {ap} damage to {target_unit:?}");
            units.entry(target_unit.id).and_modify(|t| {
              t.hp = t.hp.saturating_sub(ap);
            });
            anything_happened = true;
          }
        }
      }

      // Game ends when nothing happens in a round.
      if !anything_happened {
        println!("GAME COMPLETED AFTER {round} FULL ROUNDS. SURVIVING UNITS:\n");
        for unit in units.values() {
          println!("  {unit:?}");
        }
        // TODO: 288015 is too high for my input. I get the correct answer for the sample,
        //  and the first few moves look right, so I likely did something wrong with the
        //  tiebreaking logic?
        return Ok(round * units.values().map(|u| u.hp as usize).sum::<usize>());
      }
    }

    // Score the game: sum the HP of surviving units, multiply by the round count
    bail!("Game didn't end in a reasonable timeframe")
  }

  fn part2(&self, _lines: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    // PREDICTION: in part 1, goblins and elves both had 200 HP and 3 Attack Power.
    // If part 2 makes this assymetric it will make the game run far longer, and test
    // that the pathfinding and sorting were efficient enough to figure it out fast.
    Ok(0)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2018, 15)
}
