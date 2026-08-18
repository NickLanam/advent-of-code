use advent_lib::runner::{Day, PartId};
use anyhow::{Result, bail};
use fnv::{FnvBuildHasher, FnvHashMap, FnvHashSet};

type P1Out = usize;
type P2Out = usize;
// (contained_by for part 1, contains for part 2)
type Parsed = (
  FnvHashMap<u64, FnvHashSet<u64>>,
  FnvHashMap<u64, FnvHashMap<u64, usize>>,
);

// This makes parsing AND solving roughly 2x faster than keeping the strings.
// We simply take the first four letters of the two words in a bag name, and cram
// those into a u64, using that as the key. That's enough information to be unique
// on my input; if it doesn't work for you then try a u128 (which would be slower).
fn to_key(name: &str) -> u64 {
  let (adj, col) = name.split_once(' ').unwrap();
  let mut name_key: u64 = 0;
  for c in adj.chars().take(4) {
    name_key <<= 8;
    name_key += c as u8 as u64;
  }
  for c in col.chars().take(4) {
    name_key <<= 8;
    name_key += c as u8 as u64;
  }
  name_key
}

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  // Almost all of the time in this solution is on the parse (400µs). Solving takes <80µs total afterward.
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    let mut contained_by =
      FnvHashMap::with_capacity_and_hasher(lines.len(), FnvBuildHasher::default());
    let mut contains = FnvHashMap::with_capacity_and_hasher(lines.len(), FnvBuildHasher::default());

    for line in lines.iter() {
      let mut inner_contains = FnvHashMap::with_capacity_and_hasher(4, FnvBuildHasher::default());
      let (key_str, rest) = line.split_once(" bags contain ").unwrap();
      let key = to_key(key_str);
      // "contains no other bags" -> nothing to do
      if !rest.starts_with("no") {
        for block in rest.split(", ") {
          // The rsplit is to get rid of the / bags?\.?$/ suffix. Most performant way I found to do it.
          let (count_raw, name_raw) = block.rsplit_once(' ').unwrap().0.split_once(' ').unwrap();
          let name_key = to_key(name_raw);
          let count: usize = count_raw.parse().unwrap();
          let inner_contained_by = contained_by
            .entry(name_key)
            .or_insert_with(|| FnvHashSet::with_capacity_and_hasher(4, FnvBuildHasher::default()));
          inner_contained_by.insert(key);
          inner_contains.insert(name_key, count);
        }
      }
      contains.insert(key, inner_contains);
    }

    Ok((contained_by, contains))
  }

  fn part1(&self, (contained_by, _contains): &Parsed, _: Option<String>) -> Result<P1Out> {
    let mut reachable = FnvHashSet::with_capacity_and_hasher(300, FnvBuildHasher::default());
    let mut cursors: Vec<u64> = vec![to_key("shiny gold")];
    while !cursors.is_empty() {
      cursors = cursors
        .iter()
        .flat_map(|&bag| {
          if let Some(next) = contained_by.get(&bag) {
            next.iter().filter(|&s| !reachable.contains(s)).collect()
          } else {
            vec![]
          }
        })
        .copied()
        .collect();
      for &cursor in cursors.iter() {
        reachable.insert(cursor);
      }
    }
    Ok(reachable.len())
  }

  fn part2(&self, (_contained_by, contains): &Parsed, _: Option<String>) -> Result<P2Out> {
    // We're gonna do a tiny bit of dynamic programming to speed this up. 8 microseconds on a 3900X is pretty good.
    let mut known: FnvHashMap<u64, usize> =
      FnvHashMap::with_capacity_and_hasher(400, FnvBuildHasher::default());
    let shiny_gold_key = to_key("shiny gold");
    let mut frontier = vec![shiny_gold_key];
    while let Some(next) = frontier.pop() {
      let contents = contains.get(&next).unwrap();
      let mut unresolved: Vec<u64> = vec![];
      let mut sum = 0;
      for (&inner_key, &inner_count) in contents.iter() {
        if let Some(&inner_known) = known.get(&inner_key) {
          sum += inner_known * inner_count;
        } else {
          unresolved.push(inner_key);
        }
      }
      if unresolved.is_empty() {
        if next == shiny_gold_key {
          return Ok(sum);
        } else {
          known.insert(next, sum + 1); // Have to count the outer bag too
        }
      } else if !unresolved.is_empty() {
        frontier.push(next);
        for u in unresolved.iter() {
          frontier.push(*u);
        }
      }
    }
    bail!("Failed to walk the graph correctly");
  }
}

fn main() -> Result<()> {
  Solver {}.run(2020, 7)
}
