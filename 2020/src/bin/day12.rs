use advent_lib::{
  direction::{CardinalDirection, Rotation},
  runner::{Day, PartId},
};
use anyhow::Result;

type P1Out = usize;
type P2Out = usize;

#[derive(Debug, Copy, Clone)]
enum Instruction {
  Forward(i32),
  RotLeft(i32),
  RotRight(i32),
  North(i32),
  East(i32),
  South(i32),
  West(i32),
}
type Parsed = Vec<Instruction>;

struct StateP1 {
  x: i32,
  y: i32,
  d: CardinalDirection,
}
impl StateP1 {
  fn step(&self, inst: &Instruction) -> Self {
    match inst {
      Instruction::Forward(n) => {
        let (x, y) = self.d.apply(self.x, self.y, *n);
        StateP1 { x, y, d: self.d }
      }
      Instruction::RotLeft(deg) => {
        let mut d = self.d;
        for _ in 0..(deg / 90) {
          d += Rotation::L;
        }
        StateP1 {
          x: self.x,
          y: self.y,
          d,
        }
      }
      Instruction::RotRight(deg) => {
        let mut d = self.d;
        for _ in 0..(deg / 90) {
          d += Rotation::R;
        }
        StateP1 {
          x: self.x,
          y: self.y,
          d,
        }
      }
      Instruction::North(n) => StateP1 {
        x: self.x,
        y: self.y - n,
        d: self.d,
      },
      Instruction::East(n) => StateP1 {
        x: self.x + n,
        y: self.y,
        d: self.d,
      },
      Instruction::South(n) => StateP1 {
        x: self.x,
        y: self.y + n,
        d: self.d,
      },
      Instruction::West(n) => StateP1 {
        x: self.x - n,
        y: self.y,
        d: self.d,
      },
    }
  }
}

struct StateP2 {
  x: i32,
  y: i32,
  rel_wx: i32,
  rel_wy: i32,
}
impl StateP2 {
  fn step(self, inst: &Instruction) -> Self {
    match inst {
      Instruction::Forward(n) => {
        let dx = self.rel_wx * n;
        let dy = self.rel_wy * n;
        StateP2 {
          x: self.x + dx,
          y: self.y + dy,
          ..self
        }
      }
      Instruction::RotLeft(deg) => {
        let mut rel_wx = self.rel_wx;
        let mut rel_wy = self.rel_wy;
        for _ in 0..(deg / 90) {
          let tmp = rel_wx;
          rel_wx = rel_wy;
          rel_wy = -tmp;
        }
        StateP2 {
          rel_wx,
          rel_wy,
          ..self
        }
      }
      Instruction::RotRight(deg) => {
        let mut rel_wx = self.rel_wx;
        let mut rel_wy = self.rel_wy;
        for _ in 0..(deg / 90) {
          let tmp = rel_wx;
          rel_wx = -rel_wy;
          rel_wy = tmp;
        }
        StateP2 {
          rel_wx,
          rel_wy,
          ..self
        }
      }
      Instruction::North(n) => StateP2 {
        rel_wy: self.rel_wy - n,
        ..self
      },
      Instruction::East(n) => StateP2 {
        rel_wx: self.rel_wx + n,
        ..self
      },
      Instruction::South(n) => StateP2 {
        rel_wy: self.rel_wy + n,
        ..self
      },
      Instruction::West(n) => StateP2 {
        rel_wx: self.rel_wx - n,
        ..self
      },
    }
  }
}

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(
      lines
        .iter()
        .map(|line| {
          let (ch, rest) = line.split_at(1);
          let n = rest.parse().unwrap();
          match ch.chars().next().unwrap() {
            'F' => Instruction::Forward(n),
            'L' => Instruction::RotLeft(n),
            'R' => Instruction::RotRight(n),
            'N' => Instruction::North(n),
            'E' => Instruction::East(n),
            'S' => Instruction::South(n),
            'W' => Instruction::West(n),
            _ => panic!("Should not be seeing {ch} here"),
          }
        })
        .collect(),
    )
  }

  fn part1(&self, instructions: &Parsed, _: Option<String>) -> Result<P1Out> {
    let mut state = StateP1 {
      x: 0,
      y: 0,
      d: CardinalDirection::E,
    };
    for inst in instructions.iter() {
      state = state.step(inst);
    }
    Ok((state.x.unsigned_abs() + state.y.unsigned_abs()) as usize)
  }

  fn part2(&self, instructions: &Parsed, _: Option<String>) -> Result<P2Out> {
    let mut state = StateP2 {
      x: 0,
      y: 0,
      rel_wx: 10,
      rel_wy: -1,
    };
    for inst in instructions.iter() {
      state = state.step(inst);
    }
    Ok((state.x.unsigned_abs() + state.y.unsigned_abs()) as usize)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2020, 12)
}
