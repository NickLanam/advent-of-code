use std::collections::VecDeque;

use advent_lib::{
  grid::Infinite2dSet,
  runner::{Day, PartId},
};
use anyhow::{Result, bail};
use fnv::FnvHashMap;
use itertools::Itertools;

#[derive(Clone)]
struct Unit {
  id: usize,
  is_elf: bool,
  x: i32,
  y: i32,
  hp: u8,
  ap: u8,
}

fn simulate(
  w: usize,
  h: usize,
  walls: &Infinite2dSet,
  units: &mut FnvHashMap<usize, Unit>,
  fail_if_elves_die: bool,
) -> Result<usize> {
  for round in 0.. {
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
      if original_unit.hp == 0 {
        continue 'round;
      }
      let enemies: Vec<&Unit> = units
        .values()
        .filter(|&Unit { is_elf, hp, .. }| *is_elf != original_unit.is_elf && *hp > 0)
        .collect();
      if enemies.is_empty() {
        // Short-circuit: end the game if either team has been completely defeated.
        anything_happened = false;
        break 'round;
      }

      // 1. Find positions where we could reach at least one foe
      let mut target_positions: Vec<(i32, i32)> = vec![];
      for other in enemies.iter() {
        for (tx, ty) in [
          (other.x, other.y - 1),
          (other.x - 1, other.y),
          (other.x + 1, other.y),
          (other.x, other.y + 1),
        ] {
          if !walls.contains(tx, ty)
            && !units.values().any(|other| {
              other.hp > 0 && other.id != original_unit.id && other.x == tx && other.y == ty
            })
          {
            // Duplicates slow us down here, but not as much
            // as using a hashset for such a small list would.
            target_positions.push((tx, ty));
          }
        }
      }

      // 2. If we are not already in an attack position, try to move towards one.
      if !target_positions.is_empty()
        && !target_positions
          .iter()
          .any(|(tx, ty)| *tx == original_unit.x && *ty == original_unit.y)
      {
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

          if let Some(&(fx, fy)) = path.get(1)
            && target_positions
              .iter()
              .any(|&(tx, ty)| tx == px && ty == py)
          {
            units.entry(original_unit.id).and_modify(|u| {
              u.x = fx;
              u.y = fy;
            });
            anything_happened = true;
            break 'bfs;
          }

          // In reading order: north, then west, then east, then south
          for (nx, ny) in [(px, py - 1), (px - 1, py), (px + 1, py), (px, py + 1)] {
            if !seen.contains(nx, ny)
              && !walls.contains(nx, ny)
              && !units.values().any(|e| e.hp > 0 && e.x == nx && e.y == ny)
            {
              let mut new_path = path.clone();
              seen.insert(nx, ny);
              new_path.push((nx, ny));
              frontier.push_back(new_path);
            }
          }
          // Doing this, with the list already mostly-sorted, guarantees
          // that we meet all of the reading-order requirements for movement.
          // By chance, the only scenario where this makes a difference for me
          // is part 2's real input. None of the six samples, for either part,
          // get a different answer with or without this, and my real part 1
          // input also doesn't need it. ONLY my real part 2 input changes.
          frontier = VecDeque::from(
            frontier
              .iter()
              .sorted_by(|a, b| {
                let (ax, ay) = a.last().unwrap();
                let (bx, by) = b.last().unwrap();
                a.len().cmp(&b.len()).then(ay.cmp(by)).then(ax.cmp(bx))
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
          let mut elves_lose = false;
          let ap = updated_unit.ap;
          units.entry(target_unit.id).and_modify(|t| {
            if fail_if_elves_die && t.is_elf && t.hp <= ap {
              elves_lose = true;
            }
            t.hp = t.hp.saturating_sub(ap);
          });
          if elves_lose {
            bail!("Elves lose, because any of them dying is a failure condition");
          }
          anything_happened = true;
        }
      }
    }

    // Game ends when nothing happens in a round.
    if !anything_happened {
      return Ok(round * units.values().map(|u| u.hp as usize).sum::<usize>());
    }
  }
  bail!("Failed to find a solution");
}

type P1Out = usize;
type P2Out = usize;
// (width, height, walls, initial_units)
type Parsed = (usize, usize, Infinite2dSet, Vec<Unit>);

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
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

    simulate(*w, *h, walls, &mut units, false)
  }

  fn part2(&self, (w, h, walls, in_units): &Parsed, _: Option<String>) -> Result<P2Out> {
    for elven_ap in 4.. {
      let mut units: FnvHashMap<usize, Unit> = FnvHashMap::default();
      for unit in in_units {
        if !unit.is_elf {
          units.insert(unit.id, unit.clone());
        } else {
          let mut empowered_elf = unit.clone();
          empowered_elf.ap = elven_ap;
          units.insert(unit.id, empowered_elf);
        }
      }

      if let Ok(score) = simulate(*w, *h, walls, &mut units, true) {
        return Ok(score);
      }
    }
    bail!("Limitless power was not enough. Is a goblin hiding in a box?");
  }
}

fn main() -> Result<()> {
  Solver {}.run(2018, 15)
}
