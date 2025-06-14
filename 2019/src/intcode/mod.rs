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
      // println!("    read({pos}, {is_immediate}) -> {imm}");
      Ok(imm)
    } else if imm >= 0 && (imm as usize) < tape.len() {
      let followed = tape.get(imm as usize);
      if let Some(&out) = followed {
        // println!("    read({pos}, {is_immediate}) -> {imm} -> {out}");
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
  let &encoded = tape
    .get(pc)
    .with_context(|| "Program counter {pc} went out of bounds")?;

  let opcode = encoded % 100;
  let a_is_immediate = (encoded / 100) % 10 == 1;
  let b_is_immediate = (encoded / 1_000) % 10 == 1;

  // "Parameters that an instruction writes to will never be in immediate mode."
  // Assuming that to mean addresses are just addresses, not pointers to addresses.
  // let c_is_immediate = (encoded / 10_00) % 10 == 1;

  // println!(
  //   "pc={pc}; [{:?}={opcode}+{a_is_immediate}+{b_is_immediate}, {:?}, {:?}, {:?}]",
  //   tape.get(pc),
  //   tape.get(pc + 1),
  //   tape.get(pc + 2),
  //   tape.get(pc + 3)
  // );

  match opcode {
    // ADD
    1 => {
      let a = read_parameter(tape, pc + 1, a_is_immediate).with_context(|| {
        format!("ADD instruction, opcode {encoded}, param A, immediate mode? {a_is_immediate}")
      })?;
      let b = read_parameter(tape, pc + 2, b_is_immediate).with_context(|| {
        format!("ADD instruction, opcode {encoded}, param B, immediate mode? {b_is_immediate}")
      })?;
      // "Parameters that an instruction writes to will never be in immediate mode."
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
      // "Parameters that an instruction writes to will never be in immediate mode."
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
      let b_addr = read_parameter(tape, pc + 2, b_is_immediate)
        .with_context(|| format!("JUMP_IF_TRUE instruction, opcode {encoded}, fetching addr, is it direct? {b_is_immediate}"))?;
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
      let b_addr = read_parameter(tape, pc + 2, b_is_immediate)
        .with_context(|| format!("JUMP_IF_FALSE instruction, opcode {encoded}, fetching addr, is it direct? {b_is_immediate}"))?;
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
      // "Parameters that an instruction writes to will never be in immediate mode."
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
      // "Parameters that an instruction writes to will never be in immediate mode."
      let c_addr =
        read_parameter(tape, pc + 3, true).context("EQUALS instruction, opcode {encoded}")?;
      if c_addr < 0 || (c_addr as usize) >= tape.len() {
        bail!("EQUALS instruction would store result out of bounds");
      }
      Ok(Instruction {
        size: 4,
        action: ParsedInstruction::Equals(a, b, c_addr as usize),
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
  pub halted: bool,
}
pub fn execute(initial_tape: &[i64], inputs: &[i64], in_pc: Option<usize>) -> Result<Execution> {
  let mut tape = initial_tape.to_owned();
  let mut outputs = vec![];
  let mut input_reader = inputs.iter();
  let mut pc = in_pc.unwrap_or(0);
  let mut halted = false;

  while pc < tape.len() {
    // Cant' do this in advance, as instructions can modify each other.
    let instruction = get_instruction(&tape, pc)?;
    match instruction.action {
      ParsedInstruction::Add(a, b, dest) => {
        // println!("  ADD {a}+{b} -> dest={dest}");
        tape[dest] = a + b;
        pc += instruction.size;
      }
      ParsedInstruction::Mul(a, b, dest) => {
        // println!("  MUL {a}+{b} -> dest={dest}");
        tape[dest] = a * b;
        pc += instruction.size;
      }
      ParsedInstruction::Input(addr) => {
        let next_input = input_reader.next();
        if let Some(&next_input) = next_input {
          tape[addr] = next_input;
          pc += instruction.size;
        } else if in_pc.is_none() {
          // When in_pc is None, we don't want yielding behavior.
          bail!("Tape tried to read more inputs than were given");
        } else {
          // Yield: returns the current state (including pc) so that
          // the program can be resumed with more inputs later.
          halted = false;
          break;
        }
        // println!("  Input {next_input} to address {addr}");
      }
      ParsedInstruction::Output(a) => {
        // println!("  Output {a} to outputs array");
        outputs.push(a);
        pc += instruction.size;
      }
      ParsedInstruction::JumpIfTrue(a, dest) => {
        // println!(
        //   "  If {a}!=0, jump to {dest}, else just go to {}",
        //   pc + instruction.size
        // );
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
        // println!(
        //   "  If {a}==0, jump to {dest}, else just go to {}",
        //   pc + instruction.size
        // );
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
        // println!("  Set {dest} = if {a} < {b} then 1 else 0");
        tape[dest] = if a < b { 1 } else { 0 };
        pc += instruction.size;
      }
      ParsedInstruction::Equals(a, b, dest) => {
        // println!("  Set {dest} = if {a} == {b} then 1 else 0");
        tape[dest] = if a == b { 1 } else { 0 };
        pc += instruction.size;
      }
      ParsedInstruction::Halt => {
        // println!("  HALT INSTRUCTION");
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
    halted,
  })
}
