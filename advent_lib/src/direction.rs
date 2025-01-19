use std::fmt::{Display, Formatter};

use anyhow::bail;

#[derive(Clone, Debug, PartialEq)]
pub enum Rotation {
  L = -1,
  R = 1,
}
impl Display for Rotation {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:#?}", self)
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CardinalDirection {
  N,
  E,
  S,
  W,
}
impl CardinalDirection {
  pub fn apply(&self, x: i32, y: i32, steps: i32) -> (i32, i32) {
    match self {
      CardinalDirection::N => (x, y - steps),
      CardinalDirection::E => (x + steps, y),
      CardinalDirection::S => (x, y + steps),
      CardinalDirection::W => (x - steps, y),
    }
  }
}
impl Display for CardinalDirection {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:#?}", self)
  }
}
impl TryFrom<u8> for CardinalDirection {
  type Error = anyhow::Error;
  fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
    let dir = match value {
      0 => CardinalDirection::N,
      1 => CardinalDirection::E,
      2 => CardinalDirection::S,
      3 => CardinalDirection::W,
      _ => bail!("Unknown dir: {value}"),
    };
    Ok(dir)
  }
}
impl TryFrom<CardinalDirection> for u8 {
  type Error = anyhow::Error;

  fn try_from(value: CardinalDirection) -> Result<u8, Self::Error> {
    match value {
      CardinalDirection::N => Ok(0),
      CardinalDirection::E => Ok(1),
      CardinalDirection::S => Ok(2),
      CardinalDirection::W => Ok(3),
    }
  }
}

impl std::ops::Add<&Rotation> for CardinalDirection {
  type Output = CardinalDirection;

  fn add(self, rhs: &Rotation) -> Self::Output {
    match rhs {
      Rotation::L => ((self as u8 + 3) % 4).try_into().unwrap(),
      Rotation::R => ((self as u8 + 1) % 4).try_into().unwrap(),
    }
  }
}
impl std::ops::AddAssign<&Rotation> for CardinalDirection {
  fn add_assign(&mut self, rhs: &Rotation) {
    let v: u8 = self.clone().try_into().unwrap();
    *self = match rhs {
      Rotation::L => ((v + 3) % 4).try_into().unwrap(),
      Rotation::R => ((v + 1) % 4).try_into().unwrap(),
    }
  }
}
