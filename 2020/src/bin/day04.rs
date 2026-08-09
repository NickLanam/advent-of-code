use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = usize;
type P2Out = usize;

#[derive(Clone, Copy, Debug)]
enum Height {
  Centimeters(usize),
  Inches(usize),
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum EyeColor {
  Invalid,
  Amber,
  Blue,
  Brown,
  Grey,
  Green,
  Hazel,
  Other,
}

#[derive(Debug)]
struct Passport {
  byr: Option<usize>,
  iyr: Option<usize>,
  eyr: Option<usize>,
  hgt: Option<Height>,
  hcl: Option<String>,
  ecl: Option<EyeColor>,
  pid: Option<Vec<u8>>,
}
impl From<&str> for Passport {
  fn from(value: &str) -> Self {
    let mut passport = Passport {
      byr: None,
      iyr: None,
      eyr: None,
      hgt: None,
      hcl: None,
      ecl: None,
      pid: None,
    };
    for section in value.split_whitespace() {
      let (key, raw) = section.split_once(':').unwrap();
      match key {
        "byr" => {
          passport.byr = Some(raw.parse().unwrap());
        }
        "iyr" => {
          passport.iyr = Some(raw.parse().unwrap());
        }
        "eyr" => {
          passport.eyr = Some(raw.parse().unwrap());
        }
        "hgt" => {
          passport.hgt = Some(if raw.ends_with("cm") {
            Height::Centimeters(raw[0..(raw.len() - 2)].parse().unwrap())
          } else if raw.ends_with("in") {
            Height::Inches(raw[0..(raw.len() - 2)].parse().unwrap())
          } else {
            Height::Inches(raw.parse().unwrap())
          });
        }
        "hcl" => {
          passport.hcl = Some(raw.to_owned());
        }
        "ecl" => {
          passport.ecl = Some(match raw {
            "amb" => EyeColor::Amber,
            "blu" => EyeColor::Blue,
            "brn" => EyeColor::Brown,
            "gry" => EyeColor::Grey,
            "grn" => EyeColor::Green,
            "hzl" => EyeColor::Hazel,
            "oth" => EyeColor::Other,
            _ => EyeColor::Invalid,
          });
        }
        "pid" => {
          // ASCII shenanigans make this straightforward
          passport.pid = Some(raw.chars().map(|c| (c as u8) - 48).collect());
        }
        "cid" => { /* No-op */ }
        _ => panic!("Unrecognized passport element: {key}"),
      }
    }
    passport
  }
}

type Parsed = Vec<Passport>;

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(
      lines
        .join("\n")
        .split("\n\n")
        .map(|line| line.replace('\n', " ").as_str().into())
        .collect(),
    )
  }

  fn part1(&self, passports: &Parsed, _: Option<String>) -> Result<P1Out> {
    Ok(
      passports
        .iter()
        .filter(|p| {
          p.byr.is_some()
            && p.iyr.is_some()
            && p.eyr.is_some()
            && p.hgt.is_some()
            && p.hcl.is_some()
            && p.ecl.is_some()
            && p.pid.is_some()
        })
        .count(),
    )
  }

  fn part2(&self, passports: &Parsed, _: Option<String>) -> Result<P2Out> {
    Ok(
      passports
        .iter()
        .filter(|p| {
          p.byr.is_some_and(|byr| (1920..=2002).contains(&byr))
            && p.iyr.is_some_and(|iyr| (2010..=2020).contains(&iyr))
            && p.eyr.is_some_and(|eyr| (2020..=2030).contains(&eyr))
            && p.hgt.as_ref().is_some_and(|hgt| match hgt {
              Height::Centimeters(cm) => *cm >= 150 && *cm <= 193,
              Height::Inches(inches) => *inches >= 59 && *inches <= 76,
            })
            && p.hcl.as_ref().is_some_and(|hcl| {
              hcl.len() == 7
                && hcl.chars().take(1).last().unwrap() == '#'
                && hcl.chars().skip(1).all(|c| c.is_ascii_hexdigit())
            })
            && p.ecl.is_some_and(|ecl| ecl != EyeColor::Invalid)
            && p
              .pid
              .as_ref()
              .is_some_and(|pid| pid.len() == 9 && pid.iter().all(|d| *d < 10))
        })
        .count(),
    )
  }
}

fn main() -> Result<()> {
  Solver {}.run(2020, 4)
}
