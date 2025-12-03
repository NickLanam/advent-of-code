use advent_lib::runner::{Day, PartId};
use anyhow::Result;

type P1Out = u64;
type P2Out = u64;

#[derive(Debug)]
struct Parsed {
  player_hp: u16,
  player_mana: u16,
  boss_hp: u16,
  boss_damage: u16,
}

#[derive(Debug, Clone)]
struct State {
  turn: u64,
  player_hp: u16,
  mana: u16,
  boss_hp: u16,
  boss_damage: u16, // Does not change, but makes it easier to do this
  shield_clock: u8,
  poison_clock: u8,
  recharge_clock: u8,
  total_spent: u64,
}

#[derive(Clone, Debug, PartialEq)]
enum Spell {
  None,
  Missile,
  Drain,
  Shield,
  Poison,
  Recharge,
}

fn step(
  State {
    turn,
    player_hp,
    mana,
    boss_hp,
    boss_damage,
    shield_clock,
    poison_clock,
    recharge_clock,
    total_spent,
    ..
  }: &State,
  spell: Spell,
) -> State {
  let cost = match spell {
    Spell::None => 0,
    Spell::Missile => 53,
    Spell::Drain => 73,
    Spell::Shield => 113,
    Spell::Poison => 173,
    Spell::Recharge => 229,
  };

  let available = *mana + (if *recharge_clock > 0 { 110 } else { 0 });

  if cost > available {
    panic!("Tried to cast {spell:?} which costs {cost}, but only have {mana}!");
  }

  let mut damage_done: u16 = if *poison_clock > 0 { 3 } else { 0 };
  if turn % 2 == 0 {
    if spell == Spell::Missile {
      damage_done += 4;
    } else if spell == Spell::Drain {
      damage_done += 2;
    }
  }

  let armor = if *shield_clock > 0 { 7 } else { 0 };
  let damage_taken: u16 = if *boss_damage > armor {
    *boss_damage - armor
  } else {
    1
  };

  State {
    turn: turn + 1,
    player_hp: if turn % 2 == 0 {
      *player_hp + (if spell == Spell::Drain { 2 } else { 0 })
    } else {
      (*player_hp).saturating_sub(damage_taken)
    },
    mana: available - cost,
    boss_hp: (*boss_hp).saturating_sub(damage_done),
    boss_damage: *boss_damage,
    shield_clock: if spell == Spell::Shield {
      6
    } else if *shield_clock > 0 {
      *shield_clock - 1
    } else {
      0
    },
    poison_clock: if spell == Spell::Poison {
      6
    } else if *poison_clock > 0 {
      *poison_clock - 1
    } else {
      0
    },
    recharge_clock: if spell == Spell::Recharge {
      5
    } else if *recharge_clock > 0 {
      *recharge_clock - 1
    } else {
      0
    },
    total_spent: *total_spent + (cost as u64),
  }
}

fn allowed_spells(
  State {
    turn,
    mana,
    shield_clock,
    poison_clock,
    recharge_clock,
    ..
  }: &State,
) -> Option<Vec<Spell>> {
  let mut options: Vec<Spell> = vec![];

  let available = *mana + (if *recharge_clock > 0 { 101 } else { 0 });
  // Can only cast on the player's turn (even numbers)
  if turn % 2 == 0 {
    if available >= 53 {
      options.push(Spell::Missile);
    }
    if available >= 73 {
      options.push(Spell::Drain);
    }
    // Effects can be cast on the turn they would expire,
    // but not before. That is, effects tick before the
    // spell is cast.
    if available >= 113 && *shield_clock <= 1 {
      options.push(Spell::Shield);
    }
    if available >= 173 && *poison_clock <= 1 {
      options.push(Spell::Poison);
    }
    if available >= 229 && *recharge_clock <= 1 {
      options.push(Spell::Recharge);
    }
  }
  if !options.is_empty() {
    Some(options)
  } else {
    None
  }
}

fn solve(
  Parsed {
    player_hp,
    player_mana,
    boss_hp,
    boss_damage,
  }: &Parsed,
  part_2: bool,
) -> Result<u64> {
  let mut cheapest = u64::MAX;

  // This is a pathfinding problem with expensive-to-compute edges.
  let mut states: Vec<State> = vec![State {
    turn: 0,
    player_hp: *player_hp,
    mana: *player_mana,
    boss_hp: *boss_hp,
    boss_damage: *boss_damage,
    shield_clock: 0,
    poison_clock: 0,
    recharge_clock: 0,
    total_spent: 0,
  }];

  while let Some(state) = states.pop() {
    if state.boss_hp == 0 {
      // A victory condition. Stop here.
      cheapest = cheapest.min(state.total_spent);
      continue;
    } else if state.player_hp == 0 || state.total_spent > cheapest {
      // Player died OR already spent more mana than the best path seen so far. Prune.
      continue;
    }

    if state.turn % 2 == 0 {
      // Player turn, cast a spell if possible
      // but check for the miasma effect of part 2 first
      let s = if part_2 {
        if state.player_hp < 2 {
          continue;
        }
        let mut o = state.clone();
        o.player_hp -= 1;
        o
      } else {
        state
      };

      // If the player can't cast anything, they lose (rules of the challenge)
      if let Some(next) = allowed_spells(&s) {
        for spell in next {
          states.push(step(&s, spell));
        }
      }
    } else {
      // On the boss' turn, the player isn't casting
      states.push(step(&state, Spell::None));
    }
  }

  Ok(cheapest)
}

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, sample_name: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(Parsed {
      player_hp: if sample_name.is_some() { 10 } else { 50 },
      player_mana: if sample_name.is_some() { 250 } else { 500 },
      boss_hp: lines[0].split_once(": ").unwrap().1.parse().unwrap(),
      boss_damage: lines[1].split_once(": ").unwrap().1.parse().unwrap(),
    })
  }

  fn part1(&self, parsed: &Parsed, _sample_name: Option<String>) -> Result<P1Out> {
    solve(parsed, false)
  }

  fn part2(&self, parsed: &Parsed, _sample_name: Option<String>) -> Result<P2Out> {
    solve(parsed, true)
  }
}

fn main() -> Result<()> {
  Solver {}.run(2015, 22)
}
