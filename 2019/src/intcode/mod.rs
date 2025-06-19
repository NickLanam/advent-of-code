use anyhow::{Context, Result, bail};

#[derive(Debug)]
pub enum ParsedInstruction {
  Add(i64, i64, usize),
  Mul(i64, i64, usize),
  Input(usize),
  Output(i64),
  JumpIfTrue(i64, usize),
  JumpIfFalse(i64, usize),
  LessThan(i64, i64, usize),
  Equals(i64, i64, usize),
  AdjustRelativeBaseOffset(i64),
  Halt,
}

#[derive(Debug)]
pub struct Instruction {
  action: ParsedInstruction,
  size: usize,
}

fn read_parameter(
  tape: &[i64],
  pos: usize,
  offset: i64,
  mode: i64,
  want_address: bool,
) -> Result<i64> {
  let imm = *tape.get(pos).unwrap_or(&0);
  if mode == 2 && offset + imm < 0 {
    bail!(
      "Mode 2, offset={offset}, imm={imm}, would try to go to {}",
      offset + imm
    );
  }
  if want_address {
    // "Parameters than an instruction writes to will never be in immediate mode".
    // In fact, they'll return the address instead of following it, so that it
    // can be written to or jumped to. They totally can be in mode 1.
    match mode {
      0 | 1 => Ok(imm),
      2 => Ok(offset + imm),
      _ => bail!("Invalid mode: {mode}"),
    }
  } else {
    match mode {
      0 => Ok(*tape.get(imm as usize).unwrap_or(&0)),
      1 => Ok(imm),
      2 => Ok(*tape.get((imm + offset) as usize).unwrap_or(&0)),
      _ => bail!("Invalid mode: {mode}"),
    }
  }
}

fn get_instruction(tape: &[i64], pc: usize, ro: i64) -> Result<Instruction> {
  let &encoded = tape
    .get(pc)
    .with_context(|| "Program counter {pc} went out of bounds")?;

  let opcode = encoded % 100;
  // Modes: 0 = position, 1 = immediate, 2 = relative
  let a_mode = (encoded / 100) % 10;
  let b_mode = (encoded / 1_000) % 10;
  let c_mode = (encoded / 10_000) % 10;

  // println!(
  //   "pc={pc}; [{:?}={opcode}+{a_mode}+{b_mode}+{c_mode}, {:?}, {:?}, {:?}]",
  //   tape.get(pc),
  //   tape.get(pc + 1),
  //   tape.get(pc + 2),
  //   tape.get(pc + 3)
  // );

  match opcode {
    // ADD
    1 => {
      let a = read_parameter(tape, pc + 1, ro, a_mode, false)
        .with_context(|| format!("ADD instruction, opcode {encoded}, param A, mode? {a_mode}"))?;
      let b = read_parameter(tape, pc + 2, ro, b_mode, false)
        .with_context(|| format!("ADD instruction, opcode {encoded}, param B, mode? {b_mode}"))?;
      let c_addr = read_parameter(tape, pc + 3, ro, c_mode, true)
        .context("ADD instruction, opcode {encoded}")?;
      if c_addr < 0 {
        bail!("Add instruction would store result out of bounds");
      }
      Ok(Instruction {
        size: 4,
        action: ParsedInstruction::Add(a, b, c_addr as usize),
      })
    }
    // MUL
    2 => {
      let a = read_parameter(tape, pc + 1, ro, a_mode, false)
        .with_context(|| format!("MUL instruction, opcode {encoded}, param A, mode? {a_mode}"))?;
      let b = read_parameter(tape, pc + 2, ro, b_mode, false)
        .with_context(|| format!("MUL instruction, opcode {encoded}, param B, mode? {b_mode}"))?;
      let c_addr = read_parameter(tape, pc + 3, ro, c_mode, true)
        .with_context(|| format!("MUL instruction, opcode {encoded}"))?;
      if c_addr < 0 {
        bail!("Mul instruction would store result out of bounds");
      }
      Ok(Instruction {
        size: 4,
        action: ParsedInstruction::Mul(a, b, c_addr as usize),
      })
    }
    // INPUT
    3 => {
      // "Parameters that an instruction writes to will never be in immediate mode."
      // Which means the PARAMETER is read as an immediate (the address to write to),
      // but the mode can also be 2 (relative) on day 9, which is a special case.
      let a_addr = read_parameter(tape, pc + 1, ro, a_mode, true)
        .with_context(|| format!("INPUT instruction, opcode {encoded}"))?;
      Ok(Instruction {
        size: 2,
        action: ParsedInstruction::Input(a_addr as usize),
      })
    }
    // OUTPUT
    4 => {
      let a = read_parameter(tape, pc + 1, ro, a_mode, false).with_context(|| {
        format!("OUTPUT instruction, opcode {encoded}, param A, immediate mode? {a_mode}")
      })?;
      Ok(Instruction {
        size: 2,
        action: ParsedInstruction::Output(a),
      })
    }
    // JUMP_IF_TRUE
    5 => {
      let a = read_parameter(tape, pc + 1, ro, a_mode, false).with_context(|| {
        format!("JUMP_IF_TRUE instruction, opcode {encoded}, param A, immediate mode? {a_mode}")
      })?;
      let b_addr = read_parameter(tape, pc + 2, ro, b_mode, false).with_context(|| {
        format!("JUMP_IF_TRUE instruction, opcode {encoded}, fetching addr, is it direct? {b_mode}")
      })?;
      Ok(Instruction {
        size: 3,
        action: ParsedInstruction::JumpIfTrue(a, b_addr as usize),
      })
    }
    // JUMP_IF_FALSE
    6 => {
      let a = read_parameter(tape, pc + 1, ro, a_mode, false).with_context(|| {
        format!("JUMP_IF_FALSE instruction, opcode {encoded}, param A, immediate mode? {a_mode}")
      })?;
      let b_addr = read_parameter(tape, pc + 2, ro, b_mode, false).with_context(|| {
        format!(
          "JUMP_IF_FALSE instruction, opcode {encoded}, fetching addr, is it direct? {b_mode}"
        )
      })?;
      Ok(Instruction {
        size: 3,
        action: ParsedInstruction::JumpIfFalse(a, b_addr as usize),
      })
    }
    // LESS_THAN
    7 => {
      let a = read_parameter(tape, pc + 1, ro, a_mode, false).with_context(|| {
        format!("LESS_THAN instruction, opcode {encoded}, param A, immediate mode? {a_mode}")
      })?;
      let b = read_parameter(tape, pc + 2, ro, b_mode, false).with_context(|| {
        format!("LESS_THAN instruction, opcode {encoded}, param B, immediate mode? {b_mode}")
      })?;
      // "Parameters that an instruction writes to will never be in immediate mode."
      let c_addr = read_parameter(tape, pc + 3, ro, c_mode, true)
        .context("LESS_THAN instruction, opcode {encoded}")?;
      if c_addr < 0 {
        bail!("LESS_THAN instruction would store result out of bounds");
      }
      Ok(Instruction {
        size: 4,
        action: ParsedInstruction::LessThan(a, b, c_addr as usize),
      })
    }
    // EQUALS
    8 => {
      let a = read_parameter(tape, pc + 1, ro, a_mode, false).with_context(|| {
        format!("EQUALS instruction, opcode {encoded}, param A, immediate mode? {a_mode}")
      })?;
      let b = read_parameter(tape, pc + 2, ro, b_mode, false).with_context(|| {
        format!("EQUALS instruction, opcode {encoded}, param B, immediate mode? {b_mode}")
      })?;
      // "Parameters that an instruction writes to will never be in immediate mode."
      let c_addr = read_parameter(tape, pc + 3, ro, c_mode, true)
        .context("EQUALS instruction, opcode {encoded}")?;
      if c_addr < 0 {
        bail!("EQUALS instruction would store result out of bounds");
      }
      Ok(Instruction {
        size: 4,
        action: ParsedInstruction::Equals(a, b, c_addr as usize),
      })
    }
    // ADJUST_RELATIVE_BASE_OFFSET
    9 => {
      let a = read_parameter(tape, pc + 1, ro, a_mode, false).with_context(|| {
        format!(
          "ADJUST_RELATIVE_BASE_OFFSET instruction, opcode {encoded}, param A, mode? {a_mode}"
        )
      })?;
      Ok(Instruction {
        size: 2,
        action: ParsedInstruction::AdjustRelativeBaseOffset(a),
      })
    }
    // HALT
    99 => Ok(Instruction {
      size: 1,
      action: ParsedInstruction::Halt,
    }),
    _ => {
      bail!("Unrecognized operation: {encoded} at position {pc}")
    }
  }
}

#[derive(Debug)]
pub struct Execution {
  pub final_tape: Vec<i64>,
  pub outputs: Vec<i64>,
  pub pc: usize,
  pub ro: i64,
  pub halted: bool,
}

const DEBUGGING_EXECUTION: bool = false;

pub fn execute(initial_tape: &[i64], inputs: &[i64], in_pc: Option<usize>) -> Result<Execution> {
  let mut tape = initial_tape.to_owned();
  let mut outputs = vec![];
  let mut input_reader = inputs.iter();
  let mut pc = in_pc.unwrap_or(0);
  let mut ro = 0; // Relative offset, adjusted by instruction 9
  let mut halted = false;

  while pc < tape.len() {
    // Cant' do this in advance, as instructions can modify each other.
    let instruction = get_instruction(&tape, pc, ro)?;
    if DEBUGGING_EXECUTION {
      println!(
        "pc={pc}, ro={ro}, op={:?} / {:?} / {:?} / {:?} ; instruction={instruction:?}",
        tape.get(pc).unwrap_or(&0),
        tape.get(pc + 1).unwrap_or(&0),
        tape.get(pc + 2).unwrap_or(&0),
        tape.get(pc + 3).unwrap_or(&0)
      );
    }
    match instruction.action {
      ParsedInstruction::Add(a, b, dest) => {
        if DEBUGGING_EXECUTION {
          println!("  ADD, @{dest} = {a} + {b} = {}", a + b);
        }
        if tape.len() <= dest {
          tape.resize(dest + 1, 0);
        }
        tape[dest] = a + b;
        pc += instruction.size;
      }
      ParsedInstruction::Mul(a, b, dest) => {
        if DEBUGGING_EXECUTION {
          println!("  MUL, @{dest} = {a} * {b} = {}", a * b);
        }
        if tape.len() <= dest {
          tape.resize(dest + 1, 0);
        }
        tape[dest] = a * b;
        pc += instruction.size;
      }
      ParsedInstruction::Input(addr) => {
        let next_input = input_reader.next();
        if let Some(&next_input) = next_input {
          if DEBUGGING_EXECUTION {
            println!("  INPUT {next_input} to @{addr}");
          }
          if tape.len() <= addr {
            tape.resize(addr + 1, 0);
          }
          tape[addr] = next_input;
          pc += instruction.size;
        } else if in_pc.is_none() {
          // When in_pc is None, we don't want yielding behavior.
          bail!("Tape tried to read more inputs than were given");
        } else {
          // Yield: returns the current state (including pc) so that
          // the program can be resumed with more inputs later.
          if DEBUGGING_EXECUTION {
            println!("  YIELD");
          }
          halted = false;
          break;
        }
        // println!("  Input {next_input} to address {addr}");
      }
      ParsedInstruction::Output(a) => {
        if DEBUGGING_EXECUTION {
          println!("  Output {a} to outputs array");
        }
        outputs.push(a);
        pc += instruction.size;
      }
      ParsedInstruction::JumpIfTrue(a, dest) => {
        if DEBUGGING_EXECUTION {
          println!("  JNZ {a}, @{dest}, else @{}", pc + instruction.size);
        }
        if a != 0 {
          if dest >= tape.len() {
            tape.resize(dest + 1, 0);
          }
          pc = dest;
        } else {
          pc += instruction.size;
        }
      }
      ParsedInstruction::JumpIfFalse(a, dest) => {
        if DEBUGGING_EXECUTION {
          println!("  JEZ {a}, @{dest}, else @{}", pc + instruction.size);
        }
        if a == 0 {
          if dest >= tape.len() {
            tape.resize(dest + 1, 0);
          }
          pc = dest;
        } else {
          pc += instruction.size;
        }
      }
      ParsedInstruction::LessThan(a, b, dest) => {
        if DEBUGGING_EXECUTION {
          println!("  @{dest} = {a} < {b} -> {}", if a < b { 1 } else { 0 });
        }
        if tape.len() <= dest {
          tape.resize(dest + 1, 0);
        }
        tape[dest] = if a < b { 1 } else { 0 };
        pc += instruction.size;
      }
      ParsedInstruction::Equals(a, b, dest) => {
        if DEBUGGING_EXECUTION {
          println!("  @{dest} = {a} == {b} -> {}", if a == b { 1 } else { 0 });
        }
        if tape.len() <= dest {
          tape.resize(dest + 1, 0);
        }
        tape[dest] = if a == b { 1 } else { 0 };
        pc += instruction.size;
      }
      ParsedInstruction::AdjustRelativeBaseOffset(a) => {
        if DEBUGGING_EXECUTION {
          println!(
            "  Adjust relative offset: add {a} to {ro} to get ro={}",
            ro + a
          );
        }
        ro += a;
        pc += instruction.size;
      }
      ParsedInstruction::Halt => {
        if DEBUGGING_EXECUTION {
          println!("  HALT");
        }
        halted = true;
        break;
      }
    }
  }
  if pc >= tape.len() {
    bail!("Program counter left the tape without a halt!");
  }

  Ok(Execution {
    final_tape: tape,
    outputs,
    pc,
    ro,
    halted,
  })
}
