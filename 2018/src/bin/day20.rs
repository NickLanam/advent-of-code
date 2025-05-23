use advent_lib::runner::{Day, PartId};
use anyhow::Result;

#[derive(Debug, Default, PartialEq, Hash)]
struct Room {
  n: bool,
  e: bool,
  s: bool,
  w: bool,
}

type P1Out = usize;
type P2Out = usize;
type Parsed = Vec<char>;

/*
fn buildMap(chars: &[char]) -> Infinite2dGrid<Room> {
  //
}
*/

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    Ok(lines[0].chars().skip(1).take(lines[0].len() - 2).collect())
  }

  fn part1(&self, chars: &Parsed, _sample_name: Option<String>) -> Result<P1Out> {
    /* Actually, Imma try the dumb thing first. * /
    let map = buildMap(chars);
    let mut max_len = 0;
    // TODO: BFS until we find all paths to get the longest distance possible.
    //  This is a separate phase because of the "each room will be seen AT LEAST ONCE" rule.
    //  If we were sure to only visit each room once, we wouldn't need to build the map -
    //  only gives us one way to get it done that may have loops and inefficient routes.
    Ok(max_len)
    */

    // Okay, the faster and simpler method IF IT WORKS THAT WAY is to just
    // replace every nested group with its longest subgroup and count
    // the resulting string's length. Only if the "at least once" thing isn't
    // the trap I suspect it is, anyway.
  }

  fn part2(&self, _lines: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    Ok(0)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2018, 20)
}
