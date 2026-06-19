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
      // TODO: What happens if two executors do that in the same cycle? We'd need to know which one
      //   did it "first", and the puzzle does not tell us how to decide what "first" means here.
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

  fn part2(&self, _instructions: &Parsed, _: Option<String>) -> Result<P2Out> {
    Ok(0)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2019, 23)
}
