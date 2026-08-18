use advent_lib::runner::{Day, PartId};
use anyhow::{Result, bail};

type P1Out = u64;
type P2Out = u64;
type Parsed = (Vec<u64>, usize);

#[allow(clippy::ptr_arg)]
fn find_invalid_index(list: &Vec<u64>) -> Result<usize> {
  // A ring buffer makes this pretty fast, though there's probably an even faster way I'm not thinking of
  let mut ring: usize = 0;
  let mut buf = [0_u64; 25];
  for &n in list.iter().take(25) {
    buf[ring] = n;
    ring += 1;
  }
  ring = 0;
  for (i, &next) in list.iter().enumerate().skip(25) {
    let mut passes_checksum = false;
    for (j, &check) in buf.iter().enumerate() {
      if check < next && buf.iter().skip(j + 1).any(|&other| next == check + other) {
        passes_checksum = true;
        break;
      }
    }
    if !passes_checksum {
      return Ok(i);
    }
    buf[ring] = next;
    ring += 1;
    ring %= 25;
  }
  bail!("Every value was okay, which means no solution")
}

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    let items = lines.iter().map(|n| n.parse().unwrap()).collect();
    let first_invalid_index = find_invalid_index(&items)?;
    Ok((items, first_invalid_index))
  }

  fn part1(&self, (list, invalid_index): &Parsed, _: Option<String>) -> Result<P1Out> {
    Ok(list[*invalid_index])
  }

  fn part2(&self, (list, invalid_index): &Parsed, _: Option<String>) -> Result<P2Out> {
    let goal = list[*invalid_index];
    // Walk with two cursors, keeping a rolling sum, to make the scan do fewer calculations
    // Theretically, there's an even faster walk: walk j out to the end, then walk i towards it,
    // then shift both left one space and walk i back to one step after beginning, then j towards it,
    // and do this cocktail shaker thing until done. It'd definitely be faster, but we're already below 60µs,
    // so not worth it.
    let mut sum: u64;
    'left: for i in 0..(invalid_index - 1) {
      sum = list[i];
      for j in (i + 1)..*invalid_index {
        sum += list[j];
        if sum > goal {
          continue 'left; // Shortcut
        } else if sum == goal {
          let mut min: u64 = u64::MAX;
          let mut max: u64 = u64::MIN;
          for &n in list[i..=j].iter() {
            min = min.min(n);
            max = max.max(n);
          }
          return Ok(min + max);
        }
      }
    }
    bail!("Failed to find");
  }
}

fn main() -> Result<()> {
  Solver {}.run(2020, 9)
}
