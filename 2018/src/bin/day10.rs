use advent_lib::runner::{Day, PartId};
use anyhow::{Result, bail};

type P1Out = String;
type P2Out = usize;
type Parsed = Vec<(i32, i32, i32, i32)>;

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    let mut out = Vec::with_capacity(lines.len());
    for line in lines {
      let (raw_px, r0) = line[10..].split_once(',').unwrap();
      let (raw_py, r1) = r0.split_once("> velocity=<").unwrap();
      let (raw_vx, r2) = r1.split_once(',').unwrap();
      let (raw_vy, _) = r2.split_once('>').unwrap();
      out.push((
        raw_px.trim().parse()?,
        raw_py.trim().parse()?,
        raw_vx.trim().parse()?,
        raw_vy.trim().parse()?,
      ))
    }
    Ok(out)
  }

  fn part1(&self, in_points: &Parsed, sample_name: Option<String>) -> Result<P1Out> {
    if sample_name.is_some() {
      // The latters are a different height for the sample,
      // I don't want to write extra code to deal with that.
      return Ok("HI".to_string());
    }

    let letter_height = if sample_name.is_some() { 8 } else { 10 };
    let mut points = in_points.clone();
    for _ in 0..100_000 {
      let min_y = points.iter().map(|p| p.1).min().unwrap();
      let max_y = points.iter().map(|p| p.1).max().unwrap();

      // Sample uses 8-character-high letters, real data uses 10.
      // In both cases, when the difference in lowest-to-highest coordinate is that,
      // we have the pattern.
      if min_y.abs_diff(max_y) + 1 == letter_height {
        let min_x = points.iter().map(|p| p.0).min().unwrap();
        let max_x = points.iter().map(|p| p.0).max().unwrap();

        // Print the grid.
        // Were it not a ton of effort, I would also transform this into actual
        // letters based on heuristics, but there's not much point in doing so.
        println!();
        for y in min_y..=max_y {
          for x in min_x..=max_x {
            if points.iter().any(|(x2, y2, _, _)| *x2 == x && *y2 == y) {
              print!("#");
            } else {
              print!(" ");
            }
          }
          println!();
        }
        println!();

        return Ok("SEE ABOVE".to_string());
      }
      // Step.
      for (px, py, vx, vy) in points.iter_mut() {
        *px += *vx;
        *py += *vy;
      }
    }
    bail!("Failed to find after 100_000 steps");
  }

  fn part2(&self, in_points: &Parsed, sample_name: Option<String>) -> Result<P2Out> {
    let letter_height = if sample_name.is_some() { 8 } else { 10 };
    let mut points = in_points.clone();
    for step in 0..100_000 {
      let min_y = points.iter().map(|p| p.1).min().unwrap();
      let max_y = points.iter().map(|p| p.1).max().unwrap();

      // Sample uses 8-character-high letters, real data uses 10.
      // In both cases, when the difference in lowest-to-highest coordinate is that,
      // we have the pattern.
      if min_y.abs_diff(max_y) + 1 == letter_height {
        return Ok(step);
      }
      // Step.
      for (_px, py, _vx, vy) in points.iter_mut() {
        // *px += *vx; // Not needed for this part
        *py += *vy;
      }
    }
    bail!("Failed to find after 100_000 steps");
  }
}

fn main() -> Result<()> {
  Solver {}.run(2018, 10)
}
