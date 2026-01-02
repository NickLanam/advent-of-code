use advent_lib::{
  direction::{CardinalDirection, Rotation},
  grid::Infinite2dSet,
  runner::{Day, PartId},
};
use advent_of_code_2019::intcode::execute;
use anyhow::{Result, bail};

type P1Out = usize;
type P2Out = String;
type Parsed = Vec<i64>;

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(lines[0].split(',').map(|n| n.parse().unwrap()).collect())
  }

  fn part1(&self, init: &Parsed, _: Option<String>) -> Result<P1Out> {
    let mut paints = Infinite2dSet::new(1024);
    let mut did_paint = Infinite2dSet::new(1024);
    let mut bot_dir = CardinalDirection::N;
    let mut bot_pos = (0, 0);

    let first_input = [0];
    let mut runner = execute(init, &first_input, Some(0), None)?;
    while !runner.halted {
      match runner.outputs[0] {
        0 => paints.remove(bot_pos.0, bot_pos.1),
        1 => paints.insert(bot_pos.0, bot_pos.1),
        _ => bail!("Bot output 0 was not a paint operation"),
      };
      did_paint.insert(bot_pos.0, bot_pos.1);

      match runner.outputs[1] {
        0 => bot_dir += Rotation::L,
        1 => bot_dir += Rotation::R,
        _ => bail!("Bot output 1 was not a turn"),
      };
      bot_pos = bot_dir.apply(bot_pos.0, bot_pos.1, 1);
      let next_input = if paints.contains(bot_pos.0, bot_pos.1) {
        [1]
      } else {
        [0]
      };
      runner = execute(&runner.final_tape, &next_input, Some(runner.pc), None)?;
    }
    Ok(did_paint.len())
  }

  fn part2(&self, init: &Parsed, _: Option<String>) -> Result<P2Out> {
    // Run part 1, but now the initial panel is white.
    // It then paints ASCII art to generate a code, so print that (will need to remember the bounding box it touched).
    let mut paints = Infinite2dSet::new(1024);
    let mut bot_dir = CardinalDirection::N;
    let mut bot_pos = (0, 0);

    let (mut min_x, mut min_y, mut max_x, mut max_y) = (0, 0, 0, 0);

    let first_input = [1];
    let mut runner = execute(init, &first_input, Some(0), None)?;
    while !runner.halted {
      match runner.outputs[0] {
        0 => paints.remove(bot_pos.0, bot_pos.1),
        1 => paints.insert(bot_pos.0, bot_pos.1),
        _ => bail!("Bot output 0 was not a paint operation"),
      };
      min_x = min_x.min(bot_pos.0);
      max_x = max_x.max(bot_pos.0);
      min_y = min_y.min(bot_pos.1);
      max_y = max_y.max(bot_pos.1);

      match runner.outputs[1] {
        0 => bot_dir += Rotation::L,
        1 => bot_dir += Rotation::R,
        _ => bail!("Bot output 1 was not a turn"),
      };
      bot_pos = bot_dir.apply(bot_pos.0, bot_pos.1, 1);
      let next_input = if paints.contains(bot_pos.0, bot_pos.1) {
        [1]
      } else {
        [0]
      };
      runner = execute(
        &runner.final_tape,
        &next_input,
        Some(runner.pc),
        Some(runner.ro),
      )?;
    }

    // Now, assemble the painted result into a string that a human can read
    let mut out: String = ((min_y - 1)..=(max_y + 1))
      .map(|y| {
        ((min_x - 1)..=(max_x + 1))
          .map(|x| match paints.contains(x, y) {
            false => ' ',
            true => '█', // u+2588
          })
          .collect::<String>()
      })
      .collect::<Vec<String>>()
      .join("\n");
    out.insert(0, '\n');
    Ok(out)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2019, 11)
}
