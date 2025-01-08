use advent_lib::runner::{Day, PartId};
use anyhow::Result;
use fnv::{FnvBuildHasher, FnvHashMap};

type P1Out = u64;
type P2Out = u64;
type Parsed = Vec<Sue>;

type Sue = FnvHashMap<String, u8>;

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    if sample_name.is_some() {
      return Ok(vec![]);
    }
    Ok(
      lines
        .into_iter()
        .map(|line| {
          let mut m: FnvHashMap<String, u8> = FnvHashMap::with_hasher(FnvBuildHasher::default());
          let (_, right) = line.split_once(": ").unwrap();
          for item in right.split(", ") {
            let (what, amount) = item.split_once(": ").unwrap();
            m.insert(what.to_string(), amount.to_string().parse().unwrap());
          }
          m
        })
        .collect::<Parsed>(),
    )
  }

  fn part1(&self, aunts: &Parsed, sample_name: Option<String>) -> Result<P1Out> {
    // This puzzle doesn't have a sample input
    if sample_name.is_some() {
      return Ok(1);
    }
    'aunts: for (i, aunt) in aunts.iter().enumerate() {
      for (what, amount) in [
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
      ] {
        if aunt.contains_key(what) && *aunt.get(what).unwrap() != amount {
          continue 'aunts;
        }
      }
      return Ok((i as u64) + 1);
    }
    Ok(0)
  }

  fn part2(&self, aunts: &Parsed, sample_name: Option<String>) -> Result<P2Out> {
    if sample_name.is_some() {
      return Ok(1);
    }
    'aunts: for (i, aunt) in aunts.iter().enumerate() {
      for (what, ok_check) in [
        ("children", Box::<fn(u8) -> bool>::new(|n| n == 3)),
        ("cats", Box::<fn(u8) -> bool>::new(|n| n > 7)),
        ("samoyeds", Box::<fn(u8) -> bool>::new(|n| n == 2)),
        ("pomeranians", Box::<fn(u8) -> bool>::new(|n| n < 3)),
        ("akitas", Box::<fn(u8) -> bool>::new(|n| n == 0)),
        ("vizslas", Box::<fn(u8) -> bool>::new(|n| n == 0)),
        ("goldfish", Box::<fn(u8) -> bool>::new(|n| n < 5)),
        ("trees", Box::<fn(u8) -> bool>::new(|n| n > 3)),
        ("cars", Box::<fn(u8) -> bool>::new(|n| n == 2)),
        ("perfumes", Box::<fn(u8) -> bool>::new(|n| n == 1)),
      ] {
        if aunt.contains_key(what) && !ok_check(*aunt.get(what).unwrap()) {
          continue 'aunts;
        }
      }
      return Ok((i as u64) + 1);
    }
    Ok(0)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2015, 16)
}
