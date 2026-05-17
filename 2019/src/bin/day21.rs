use advent_lib::runner::{Day, PartId};
use advent_of_code_2019::intcode::execute;
use anyhow::{Result, bail};

type P1Out = usize;
type P2Out = usize;
type Parsed = Vec<i64>;

fn springcode_to_intcode_input(lines: &Vec<&str>, extend_range: bool) -> Result<Vec<i64>> {
  if lines.len() > 15 {
    bail!(
      "The bot can only remember at most 15 instructions, got {}",
      lines.len()
    )
  }
  Ok(
    lines
      .iter()
      .chain(&[if extend_range { "RUN" } else { "WALK" }])
      .map(|line| {
        line
          .chars()
          .collect::<Vec<char>>()
          .iter()
          .chain(&['\n'])
          .map(|&c| c as u8 as i64)
          .collect::<Vec<i64>>()
      })
      .flatten()
      .collect(),
  )
}

fn print_springcode_output(outputs: &Vec<i64>) {
  for &n in outputs.iter() {
    // The main printable range in ASCII, plus newline (10)
    if n == 10 || (n >= 32 && n <= 126) {
      print!("{}", n as u8 as char);
    } else {
      print!("{n}");
    }
  }
}

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(lines[0].split(',').map(|n| n.parse().unwrap()).collect())
  }

  fn part1(&self, instructions: &Parsed, _: Option<String>) -> Result<P1Out> {
    // Naive program: if there's ground to land on in four squares,
    // and there is not ground in A or B or C, jump.
    // If we could see one space further than we can jump, we could wait until
    // the last possible moment and save some hull damage. But that's part 2.
    let inputs = springcode_to_intcode_input(
      &vec![
        "OR A T", "AND B T", "AND C T", "NOT T T", "AND D T", "OR T J",
      ],
      false,
    )?;
    let result = execute(instructions, &inputs, None, None)?;
    let answer = *result.outputs.last().unwrap();
    if answer > 255 {
      Ok(answer as usize)
    } else {
      print_springcode_output(&result.outputs);
      bail!("Program did not return a hull damage value");
    }
  }

  fn part2(&self, instructions: &Parsed, _: Option<String>) -> Result<P2Out> {
    // Scenarios:
    // - @
    //   #.--#.---- -> MUST JUMP (or we lose)
    // - @
    //   ##..#.--#- -> MUST JUMP (won't be able to next frame)
    // - @
    //   ##--##---- -> WAIT
    // - @
    //   #.--.----- -> CAN'T JUMP (we already failed if this happens)
    //
    // Logic: J = !A || ((!B || !C) && D && H)
    //  That is, we jump only when we'll die in the next 8 steps if we don't
    let inputs = springcode_to_intcode_input(
      &vec![
        "NOT B J", "NOT C T", "OR T J", "AND D J", "AND H J", "NOT A T", "OR T J",
      ],
      true,
    )?;
    let result = execute(instructions, &inputs, None, None)?;
    let answer = *result.outputs.last().unwrap();
    if answer > 255 {
      Ok(answer as usize)
    } else {
      print_springcode_output(&result.outputs);
      bail!("Program did not return a hull damage value");
    }
  }
}

fn main() -> Result<()> {
  Solver {}.run(2019, 21)
}
