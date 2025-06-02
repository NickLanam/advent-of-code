use advent_lib::runner::{Day, PartId};
use anyhow::{Result, bail};

#[derive(Clone, Copy, PartialEq)]
enum Team {
  ImmuneSystem,
  Infection,
}

#[derive(Clone)]
struct Group {
  id: usize,
  team: Team,
  count: usize,
  hp: usize,
  weaknesses: Vec<String>,
  immunities: Vec<String>,
  attack_type: String,
  attack_damage: usize,
  initiative: usize,
}
impl PartialEq for Group {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
  }
}
impl Group {
  // total_damage, units_killed, is_fatal
  fn damage_to(&self, other: &Group, boost: usize) -> (usize, usize, bool) {
    if other.immunities.contains(&self.attack_type) || other.count == 0 {
      (0, 0, false)
    } else {
      let mut damage = self.count * self.attack_damage;
      if self.team == Team::ImmuneSystem {
        damage += self.count * boost;
      }
      if other.weaknesses.contains(&self.attack_type) {
        damage *= 2;
      }
      (damage, damage / other.hp, damage >= other.hp * other.count)
    }
  }

  fn effective_power(&self, boost: usize) -> usize {
    if self.team == Team::ImmuneSystem {
      self.count * (self.attack_damage + boost)
    } else {
      self.count * self.attack_damage
    }
  }
}

type P1Out = usize;
type P2Out = usize;
type Parsed = Vec<Group>;

fn check_win(groups: &[Group]) -> Result<(Team, usize)> {
  let mut attackers_left = 0;
  let mut defenders_left = 0;
  for group in groups.iter() {
    match group.team {
      Team::ImmuneSystem => {
        defenders_left += group.count;
      }
      Team::Infection => {
        attackers_left += group.count;
      }
    }
  }
  if attackers_left == 0 {
    return Ok((Team::ImmuneSystem, defenders_left));
  } else if defenders_left == 0 {
    return Ok((Team::Infection, attackers_left));
  }
  bail!("Not yet a win condition");
}

/// NOTE: Assumes that input order is already turn order
fn target_phase(groups: &[Group], boost: usize) -> Result<Vec<(usize, usize)>> {
  // Flattened for simpler logic: [attacker_id, defender_id, attacker_id, defender_id, ...]
  let mut pairs: Vec<(usize, usize)> = vec![];

  for group in groups.iter() {
    let mut target_damage = 0;
    let mut target: Option<&Group> = None;
    for other in groups.iter() {
      // Do not attack one's own team, and no unit may be under attack by two others simultaneously,
      if other.team == group.team
        || other.hp == 0
        || pairs
          .iter()
          .any(|&(_, defender_id)| defender_id == other.id)
      {
        continue;
      }
      let (total_damage, _units_killed, _is_fatal) = group.damage_to(other, boost);
      let better: bool;
      if let Some(target) = target {
        better = total_damage
          .cmp(&target_damage)
          .then((other.effective_power(boost)).cmp(&(target.effective_power(boost))))
          .then(other.initiative.cmp(&target.initiative))
          .is_ge();
      } else {
        better = total_damage > target_damage;
      }
      if better {
        target_damage = total_damage;
        target = Some(other);
      }
    }
    if let Some(target) = target {
      pairs.push((group.id, target.id));
    }
  }

  Ok(pairs)
}

fn solve(in_groups: &[Group], boost: usize) -> Result<(Team, usize)> {
  let mut groups: Vec<Group> = in_groups.to_vec();

  let mut prev_survivors = 0;
  loop {
    // Remove dead groups and exit if the game has ended
    groups.retain(|g| g.count > 0);
    let win = check_win(&groups);
    if win.is_ok() {
      return win;
    }

    // TARGET SELECTION PHASE
    // Units take turns, schoolyard style, picking who to attack.
    // Highest effective power (tiebreak: highest initiative) goes first.
    groups.sort_by(|a, b| {
      b.effective_power(boost)
        .cmp(&(a.effective_power(boost)))
        .then(b.initiative.cmp(&a.initiative))
    });
    let target_id_pairs = target_phase(&groups, boost)?;

    // ATTACK PHASE
    // Attacks happen in initiative order, with... no tiebreaker? Hope stable sort is what we want!
    groups.sort_by(|a, b| b.initiative.cmp(&a.initiative));

    // Have to iterate on IDs since we're mutating the list as we run through it
    let ids: Vec<usize> = groups.iter().map(|g| g.id).collect();
    for id in ids {
      // TODO: When let-chaining lands in Rust 1.88 (likely), this gets much easier to read...
      if let Some(&(_, defender_id)) = target_id_pairs.iter().find(|&&(a, _)| a == id) {
        if let Some(attacker) = groups.iter().find(|g| g.id == id) {
          if attacker.hp > 0 {
            if let Some(defender_pos) = groups.iter().position(|g| g.id == defender_id) {
              if groups[defender_pos].hp > 0 {
                let (_total_damage, units_killed, _is_fatal) =
                  attacker.damage_to(&groups[defender_pos], boost);
                groups[defender_pos].count =
                  groups[defender_pos].count.saturating_sub(units_killed);
              }
            }
          }
        }
      }
    }

    // Check for a stalemate, abort if it happens
    let survivors: usize = groups.iter().map(|g| g.count).sum();
    if survivors == prev_survivors {
      // Stalemate. For part 2's control flow, pretend the infection wins with no survivors.
      return Ok((Team::Infection, 0));
    } else {
      prev_survivors = survivors;
    }
  }
}

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    let mut groups: Vec<Group> = Vec::with_capacity(lines.len() - 3);
    let mut team = Team::ImmuneSystem;
    let mut id = 0;
    for line in lines.iter() {
      if line.is_empty() {
        team = Team::Infection;
        continue;
      } else if line.ends_with(':') {
        continue;
      }

      id += 1;
      let mut weaknesses: Vec<String> = vec![];
      let mut immunities: Vec<String> = vec![];

      let (count, r0) = line.split_once(" units each with ").unwrap();
      let (hp, r1) = r0.split_once(" hit points ").unwrap();

      // Real input skips the entire parenthetical for some groups, which is annoying to account for.
      let attack_damage: &str;
      let attack_type: &str;
      let initiative: &str;
      let r3: &str;
      if let Some(r1) = r1.strip_prefix("(") {
        let (parenthetical, r2) = r1.split_once(") with an attack that does ").unwrap();
        (attack_damage, r3) = r2.split_once(' ').unwrap();
        (attack_type, initiative) = r3.split_once(" damage at initiative ").unwrap();
        for section in parenthetical.split("; ") {
          if let Some(section) = section.strip_prefix("weak to ") {
            for weakness in section.split(", ") {
              weaknesses.push(weakness.to_string());
            }
          } else if let Some(section) = section.strip_prefix("immune to ") {
            for immunity in section.split(", ") {
              immunities.push(immunity.to_string());
            }
          } else {
            bail!("What is this section? {section}");
          }
        }
      } else {
        (attack_damage, r3) = r1[25..].split_once(' ').unwrap();
        (attack_type, initiative) = r3.split_once(" damage at initiative ").unwrap();
      }

      groups.push(Group {
        id,
        team,
        count: count.parse()?,
        hp: hp.parse()?,
        weaknesses,
        immunities,
        attack_type: attack_type.to_string(),
        attack_damage: attack_damage.parse()?,
        initiative: initiative.parse()?,
      });
    }
    Ok(groups)
  }

  fn part1(&self, in_groups: &Parsed, _: Option<String>) -> Result<P1Out> {
    let (_team, survivors) = solve(in_groups, 0)?;
    Ok(survivors)
  }

  fn part2(&self, in_groups: &Parsed, _: Option<String>) -> Result<P2Out> {
    // Binary searching possible boost amounts to dramatically speed up the search.
    let mut min_boost = 0;
    let mut max_boost = usize::MAX;

    while min_boost <= max_boost {
      let boost = min_boost + ((max_boost - min_boost) / 2);
      let (team, survivors) = solve(in_groups, boost)?;
      if team == Team::Infection {
        min_boost = boost + 1;
      } else {
        max_boost = boost - 1;
      }
      if max_boost < min_boost {
        return Ok(survivors);
      }
    }
    bail!("Failed to find a solution");
  }
}

fn main() -> Result<()> {
  Solver {}.run(2018, 24)
}
