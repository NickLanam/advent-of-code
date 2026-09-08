use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = usize;
type P2Out = usize;

#[derive(Clone, Debug, PartialEq)]
enum Rule {
  A,
  B,
  Composite(Vec<Vec<usize>>),
}

#[derive(Debug)]
struct Parsed {
  rules: Vec<Rule>,
  // Note on the u128: The longest test string in my input is 97 a/b characters. a=0, b=1.
  // That happens to fit into a u128 that uses the first 16 bits for a length prefix.
  // This cuts the runtime of both parts in half compared to using the strings directly.
  tests: Vec<u128>,
}

fn prepend(prefix: &[usize], suffix: &[usize]) -> Vec<usize> {
  prefix.iter().chain(suffix).cloned().collect()
}

fn matches_recursive(rules: &Vec<Rule>, rule_chain: &[usize], test: u128) -> bool {
  if rule_chain.is_empty() {
    // 0 length -> we're comparing against nothing
    return test == 0;
  }

  let head = rule_chain[0];
  let tail = &rule_chain[1..];
  match &rules[head] {
    Rule::A | Rule::B => {
      let expect: u128 = if rules[head] == Rule::A { 0 } else { 1 };
      let len = test >> 112;
      let first_bit_matches = ((test >> (len - 1)) & 0b1) == expect;
      let suffix = ((len - 1) << 112) + (test - (len << 112) - (expect << (len - 1)));
      first_bit_matches && matches_recursive(rules, tail, suffix)
    }
    Rule::Composite(sequences) => sequences
      .iter()
      .any(|seq| matches_recursive(rules, &prepend(seq, tail), test)),
  }
}

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    let (rule_lines, tests) = lines.split_at(lines.iter().position(|l| l.is_empty()).unwrap());

    // Using a simple known-length vector is okay, since the rules are numbered 0..n, and n < 200, so nice for performance.
    let mut rules: Vec<Rule> = vec![Rule::A; rule_lines.len()];
    for line in rule_lines.iter() {
      let (i_raw, rest) = line.split_once(": ").unwrap();
      let i: usize = i_raw.parse().unwrap();
      match rest {
        "\"a\"" => rules[i] = Rule::A,
        "\"b\"" => rules[i] = Rule::B,
        _ => {
          let mut sequences: Vec<Vec<usize>> = vec![];
          for seq in rest.split(" | ") {
            sequences.push(seq.split(' ').map(|n| n.parse().unwrap()).collect());
          }
          rules[i] = Rule::Composite(sequences);
        }
      };
    }

    Ok(Parsed {
      rules,
      tests: tests[1..]
        .iter()
        .map(|s| {
          let l = s.len() as u128; // We're only using 16 bits, and the other 112 for the value.
          let mut v: u128 = l << 112;
          for (i, ch) in s.chars().rev().enumerate() {
            if ch == 'b' {
              v += 1 << i;
            }
          }
          v
        })
        .collect(),
    })
  }

  fn part1(&self, Parsed { rules, tests }: &Parsed, _: Option<String>) -> Result<P1Out> {
    Ok(
      tests
        .iter()
        .filter(|&&t| matches_recursive(rules, &[0], t))
        .count(),
    )
  }

  fn part2(&self, Parsed { rules, tests }: &Parsed, _: Option<String>) -> Result<P2Out> {
    let modified_rules: Vec<Rule> = rules
      .iter()
      .enumerate()
      .map(|(i, rule)| {
        if i == 8 {
          Rule::Composite(vec![vec![42], vec![42, 8]])
        } else if i == 11 {
          Rule::Composite(vec![vec![42, 31], vec![42, 11, 31]])
        } else {
          rule.clone()
        }
      })
      .collect();
    Ok(
      tests
        .iter()
        .filter(|&&t| matches_recursive(&modified_rules, &[0], t))
        .count(),
    )
  }
}

fn main() -> Result<()> {
  Solver {}.run(2020, 19)
}
