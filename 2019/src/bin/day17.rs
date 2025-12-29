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

fn print_grid(map: &Map) {
  let mut min_x = 0;
  let mut max_x = 0;
  let mut min_y = 0;
  let mut max_y = 0;

  for (x, y) in map.scaffolds.keys() {
    min_x = min_x.min(x);
    max_x = max_x.max(x);
    min_y = min_y.min(y);
    max_y = max_y.max(y);
  }

  for y in min_y..=max_y {
    for x in min_x..=max_x {
      if x == map.bot_x && y == map.bot_y {
        match map.bot_dir {
          CardinalDirection::N => print!("^"),
          CardinalDirection::E => print!(">"),
          CardinalDirection::S => print!("v"),
          CardinalDirection::W => print!("<"),
        }
      } else {
        print!(
          "{}",
          if map.scaffolds.contains(x, y) {
            "#"
          } else {
            " "
          }
        );
      }
    }
    println!();
  }
}

/// The real input looks like the sample: a series of loops that only
/// touch at perpendicular crossings, with a clear start and end.
/// Thus, finding the full path is trivial: go forward if possible,
/// left if not, right if not, and stop when back is the only option.
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

  let mut path = vec![];
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
      bot_dir = bot_dir + Rotation::L;
      if streak > 0 {
        path.push((streak + 1).to_string());
        streak = 0;
      }
      path.push("L".to_owned());
    } else if r_set {
      (bot_x, bot_y) = r_coord;
      bot_dir = bot_dir + Rotation::R;
      if streak > 0 {
        path.push((streak + 1).to_string());
        streak = 0;
      }
      path.push("R".to_owned());
    } else {
      // Can't move any direction other than backwards, we must be done
      break;
    }
  }
  if streak > 0 {
    path.push((streak + 1).to_string());
  }

  // TODO: Okay, this is correct now and so is to_inputs.
  // The only remaining task is compress_path (the actual hard part).
  println!("The grid looks like this:");
  print_grid(&map);
  println!("\nA full path would look like this:");
  println!("{path:?}");
  println!("\nWhich looks like this to the intcode:");
  println!("{:?}", to_inputs(&path));

  path
}

fn compress_path(full_path: &Vec<String>) -> (Vec<String>, [Vec<String>; 3]) {
  // TODO: We're looking for three sub-patterns that, when repeated in any permutation,
  // can reconstruct the original sequence (A,B,C,A,B,B,C for example).
  // Each sub-pattern, when turned into ASCII via to_input, needs to be 20 bytes max.
  // Same for the main routine that references them.

  // This looks an awful lot like a Huffman code to me, or perhaps something simpler.
  // Look into primitive-by-modern-standards lossless compression techniques and see
  // if one of them matches what we're doing here.
  // This puzzle smells similar to something telephony companies would have had to
  // do in the 80's with the hardware of the time, maybe start there?
  (vec![], [vec![], vec![], vec![]])
}

fn to_inputs(from: &Vec<String>) -> Vec<i64> {
  from
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
        score += (x * y).abs() as usize;
      }
    }
    Ok(score)
  }

  /// Part 2 is several easy steps, with one hard step being the real puzzle:
  /// - (Easy) Build the map, which we verified in part 1
  /// - (Easy) Find the route to touch every point at least once
  /// - (Hard) Split that into three subroutines that can reconstruct the full path
  /// - (Easy) Transform that into inputs to the intcode program
  /// - (Easy) Run the intcode program and return its output directly
  fn part2(&self, instructions: &Parsed, _: Option<String>) -> Result<P2Out> {
    let map = build_map(instructions)?;
    let full_path = find_full_path(&map);
    let (main_routine, [sub_a, sub_b, sub_c]) = compress_path(&full_path);

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
      &[110], // Lowercase 'n', for "no I do not want video output"
    ]
    .concat();

    let mut real_instructions = instructions.clone();
    real_instructions[0] = 2;
    let result = execute(&real_instructions, &input, None, None)?;

    Ok(result.outputs[0])
  }
}

fn main() -> Result<()> {
  Solver {}.run(2019, 17)
}
