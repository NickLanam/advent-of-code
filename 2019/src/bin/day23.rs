use advent_lib::runner::{Day, PartId};
use advent_of_code_2019::intcode::{Execution, execute};
use anyhow::{Result, anyhow};

type P1Out = usize;
type P2Out = usize;
type Parsed = Vec<i64>;

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(lines[0].split(',').map(|n| n.parse().unwrap()).collect())
  }

  fn part1(&self, instructions: &Parsed, _: Option<String>) -> Result<P1Out> {
    let mut executors: Vec<Result<Execution>> = (0..50)
      .map(|n| execute(instructions, &[n], Some(0), None))
      .collect();

    // We run all CPUs until they block waiting for input.
    // Then, we figure out what packets they sent to each other.
    // Then, we give each waiting program whatever it was sent, or [-1] if nothing was.
    // That simulates the "they're all non-blocking" requirement, which does not specify at all
    // how it wants us to deal with concurrency. That is to say, I'm guessing this is how it's wanted.
    // Repeat until someone tries to send a packet to address 255, the payload of which is our answer.
    loop {
      // Capture output packets first.
      // If any of them are for address 255, return that payload immediately.
      // NOTE: There's no case in my input where two executors do this in the same cycle,
      // so we don't need to figure out which one is "first" here. The puzzle doesn't specify anyway.
      let mut pending_inputs: Vec<Vec<i64>> = (0..50).map(|_| vec![]).collect();
      for (addr, result) in executors.iter().enumerate() {
        if let Ok(execution) = result {
          for output in execution.outputs.chunks_exact(3) {
            let [dst, x, y] = *output else {
              return Err(anyhow!("chunks_exact broke"));
            };
            if dst == 255 {
              return Ok(y as usize);
            }
            pending_inputs[dst as usize].push(x);
            pending_inputs[dst as usize].push(y);
          }
        } else if let Err(err) = result {
          // Throw the error up, wrapped in the target result type
          return Err(anyhow!(
            format!("Executor {} threw an error", addr) + err.to_string().as_str()
          ));
        }
      }

      // For any executors who are waiting for input, send those packets (or [-1] if nothing is queued)
      let mut next_executors = vec![];
      for (addr, result) in executors.iter().enumerate() {
        if let Ok(result) = result {
          let inputs = if pending_inputs[addr].is_empty() {
            vec![-1]
          } else {
            pending_inputs[addr].clone()
          };
          next_executors.push(execute(
            &result.final_tape,
            &inputs,
            Some(result.pc),
            Some(result.ro),
          ));
        }
      }
      executors = next_executors;
    }
  }

  fn part2(&self, instructions: &Parsed, _: Option<String>) -> Result<P2Out> {
    let mut executors: Vec<Result<Execution>> = (0..50)
      .map(|n| execute(instructions, &[n], Some(0), None))
      .collect();

    // Very similar to part 1, except the stop condition is no longer "packet sent to 255".
    // We watch for those packets, and remember the most recent, then emit that to addr 0
    // whenever a cycle goes by where all executors are awaiting input but none sent any.
    // We track what we send to address 0 this way; the first time we see the same Y value
    // twice, that's our answer.
    let mut latest_nat: Option<i64> = None;

    loop {
      let mut nat_packet: Option<(i64, i64)> = None;

      let mut any_pending = false;
      let mut pending_inputs: Vec<Vec<i64>> = (0..50).map(|_| vec![]).collect();
      for (addr, result) in executors.iter().enumerate() {
        if let Ok(execution) = result {
          for output in execution.outputs.chunks_exact(3) {
            let [dst, x, y] = *output else {
              return Err(anyhow!("chunks_exact broke"));
            };
            if dst == 255 {
              nat_packet = Some((x, y));
              continue;
            }
            any_pending = true;
            pending_inputs[dst as usize].push(x);
            pending_inputs[dst as usize].push(y);
          }
        } else if let Err(err) = result {
          // Throw the error up, wrapped in the target result type
          return Err(anyhow!(
            format!("Executor {} threw an error", addr) + err.to_string().as_str()
          ));
        }
      }

      // Idle: push the stored packet to address 0.
      // If the most recent such packet we sent had the same Y value, we're done.
      if !any_pending && let Some(nat_packet) = nat_packet {
        if let Some(latest_nat) = latest_nat
          && nat_packet.1 == latest_nat
        {
          return Ok(nat_packet.1 as usize);
        } else {
          latest_nat = Some(nat_packet.1);
        }
        pending_inputs[0].push(nat_packet.0);
        pending_inputs[0].push(nat_packet.1);
      }

      // For any executors who are waiting for input, send those packets (or [-1] if nothing is queued)
      let mut next_executors = vec![];
      for (addr, result) in executors.iter().enumerate() {
        if let Ok(result) = result {
          let inputs = if pending_inputs[addr].is_empty() {
            vec![-1]
          } else {
            pending_inputs[addr].clone()
          };
          next_executors.push(execute(
            &result.final_tape,
            &inputs,
            Some(result.pc),
            Some(result.ro),
          ));
        }
      }
      executors = next_executors;
    }
  }
}

fn main() -> Result<()> {
  Solver {}.run(2019, 23)
}
