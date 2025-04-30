use advent_lib::runner::{Day, PartId};
use anyhow::{Context, Result, bail};
use regex::Regex;

#[derive(Debug)]
struct Rect {
  id: usize,
  l: i32,
  t: i32,
  r: i32,
  b: i32,
}

type P1Out = usize;
type P2Out = usize;
type Parsed = Vec<Rect>;

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    let parse_re = Regex::new(r"#(?<id>\d+) @ (?<x>\d+),(?<y>\d+): (?<w>\d+)x(?<h>\d+)")?;
    let mut rects: Vec<Rect> = Vec::with_capacity(lines.len());
    for line in lines {
      let captures = parse_re.captures(line.as_str()).context("")?;
      let id: usize = captures.name("id").context("")?.as_str().parse()?;
      let x: i32 = captures.name("x").context("")?.as_str().parse()?;
      let y: i32 = captures.name("y").context("")?.as_str().parse()?;
      let w: i32 = captures.name("w").context("")?.as_str().parse()?;
      let h: i32 = captures.name("h").context("")?.as_str().parse()?;
      rects.push(Rect {
        id,
        l: x,
        t: y,
        r: x + w,
        b: y + h,
      });
    }
    // Sorting by x lets both parts skip tons of iterations
    rects.sort_by_key(|r| r.l);
    Ok(rects)
  }

  fn part1(&self, rects: &Parsed, _: Option<String>) -> Result<P1Out> {
    // Instead of testing collisions between all rectangles, we can scan the x coordinate
    // and only look at rectangles that contain it, then scan y coordinates in that set
    // to find which ones have multiple matches.
    // Sorting by x in parse lets this ignore most iterations and filter on fewer elements,
    // which is well worth the cost of sorting.
    let mut score = 0;
    let mut i = 0;
    for x in rects[0].l.. {
      while i < rects.len() && rects[i].r <= x {
        i += 1;
      }
      if i >= rects.len() - 1 {
        // Need at least two for overlap to be possible
        break;
      }
      let mut relevant: Vec<&Rect> = rects[i..].iter().filter(|r| r.l <= x && r.r > x).collect();
      if relevant.len() > 1 {
        // Same optimization as for x coord: if we sort by y, skipping iterations is simple.
        // This sort is cheap, too, as `relevant` is a tiny subset of `rects`.
        relevant.sort_by_key(|r| r.t);
        let mut j = 0;
        for y in relevant[0].t.. {
          while j < relevant.len() && relevant[j].b <= y {
            j += 1;
          }
          if j >= relevant.len() - 1 {
            break;
          }
          if relevant[j].t <= y && relevant[(j + 1)..].iter().any(|r| r.t <= y && r.b > y) {
            score += 1;
          }
        }
      }
    }
    Ok(score)
  }

  fn part2(&self, rects: &Parsed, _: Option<String>) -> Result<P2Out> {
    // No need to actually compute the grid - just check each pair
    // of rectangles for overlaps directly.
    // It looks inefficient to compute so many duplicates instead of
    // removing them and looping again, but mutating vectors is slower
    // than just doing the duplicate collision checks.
    'search: for (i, a) in rects.iter().enumerate() {
      for (j, b) in rects.iter().enumerate() {
        if i == j {
          continue;
        }
        // If the two overlap at all, it isn't this one.
        // Computing "did it miss" is a bit faster than computing "did it hit".
        let miss = a.l >= b.r || b.l >= a.r || a.t >= b.b || b.t >= a.b;
        if !miss {
          continue 'search;
        }
      }
      return Ok(a.id);
    }
    bail!("No match");
  }
}

fn main() -> Result<()> {
  Solver {}.run(2018, 3)
}
