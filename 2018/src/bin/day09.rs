use std::collections::VecDeque;

use advent_lib::runner::{Day, PartId};
use anyhow::{Context, Result};

type P1Out = usize;
type P2Out = usize;
type Parsed = (usize, usize);

fn solve(num_players: usize, num_marbles: usize) -> Result<usize> {
  let mut scores = vec![0_usize; num_players];
  let mut ring: VecDeque<usize> = VecDeque::with_capacity(num_marbles);
  ring.push_back(0);

  let mut player = 0;
  for m in 1..num_marbles {
    if m % 23 == 0 {
      // Special rule
      ring.rotate_right(7);
      scores[player] += m + ring.pop_back().unwrap();
      ring.rotate_left(1);
    } else {
      ring.rotate_left(1);
      ring.push_back(m);
    }
    player += 1;
    if player >= num_players {
      player -= num_players;
    }
  }
  Ok(*scores.iter().max().unwrap())
}

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    let mut w = lines[0].split_whitespace();
    let num_players: usize = w.next().context("How many players?")?.parse()?;
    let last_marble_score: usize = w
      .nth(5)
      .context("How many points is the last marble worth?")?
      .parse()?;
    Ok((num_players, last_marble_score + 1))
  }

  fn part1(&self, &(num_players, num_marbles): &Parsed, _: Option<String>) -> Result<P1Out> {
    solve(num_players, num_marbles)
  }

  fn part2(&self, &(num_players, num_marbles): &Parsed, _: Option<String>) -> Result<P2Out> {
    solve(num_players, num_marbles * 100)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2018, 9)
}
