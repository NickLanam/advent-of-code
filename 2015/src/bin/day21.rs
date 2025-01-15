use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = u16;
type P2Out = u16;

#[derive(Debug)]
struct Parsed {
  player_hp: u16,
  boss_hp: u16,
  boss_damage: u16,
  boss_armor: u16,
}

// Pre-generating these tables is fast, but I did it manually to be even faster.
fn cheapest_purchase(target_damage: u16, target_armor: u16) -> u16 {
  // Cheapest way to get that amount of armor with taking no rings, with taking exactly one ring, and exactly two rings
  // 1_000 means "not possible" here
  let (d0, d1, d2) = match target_damage {
    0..=3 => (1_000, 1_000, 1_000), // "You must buy exactly one weapon" and the cheapest one has 4 damage
    4 => (8_u16, 1_000, 1_000),
    5 => (10, 33, 1_000),
    6 => (25, 35, 1_000),
    7 => (40, 50, 83),
    8 => (74, 65, 85),
    9 => (1_000, 90, 100),
    10 => (1_000, 124, 124),
    11 => (1_000, 174, 149),
    12 => (1_000, 1_000, 190),
    13 => (1_000, 1_000, 224),
    _ => (1_000, 1_000, 1_000),
  };

  // Same, but for armor
  let (a0, a1, a2) = match target_armor {
    0 => (0_u16, 0_u16, 0_u16), // "Armor is optional, but you can't use more than one"
    1 => (13, 25, 1_000),
    2 => (31, 23, 1_000),
    3 => (53, 51, 60),
    4 => (75, 63, 73),
    5 => (102, 95, 91),
    6 => (1_000, 115, 113),
    7 => (1_000, 142, 135),
    8 => (1_000, 182, 162),
    9 => (1_000, 1_000, 195),
    10 => (1_000, 1_000, 222),
    _ => (1_000, 1_000, 1_000),
  };

  *[d0 + a0, d0 + a1, d0 + a2, d1 + a0, d1 + a1, d2 + a0]
    .iter()
    .min()
    .unwrap()
}

// Part 2 wants the costliest way to lose instead of the cheapest way to win
fn costliest_purchase(target_damage: u16, target_armor: u16) -> u16 {
  // Since we're looking for the highest cost, "impossible" is now represented by 0.
  let (d0, d1, d2) = match target_damage {
    0..=3 => (0_u16, 0_u16, 0_u16), // "You must buy exactly one weapon" and the cheapest one has 4 damage
    4 => (8, 0, 0),
    5 => (10, 33, 0),
    6 => (25, 58, 0),
    7 => (40, 108, 83),
    8 => (74, 110, 133),
    9 => (0, 125, 158),
    10 => (0, 140, 160),
    11 => (0, 174, 175),
    12 => (0, 0, 199),
    13 => (0, 0, 224),
    _ => (0, 0, 0),
  };

  // Same, but for armor
  let (a0, a1, a2) = match target_armor {
    0 => (0_u16, 0_u16, 0_u16), // "Armor is optional, but you can't use more than one"
    1 => (13, 20, 0),
    2 => (31, 40, 0),
    3 => (53, 80, 60),
    4 => (75, 93, 100),
    5 => (102, 111, 120),
    6 => (0, 133, 133),
    7 => (0, 155, 151),
    8 => (0, 182, 175),
    9 => (0, 0, 202),
    10 => (0, 0, 222),
    _ => (0, 0, 0),
  };

  *[
    if d0 > 0 && a0 > 0 { d0 + a0 } else { 0 },
    if d0 > 0 && a1 > 0 { d0 + a1 } else { 0 },
    if d0 > 0 && a2 > 0 { d0 + a2 } else { 0 },
    if d1 > 0 && a0 > 0 { d1 + a0 } else { 0 },
    if d1 > 0 && a1 > 0 { d1 + a1 } else { 0 },
    if d2 > 0 && a0 > 0 { d2 + a0 } else { 0 },
  ]
  .iter()
  .max()
  .unwrap()
}

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(
    &self,
    lines: Vec<String>,
    sample_name: Option<String>,
    _for_part: PartId,
  ) -> Result<Parsed> {
    Ok(Parsed {
      player_hp: if sample_name.is_some() { 8 } else { 100 },
      boss_hp: lines[0].split_once(": ").unwrap().1.parse().unwrap(),
      boss_damage: lines[1].split_once(": ").unwrap().1.parse().unwrap(),
      boss_armor: lines[2].split_once(": ").unwrap().1.parse().unwrap(),
    })
  }

  fn part1(
    &self,
    Parsed {
      player_hp,
      boss_hp,
      boss_damage,
      boss_armor,
    }: &Parsed,
    _sample_name: Option<String>,
  ) -> Result<P1Out> {
    let mut cheapest = u16::MAX;

    // The equipment rules and contents of the shop mean there's no way to get past 13 damage or 10 armor,
    // and even then you can't get that high in one without sacrificing the other.
    for player_damage in 4..=13_u16 {
      let player_output = if player_damage > *boss_armor {
        player_damage - boss_armor
      } else {
        1
      };
      for player_armor in 0..=10_u16 {
        let boss_output = if *boss_damage > player_armor {
          boss_damage - player_armor
        } else {
          1
        };

        let player_wins_on_turn = boss_hp.div_ceil(player_output);
        let boss_wins_on_turn = player_hp.div_ceil(boss_output);
        // The player goes first (instead of going at the same time), meaning the player wins in a tie
        if player_wins_on_turn <= boss_wins_on_turn {
          cheapest = cheapest.min(cheapest_purchase(player_damage, player_armor))
        }
      }
    }
    Ok(cheapest)
  }

  fn part2(
    &self,
    Parsed {
      player_hp,
      boss_hp,
      boss_damage,
      boss_armor,
    }: &Parsed,
    _sample_name: Option<String>,
  ) -> Result<P2Out> {
    // Same task, same approach, but instead of "cheapest way to win", it's "most expensive way to lose".
    let mut costliest = 0;

    for player_damage in 4..=13_u16 {
      let player_output = if player_damage > *boss_armor {
        player_damage - boss_armor
      } else {
        1
      };
      for player_armor in 0..=10_u16 {
        let boss_output = if *boss_damage > player_armor {
          boss_damage - player_armor
        } else {
          1
        };

        let player_wins_on_turn = boss_hp.div_ceil(player_output);
        let boss_wins_on_turn = player_hp.div_ceil(boss_output);
        // The player goes first (instead of going at the same time), meaning the player wins in a tie
        // This time, we're figuring out the MOST we can spend AND STILL LOSE, rather than the least we can spend and still win.
        if player_wins_on_turn > boss_wins_on_turn {
          costliest = costliest.max(costliest_purchase(player_damage, player_armor))
        }
      }
    }
    // 189 is too high!
    Ok(costliest)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2015, 21)
}
