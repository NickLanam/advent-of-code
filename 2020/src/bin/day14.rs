use advent_lib::runner::{Day, PartId};
use anyhow::Result;
use fnv::{FnvBuildHasher, FnvHashMap};

type P1Out = usize;
type P2Out = usize;
struct Group {
  mask: Vec<char>,
  setters: Vec<(usize, usize)>,
}
type Parsed = Vec<Group>;

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    let mut groups = vec![];
    for line in lines.iter() {
      if line.starts_with("mask") {
        let mask = line.split_once(" = ").unwrap().1.chars().collect();
        groups.push(Group {
          mask,
          setters: vec![],
        });
      } else if let Some(g) = groups.last_mut() {
        let binding = line[4..].to_owned();
        let (addr_raw, v_raw) = binding.split_once("] = ").unwrap();
        g.setters
          .push((addr_raw.parse().unwrap(), v_raw.parse().unwrap()));
      }
    }
    Ok(groups)
  }

  fn part1(&self, groups: &Parsed, _: Option<String>) -> Result<P1Out> {
    let mut mem: FnvHashMap<usize, usize> =
      FnvHashMap::with_capacity_and_hasher(600, FnvBuildHasher::default());

    for Group { mask, setters } in groups.iter() {
      for (addr, unmasked) in setters.iter() {
        let mut masked = *unmasked;
        for (i, ch) in mask.iter().rev().enumerate() {
          match ch {
            '0' => {
              masked &= usize::MAX - (1 << i);
            }
            '1' => {
              masked |= 1 << i;
            }
            _ => {}
          }
        }
        mem.insert(*addr, masked);
      }
    }
    Ok(mem.values().sum())
  }

  fn part2(&self, groups: &Parsed, _: Option<String>) -> Result<P2Out> {
    let mut mem: FnvHashMap<usize, usize> =
      FnvHashMap::with_capacity_and_hasher(600, FnvBuildHasher::default());

    for Group { mask, setters } in groups.iter() {
      let base_mask =
        mask.iter().rev().enumerate().fold(
          0,
          |acc, (i, ch)| {
            if *ch == '1' { acc | (1 << i) } else { acc }
          },
        );
      let x_indices: Vec<usize> = mask
        .iter()
        .rev()
        .enumerate()
        .filter_map(|(i, ch)| if *ch == 'X' { Some(i) } else { None })
        .collect();
      for (unmasked_addr, value) in setters.iter() {
        let masked_base_addr = *unmasked_addr | base_mask;
        for counter in 0_usize..(2_usize.pow(x_indices.len() as u32)) {
          let mut addr = masked_base_addr;
          for (x_index, x_offset) in x_indices.iter().enumerate() {
            if (counter & (0b1 << x_index)) > 0 {
              addr |= 0b1 << x_offset;
            } else {
              addr &= usize::MAX - (1 << x_offset);
            }
          }
          mem.insert(addr, *value);
        }
      }
    }
    Ok(mem.values().sum())
  }
}

fn main() -> Result<()> {
  Solver {}.run(2020, 14)
}
