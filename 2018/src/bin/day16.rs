use advent_lib::runner::{Day, PartId};
use anyhow::Result;

#[derive(Default, Debug)]
struct Instruction {
  op: usize,
  a: usize,
  b: usize,
  c: usize,
}
#[derive(Debug)]
struct Sample {
  pre: [usize; 4],
  post: [usize; 4],
  instruction: Instruction,
}

const MATCHERS: [for<'a> fn(&'a Sample) -> bool; 16] = [
  // addr
  |s: &Sample| -> bool {
    s.post[s.instruction.c] == s.pre[s.instruction.a] + s.pre[s.instruction.b]
  },
  // addi
  |s: &Sample| -> bool { s.post[s.instruction.c] == s.pre[s.instruction.a] + s.instruction.b },
  // mulr
  |s: &Sample| -> bool {
    s.post[s.instruction.c] == s.pre[s.instruction.a] * s.pre[s.instruction.b]
  },
  // muli
  |s: &Sample| -> bool { s.post[s.instruction.c] == s.pre[s.instruction.a] * s.instruction.b },
  // banr
  |s: &Sample| -> bool {
    s.post[s.instruction.c] == s.pre[s.instruction.a] & s.pre[s.instruction.b]
  },
  // bani
  |s: &Sample| -> bool { s.post[s.instruction.c] == s.pre[s.instruction.a] & s.instruction.b },
  // borr
  |s: &Sample| -> bool {
    s.post[s.instruction.c] == s.pre[s.instruction.a] | s.pre[s.instruction.b]
  },
  // bori
  |s: &Sample| -> bool { s.post[s.instruction.c] == s.pre[s.instruction.a] | s.instruction.b },
  // setr
  |s: &Sample| -> bool { s.post[s.instruction.c] == s.pre[s.instruction.a] },
  // seti
  |s: &Sample| -> bool { s.post[s.instruction.c] == s.instruction.a },
  // gtir
  |s: &Sample| -> bool {
    if s.instruction.a > s.pre[s.instruction.b] {
      s.post[s.instruction.c] == 1
    } else {
      s.post[s.instruction.c] == 0
    }
  },
  // gtri
  |s: &Sample| -> bool {
    if s.pre[s.instruction.a] > s.instruction.b {
      s.post[s.instruction.c] == 1
    } else {
      s.post[s.instruction.c] == 0
    }
  },
  // gtrr
  |s: &Sample| -> bool {
    if s.pre[s.instruction.a] > s.pre[s.instruction.b] {
      s.post[s.instruction.c] == 1
    } else {
      s.post[s.instruction.c] == 0
    }
  },
  // eqir
  |s: &Sample| -> bool {
    if s.instruction.a == s.pre[s.instruction.b] {
      s.post[s.instruction.c] == 1
    } else {
      s.post[s.instruction.c] == 0
    }
  },
  // eqri
  |s: &Sample| -> bool {
    if s.pre[s.instruction.a] == s.instruction.b {
      s.post[s.instruction.c] == 1
    } else {
      s.post[s.instruction.c] == 0
    }
  },
  // eqrr
  |s: &Sample| -> bool {
    if s.pre[s.instruction.a] == s.pre[s.instruction.b] {
      s.post[s.instruction.c] == 1
    } else {
      s.post[s.instruction.c] == 0
    }
  },
];

const OPERATORS: [for<'a> fn(&'a Instruction, &'a mut [usize; 4]); 16] = [
  // addr
  |inst, reg| {
    reg[inst.c] = reg[inst.a] + reg[inst.b];
  },
  // addi
  |inst, reg| {
    reg[inst.c] = reg[inst.a] + inst.b;
  },
  // mulr
  |inst, reg| {
    reg[inst.c] = reg[inst.a] * reg[inst.b];
  },
  // muli
  |inst, reg| {
    reg[inst.c] = reg[inst.a] * inst.b;
  },
  // banr
  |inst, reg| {
    reg[inst.c] = reg[inst.a] & reg[inst.b];
  },
  // bani
  |inst, reg| {
    reg[inst.c] = reg[inst.a] & inst.b;
  },
  // borr
  |inst, reg| {
    reg[inst.c] = reg[inst.a] | reg[inst.b];
  },
  // bori
  |inst, reg| {
    reg[inst.c] = reg[inst.a] | inst.b;
  },
  // setr
  |inst, reg| {
    reg[inst.c] = reg[inst.a];
  },
  // seti
  |inst, reg| {
    reg[inst.c] = inst.a;
  },
  // gtir
  |inst, reg| {
    reg[inst.c] = if inst.a > reg[inst.b] { 1 } else { 0 };
  },
  // gtri
  |inst, reg| {
    reg[inst.c] = if reg[inst.a] > inst.b { 1 } else { 0 };
  },
  // gtrr
  |inst, reg| {
    reg[inst.c] = if reg[inst.a] > reg[inst.b] { 1 } else { 0 };
  },
  // eqir
  |inst, reg| {
    reg[inst.c] = if inst.a == reg[inst.b] { 1 } else { 0 };
  },
  // eqri
  |inst, reg| {
    reg[inst.c] = if reg[inst.a] == inst.b { 1 } else { 0 };
  },
  // eqrr
  |inst, reg| {
    reg[inst.c] = if reg[inst.a] == reg[inst.b] { 1 } else { 0 };
  },
];

type P1Out = usize;
type P2Out = usize;
type Parsed = (Vec<Sample>, Vec<Instruction>);

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    let mut samples: Vec<Sample> = vec![];
    let mut instructions: Vec<Instruction> = vec![];

    let mut line_iter = lines.iter();
    while let Some(l0) = line_iter.next() {
      if l0.starts_with('B') {
        let mut pre = [0_usize; 4];
        let mut pre_chars = l0.chars();
        pre[0] = pre_chars.nth(9).unwrap().to_digit(10).unwrap() as usize;
        pre[1] = pre_chars.nth(2).unwrap().to_digit(10).unwrap() as usize;
        pre[2] = pre_chars.nth(2).unwrap().to_digit(10).unwrap() as usize;
        pre[3] = pre_chars.nth(2).unwrap().to_digit(10).unwrap() as usize;

        let mut instruction = Instruction::default();
        let mut inst_parts = line_iter.next().unwrap().split_whitespace();
        instruction.op = inst_parts.next().unwrap().parse()?;
        instruction.a = inst_parts.next().unwrap().parse()?;
        instruction.b = inst_parts.next().unwrap().parse()?;
        instruction.c = inst_parts.next().unwrap().parse()?;

        let mut post = [0_usize; 4];
        let mut post_chars = line_iter.next().unwrap().chars();
        post[0] = post_chars.nth(9).unwrap().to_digit(10).unwrap() as usize;
        post[1] = post_chars.nth(2).unwrap().to_digit(10).unwrap() as usize;
        post[2] = post_chars.nth(2).unwrap().to_digit(10).unwrap() as usize;
        post[3] = post_chars.nth(2).unwrap().to_digit(10).unwrap() as usize;

        samples.push(Sample {
          pre,
          post,
          instruction,
        })
      } else if !l0.is_empty() {
        let mut instruction = Instruction::default();
        let mut inst_parts = l0.split_whitespace();
        instruction.op = inst_parts.next().unwrap().parse()?;
        instruction.a = inst_parts.next().unwrap().parse()?;
        instruction.b = inst_parts.next().unwrap().parse()?;
        instruction.c = inst_parts.next().unwrap().parse()?;
        instructions.push(instruction);
      }
    }

    Ok((samples, instructions))
  }

  fn part1(&self, (samples, _instructions): &Parsed, _: Option<String>) -> Result<P1Out> {
    let mut out = 0;
    'tests: for sample in samples {
      let mut plausible_matches = 0;
      for matcher in MATCHERS.iter() {
        if matcher(sample) {
          plausible_matches += 1;
          if plausible_matches >= 3 {
            out += 1;
            continue 'tests;
          }
        }
      }
    }
    Ok(out)
  }

  fn part2(&self, (samples, instructions): &Parsed, sample_name: Option<String>) -> Result<P2Out> {
    if sample_name.is_some() {
      // The sample does not contain a program at all.
      return Ok(0);
    }

    // Outer index is the remapped ID; inner index is the real one
    let mut possibilities: [[bool; 16]; 16] = [[true; 16]; 16];

    // Remove possibilities until we're out of samples
    for sample in samples {
      #[allow(clippy::needless_range_loop)]
      for i in 0..16 {
        if !MATCHERS[i](sample) {
          possibilities[sample.instruction.op][i] = false;
        }
      }
    }

    // Now, find mappings that are final. Remove those
    // from other possibilities (that is: if we figure out
    // which one has to be `addr`, none of the others are `addr`).
    // When we're done, we'll have final mappings for them all.

    let mut mapping = [usize::MAX; 16];

    // There are still multiple possibilities for some instructions.
    // However, at least one now only has one possibility.
    // Until we've solved them all, find the first such instruction,
    // save the mapping, and remove that possibility from the rest.
    while mapping.iter().any(|&m| m > 15) {
      'poss: for (n, opts) in possibilities.iter().enumerate() {
        if mapping[n] < usize::MAX {
          continue;
        }
        let mut found: Option<usize> = None;
        #[allow(clippy::needless_range_loop)]
        for m in 0..16 {
          if opts[m] {
            if found.is_some() {
              continue 'poss;
            } else {
              found = Some(m);
            }
          }
        }
        if let Some(m) = found {
          mapping[n] = m;
        }
      }
      // Discard possibilities we've already solved
      for m in 0..16 {
        if mapping[m] < usize::MAX {
          for p in possibilities.iter_mut() {
            p[mapping[m]] = false;
          }
        }
      }
    }

    // Now that we know which instruction is which, running the
    // program is the easy part.
    let mut registers = [0_usize; 4];

    for instruction in instructions {
      let op = mapping[instruction.op];
      OPERATORS[op](instruction, &mut registers);
    }

    Ok(registers[0])
  }
}

fn main() -> Result<()> {
  Solver {}.run(2018, 16)
}
