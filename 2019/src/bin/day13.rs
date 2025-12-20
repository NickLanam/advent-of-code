use std::cmp::Ordering;

use advent_lib::{
  grid::Infinite2dGrid,
  runner::{Day, PartId},
};
use advent_of_code_2019::intcode::execute;
use anyhow::{Result, bail};

type P1Out = usize;
type P2Out = usize;
type Parsed = Vec<i64>;

#[derive(Clone, PartialEq, Eq)]
enum Tile {
  #[allow(unused)]
  Empty,
  Wall,
  Block,
  HorizontalPaddle,
  Ball,
}

struct State {
  score: i64,
  ball_x: i64,
  #[allow(unused)]
  ball_y: i64,
  paddle_x: i64,
  #[allow(unused)]
  paddle_y: i64,
}

fn update_state(outputs: &[i64], grid: &mut Infinite2dGrid<Tile>) -> Result<State> {
  let mut score = 0;
  let mut ball_x = 0;
  let mut ball_y = 0;
  let mut paddle_x = 0;
  let mut paddle_y = 0;
  for chunk in outputs.chunks_exact(3) {
    if let &[x, y, t] = chunk {
      match (x, y, t) {
        (-1, 0, t) => {
          score = t;
        }
        (_, _, 0) => {
          grid.remove(x as i32, y as i32);
        }
        (_, _, 1) => {
          grid.insert(x as i32, y as i32, Tile::Wall);
        }
        (_, _, 2) => {
          grid.insert(x as i32, y as i32, Tile::Block);
        }
        (_, _, 3) => {
          paddle_x = x;
          paddle_y = y;
          grid.insert(x as i32, y as i32, Tile::HorizontalPaddle);
        }
        (_, _, 4) => {
          ball_x = x;
          ball_y = y;
          grid.insert(x as i32, y as i32, Tile::Ball);
        }
        _ => {
          bail!("Unknown tile type: {t}");
        }
      }
    } else {
      bail!("Chunked wrong");
    };
  }
  Ok(State {
    score,
    ball_x,
    ball_y,
    paddle_x,
    paddle_y,
  })
}

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(lines[0].split(',').map(|n| n.parse().unwrap()).collect())
  }

  fn part1(&self, instructions: &Parsed, _: Option<String>) -> Result<P1Out> {
    let result = execute(instructions, &[], None, None)?;
    let mut grid: Infinite2dGrid<Tile> = Infinite2dGrid::new(1_000);
    update_state(&result.outputs, &mut grid)?;

    Ok(grid.values().filter(|&t| *t == Tile::Block).count())
  }

  fn part2(&self, init_tape: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    // Luckily for us, the rules are simple enough that we need only do one thing:
    // Move the paddle left and right so it stays under the ball. That's it.
    // Keep track of the position of the ball, the position of the paddle, and the score.
    // Then spit out the score when the program halts.
    let mut tape = init_tape.clone();
    tape[0] = 2; // Two quarters, per problem description

    let mut result = execute(&tape, &[], Some(0), None)?;
    let mut grid: Infinite2dGrid<Tile> = Infinite2dGrid::new(1_000);
    loop {
      let new_state = update_state(&result.outputs, &mut grid)?;
      if result.halted {
        return Ok(new_state.score as usize);
      }
      let paddle_move = match new_state.ball_x.cmp(&new_state.paddle_x) {
        Ordering::Less => [-1],
        Ordering::Equal => [0],
        Ordering::Greater => [1],
      };
      result = execute(
        &result.final_tape,
        &paddle_move,
        Some(result.pc),
        Some(result.ro),
      )?;
    }
  }
}

fn main() -> Result<()> {
  Solver {}.run(2019, 13)
}
