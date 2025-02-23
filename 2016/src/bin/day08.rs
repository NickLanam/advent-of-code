use advent_lib::runner::{Day, PartId};
use anyhow::{Context, Result, bail};

#[derive(PartialEq, Debug)]
enum Direction {
  Horizontal,
  Vertical,
}

#[derive(PartialEq, Debug)]
enum Instruction {
  Rect(u8, u8),
  Rotate(Direction, u8, u8),
}

type P1Out = u16;
type P2Out = String;
type Parsed = Vec<Instruction>;

const GRID_WIDTH: usize = 50;
const GRID_HEIGHT: usize = 6;
fn exec(instructions: &Parsed) -> Result<[[bool; GRID_WIDTH]; GRID_HEIGHT]> {
  let mut grid: [[bool; GRID_WIDTH]; GRID_HEIGHT] = [[false; GRID_WIDTH]; GRID_HEIGHT];

  for inst in instructions {
    match inst {
      Instruction::Rect(w, h) => {
        if *w as usize >= GRID_WIDTH || *h as usize >= GRID_HEIGHT {
          bail!("Out of bounds: {inst:?}");
        }
        for row in grid.iter_mut().take(*h as usize) {
          for col in row.iter_mut().take(*w as usize) {
            *col = true;
          }
        }
      }
      Instruction::Rotate(Direction::Horizontal, y_u8, offset_u8) => {
        let row = *y_u8 as usize;
        let offset = (*offset_u8 as usize) % GRID_WIDTH;

        if row >= GRID_HEIGHT {
          bail!("Out of bounds: {inst:?}");
        }

        let old_row = grid[row];
        for (x, old_val) in old_row.iter().enumerate() {
          grid[row][(x + offset) % GRID_WIDTH] = *old_val;
        }
      }
      Instruction::Rotate(Direction::Vertical, x_u8, offset_u8) => {
        let col = *x_u8 as usize;
        let offset = (*offset_u8 as usize) % GRID_WIDTH;

        if col >= GRID_WIDTH {
          bail!("Out of bounds: {inst:?}");
        }

        let old_col = grid.iter().map(|row| row[col]).collect::<Vec<bool>>();
        for (y, old_val) in old_col.iter().enumerate() {
          grid[(y + offset) % GRID_HEIGHT][col] = *old_val;
        }
      }
    }
  }
  Ok(grid)
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
      if line.starts_with("rect ") {
        let (_, mid) = line.split_once(' ').context("Bad rect line")?;
        let (x_raw, y_raw) = mid.split_once('x').context("Bad rect coords")?;
        out.push(Instruction::Rect(x_raw.parse()?, y_raw.parse()?));
      } else if line.starts_with("rotate row") {
        let (_, mid) = line.split_once('=').context("Bad row rotation")?;
        let (row_raw, offset_raw) = mid.split_once(" by ").context("Bad row rotation")?;
        out.push(Instruction::Rotate(
          Direction::Horizontal,
          row_raw.parse()?,
          offset_raw.parse()?,
        ));
      } else if line.starts_with("rotate column") {
        let (_, mid) = line.split_once('=').context("Bad column rotation")?;
        let (column_raw, offset_raw) = mid.split_once(" by ").context("Bad column rotation")?;
        out.push(Instruction::Rotate(
          Direction::Vertical,
          column_raw.parse()?,
          offset_raw.parse()?,
        ));
      } else {
        bail!("Unrecognized instruction? {line}");
      }
    }
    Ok(out)
  }

  fn part1(&self, instructions: &Parsed, _sample_name: Option<String>) -> Result<P1Out> {
    let grid = exec(instructions)?;
    let mut count = 0;
    for row in grid {
      for col in row {
        if col {
          count += 1;
        }
      }
    }
    Ok(count)
  }

  fn part2(&self, instructions: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    let grid = exec(instructions)?;

    // Hypothetically, I could actually read the characters printed here.
    // But it's easier to just print the thing.
    Ok(
      "!\n".to_string() // To force all of the output lines to line up
        + grid
          .iter()
          .map(|row| {
            row
              .iter()
              .map(|b| if *b { '#' } else { '_' })
              .collect::<String>()
          })
          .collect::<Vec<String>>()
          .join("\n")
          .as_str(),
    )
  }
}

fn main() -> Result<()> {
  Solver {}.run(2016, 8)
}
