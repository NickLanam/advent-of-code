use std::cmp::Ordering;

use advent_lib::runner::{Day, PartId};
use anyhow::{Result, bail};

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
struct Point3d {
  x: i64,
  y: i64,
  z: i64,
}
impl Point3d {
  fn energy(self: &Self) -> usize {
    (self.x.abs() + self.y.abs() + self.z.abs()) as usize
  }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
struct Particle {
  pos: Point3d,
  vel: Point3d,
}
impl Particle {
  fn energy(self: &Self) -> usize {
    self.pos.energy() * self.vel.energy()
  }

  fn step_gravity(self: &mut Self, other: &mut Particle) {
    match self.pos.x.cmp(&other.pos.x) {
      Ordering::Less => {
        self.vel.x += 1;
        other.vel.x -= 1;
      }
      Ordering::Greater => {
        self.vel.x -= 1;
        other.vel.x += 1;
      }
      Ordering::Equal => {}
    }

    match self.pos.y.cmp(&other.pos.y) {
      Ordering::Less => {
        self.vel.y += 1;
        other.vel.y -= 1;
      }
      Ordering::Greater => {
        self.vel.y -= 1;
        other.vel.y += 1;
      }
      Ordering::Equal => {}
    }

    match self.pos.z.cmp(&other.pos.z) {
      Ordering::Less => {
        self.vel.z += 1;
        other.vel.z -= 1;
      }
      Ordering::Greater => {
        self.vel.z -= 1;
        other.vel.z += 1;
      }
      Ordering::Equal => {}
    }
  }

  fn step_position(self: &mut Self) {
    self.pos.x += self.vel.x;
    self.pos.y += self.vel.y;
    self.pos.z += self.vel.z;
  }
}

type P1Out = usize;
type P2Out = usize;
type Parsed = Vec<Particle>;

fn step_simulation(ps: &mut [Particle]) {
  for i in 0..(ps.len() - 1) {
    let (left, right) = ps.split_at_mut(i + 1);
    for other in right.iter_mut() {
      left[i].step_gravity(other);
    }
  }

  for p in ps.iter_mut() {
    p.step_position();
  }
}

fn score_particles(particles: &[Particle]) -> usize {
  particles.iter().map(|p| p.energy()).sum()
}

fn find_cycle_len(init_positions: Vec<i64>) -> usize {
  let l = init_positions.len();
  let mut positions = init_positions.clone();
  let init_velocities = vec![0_i64; l];
  let mut velocities = init_velocities.clone();

  for cycle_len in 1.. {
    for i in 0..(l - 1) {
      for j in (i + 1)..l {
        match positions[i].cmp(&positions[j]) {
          Ordering::Less => {
            velocities[i] += 1;
            velocities[j] -= 1;
          }
          Ordering::Greater => {
            velocities[i] -= 1;
            velocities[j] += 1;
          }
          Ordering::Equal => {}
        }
      }
    }
    for (p, v) in positions.iter_mut().zip(&velocities) {
      *p += *v;
    }
    if velocities == init_velocities && positions == init_positions {
      return cycle_len;
    }
  }
  0
}

fn gcd(a: usize, b: usize) -> usize {
  let mut x = a;
  let mut y = b;
  while y != 0 {
    let t = y;
    y = x % y;
    x = t;
  }
  x
}

fn lcm(a: usize, b: usize) -> usize {
  a * b / gcd(a, b)
}

struct Solver;
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(
      lines
        .iter()
        .map(|line| {
          // Lines look like this, which is trivial to parse without an expensive regex.
          // <x=2, y=-10, z=-7>
          let (x_raw, r0) = line[3..(line.len() - 1)].split_once(", y=").unwrap();
          let (y_raw, z_raw) = r0.split_once(", z=").unwrap();
          Particle {
            pos: Point3d {
              x: x_raw.parse().unwrap(),
              y: y_raw.parse().unwrap(),
              z: z_raw.parse().unwrap(),
            },
            vel: Point3d { x: 0, y: 0, z: 0 },
          }
        })
        .collect(),
    )
  }

  fn part1(&self, in_particles: &Parsed, sample_name: Option<String>) -> Result<P1Out> {
    let iterations = match sample_name {
      Some(s) if s == "test01" => 10,
      Some(s) if s == "test02" => 100,
      None => 1_000,
      _ => bail!("Unrecognized test, how many iterations should it do?"),
    };

    let mut particles = in_particles.clone();

    for _ in 0..iterations {
      step_simulation(&mut particles);
    }

    Ok(score_particles(&particles))
  }

  fn part2(&self, in_particles: &Parsed, _: Option<String>) -> Result<P2Out> {
    // Two facts make it so we can avoid simulating the entire cycle:
    // - Since the step function is both 1:1 and reversible, the cycle must include the initial state
    // - The x, y, and z axes are affected independently, so we can simulate them independently
    // Therefore, we can simulate each axis until it returns to that axis' initial state,
    // then find the lowest common multiple of the three cycle lengths.
    let cycle_len_x = find_cycle_len(in_particles.iter().map(|p| p.pos.x).collect());
    let cycle_len_y = find_cycle_len(in_particles.iter().map(|p| p.pos.y).collect());
    let cycle_len_z = find_cycle_len(in_particles.iter().map(|p| p.pos.z).collect());
    Ok(lcm(cycle_len_x, lcm(cycle_len_y, cycle_len_z)))
  }
}

fn main() -> Result<()> {
  Solver {}.run(2019, 12)
}
