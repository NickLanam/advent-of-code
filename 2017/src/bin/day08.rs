use advent_lib::runner::{Day, PartId};
use anyhow::{Context, Result};
use fnv::{FnvBuildHasher, FnvHashMap};

type P1Out = i32;
type P2Out = i32;
type Parsed = (i32, i32);

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    // This one is simple enough to solve both halves in the parse function.
    let mut registers: FnvHashMap<&str, i32> =
      FnvHashMap::with_capacity_and_hasher(lines.len(), FnvBuildHasher::default());
    let mut record = 0;

    for line in lines.iter() {
      let mut parts = line.split_whitespace();
      let reg_a = parts.next().context("")?;
      let inc_dec = parts.next().context("")?;
      let amt: i32 = parts.next().context("")?.parse()?;
      parts.next(); // Skip the "if"
      let reg_b = parts.next().context("")?;
      let op = parts.next().context("")?;
      let test: i32 = parts.next().context("")?.parse()?;

      let &b = registers.get(reg_b).unwrap_or(&0);
      let should_act = match op {
        ">" => b > test,
        "<" => b < test,
        ">=" => b >= test,
        "<=" => b <= test,
        "==" => b == test,
        "!=" => b != test,
        _ => panic!("Unknown op in line {line}"),
      };
      if should_act {
        let a = registers.entry(reg_a).or_insert(0);
        if inc_dec == "inc" {
          *a += amt;
        } else if inc_dec == "dec" {
          *a -= amt;
        } else {
          panic!("Unknown inc/dec in line {line}");
        }
        record = record.max(*a);
      }
    }

    let (_, &final_highest) = registers.iter().max_by_key(|&(_, &x)| x).context("")?;
    Ok((final_highest, record))
  }

  fn part1(&self, &(final_highest, _): &Parsed, _: Option<String>) -> Result<P1Out> {
    Ok(final_highest)
  }

  fn part2(&self, &(_, record): &Parsed, _: Option<String>) -> Result<P2Out> {
    Ok(record)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2017, 8)
}
