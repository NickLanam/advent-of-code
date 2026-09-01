use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = usize;
type P2Out = usize;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum SeatState {
  #[default]
  None,
  Empty,
  Occupied,
}

#[derive(Clone, Default, PartialEq)]
struct State {
  w: usize,
  h: usize,
  grid: Vec<SeatState>,
}

impl State {
  fn get(&self, x: usize, y: usize) -> &SeatState {
    if x < self.w && y < self.h {
      &self.grid[(y * self.w) + x]
    } else {
      &SeatState::None
    }
  }

  fn is_occupied_direction(&self, x: usize, y: usize, dx: i32, dy: i32) -> bool {
    let mut xx = x as i32;
    let mut yy = y as i32;
    let ww = self.w as i32;
    let hh = self.h as i32;
    loop {
      xx += dx;
      yy += dy;
      if xx < 0 || xx >= ww || yy < 0 || yy >= hh {
        return false;
      }

      match self.grid[((yy as usize) * self.w) + (xx as usize)] {
        SeatState::None => {}
        SeatState::Empty => {
          return false;
        }
        SeatState::Occupied => {
          return true;
        }
      }
    }
  }

  fn next_p1(&self) -> State {
    let next = self
      .grid
      .iter()
      .enumerate()
      .map(|(i, s)| {
        let y = i / self.w;
        let x = i % self.w;
        let mut adj_occupied = 0;
        for (xx, yy) in [
          (x - 1, y - 1),
          (x, y - 1),
          (x + 1, y - 1),
          (x - 1, y),
          (x + 1, y),
          (x - 1, y + 1),
          (x, y + 1),
          (x + 1, y + 1),
        ] {
          if *self.get(xx, yy) == SeatState::Occupied {
            adj_occupied += 1;
          }
        }
        match (self.get(x, y), adj_occupied) {
          (SeatState::Empty, 0) => SeatState::Occupied,
          (SeatState::Occupied, 4..) => SeatState::Empty,
          _ => *s,
        }
      })
      .collect();
    State {
      w: self.w,
      h: self.h,
      grid: next,
    }
  }

  fn next_p2(&self) -> State {
    let next = self
      .grid
      .iter()
      .enumerate()
      .map(|(i, s)| {
        let y = i / self.w;
        let x = i % self.w;
        let mut adj_occupied = 0;
        for (dx, dy) in [
          (-1, -1),
          (0, -1),
          (1, -1),
          (-1, 0),
          (1, 0),
          (-1, 1),
          (0, 1),
          (1, 1),
        ] {
          if self.is_occupied_direction(x, y, dx, dy) {
            adj_occupied += 1;
          }
        }
        match (self.get(x, y), adj_occupied) {
          (SeatState::Empty, 0) => SeatState::Occupied,
          (SeatState::Occupied, 5..) => SeatState::Empty,
          _ => *s,
        }
      })
      .collect();
    State {
      w: self.w,
      h: self.h,
      grid: next,
    }
  }
}

struct Solver;
impl Day<State, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<State> {
    let h = lines.len();
    let w = lines[0].len();
    let grid = lines
      .iter()
      .flat_map(|line| {
        line.chars().map(|ch| match ch {
          '.' => SeatState::None,
          'L' => SeatState::Empty,
          '#' => SeatState::Occupied,
          _ => panic!("{ch} is not supposed to be there"),
        })
      })
      .collect();
    Ok(State { w, h, grid })
  }

  fn part1(&self, init: &State, _: Option<String>) -> Result<P1Out> {
    let mut prev: State = Default::default();
    let mut current: State = init.clone();
    while current != prev {
      prev = current.clone();
      current = current.next_p1();
    }
    Ok(
      current
        .grid
        .iter()
        .filter(|s| **s == SeatState::Occupied)
        .count(),
    )
  }

  fn part2(&self, init: &State, _: Option<String>) -> Result<P2Out> {
    let mut prev: State = Default::default();
    let mut current: State = init.clone();
    while current != prev {
      prev = current.clone();
      current = current.next_p2();
    }
    Ok(
      current
        .grid
        .iter()
        .filter(|s| **s == SeatState::Occupied)
        .count(),
    )
  }
}

fn main() -> Result<()> {
  Solver {}.run(2020, 11)
}
