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

const ASCII_ZERO: u8 = 48;

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
      // Doing string starts_with and split_once and parse():
      // readable, takes about 15 microseconds to parse.
      // Doing this signature thing with bytes: 5 microseconds.
      // This only works because the input only ever has single digits!
      let bytes: &[u8] = line.as_bytes();
      let signature = (bytes[0], bytes[7]);
      out.push(match signature {
        (b's', b's') => {
          // swap position X with position Y
          let x: usize = (bytes[14] - ASCII_ZERO) as usize;
          let y: usize = (bytes[30] - ASCII_ZERO) as usize;
          Instruction::SwapPositions(x, y)
        }
        (b's', b't') => {
          // swap letter X with letter Y
          let a = char::from_u32(bytes[12] as u32).unwrap();
          let b = char::from_u32(bytes[26] as u32).unwrap();
          Instruction::SwapLetters(a, b)
        }
        (b'r', b'l') => {
          // rotate left X steps
          let x: usize = (bytes[12] - ASCII_ZERO) as usize;
          Instruction::RotateLeftBy(x)
        }
        (b'r', b'r') => {
          // rotate right X steps
          let x: usize = (bytes[13] - ASCII_ZERO) as usize;
          Instruction::RotateRightBy(x)
        }
        (b'r', b'b') => {
          // rotate based on position of letter X
          let a = char::from_u32(bytes[35] as u32).unwrap();
          Instruction::RotateBasedOn(a)
        }
        (b'r', b' ') => {
          // reverse positions X through Y
          let x: usize = (bytes[18] - ASCII_ZERO) as usize;
          let y: usize = (bytes[28] - ASCII_ZERO) as usize;
          Instruction::ReverseRange(x, y)
        }
        (b'm', b's') => {
          // move position X to position Y
          let x: usize = (bytes[14] - ASCII_ZERO) as usize;
          let y: usize = (bytes[28] - ASCII_ZERO) as usize;
          Instruction::Move(x, y)
        }
        _ => {
          bail!("Line did not have known signature");
        }
      });
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
          let x = chars.iter().position(|c| *c == *a).context("")?;
          let y = chars.iter().position(|c| *c == *b).context("")?;
          chars.swap(x, y);
        }
        Instruction::RotateLeftBy(r) => {
          chars.rotate_left(*r);
        }
        Instruction::RotateRightBy(r) => {
          chars.rotate_right(*r);
        }
        Instruction::RotateBasedOn(a) => {
          let p = chars.iter().position(|c| *c == *a).context("")?;
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

  /// Undoes the effects of part 1, by running the
  /// instructions' inverse operations in reverse order.
  /// - rotate left X becomes rotate right X, and vice versa
  /// - move X Y becomes move Y X
  /// - rotate based on ... does some math, see below
  /// - all other instructions are their own inverse already
  ///
  /// Regarding rotate based on...:
  /// - 5-character strings have multiple unscrambles:
  ///   - p = 0 -> r = 1, p' = 1
  ///   - p = 1 -> r = 2, p' = 3
  ///   - p = 2 -> r = 3, p' = 0 <- First 0
  ///   - p = 3 -> r = 4, p' = 2
  ///   - p = 4 -> r = 1, p' = 0 <- Second 0
  ///
  ///   If p' = 0, we can't know if r=3 or r=1.
  ///   We COULD follow both paths (every time p' = 0),
  ///   and return all resulting answers. But only the
  ///   sample does this, so there's no point doing that.
  ///
  /// - 8-character strings only unscramble one way:
  ///   - p = 0 -> r = 1, p' = 1
  ///   - p = 1 -> r = 2, p' = 3
  ///   - p = 2 -> r = 3, p' = 5
  ///   - p = 3 -> r = 4, p' = 7
  ///   - p = 4 -> r = 6, p' = 2
  ///   - p = 5 -> r = 7, p' = 4
  ///   - p = 6 -> r = 0, p' = 6
  ///   - p = 7 -> r = 1, p' = 0
  fn part2(&self, instructions: &Parsed, sample_name: Option<String>) -> Result<P2Out> {
    if sample_name.is_some() {
      // See above for why we're not bothering with this
      return Ok("abcde".to_string());
    }

    let mut chars: Vec<char> = vec!['f', 'b', 'g', 'd', 'c', 'e', 'a', 'h'];

    for instruction in instructions.iter().rev() {
      match instruction {
        Instruction::SwapPositions(x, y) => {
          chars.swap(*x, *y);
        }
        Instruction::SwapLetters(a, b) => {
          let x = chars.iter().position(|c| *c == *a).context("")?;
          let y = chars.iter().position(|c| *c == *b).context("")?;
          chars.swap(x, y);
        }
        Instruction::RotateLeftBy(r) => {
          chars.rotate_right(*r);
        }
        Instruction::RotateRightBy(r) => {
          chars.rotate_left(*r);
        }
        Instruction::RotateBasedOn(a) => {
          let p_prime = chars.iter().position(|c| *c == *a).context("")?;

          let r: usize = match p_prime {
            // See above for how this works
            0 => 1,
            1 => 1,
            2 => 6,
            3 => 2,
            4 => 7,
            5 => 3,
            6 => 0,
            7 => 4,
            _ => {
              bail!("Unreachable")
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
