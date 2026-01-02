use std::collections::VecDeque;

use advent_lib::{
  direction::{CardinalDirection, Rotation},
  grid::Infinite2dSet,
  runner::{Day, PartId},
};
use advent_of_code_2019::intcode::execute;
use anyhow::{Result, bail};

type P1Out = usize;
type P2Out = i64;
type Parsed = Vec<i64>;

/// Exists only to make build_map easier to read
#[derive(PartialEq)]
enum Tile {
  Empty,
  Scaffold,
  BotFalling,
  BotNorth,
  BotSouth,
  BotWest,
  BotEast,
  NewLine,
}
impl From<i64> for Tile {
  fn from(value: i64) -> Self {
    match value {
      10 => Tile::NewLine,    // Literal \n
      35 => Tile::Scaffold,   // #
      46 => Tile::Empty,      // .
      60 => Tile::BotWest,    // <
      62 => Tile::BotEast,    // >
      88 => Tile::BotFalling, // X
      94 => Tile::BotNorth,   // ^
      118 => Tile::BotSouth,  // v
      _ => panic!("{value} is not one of the options"),
    }
  }
}

struct Map {
  scaffolds: Infinite2dSet,
  bot_x: i32,
  bot_y: i32,
  bot_dir: CardinalDirection,
}
/// Trivial parsing exercise: run the program, it outputs a grid of . and #,
/// turn that into an Infinite2dSet like we do with puzzles where that's the
/// original input. Just a slightly roundabout way to get there.
fn build_map(instructions: &[i64]) -> Result<Map> {
  let result = execute(instructions, &[], None, None)?;
  let mut scaffolds = Infinite2dSet::new(1_000);
  let mut bot_x = 0;
  let mut bot_y = 0;
  let mut bot_dir = CardinalDirection::N;
  let mut x = 0;
  let mut y = 0;
  for &o in result.outputs.iter() {
    let t = Tile::from(o);
    match t {
      Tile::Empty => {
        x += 1;
      }
      Tile::Scaffold => {
        scaffolds.insert(x, y);
        x += 1;
      }
      Tile::NewLine => {
        y += 1;
        x = 0;
      }
      Tile::BotFalling => {
        bot_x = x;
        bot_y = y;
        x += 1;
      }
      Tile::BotNorth | Tile::BotSouth | Tile::BotWest | Tile::BotEast => {
        scaffolds.insert(x, y);
        bot_x = x;
        bot_y = y;
        bot_dir = match t {
          Tile::BotNorth => CardinalDirection::N,
          Tile::BotSouth => CardinalDirection::S,
          Tile::BotWest => CardinalDirection::W,
          Tile::BotEast => CardinalDirection::E,
          // Covered by the outer match, but rustc doesn't know to narrow enum types here
          _ => bail!("Impossible"),
        };
        x += 1;
      }
    }
  }

  Ok(Map {
    scaffolds,
    bot_x,
    bot_y,
    bot_dir,
  })
}

/// The real input looks like the sample: a series of loops that only
/// touch at perpendicular crossings, with a clear start and end.
/// Thus, finding the full path is trivial: try forward, then left,
/// then right, and if none of those work then we must be done.
fn find_full_path(map: &Map) -> Vec<String> {
  let Map {
    scaffolds,
    bot_x,
    bot_y,
    bot_dir,
  } = map;

  let mut bot_x = *bot_x;
  let mut bot_y = *bot_y;
  let mut bot_dir = *bot_dir;

  let mut path: Vec<String> = vec![];
  let mut streak = 0;

  loop {
    let l_coord = (bot_dir + Rotation::L).apply(bot_x, bot_y, 1);
    let l_set = scaffolds.contains(l_coord.0, l_coord.1);
    let f_coord = bot_dir.apply(bot_x, bot_y, 1);
    let f_set = scaffolds.contains(f_coord.0, f_coord.1);
    let r_coord = (bot_dir + Rotation::R).apply(bot_x, bot_y, 1);
    let r_set = scaffolds.contains(r_coord.0, r_coord.1);

    if f_set {
      // Always move forward if possible
      (bot_x, bot_y) = f_coord;
      streak += 1;
    } else if l_set {
      (bot_x, bot_y) = l_coord;
      bot_dir += Rotation::L;
      if streak > 0 {
        let l = path.len();
        path[l - 1].push_str(&(streak + 1).to_string());
        streak = 0;
      }
      path.push("L".to_owned());
    } else if r_set {
      (bot_x, bot_y) = r_coord;
      bot_dir += Rotation::R;
      if streak > 0 {
        let l = path.len();
        path[l - 1].push_str(&(streak + 1).to_string());
        streak = 0;
      }
      path.push("R".to_owned());
    } else {
      // Can't move any direction other than backwards, we must be done
      break;
    }
  }
  if streak > 0 {
    let l = path.len();
    path[l - 1].push_str(&(streak + 1).to_string());
  }

  path
}

/// Each of (main, A, B, C) must be at most 20 bytes when input, per puzzle description
fn too_long_path(slice: &Vec<&str>) -> bool {
  to_inputs(slice).len() > 20
}

/// The only hard part of the puzzle. It's really just BFS, but with tricky
/// logic for computing valid neighbors and verifying goal state.
fn compress_path(full_path: &[String]) -> Result<(Vec<&str>, [Vec<&str>; 3])> {
  // Use BFS to try patterns until we find one that works
  #[allow(clippy::type_complexity)]
  let mut frontier: VecDeque<(Vec<&str>, [Option<Vec<&str>>; 3])> = VecDeque::new();
  frontier.push_back((vec![], [None, None, None]));

  'bfs: while let Some((main_routine, [a_opt, b_opt, c_opt])) = frontier.pop_front() {
    // Start with only the path that hasn't been consumed yet
    let mut remain: Vec<&str> = full_path.iter().map(|s| s.as_str()).collect();
    for &r in main_routine.iter() {
      if r == "A"
        && let Some(pat_a) = &a_opt
      {
        remain.drain(0..pat_a.len());
      } else if r == "B"
        && let Some(pat_b) = &b_opt
      {
        remain.drain(0..pat_b.len());
      } else if r == "C"
        && let Some(pat_c) = &c_opt
      {
        remain.drain(0..pat_c.len());
      } else {
        bail!("main_routine had {r} but not a pattern for it");
      }
    }

    // If we have a solution at this point, stop
    if remain.is_empty() {
      if !too_long_path(&main_routine)
        && let Some(pat_a) = a_opt
        && let Some(pat_b) = b_opt
        && let Some(pat_c) = c_opt
      {
        return Ok((main_routine, [pat_a, pat_b, pat_c]));
      } else if too_long_path(&main_routine) {
        // This solution can't work, try pursuing others
        continue 'bfs;
      } else {
        bail!(
          "Entire path was consumed but did not set all three subroutines. This should not be possible."
        );
      }
    }

    // If there's more to consume, search those spaces before we continue
    let mut explore_more_first = false;
    if let Some(pat_a) = &a_opt
      && remain.starts_with(pat_a)
    {
      explore_more_first = true;
      let next_main = [&main_routine[..], &["A"]].concat();
      frontier.push_back((next_main, [a_opt.clone(), b_opt.clone(), c_opt.clone()]));
    }
    if let Some(pat_b) = &b_opt
      && remain.starts_with(pat_b)
    {
      explore_more_first = true;
      let next_main = [&main_routine[..], &["B"]].concat();
      frontier.push_back((next_main, [a_opt.clone(), b_opt.clone(), c_opt.clone()]));
    }
    if let Some(pat_c) = &c_opt
      && remain.starts_with(pat_c)
    {
      explore_more_first = true;
      let next_main = [&main_routine[..], &["C"]].concat();
      frontier.push_back((next_main, [a_opt.clone(), b_opt.clone(), c_opt.clone()]));
    }

    if explore_more_first {
      continue 'bfs;
    }

    // If we got here, we need to discover more paths to try.
    for i in 1..remain.len() {
      let mut next_main = main_routine.clone();
      let mut next_a = a_opt.clone();
      let mut next_b = b_opt.clone();
      let mut next_c = c_opt.clone();
      let candidate: Vec<&str> = remain[0..i].to_vec();
      if too_long_path(&candidate) {
        break;
      }
      if a_opt.is_some() {
        if b_opt.is_some() {
          if c_opt.is_some() {
            // This solution doesn't consume the whole string, skip it
            continue 'bfs;
          } else {
            next_main.push("C");
            next_c = Some(candidate);
          }
        } else {
          next_main.push("B");
          next_b = Some(candidate);
        }
      } else {
        next_main.push("A");
        next_a = Some(candidate);
      }
      frontier.push_back((next_main, [next_a, next_b, next_c]));
    }
  }

  bail!("Failed to find a solution");
}

/// We have either the main routine ["A","B","C",..], or one of the
/// subroutines like ["L1","R2","L10",..]. Break the latter into a
/// form like ["L","1","R","2","L","10",..], leave the former alone.
/// Then convert to their ASCII representations separated by commas.
fn to_inputs(from: &Vec<&str>) -> Vec<i64> {
  from
    .iter()
    .map(|&part| {
      if part.starts_with("L") || part.starts_with("R") {
        let (d, c) = part.split_at(1);
        vec![d, c]
      } else {
        vec![part]
      }
    })
    .collect::<Vec<Vec<&str>>>()
    .concat()
    .join(",")
    .chars()
    .map(|c| c as u8)
    .map(|u| u as i64)
    .collect()
}

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(lines[0].split(',').map(|n| n.parse().unwrap()).collect())
  }

  /// More or less a checksum to prove that we parsed the map correctly.
  /// Part 2 is the real puzzle.
  fn part1(&self, instructions: &Parsed, _: Option<String>) -> Result<P1Out> {
    let Map { scaffolds, .. } = build_map(instructions)?;

    let mut score = 0;
    for (x, y) in scaffolds.keys() {
      if scaffolds.contains(x, y)
        && scaffolds.contains(x - 1, y)
        && scaffolds.contains(x + 1, y)
        && scaffolds.contains(x, y - 1)
        && scaffolds.contains(x, y + 1)
      {
        score += (x * y).unsigned_abs() as usize;
      }
    }
    Ok(score)
  }

  /// Part 2 is several steps:
  /// - Build the map, which we verified in part 1
  /// - Find the route to touch every point at least once
  /// - Split that into three subroutines that can reconstruct the full path
  ///   This is the only tricky part, but is ultimately a messy BFS.
  /// - Transform that into inputs to the intcode program
  /// - Run the intcode program and return its output directly
  fn part2(&self, instructions: &Parsed, _: Option<String>) -> Result<P2Out> {
    let map = build_map(instructions)?;
    let full_path = find_full_path(&map);
    let (main_routine, [sub_a, sub_b, sub_c]) = compress_path(&full_path)?;

    let main_inputs = to_inputs(&main_routine);
    let sub_a_inputs = to_inputs(&sub_a);
    let sub_b_inputs = to_inputs(&sub_b);
    let sub_c_inputs = to_inputs(&sub_c);

    let input: Vec<i64> = [
      &main_inputs[..],
      &[10], // Literal '\n'
      &sub_a_inputs[..],
      &[10],
      &sub_b_inputs[..],
      &[10],
      &sub_c_inputs[..],
      &[10],
      // Lowercase 'n', for "no I do not want video output".
      // It gives one frame of it at the start anyway though.
      // It also gives a second frame if the inputs do something wrong.
      &[110],
      &[10],
    ]
    .concat();

    let mut real_instructions = instructions.clone();
    real_instructions[0] = 2;
    let result = execute(&real_instructions, &input, None, None)?;

    // The program outputs text to query for stuff, then outputs the final video feed
    // even if 'n' was specified, THEN outputs the answer... so get the final number.
    Ok(*result.outputs.last().unwrap())
  }
}

fn main() -> Result<()> {
  Solver {}.run(2019, 17)
}
