use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = usize;
type P2Out = usize;

#[derive(Clone, Debug)]
enum Op {
  Add,
  Mul,
}

type Parsed = Vec<(Op, Vec<usize>, usize, usize)>;

fn solve(columns: &Parsed) -> usize {
  columns.iter().fold(
    0,
    |sum, (op, values, _column_start, _column_len)| match op {
      Op::Add => sum + values.iter().sum::<usize>(),
      Op::Mul => sum + values.iter().product::<usize>(),
    },
  )
}

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, for_part: PartId) -> Result<Parsed> {
    // Parsing: read the last line to get the operators and number of columns, then parse the columns themselves
    let mut columns: Vec<(Op, Vec<usize>, usize, usize)> = vec![];

    // Get the operators first, to simplify the rest of parsing.
    // Remember where we found them as well, to simplify the parsing in part 2
    for (i, ch) in lines.last().unwrap_or(&"".to_string()).chars().enumerate() {
      match ch {
        '*' => {
          if let Some((.., prev_end)) = columns.last_mut() {
            // Get rid of the extra space between columns
            *prev_end = i - 2;
          }
          columns.push((Op::Mul, vec![], i, lines[0].len() - 1));
        }
        '+' => {
          if let Some((.., prev_end)) = columns.last_mut() {
            // Get rid of the extra space between columns
            *prev_end = i - 2;
          }
          columns.push((Op::Add, vec![], i, lines[0].len() - 1));
        }
        _ => {}
      }
    }

    // Easier to handle the lines now
    match for_part {
      PartId::P1 => {
        // Trivial to split on whitespace, and grab the numbers in each column
        for line in lines[0..(lines.len() - 1)].iter() {
          for (col, n) in line.split_whitespace().enumerate() {
            columns[col].1.push(n.parse().unwrap());
          }
        }
      }
      PartId::P2 => {
        // Somewhat more complex, because now digits are column-aligned and we read
        // the numbers top-down instead of left-right. Knowing where each column
        // ranges makes this much easier to figure out.
        for (_op, values, column_start, column_end) in columns.iter_mut() {
          values.resize(*column_end - *column_start + 1, 0);
          for chunk in lines[0..(lines.len() - 1)].iter().map(|line| {
            line
              .chars()
              .skip(*column_start)
              .enumerate()
              .take(*column_end - *column_start + 1)
          }) {
            for (j, ch) in chunk {
              if ch.is_ascii_digit() {
                values[j] = values[j] * 10 + ch.to_digit(10).unwrap() as usize;
              }
            }
          }
        }
      }
    }

    Ok(columns)
  }

  fn part1(&self, columns: &Parsed, _: Option<String>) -> Result<P1Out> {
    Ok(solve(columns))
  }

  fn part2(&self, columns: &Parsed, _: Option<String>) -> Result<P2Out> {
    Ok(solve(columns))
  }
}

fn main() -> Result<()> {
  Solver {}.run(2025, 6)
}
