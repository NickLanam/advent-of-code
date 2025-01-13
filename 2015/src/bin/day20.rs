use advent_lib::runner::{Day, PartId};
use anyhow::{bail, Result};

type P1Out = usize;
type P2Out = usize;
type Parsed = usize;

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    Ok(lines[0].parse().unwrap())
  }

  fn part1(&self, target_score: &Parsed, _sample_name: Option<String>) -> Result<P1Out> {
    // This method _seems_ dumber than brute force due to how much memory it allocates,
    // but the time complexity is several orders of magnitude less so it works out to take
    // under a second while the brute force way takes over ten minutes.
    let size = target_score / 10;
    let mut houses: Vec<usize> = (0..=size).map(|_| 0).collect();
    houses.fill(0);
    for elf in 2..=size {
      for house in (elf..=(size - 1)).step_by(elf) {
        houses[house] += 10 * elf;
      }
    }
    if let Some(house) = houses.iter().position(|p| p >= target_score) {
      Ok(house)
    } else {
      bail!("Failed to find the right house")
    }
  }

  fn part2(&self, target_score: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    // This method _seems_ dumber than brute force due to how much memory it allocates,
    // but the time complexity is several orders of magnitude less so it works out to take
    // under a second while the brute force way takes over ten minutes.
    let size = target_score / 10;
    let mut houses: Vec<usize> = (0..=size).map(|_| 0).collect();
    houses.fill(0);
    for elf in 1..=size {
      let mut counter: u8 = 0;
      for house in (elf..=(size - 1)).step_by(elf) {
        houses[house] += 11 * elf;
        counter += 1;
        if counter >= 50 {
          break;
        }
      }
    }
    if let Some(house) = houses.iter().position(|p| p >= target_score) {
      Ok(house)
    } else {
      bail!("Failed to find the right house")
    }
  }
}

fn main() -> Result<()> {
  Solver {}.run(2015, 20)
}
