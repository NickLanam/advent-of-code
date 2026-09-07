use advent_lib::runner::{Day, PartId};
use anyhow::{Result, bail};
use itertools::Itertools;

type P1Out = i64;
type P2Out = i64;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Token {
  GroupStart,
  GroupEnd,
  Num(i64),
  Plus,
  Times,
}

type Parsed = Vec<Vec<Token>>;

// Being fully left-to-right lets us do something more efficient than part 2,
// just iteratively replacing the first trio we can until we get to just a number.
// Part 2 has to handle operator precedence, and therefore jump around a fair bit.
fn reduce_p1(root: &[Token]) -> Result<Vec<Token>> {
  let mut remain: Vec<Token> = root.to_vec();

  while remain.len() > 1 {
    // Collapse (num, times|plus, num) and (open, num, close) into (num), repeatedly, until we can't.
    let mut next: Vec<Token> = remain.clone();
    'tuples: for ((i, e0), (_, e1), (_, e2)) in remain.iter().enumerate().tuple_windows() {
      match (e0, e1, e2) {
        (Token::GroupStart, Token::Num(n), Token::GroupEnd) => {
          next.splice(i..(i + 3), [Token::Num(*n)]);
          break 'tuples;
        }
        (Token::Num(n0), Token::Plus, Token::Num(n1)) => {
          next.splice(i..(i + 3), [Token::Num(*n0 + *n1)]);
          break 'tuples;
        }
        (Token::Num(n0), Token::Times, Token::Num(n1)) => {
          next.splice(i..(i + 3), [Token::Num(*n0 * *n1)]);
          break 'tuples;
        }
        _ => {}
      }
    }
    if remain.len() == next.len() {
      // No further reduction was possible with these rules, return.
      break;
    }
    remain = next;
  }

  Ok(remain)
}

// Part 2 needs operator precedence: () > + > *. This means a more complex replacement
// mechanism than part 1, and some recursion to keep it reasonable.
fn reduce_p2(root: &[Token]) -> Result<Vec<Token>> {
  let mut remain: Vec<Token> = root.to_vec();

  while remain.len() > 1 {
    // First, find the first group that has no more groups inside it.
    // If it exists, replace that section with a reduce_p2 of just section.
    // Otherwise, if there's an addition available, do that.
    // Otherwise, if there's a multiplication available, do that.
    // If none of those managed to reduce the equation further, we're done.

    let (mut depth, mut max_depth, mut start, mut end) = (0, 0, 0, remain.len() - 1);
    for (i, t) in remain.iter().enumerate() {
      if *t == Token::GroupStart {
        depth += 1;
        if depth > max_depth {
          max_depth = depth;
          start = i;
          end = 0;
        }
      } else if *t == Token::GroupEnd {
        if depth == max_depth && end == 0 {
          end = i;
        }
        depth -= 1;
      }
    }
    if max_depth > 0 {
      remain.splice(start..=end, reduce_p2(&remain[(start + 1)..=(end - 1)])?);
      continue;
    }

    // We didn't collapse a sub-group, which means there are no GroupStart/GroupEnd tokens left.
    // Therefore: run an addition if available. If not, try a multiplication. Otherwise, we're done.
    let mut next: Vec<Token> = remain.clone();
    'tuples: for ((i, e0), (_, e1), (_, e2)) in remain.iter().enumerate().tuple_windows() {
      if let (Token::Num(n0), Token::Plus, Token::Num(n1)) = (e0, e1, e2) {
        next.splice(i..(i + 3), [Token::Num(*n0 + *n1)]);
        break 'tuples;
      }
    }
    if remain.len() == next.len() {
      // No additions happened, so try multiplication.
      'tuples: for ((i, e0), (_, e1), (_, e2)) in next.iter().enumerate().tuple_windows() {
        if let (Token::Num(n0), Token::Times, Token::Num(n1)) = (e0, e1, e2) {
          next.splice(i..(i + 3), [Token::Num(*n0 * *n1)]);
          break 'tuples;
        }
      }
    }
    if remain.len() == next.len() {
      // No further reduction was possible with these rules, return.
      break;
    }
    remain = next;
  }

  Ok(remain)
}

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    lines
      .iter()
      .map(|line| -> Result<Vec<Token>> {
        let mut out: Vec<Token> = vec![];
        for ch in line.chars() {
          match ch {
            '(' => {
              out.push(Token::GroupStart);
            }
            ')' => {
              out.push(Token::GroupEnd);
            }
            '+' => {
              out.push(Token::Plus);
            }
            '*' => {
              out.push(Token::Times);
            }
            ' ' => {
              // no-op
            }
            ch if ch.is_numeric() => {
              let digit = ((ch as u8) - 48) as i64;
              if let Some(Token::Num(n)) = out.last_mut() {
                *n *= 10;
                *n += digit;
              } else {
                out.push(Token::Num(digit));
              }
            }
            _ => {
              bail!("Unexpected character: {ch}");
            }
          }
        }
        Ok(out)
      })
      .collect()
  }

  fn part1(&self, roots: &Parsed, _: Option<String>) -> Result<P1Out> {
    roots
      .iter()
      .map(|root| -> Result<i64> {
        let remain = reduce_p1(root)?;

        if remain.len() == 1
          && let Token::Num(n) = remain[0]
        {
          Ok(n)
        } else {
          bail!("Did not resolve properly: {root:?} -> {remain:?}");
        }
      })
      .sum()
  }

  fn part2(&self, roots: &Parsed, _: Option<String>) -> Result<P2Out> {
    roots
      .iter()
      .map(|root| -> Result<i64> {
        let remain = reduce_p2(root)?;

        if remain.len() == 1
          && let Token::Num(n) = remain[0]
        {
          Ok(n)
        } else {
          bail!("Did not resolve properly: {root:?} -> {remain:?}");
        }
      })
      .sum()
  }
}

fn main() -> Result<()> {
  Solver {}.run(2020, 18)
}
