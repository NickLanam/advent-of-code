use advent_lib::runner::{Day, PartId};
use anyhow::{Context, Result, bail};
use fnv::{FnvBuildHasher, FnvHashMap};

#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
enum BinType {
  Bot,
  Output,
}

type BinId = (BinType, u8);
type Amount = u8;

#[derive(Clone, Debug, PartialEq)]
enum Instruction {
  Give(BinId, BinId, BinId),
  Value(Amount, BinId),
}

type Bins = FnvHashMap<BinId, Vec<Amount>>;

type P1Out = u8;
type P2Out = u64;
type Parsed = Vec<Instruction>;

fn solve(instructions: &Parsed) -> Bins {
  let mut bins: Bins =
    FnvHashMap::with_capacity_and_hasher(instructions.len() * 2, FnvBuildHasher::default());

  let push_to_bin = |bins: &mut Bins, dst: &BinId, amount: &Amount| {
    if bins.contains_key(dst) {
      let entry = bins.get_mut(dst).unwrap();
      entry.push(*amount);
    } else {
      let mut v = Vec::with_capacity(2);
      v.push(*amount);
      bins.insert(dst.to_owned(), v);
    }
  };

  let mut unsatisfied: Vec<Instruction> = instructions.to_vec();

  while !unsatisfied.is_empty() {
    let mut to_remove: Vec<usize> = Vec::with_capacity(unsatisfied.len());
    for (index, inst) in unsatisfied.iter().enumerate() {
      match inst {
        Instruction::Value(amount, dst) => {
          push_to_bin(&mut bins, dst, amount);
          to_remove.push(index);
        }
        Instruction::Give(src, dst_lo, dst_hi) => {
          if let Some(entry) = bins.get(src)
            && entry.len() >= 2
          {
            let lo = entry.iter().min();
            let hi = entry.iter().max();
            if let (Some(lo), Some(hi)) = (lo, hi) {
              // Have to explicitly clone these, otherwise
              // we both immutable and mutable borrow bins.
              let commit_lo = *lo;
              let commit_hi = *hi;
              push_to_bin(&mut bins, dst_lo, &commit_lo);
              push_to_bin(&mut bins, dst_hi, &commit_hi);
              to_remove.push(index);
            }
          }
        }
      }
    }

    // Otherwise we'd be doing concurrent modification.
    // Reversed so as not to change indices along the way.
    for index in to_remove.iter().rev() {
      unsatisfied.remove(*index);
    }
  }
  bins
}

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    let mut out: Vec<Instruction> = Vec::with_capacity(lines.len());
    for line in lines {
      let parts: Vec<&str> = line.split(' ').collect();
      if parts.len() == 12 && line.starts_with("bot") {
        let id1: BinId = (BinType::Bot, parts[1].parse()?);
        let t2: BinType = if parts[5] == "bot" {
          BinType::Bot
        } else {
          BinType::Output
        };
        let id2: BinId = (t2, parts[6].parse()?);
        let t3: BinType = if parts[10] == "bot" {
          BinType::Bot
        } else {
          BinType::Output
        };
        let id3: BinId = (t3, parts[11].parse()?);
        out.push(Instruction::Give(id1, id2, id3));
      } else if parts.len() == 6 && line.starts_with("value") {
        let amount: Amount = parts[1].parse()?;
        let to_type = if parts[4] == "bot" {
          BinType::Bot
        } else {
          BinType::Output
        };
        let bot_id: BinId = (to_type, parts[5].parse()?);
        out.push(Instruction::Value(amount, bot_id));
      } else {
        bail!("Line does not match the pattern: {line}");
      }
    }
    Ok(out)
  }

  fn part1(&self, instructions: &Parsed, sample_name: Option<String>) -> Result<P1Out> {
    let bins = solve(instructions);

    let cmp_seek: Vec<Amount> = if sample_name.is_some() {
      vec![2, 5]
    } else {
      vec![17, 61]
    };

    for ((place_type, place_id), contents) in bins.iter() {
      if *place_type == BinType::Bot && cmp_seek.iter().all(|seek| contents.contains(seek)) {
        return Ok(*place_id);
      }
    }
    bail!("{bins:#?}");
  }

  fn part2(&self, instructions: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    let bins = solve(instructions);
    let pto = BinType::Output;
    let o0 = bins.get(&(pto.clone(), 0)).context("Bin 0 is empty")?[0] as u64;
    let o1 = bins.get(&(pto.clone(), 1)).context("Bin 1 is empty")?[0] as u64;
    let o2 = bins.get(&(pto.clone(), 2)).context("Bin 2 is empty")?[0] as u64;
    Ok(o0 * o1 * o2)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2016, 10)
}
