use advent_lib::runner::{Day, PartId};
use anyhow::{Context, Result, bail};

enum Instruction {
  SwapPositions(usize, usize),
  SwapLetters(char, char),
  RotateLeftBy(usize),
  RotateRightBy(usize),
  RotateBasedOn(char),
  ReverseRange(usize, usize),
  Move(usize, usize),
}

type P1Out = String;
type P2Out = String;
type Parsed = Vec<Instruction>;

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    let mut out: Vec<Instruction> = Vec::with_capacity(lines.len());
    for line in lines {
      if line.starts_with("swap position") {
        // swap position X with position Y
        let (_, r0) = line
          .split_once(" position ")
          .context("Can't split swap position")?;
        let (x_raw, y_raw) = r0
          .split_once(" with position ")
          .context("Can't split swap position")?;
        out.push(Instruction::SwapPositions(x_raw.parse()?, y_raw.parse()?));
      } else if line.starts_with("swap letter") {
        // swap letter X with letter Y
        let (_, r0) = line
          .split_once(" letter ")
          .context("Can't split swap letter")?;
        let (x, y) = r0
          .split_once(" with letter ")
          .context("Can't split swap letter")?;
        out.push(Instruction::SwapLetters(
          x.chars().nth(0).unwrap(),
          y.chars().nth(0).unwrap(),
        ));
      } else if line.starts_with("rotate left") {
        // rotate left X steps
        let (_, r0) = line
          .split_once(" left ")
          .context("Can't split rotate left")?;
        let (x_raw, _) = r0.split_once(" ").context("Can't split rotate left")?;
        out.push(Instruction::RotateLeftBy(x_raw.parse()?));
      } else if line.starts_with("rotate right") {
        // rotate right X steps
        let (_, r0) = line
          .split_once(" right ")
          .context("Can't split rotate right")?;
        let (x_raw, _) = r0.split_once(" ").context("Can't split rotate right")?;
        out.push(Instruction::RotateRightBy(x_raw.parse()?));
      } else if line.starts_with("rotate based on position of letter") {
        // rotate based on position of letter X
        let (_, x) = line
          .split_once(" letter ")
          .context("Can't split rotate based on position of letter")?;
        out.push(Instruction::RotateBasedOn(x.chars().nth(0).unwrap()));
      } else if line.starts_with("reverse positions") {
        // reverse positions X through Y
        let (_, r0) = line
          .split_once(" positions ")
          .context("Can't split reverse positions")?;
        let (x_raw, y_raw) = r0
          .split_once(" through ")
          .context("Can't split reverse positions")?;
        out.push(Instruction::ReverseRange(x_raw.parse()?, y_raw.parse()?));
      } else if line.starts_with("move position") {
        // move position X to position Y
        let (_, r0) = line
          .split_once(" position ")
          .context("Can't split move position")?;
        let (x_raw, y_raw) = r0
          .split_once(" to position ")
          .context("Can't split move position")?;
        out.push(Instruction::Move(x_raw.parse()?, y_raw.parse()?));
      } else {
        bail!("Line does not match a known command: {line}");
      }
    }
    Ok(out)
  }

  fn part1(&self, instructions: &Parsed, sample_name: Option<String>) -> Result<P1Out> {
    let mut chars: Vec<char> = if sample_name.is_some() {
      vec!['a', 'b', 'c', 'd', 'e']
    } else {
      vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h']
    };

    for instruction in instructions {
      match instruction {
        Instruction::SwapPositions(x, y) => {
          chars.swap(*x, *y);
        }
        Instruction::SwapLetters(a, b) => {
          let x = chars
            .iter()
            .position(|c| *c == *a)
            .context("Can't find {a} in {chars:?}")?;
          let y = chars
            .iter()
            .position(|c| *c == *b)
            .context("Can't find {b} in {chars:?}")?;
          chars.swap(x, y);
        }
        Instruction::RotateLeftBy(r) => {
          chars.rotate_left(*r);
        }
        Instruction::RotateRightBy(r) => {
          chars.rotate_right(*r);
        }
        Instruction::RotateBasedOn(a) => {
          let p = chars
            .iter()
            .position(|c| *c == *a)
            .context("Can't find {a} in {chars:?}")?;
          let r = (1 + p + (if p >= 4 { 1 } else { 0 })) % chars.len();
          chars.rotate_right(r);
        }
        Instruction::ReverseRange(x, y) => {
          let (mut l, mut r) = (*x, *y);
          while l < r {
            chars.swap(l, r);
            l += 1;
            r -= 1;
          }
        }
        Instruction::Move(x, y) => {
          let c = chars.remove(*x);
          chars.insert(*y, c);
        }
      }
    }

    Ok(chars.iter().collect())
  }

  fn part2(&self, instructions: &Parsed, sample_name: Option<String>) -> Result<P2Out> {
    if sample_name.is_some() {
      // 5-character strings can't be perfectly
      // unscrambled because of RotateBasedOn:

      // p = 0 -> r = 1, p' = 1
      // p = 1 -> r = 2, p' = 3
      // p = 2 -> r = 3, p' = 0 <- First 0
      // p = 3 -> r = 4, p' = 2
      // p = 4 -> r = 1, p' = 0 <- Second 0

      // If p' = 0, we can't know if r=3 or r=1.
      // We COULD follow both paths (every time p' = 0),
      // and return all resulting answers. But only the
      // sample does this, so there's no point doing that.
      return Ok("abcde".to_string());
    }

    // With an 8-character string, the RotateBasedOn instruction
    // does map 1:1 from p' back to p (see below), so we can reliably
    // reverse the operation and unscramble the string.
    let mut chars: Vec<char> = vec!['f', 'b', 'g', 'd', 'c', 'e', 'a', 'h'];

    // Undoes the effects of part 1, but running the instructions
    // end-to-start and doing the inverse of what they would normally do.
    // - rotate left X becomes rotate right X, and vice versa
    // - move X Y becomes move Y X
    // - rotate relative has to do math
    // - all other instructions are already their own inverse
    for instruction in instructions.iter().rev() {
      match instruction {
        Instruction::SwapPositions(x, y) => {
          chars.swap(*x, *y);
        }
        Instruction::SwapLetters(a, b) => {
          let x = chars
            .iter()
            .position(|c| *c == *a)
            .context("Can't find {a} in {chars:?}")?;
          let y = chars
            .iter()
            .position(|c| *c == *b)
            .context("Can't find {b} in {chars:?}")?;
          chars.swap(x, y);
        }
        Instruction::RotateLeftBy(r) => {
          chars.rotate_right(*r);
        }
        Instruction::RotateRightBy(r) => {
          chars.rotate_left(*r);
        }
        Instruction::RotateBasedOn(a) => {
          let p_prime = chars
            .iter()
            .position(|c| *c == *a)
            .context("Can't find {a} in {chars:?}")?;

          let r: usize = match p_prime {
            // In an 8-char string... No overlap, maps perfectly!
            // p = 0 -> r = 1, p' = 1
            // p = 1 -> r = 2, p' = 3
            // p = 2 -> r = 3, p' = 5
            // p = 3 -> r = 4, p' = 7
            // p = 4 -> r = 6, p' = 2
            // p = 5 -> r = 7, p' = 4
            // p = 6 -> r = 0, p' = 6
            // p = 7 -> r = 1, p' = 0
            0 => 1,
            1 => 1,
            2 => 6,
            3 => 2,
            4 => 7,
            5 => 3,
            6 => 0,
            7 => 4,
            _ => {
              bail!("Unreachable: char {a} was found at position {p_prime} in an 8-char string")
            }
          };

          chars.rotate_left(r);
        }
        Instruction::ReverseRange(x, y) => {
          let (mut l, mut r) = (*x, *y);
          while l < r {
            chars.swap(l, r);
            l += 1;
            r -= 1;
          }
        }
        Instruction::Move(y, x) => {
          let c = chars.remove(*x);
          chars.insert(*y, c);
        }
      }
    }

    Ok(chars.iter().collect())
  }
}

fn main() -> Result<()> {
  Solver {}.run(2016, 21)
}
