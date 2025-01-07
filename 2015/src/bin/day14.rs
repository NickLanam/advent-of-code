use std::collections::HashMap;

use advent_lib::runner::{Day, PartId};
use anyhow::Result;
use regex::Regex;

type P1Out = u64;
type P2Out = u64;
type Parsed = Vec<Reindeer>;

#[derive(Debug)]
struct Reindeer {
  name: String,
  speed: u64,
  sprint_time: u64,
  rest_time: u64,
}

fn who_leads(reindeer: &Vec<Reindeer>, race_time: u64) -> (Vec<&str>, u64) {
  let mut winners: Vec<&str> = Vec::with_capacity(reindeer.len());
  let mut furthest: u64 = 0;
  for rd in reindeer {
    let bursts = race_time / (rd.sprint_time + rd.rest_time);
    let trivial_distance = rd.speed * rd.sprint_time * bursts;
    let last_burst_time = race_time % (rd.sprint_time + rd.rest_time);
    let last_burst_distance = rd.speed * rd.sprint_time.min(last_burst_time);
    let total_distance = trivial_distance + last_burst_distance;
    #[allow(clippy::comparison_chain)]
    if total_distance == furthest {
      winners.push(rd.name.as_str());
    } else if total_distance > furthest {
      furthest = total_distance;
      winners.clear();
      winners.push(rd.name.as_str());
    }
  }
  (winners, furthest)
}

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    _sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    let re = Regex::new(r"^(?<name>[a-zA-Z]+) can fly (?<speed>\d+) km/s for (?<sprint_time>\d+) seconds, but then must rest for (?<rest_time>\d+) seconds.$").unwrap();
    Ok(
      lines
        .into_iter()
        .map(|line| {
          let caps = re.captures(line.as_str()).unwrap();
          let name = caps.name("name").unwrap().as_str().to_string();
          let speed = caps.name("speed").unwrap().as_str().parse::<u64>().unwrap();
          let sprint_time = caps
            .name("sprint_time")
            .unwrap()
            .as_str()
            .parse::<u64>()
            .unwrap();
          let rest_time = caps
            .name("rest_time")
            .unwrap()
            .as_str()
            .parse::<u64>()
            .unwrap();
          Reindeer {
            name,
            speed,
            sprint_time,
            rest_time,
          }
        })
        .collect::<Vec<Reindeer>>(),
    )
  }

  fn part1(&self, reindeer: &Parsed, sample_name: Option<String>) -> Result<P1Out> {
    let race_time: u64 = if sample_name.is_some() { 1_000 } else { 2_503 };
    Ok(who_leads(reindeer, race_time).1)
  }

  fn part2(&self, reindeer: &Parsed, sample_name: Option<String>) -> Result<P2Out> {
    let race_time: u64 = if sample_name.is_some() { 1_000 } else { 2_503 };
    let mut scores = HashMap::<&str, u64>::new();
    for rd in reindeer {
      scores.insert(rd.name.as_str(), 0);
    }
    for t in 1..=race_time {
      let (leaders, _) = who_leads(reindeer, t);
      for l in leaders {
        scores.insert(l, scores.get(l).unwrap() + 1);
      }
    }
    Ok(*scores.values().max().unwrap())
  }
}

fn main() -> Result<()> {
  Solver {}.run(2015, 14)
}
