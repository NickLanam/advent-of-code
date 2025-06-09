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
  Halt,
}

#[derive(Debug)]
pub struct Instruction {
  action: ParsedInstruction,
  size: usize,
}

fn read_parameter(tape: &[i64], pos: usize, is_immediate: bool) -> Result<i64> {
  let immediate = tape.get(pos);
  if let Some(&imm) = immediate {
    if is_immediate {
      Ok(imm)
    } else if imm >= 0 && (imm as usize) < tape.len() {
      let followed = tape.get(imm as usize);
      if let Some(&out) = followed {
        Ok(out)
      } else {
        bail!("Parameter at {pos} = {imm}, which failed to reference a value")
      }
    } else {
      bail!("Parameter at {pos} = {imm}, which refers to a position out of bounds")
    }
  } else {
    bail!("Could not read immediate value at {pos}")
  }
}

fn get_instruction(tape: &[i64], pc: usize) -> Result<Instruction> {
  let &encoded = tape.get(pc).context("Program counter went out of bounds")?;

  let opcode = encoded % 100;
  let a_is_immediate = (encoded / 100) % 10 == 1;
  let b_is_immediate = (encoded / 1_000) % 10 == 1;
  // Unused in instructions up to and including day 5, uncomment on a later day when used.
  // let c_is_immediate = (opcode / 10_00) % 10 == 1;

  match opcode {
    // ADD
    1 => {
      let a = read_parameter(tape, pc + 1, a_is_immediate).with_context(|| {
        format!("ADD instruction, opcode {encoded}, param A, immediate mode? {a_is_immediate}")
      })?;
      let b = read_parameter(tape, pc + 2, b_is_immediate).with_context(|| {
        format!("ADD instruction, opcode {encoded}, param B, immediate mode? {b_is_immediate}")
      })?;
      let c_addr =
        read_parameter(tape, pc + 3, true).context("ADD instruction, opcode {encoded}")?;
      if c_addr < 0 || (c_addr as usize) >= tape.len() {
        bail!("Add instruction would store result out of bounds");
      }
      Ok(Instruction {
        size: 4,
        action: ParsedInstruction::Add(a, b, c_addr as usize),
      })
    }
    // MUL
    2 => {
      let a = read_parameter(tape, pc + 1, a_is_immediate).with_context(|| {
        format!("MUL instruction, opcode {encoded}, param A, immediate mode? {a_is_immediate}")
      })?;
      let b = read_parameter(tape, pc + 2, b_is_immediate).with_context(|| {
        format!("MUL instruction, opcode {encoded}, param B, immediate mode? {b_is_immediate}")
      })?;
      let c_addr = read_parameter(tape, pc + 3, true)
        .with_context(|| format!("MUL instruction, opcode {encoded}"))?;
      if c_addr < 0 || (c_addr as usize) >= tape.len() {
        bail!("Mul instruction would store result out of bounds");
      }
      Ok(Instruction {
        size: 4,
        action: ParsedInstruction::Mul(a, b, c_addr as usize),
      })
    }
    // INPUT
    3 => {
      let a_addr = read_parameter(tape, pc + 1, true)
        .with_context(|| format!("INPUT instruction, opcode {encoded}"))?;
      Ok(Instruction {
        size: 2,
        action: ParsedInstruction::Input(a_addr as usize),
      })
    }
    // OUTPUT
    4 => {
      let a = read_parameter(tape, pc + 1, a_is_immediate).with_context(|| {
        format!("OUTPUT instruction, opcode {encoded}, param A, immediate mode? {a_is_immediate}")
      })?;
      Ok(Instruction {
        size: 2,
        action: ParsedInstruction::Output(a),
      })
    }
    // JUMP_IF_TRUE
    5 => {
      let a = read_parameter(tape, pc + 1, a_is_immediate).with_context(|| {
        format!(
          "JUMP_IF_TRUE instruction, opcode {encoded}, param A, immediate mode? {a_is_immediate}"
        )
      })?;
      let b_addr = read_parameter(tape, pc + 2, true)
        .with_context(|| format!("JUMP_IF_TRUE instruction, opcode {encoded}, fetching addr"))?;
      Ok(Instruction {
        size: 3,
        action: ParsedInstruction::JumpIfTrue(a, b_addr as usize),
      })
    }
    // JUMP_IF_FALSE
    6 => {
      let a = read_parameter(tape, pc + 1, a_is_immediate).with_context(|| {
        format!(
          "JUMP_IF_FALSE instruction, opcode {encoded}, param A, immediate mode? {a_is_immediate}"
        )
      })?;
      let b_addr = read_parameter(tape, pc + 2, true)
        .with_context(|| format!("JUMP_IF_FALSE instruction, opcode {encoded}, fetching addr"))?;
      Ok(Instruction {
        size: 3,
        action: ParsedInstruction::JumpIfFalse(a, b_addr as usize),
      })
    }
    // LESS_THAN
    7 => {
      let a = read_parameter(tape, pc + 1, a_is_immediate).with_context(|| {
        format!(
          "LESS_THAN instruction, opcode {encoded}, param A, immediate mode? {a_is_immediate}"
        )
      })?;
      let b = read_parameter(tape, pc + 2, b_is_immediate).with_context(|| {
        format!(
          "LESS_THAN instruction, opcode {encoded}, param B, immediate mode? {b_is_immediate}"
        )
      })?;
      let c_addr =
        read_parameter(tape, pc + 3, true).context("LESS_THAN instruction, opcode {encoded}")?;
      if c_addr < 0 || (c_addr as usize) >= tape.len() {
        bail!("LESS_THAN instruction would store result out of bounds");
      }
      Ok(Instruction {
        size: 4,
        action: ParsedInstruction::LessThan(a, b, c_addr as usize),
      })
    }
    // EQUALS
    8 => {
      let a = read_parameter(tape, pc + 1, a_is_immediate).with_context(|| {
        format!("EQUALS instruction, opcode {encoded}, param A, immediate mode? {a_is_immediate}")
      })?;
      let b = read_parameter(tape, pc + 2, b_is_immediate).with_context(|| {
        format!("EQUALS instruction, opcode {encoded}, param B, immediate mode? {b_is_immediate}")
      })?;
      let c_addr =
        read_parameter(tape, pc + 3, true).context("EQUALS instruction, opcode {encoded}")?;
      if c_addr < 0 || (c_addr as usize) >= tape.len() {
        bail!("EQUALS instruction would store result out of bounds");
      }
      Ok(Instruction {
        size: 4,
        action: ParsedInstruction::LessThan(a, b, c_addr as usize),
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
}
pub fn execute(initial_tape: &[i64], inputs: &[i64]) -> Result<Execution> {
  let mut tape = initial_tape.to_owned();
  let mut outputs = vec![];
  let mut input_reader = inputs.iter();
  let mut pc = 0;

  while pc < tape.len() {
    let instruction = get_instruction(&tape, pc)?;
    match instruction.action {
      ParsedInstruction::Add(a, b, dest) => {
        tape[dest] = a + b;
        pc += instruction.size;
      }
      ParsedInstruction::Mul(a, b, dest) => {
        tape[dest] = a * b;
        pc += instruction.size;
      }
      ParsedInstruction::Input(addr) => {
        tape[addr] = *input_reader
          .next()
          .context("Ran out of inputs, but another was requested")?;
        pc += instruction.size;
      }
      ParsedInstruction::Output(a) => {
        outputs.push(a);
        pc += instruction.size;
      }
      ParsedInstruction::JumpIfTrue(a, dest) => {
        if a != 0 {
          if dest >= tape.len() {
            bail!("JUMP_IF_TRUE tried to jump to {dest}");
          }
          pc = dest;
        } else {
          pc += instruction.size;
        }
      }
      ParsedInstruction::JumpIfFalse(a, dest) => {
        if a == 0 {
          if dest >= tape.len() {
            bail!("JUMP_IF_FALSE tried to jump to {dest}");
          }
          pc = dest;
        } else {
          pc += instruction.size;
        }
      }
      ParsedInstruction::LessThan(a, b, dest) => {
        tape[dest] = if a < b { 1 } else { 0 };
        pc += instruction.size;
      }
      ParsedInstruction::Equals(a, b, dest) => {
        tape[dest] = if a == b { 1 } else { 0 };
        pc += instruction.size;
      }
      ParsedInstruction::Halt => {
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
  })
}
