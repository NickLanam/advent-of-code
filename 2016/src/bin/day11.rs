use std::collections::VecDeque;

use advent_lib::runner::{Day, PartId};
use anyhow::{Result, bail};
use fnv::{FnvBuildHasher, FnvHashSet};
use itertools::Itertools;

#[derive(Clone, Debug, PartialEq)]
enum ItemKind {
  Generator,
  Microchip,
}

#[derive(Clone)]
struct Floor {
  #[allow(dead_code)]
  id: u8, // 1, 2, 3, or 4
  items: Vec<(ItemKind, char)>,
}
impl std::fmt::Debug for Floor {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "Floor {}: {}",
      &self.id,
      &self
        .items
        .iter()
        .map(|(kind, c)| {
          match kind {
            ItemKind::Generator => format!("{c}G"),
            ItemKind::Microchip => format!("{c}M"),
          }
        })
        .collect::<Vec<String>>()
        .join(" ")
    )
  }
}

#[derive(Clone)]
struct State {
  elevator_pos: u8, // 1, 2, 3, or 4
  floors: [Floor; 4],
}

impl std::fmt::Debug for State {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      &self
        .floors
        .iter()
        .enumerate()
        .rev()
        .map(|(i, floor)| {
          format!(
            "{}{floor:?}",
            if i as u8 + 1_u8 == self.elevator_pos {
              "E "
            } else {
              "  "
            }
          )
        })
        .collect::<Vec<String>>()
        .join("\n")
    )
  }
}

impl std::fmt::Display for State {
  /// This one is actually a serializer method!
  /// It doesn't keep the exact contents of the floors,
  /// but it does keep the details that make one state equivalent to another:
  ///
  /// elevator_pos;floor1_len,floor2_len,..;sorted_list_of_distances_between_chip_and_generator
  ///
  /// When searching, that's enough to tell if two states have the same solution.
  /// That is, these two states have the same solution and the same serialization as such:
  /// * `F1: HG; F2: HM; F3: LG LM; F4: empty (E on 2)`
  /// * `F1: LG; F2: LM; F3: HG HM; F4: empty (E on 2)`
  /// * Result for both is: `2;1,1,1,0;1,0`
  ///
  /// Note: this fact also holds when deciding if two MOVES will be functionally equal!
  /// Remember to prune duplicates in this way.
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let mut distances: Vec<i8> = vec![];
    for (fi, floor) in self.floors.iter().enumerate() {
      for (kind, c) in floor.items.iter() {
        if *kind == ItemKind::Microchip {
          'search: for (fi2, floor2) in self.floors.iter().enumerate() {
            for (kind2, c2) in floor2.items.iter() {
              if *kind2 == ItemKind::Generator && c == c2 {
                distances.push(fi as i8 - fi2 as i8);
                break 'search;
              }
            }
          }
        }
      }
    }
    distances.sort();
    write!(
      f,
      "{};{},{},{},{};{}",
      self.elevator_pos,
      self.floors[0].items.len(),
      self.floors[1].items.len(),
      self.floors[2].items.len(),
      self.floors[3].items.len(),
      distances
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(",")
    )
  }
}

fn would_explode(items: &Vec<&(ItemKind, char)>) -> bool {
  if items.len() <= 1 {
    false
  } else if items.len() == 2 {
    // Two chips or two generators is safe
    // A chip and a generator is safe if they are the same type
    // A chip and a generator of different types causes an explosion
    items[0].0 != items[1].0 && items[0].1 != items[1].1
  } else {
    // The expensive test: an explosion happens if there is a chip that is not matched
    // to a generator, and there is also a generator present (even if matched to another chip)
    'chip_check: for (kind, c) in items {
      if *kind == ItemKind::Microchip {
        let mut has_generators = false;
        for (kind2, c2) in items {
          if *kind2 == ItemKind::Generator {
            has_generators = true;
            if c2 == c {
              continue 'chip_check;
            }
          }
        }
        // If we have generators but none of them match this chip, the chip would cause an explosion
        if has_generators {
          return true;
        }
      }
    }
    false
  }
}

fn move_items(
  prev: &State,
  movers: Vec<(ItemKind, char)>,
  from_floor: u8,
  to_floor: u8,
) -> Result<State> {
  let mut target_floor_items: Vec<&(ItemKind, char)> = Vec::with_capacity(10);
  for item in prev.floors[to_floor as usize - 1].items.iter() {
    target_floor_items.push(item);
  }
  for item in movers.iter() {
    target_floor_items.push(item);
  }

  if would_explode(&target_floor_items) {
    bail!("Would explode, move is not valid");
  }

  let mut new_floors = prev.floors.clone();
  for item_to_move in movers {
    // Remove from the floor it was on
    let ri = new_floors[from_floor as usize - 1]
      .items
      .iter()
      .position(|p| *p == item_to_move);
    new_floors[from_floor as usize - 1]
      .items
      .remove(ri.unwrap());
    // Add to the floor it's going to
    new_floors[to_floor as usize - 1].items.push(item_to_move);
  }

  Ok(State {
    elevator_pos: to_floor,
    floors: new_floors,
  })
}

/// BFS, with the following rules:
/// - A solved state is any where the fourth floor has all of the original elements,
///   and the other three floors are empty, and the elevator is on the fourth floor.
/// - Because this is BFS, the first solve state we find is the shortest path.
///   the answer is the length of that path (depth of the search).
/// - We must not visit any equivalent state twice, as doing so is a backtrack
///   So, we keep a (cheap!) set of seen serializations.
/// - We do some extra pruning when checking legal neighbors:
///   - If it's possible to take two items up, skip checking options that take one item up
///   - If it's possible to take one item down, skip checking options that take two down
///   - When discovering an option, mark it as seen if it wasn't already
///   - If it was already, don't bother pushing it to the explore list.
fn solve(initial_state: &State) -> Result<u64> {
  // How many items exist total? Goal state will have them all on the 4th floor
  let goal_len = initial_state
    .floors
    .iter()
    .fold(0_usize, |acc, floor| acc + floor.items.len());

  let mut seen: FnvHashSet<String> =
    FnvHashSet::with_capacity_and_hasher(1_000_000, FnvBuildHasher::default());

  let mut explore: VecDeque<(u64, State, String)> =
    VecDeque::from([(0, initial_state.clone(), initial_state.to_string())]);
  while let Some((depth, current, current_serialization)) = explore.pop_front() {
    // println!("\n\nCheck @ depth {depth}, serialization {current_serialization}:\n{current:?}");
    // Is this the goal state?
    if current.elevator_pos == 4
      && current.floors[3].items.len() == goal_len
      && current.floors[2].items.is_empty()
      && current.floors[1].items.is_empty()
      && current.floors[0].items.is_empty()
    {
      return Ok(depth);
    }

    // If this state was seen already, we probably added the same option twice from one state
    if seen.contains(&current_serialization) {
      continue;
    }
    seen.insert(current_serialization.to_string());

    let can_go_up = current.elevator_pos < 4;

    // If all floors below the elevator are empty, then any downward move would result in a dead end later.
    let can_go_down = current.elevator_pos > 1
      && current
        .floors
        .iter()
        .take(current.elevator_pos as usize - 1)
        .any(|floor| !floor.items.is_empty());

    // Skip if we can't move
    if !can_go_up && !can_go_down {
      continue;
    }

    let current_items = &current.floors[current.elevator_pos as usize - 1].items;

    // Items that can be removed from the current floor without causing an explosion
    let take_one: Vec<(ItemKind, char)> = current_items
      .iter()
      // Skip those that would cause the floor we're leaving to explode
      .filter(|item| !would_explode(&current_items.iter().filter(|other| item != other).collect()))
      .cloned()
      .collect();

    // Same, but take two at a time and make sure they don't blow up on the elevator itself
    let take_two: Vec<Vec<(ItemKind, char)>> = current_items
      .iter()
      .combinations(2)
      .filter_map(|pair| {
        let plain_pair: Vec<(ItemKind, char)> = pair.iter().map(|p| (**p).clone()).collect();
        if would_explode(&pair) {
          return None;
        }
        let filtered: Vec<&(ItemKind, char)> = current_items
          .iter()
          .filter(|other| plain_pair[0] != **other && plain_pair[1] != **other)
          .collect();
        if would_explode(&filtered) {
          None
        } else {
          Some(plain_pair)
        }
      })
      .collect();

    if can_go_up {
      let mut can_take_two_up = false;
      for pair in &take_two {
        if let Ok(next) = move_items(
          &current,
          pair.clone(),
          current.elevator_pos,
          current.elevator_pos + 1,
        ) {
          can_take_two_up = true;
          let new_string = next.to_string();
          if !seen.contains(&new_string) {
            explore.push_back((depth + 1, next, new_string));
          }
        }
      }

      // Try taking one up
      if !can_take_two_up {
        for item in &take_one {
          if let Ok(next) = move_items(
            &current,
            vec![item.clone()],
            current.elevator_pos,
            current.elevator_pos + 1,
          ) {
            let new_string = next.to_string();
            if !seen.contains(&new_string) {
              explore.push_back((depth + 1, next, new_string));
            }
          }
        }
      }
    }

    if can_go_down {
      let mut can_take_one_down = false;
      for item in &take_one {
        if let Ok(next) = move_items(
          &current,
          vec![item.clone()],
          current.elevator_pos,
          current.elevator_pos - 1,
        ) {
          can_take_one_down = true;
          let new_string = next.to_string();
          if !seen.contains(&new_string) {
            explore.push_back((depth + 1, next, new_string));
          }
        }
      }

      // Try taking two down
      if !can_take_one_down {
        for pair in &take_two {
          if let Ok(next) = move_items(
            &current,
            pair.clone(),
            current.elevator_pos,
            current.elevator_pos - 1,
          ) {
            let new_string = next.to_string();
            if !seen.contains(&new_string) {
              explore.push_back((depth + 1, next, new_string));
            }
          }
        }
      }
    }
  }

  bail!(
    "Saw {} state(s), but none of them were solutions",
    seen.len()
  )
}

type P1Out = u64;
type P2Out = u64;
type Parsed = State;

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    let mut floors: [Floor; 4] = [
      Floor {
        id: 1,
        items: vec![],
      },
      Floor {
        id: 2,
        items: vec![],
      },
      Floor {
        id: 3,
        items: vec![],
      },
      Floor {
        id: 4,
        items: vec![],
      },
    ];
    // Example lines:
    // The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
    // The first floor contains a strontium generator, a strontium-compatible microchip, a plutonium generator, and a plutonium-compatible microchip.
    for (i, line) in lines.iter().enumerate() {
      // Transform the string into something that's trivially splittable on commas first
      let patched = line
        // "... contains a foo and a bar." -> "... contains a foo, and a bar"
        .replace("p and", "p, and")
        .replace("r and", "r, and")
        // "... contains a foo, a bar, and a baz." -> "... contains a foo, a bar, a baz."
        .replace(" and ", " ")
        // "... contains a foo, a bar, a baz." -> "... contains foo, bar, baz."
        .replace(" a ", " ")
        // Drop the trailing dot for easier splitting
        .replace(".", "");
      // "... contains foo, bar, baz" -> ["foo", "bar", "baz"]
      let items = patched.split_once(" contains ").unwrap().1.split(", ");
      for item in items {
        if item.contains('-') {
          // It's a microchip
          let (name, _) = item.split_once('-').unwrap();
          floors[i].items.push((
            ItemKind::Microchip,
            name.chars().next().unwrap().to_ascii_uppercase(),
          ));
        } else {
          let (name, kind) = item.split_once(' ').unwrap();
          // Floor 4 starts with "nothing relevant", ignore that one
          if kind == "generator" {
            floors[i].items.push((
              ItemKind::Generator,
              name.chars().next().unwrap().to_ascii_uppercase(),
            ));
          }
        }
      }
    }
    Ok(State {
      elevator_pos: 1,
      floors,
    })
  }

  fn part1(&self, initial_state: &Parsed, _sample_name: Option<String>) -> Result<P1Out> {
    solve(initial_state)
  }

  fn part2(&self, initial_state: &Parsed, sample_name: Option<String>) -> Result<P2Out> {
    if sample_name.is_some() {
      // Doing the modification to the sample causes it to become unsolveable
      return Ok(1);
    }

    let mut updated_initial_state: State = initial_state.clone();
    updated_initial_state.floors[0]
      .items
      .push((ItemKind::Generator, 'e'));
    updated_initial_state.floors[0]
      .items
      .push((ItemKind::Microchip, 'e'));
    updated_initial_state.floors[0]
      .items
      .push((ItemKind::Generator, 'd'));
    updated_initial_state.floors[0]
      .items
      .push((ItemKind::Microchip, 'd'));

    solve(&updated_initial_state)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2016, 11)
}
