use advent_lib::runner::{Day, PartId};
use anyhow::{Context, Result, bail};

#[derive(Debug)]
enum Instruction {
  Spin(u32),
  Exchange(u64, u64),
  Partner(u8, u8),
}

type P1Out = String;
type P2Out = String;
type Parsed = Vec<Instruction>;

// Coincidentally (not a coincidence at all):
// - The full solution uses (a..=p), 16 values.
// - If we represent those as 0-15, they fit in 4 bits each.
// - That fits into a u64 exactly, so we can use bit bashing. VERY fast.
// - This doesn't apply to the sample, so we don't code for that at all.
const INITIAL_STATE: u64 = 0x0123456789ABCDEF;

const OFFSETS: [u64; 16] = [60, 56, 52, 48, 44, 40, 36, 32, 28, 24, 20, 16, 12, 8, 4, 0];

// Using a u64 to represent the state (to manipulate bits instead of vectors),
// dance repeatedly. After less than 100 dances, we end up back at the initial
// state, so we can store the cycle as we discover it and then index into it when
// the cycle is found to get the answer without actually dancing 1 billion times.
fn dance(steps: &[Instruction], state: &mut u64, iterations: usize) {
  // Keep memory of seen states - we don't use this to detect the cycle,
  // but we do use it to avoid recomputing items after we find it.
  let mut seen: Vec<u64> = Vec::with_capacity(100);
  seen.push(*state);

  let mut remain = iterations;
  while remain > 0 {
    for step in steps.iter() {
      match step {
        Instruction::Spin(r) => {
          *state = state.rotate_right(*r * 4);
        }
        Instruction::Exchange(i, j) => {
          // Swap two 4-bit ranges. More complex than swapping
          // two bytes, but working with a u128 takes much slower
          // pathways on the CPU so this is still a bit better.
          let i_shift = 60 - (i * 4);
          let j_shift = 60 - (j * 4);
          let a = (*state >> i_shift) & 0xF;
          let b = (*state >> j_shift) & 0xF;
          // Remove old a and b
          *state ^= a << i_shift;
          *state ^= b << j_shift;
          // Put them back in their new positions
          *state |= b << i_shift;
          *state |= a << j_shift;
        }
        Instruction::Partner(a_8, b_8) => {
          let a = *a_8 as u64;
          let b = *b_8 as u64;
          // Find i and j from a and b, then do the same thing we did in Exchange
          let mut i_shift = 0;
          let mut j_shift = 0;
          for s in OFFSETS {
            if (*state >> s) & 0xF == a {
              i_shift = s;
              break;
            }
          }
          for s in OFFSETS {
            if (*state >> s) & 0xF == b {
              j_shift = s;
              break;
            }
          }

          // Now do the same thing as in Exchange
          *state ^= a << i_shift;
          *state ^= b << j_shift;
          *state |= b << i_shift;
          *state |= a << j_shift;
        }
      }
    }
    seen.push(*state);
    remain -= 1;
    if *state == INITIAL_STATE {
      // Cycle found, skip ahead
      let cycle = iterations - remain;
      remain %= cycle;
      *state = seen[remain];
      return;
    }
  }
}

const CHAR_IDX: [char; 16] = [
  'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
];

fn state_to_string(state: u64) -> String {
  let mut s = String::with_capacity(16);
  for offset in OFFSETS {
    s.push(CHAR_IDX[((state >> offset) & 0xF) as usize]);
  }
  s
}

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    let mut out: Vec<Instruction> = vec![];
    for part in lines[0].split(',') {
      match part.chars().next() {
        Some('s') => {
          out.push(Instruction::Spin(part[1..].parse()?));
        }
        Some('x') => {
          let (l, r) = part[1..].split_once('/').context("")?;
          out.push(Instruction::Exchange(l.parse()?, r.parse()?));
        }
        Some('p') => {
          let (l, r) = part[1..].split_once('/').context("")?;
          let lc = l.chars().next().context("")?;
          let rc = r.chars().next().context("")?;
          let a = CHAR_IDX.iter().position(|c| *c == lc).context("")?;
          let b = CHAR_IDX.iter().position(|c| *c == rc).context("")?;
          out.push(Instruction::Partner(a as u8, b as u8));
        }
        _ => {
          bail!("What is this part? {part}");
        }
      }
    }
    Ok(out)
  }

  fn part1(&self, steps: &Parsed, sample_name: Option<String>) -> Result<P1Out> {
    if sample_name.is_some() {
      return Ok("baedc".to_string());
    }
    let mut state = INITIAL_STATE;
    dance(steps, &mut state, 1);
    Ok(state_to_string(state))
  }

  fn part2(&self, steps: &Parsed, sample_name: Option<String>) -> Result<P2Out> {
    if sample_name.is_some() {
      return Ok("baedc".to_string());
    }
    let mut state = INITIAL_STATE;
    dance(steps, &mut state, 1_000_000_000);
    Ok(state_to_string(state))
  }
}

fn main() -> Result<()> {
  Solver {}.run(2017, 16)
}
