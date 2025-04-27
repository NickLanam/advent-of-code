use advent_lib::{
  direction::Rotation,
  runner::{Day, PartId},
};
use anyhow::Result;
use fnv::{FnvBuildHasher, FnvHashMap, FnvHashSet};

#[derive(Debug)]
struct Instruction {
  write: bool,
  movement: Rotation,
  next_state: char,
}

type P1Out = usize;
type P2Out = usize;

// Initial state, number of steps to run, then the map of state -> (action if 0, action if 1)
type Parsed = (char, u64, FnvHashMap<char, (Instruction, Instruction)>);

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    let initial_state = lines[0].chars().nth(15).unwrap();
    let iterations: u64 = lines[1].split_whitespace().nth(5).unwrap().parse()?;
    let mut map: FnvHashMap<char, (Instruction, Instruction)> =
      FnvHashMap::with_capacity_and_hasher(1 + (lines.len() - 3) / 10, FnvBuildHasher::default());
    for block in lines[3..].chunks(10) {
      let from_state = block[0].chars().nth(9).unwrap();
      let write_0 = block[2].chars().nth(22).unwrap() == '1';
      let write_1 = block[6].chars().nth(22).unwrap() == '1';
      let move_0 = if block[3].split_whitespace().nth(6).unwrap() == "right." {
        Rotation::R
      } else {
        Rotation::L
      };
      let move_1 = if block[7].split_whitespace().nth(6).unwrap() == "right." {
        Rotation::R
      } else {
        Rotation::L
      };
      let next_0 = block[4].chars().nth(26).unwrap();
      let next_1 = block[8].chars().nth(26).unwrap();
      map.insert(
        from_state,
        (
          Instruction {
            write: write_0,
            movement: move_0,
            next_state: next_0,
          },
          Instruction {
            write: write_1,
            movement: move_1,
            next_state: next_1,
          },
        ),
      );
    }
    Ok((initial_state, iterations, map))
  }

  fn part1(
    &self,
    (initial_state, iterations, instructions): &Parsed,
    _: Option<String>,
  ) -> Result<P1Out> {
    let mut state = *initial_state;

    let mut pos: i32 = 0;
    let mut tape: FnvHashSet<i32> = FnvHashSet::with_hasher(FnvBuildHasher::default());

    for _ in 0..*iterations {
      let is_set = tape.contains(&pos);
      let options = instructions.get(&state).unwrap();
      let instruction = if is_set { &options.1 } else { &options.0 };
      if instruction.write {
        tape.insert(pos);
      } else {
        tape.remove(&pos);
      }
      if instruction.movement == Rotation::L {
        pos -= 1;
      } else {
        pos += 1;
      };
      state = instruction.next_state;
    }
    Ok(tape.len())
  }

  fn part2(&self, _: &Parsed, _: Option<String>) -> Result<P2Out> {
    // On day 25, there is no part 2. This star is granted by earning the other 49.
    Ok(0)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2017, 25)
}
