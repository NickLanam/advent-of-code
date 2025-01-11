use advent_lib::runner::{Day, PartId};
use anyhow::{bail, Result};
use fnv::{FnvBuildHasher, FnvHashMap, FnvHashSet};

type P1Out = usize;
type P2Out = usize;
type Parsed = (String, FnvHashMap<String, Vec<String>>);

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    let molecule = &lines[lines.len() - 1];
    let mut transforms: FnvHashMap<String, Vec<String>> =
      FnvHashMap::with_capacity_and_hasher(lines.len() - 2, FnvBuildHasher::default());
    for line in lines[0..(lines.len() - 2)].iter() {
      let (l, r) = line.split_once(" => ").unwrap();
      match transforms.get_mut(l) {
        Some(v) => {
          v.push(r.to_owned());
        }
        None => {
          transforms.insert(l.to_owned(), vec![r.to_owned()]);
        }
      }
    }
    Ok((molecule.to_owned(), transforms))
  }

  fn part1(&self, (molecule, transforms): &Parsed, _sample_name: Option<String>) -> Result<P1Out> {
    let mut states: FnvHashSet<String> = FnvHashSet::with_hasher(FnvBuildHasher::default());
    for (target, replacements) in transforms.iter() {
      for (i, _) in molecule.match_indices(target) {
        for replacement in replacements {
          states.insert(
            [
              &molecule[0..i],
              replacement.as_str(),
              &molecule[(i + target.len())..],
            ]
            .join(""),
          );
        }
      }
    }
    Ok(states.len())
  }

  fn part2(
    &self,
    (input_molecule, transforms): &Parsed,
    sample_name: Option<String>,
  ) -> Result<P2Out> {
    let target = if sample_name.is_some() {
      "HOHOHO"
    } else {
      input_molecule
    };

    let mut back_transforms: FnvHashMap<String, String> =
      FnvHashMap::with_hasher(FnvBuildHasher::default());
    for (res, req) in transforms {
      for t in req {
        back_transforms.insert(t.to_owned(), res.to_owned());
      }
    }

    let mut checks = back_transforms.keys().collect::<Vec<&String>>();
    checks.sort_by_key(|k| k.len());

    // Working backwards from the final string, and preferring the largest reductions,
    // finds our answer reasonably quickly. This relies on some of the input's properties:
    // - Every output is unique (even in the sample)
    // - Every transform grows the string (except `e => H` and `e => O` in the sample)
    // - Some elements may only exist as outputs.
    //   For example, my input has rules that generate Argon, but none that tranform it further.
    // With that in mind, we do a breadth first search trying the biggest ouputs first,
    // looking for paths that lead to just `e`, and discarding dead end paths.
    let mut stack = vec![(target.to_string(), 0_usize)];
    while let Some((next, step)) = stack.pop() {
      for k in &checks {
        if next.contains(*k) {
          let reduction = next.replacen(*k, back_transforms.get(*k).unwrap(), 1);
          if reduction == "e" {
            // Because we searched breadth-first, this is guaranteed to be the shortest route.
            return Ok(step + 1);
          } else if reduction.len() > 1 && reduction.contains("e") {
            // The only transformations including a lowercase 'e' on either side are the final ones,
            // so we can prune paths that try to do that conversion early.
            // (That is, none of the reactions include Helium, Neon, Xenon, Beryllium, etc)
            continue;
          }
          stack.push((reduction, step + 1));
        }
      }
    }

    bail!("Failed to find a path to a single electron");
  }
}

fn main() -> Result<()> {
  Solver {}.run(2015, 19)
}
