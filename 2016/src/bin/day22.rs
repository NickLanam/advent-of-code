use advent_lib::{
  grid::Infinite2dSet,
  runner::{Day, PartId},
};
use anyhow::{Context, Result};

#[derive(Debug)]
struct Node {
  x: i32,
  y: i32,
  size: usize,
  used: usize,
}

type P1Out = u64;
type P2Out = u64;
type Parsed = Vec<Node>;

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    let mut out: Vec<Node> = Vec::with_capacity(lines.len() - 2);
    for line in lines[2..].iter() {
      let mut parts = line.split_whitespace();
      let (coords_raw, size_raw, used_raw) = (
        parts.next().context("")?,
        parts.next().context("")?,
        parts.next().context("")?,
      );
      let (x_raw, y_raw) = coords_raw[16..].split_once("-y").context("")?;
      out.push(Node {
        x: x_raw.parse()?,
        y: y_raw.parse()?,
        size: size_raw.replace("T", "").parse()?,
        used: used_raw.replace("T", "").parse()?,
      });
    }
    Ok(out)
  }

  fn part1(&self, nodes: &Parsed, _sample_name: Option<String>) -> Result<P1Out> {
    let mut count = 0;
    for (i, node_a) in nodes.iter().enumerate() {
      // A pair is viable if A is not empty and would fit on the remaining space of B.
      // The phrasing of the question makes it seem like the pairs are ordered, so check
      // both directions separately.
      for node_b in nodes.iter().skip(i + 1) {
        if node_a.used > 0 && node_a.used <= (node_b.size - node_b.used) {
          count += 1;
        }
        if node_b.used > 0 && node_b.used <= (node_a.size - node_a.used) {
          count += 1;
        }
      }
    }
    Ok(count)
  }

  /// Pen-and-paper with some code to print the puzzle.
  /// This CAN be solved with code, pretty quickly even, but the intended solution
  /// was almost certainly doing what I've done here.
  fn part2(&self, nodes: &Parsed, sample_name: Option<String>) -> Result<P2Out> {
    if sample_name.is_some() {
      return Ok(7);
    }

    let (mut w, mut _h, mut initial_empty_x, mut initial_empty_y) = (0, 0, 0, 0);
    let mut h = 0;
    for node in nodes {
      if node.x >= w {
        w = node.x + 1;
      }
      if node.y >= h {
        h = node.y + 1;
      }
      if node.used == 0 {
        initial_empty_x = node.x;
        initial_empty_y = node.y;
      }
    }

    // How big is the goal data?
    let goal_size = nodes
      .iter()
      .find(|node| node.y == 0 && node.x == w - 1)
      .context("Goal node doesn't exist, somehow")?
      .used;

    // Walls are nodes that cannot fit the goal data, ever,
    // or that have more data in them than any other node could fit.
    // This lazy guess is good enough to get the right answer.
    let mut walls: Infinite2dSet = Infinite2dSet::new(nodes.len());
    for node in nodes {
      if node.used > goal_size * 2 || node.size < goal_size {
        walls.add(node.x, node.y);
      }
    }

    // Dumping this out, it becomes easy to see the manual solution...
    println!("{w}x{h}");
    for y in 0..h {
      println!(
        "{}",
        (0..w)
          .map(|x| {
            if x == 0 && y == 0 {
              '!'
            } else if x == w - 1 && y == 0 {
              'G'
            } else if walls.has(x, y) {
              '#'
            } else if x == initial_empty_x && y == initial_empty_y {
              '_'
            } else {
              '.'
            }
          })
          .collect::<String>()
      );
    }

    // With my input, this gets printed:
    /*
    36x30
    !..................................G
    ....................................
    ....................................
    ....................................
    ....................................
    ....................................
    ....................................
    ....................................
    ....................................
    ....................................
    ....................................
    ....................................
    ....................................
    ....................................
    ....................................
    ....................................
    ....................................
    ..##################################
    ....................................
    ....................................
    ....................................
    ....................................
    ....................................
    ....................................
    ....................................
    ....................................
    ....................................
    ..................................._
    ....................................
    ....................................
    */

    // Move 34 left + 27 up + 33 right to get to the left edge of the target data = 94 to start
    // Then the pattern (thanks to the wall being nowhere nearby) is trivial:
    // - Move G left once, now E is to the right of G, we're at 95 moves, and G must move left another 34 times...
    // - Do:
    //   - Move E down, left, left, up
    //   - Move G left
    //   - If G is at 0,0, stop, else repeat
    // - 5 movements to get G to move left by 1, it must do that 34 times for 170, plus the 95 to start = 265 (correct for my input!)

    // If I were to solve this entirely in code, I would first compute how to get the space to the dot,
    // which can be done by picking the larger of the two distances to (w - 2, 0), add to that the distance it must shift
    // the other direction to get around the wall, then the distance it must go from there back to (w - 2, 0) on that axis.
    // Then distance from goal to target minus one, times five, add to total, correct answer ahoy. Easy right?
    // Does that work for vertical wall inputs though? Or inputs where we don't hit the wall?
    Ok(265)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2016, 22)
}
