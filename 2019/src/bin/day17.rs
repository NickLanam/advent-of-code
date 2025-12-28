use advent_lib::{
  grid::Infinite2dSet,
  runner::{Day, PartId},
};
use advent_of_code_2019::intcode::execute;
use anyhow::Result;

type P1Out = usize;
type P2Out = usize;
type Parsed = Vec<i64>;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
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
}
fn build_map(instructions: &[i64]) -> Result<Map> {
  let result = execute(instructions, &[], None, None)?;
  let mut scaffolds = Infinite2dSet::new(1_000);
  let mut bot_x = 0;
  let mut bot_y = 0;
  let mut x = 0;
  let mut y = 0;
  for &o in result.outputs.iter() {
    match Tile::from(o) {
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
      _ => {
        scaffolds.insert(x, y);
        bot_x = x;
        bot_y = y;
        x += 1;
      }
    }
  }

  Ok(Map {
    scaffolds,
    bot_x,
    bot_y,
  })
}

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(lines[0].split(',').map(|n| n.parse().unwrap()).collect())
  }

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

  fn part2(&self, _instructions: &Parsed, _: Option<String>) -> Result<P2Out> {
    // What we must do:
    // - build the map
    // - determine a path, taken as left/right turns and numbers of steps to take after,
    //   that touches each location once (if it looks like the sample, this is nearly trivial)
    // - THE HARD PART: find three subsequences of moves in there, A,B,C, that can be ordered
    //   and repeated in any combination that results in the original path (such as A,B,A,C,B)
    // - Each subsequence, and the main sequence, must be at most 20 characters long (excluding the \n)
    // - The input must be transformed to ASCII, as comma-separated commands split by newlines
    // - After the last newline, pass the char 'n' to decline the video output feature
    // - Modify the instructions by setting address 0 to 2
    // - The output of running that execution with that mess of input will be the correct answer
    Ok(0)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2019, 17)
}
