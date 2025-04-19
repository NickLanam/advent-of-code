use std::rc::Rc;

use advent_lib::runner::{Day, PartId};
use anyhow::Result;
use fnv::{FnvBuildHasher, FnvHashMap};
use itertools::Itertools;

#[derive(Copy, Clone, Debug, PartialEq)]
struct Particle {
  p: (f32, f32, f32),
  v: (f32, f32, f32),
  a: (f32, f32, f32),
}
impl Particle {
  fn find_collision(&self, &other: &Particle) -> Vec<usize> {
    // First, check if they'll ever collide on the X axis (and when).
    // Then, check if they collide at that same time (cheaper math!) on Y and Z.
    let a = (self.a.0 - other.a.0) / 2.0;
    let b = self.v.0 + (self.a.0) / 2.0 - other.v.0 - (other.a.0) / 2.0;
    let c = self.p.0 - other.p.0;

    // The position at time t of a particle is this quadratic equation: x = px + vx*t + ax*t*(t + 1)/2
    // We're looking for values of t where self.x == other.x at that t, so do some algebra:
    let mut answers: Vec<f32> = vec![];
    if a == 0.0 && b != 0.0 {
      answers.push(-c / b);
    } else if a != 0.0 {
      let ac4 = 4.0 * a * c;
      let bb = b * b;
      if bb < ac4 {
        return vec![];
      } else if bb == ac4 {
        answers.push(-b / (2.0 * a));
      } else {
        let root = (bb - ac4).sqrt();
        answers.push((-b + root) / (2.0 * a));
        answers.push((-b - root) / (2.0 * a));
      }
    }

    answers.retain(|&t| {
      // These times collide on the x axis, but do they also collide on y and z?
      t >= 0.0 && t.floor() == t && self.yz_at_time(t) == other.yz_at_time(t)
    });

    answers.iter().map(|&f| f as usize).collect()
  }

  fn yz_at_time(&self, t: f32) -> (f32, f32) {
    let y = self.p.1 + (self.v.1 * t) + (self.a.1 * t * (t + 1.0) / 2.0);
    let z = self.p.2 + (self.v.2 * t) + (self.a.2 * t * (t + 1.0) / 2.0);
    (y, z)
  }
}

type P1Out = usize;
type P2Out = usize;
type Parsed = Vec<Particle>;

struct Solver {}
impl Day<Parsed, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<Parsed> {
    Ok(
      lines
        .iter()
        .map(|line| {
          let mut parts = line.split(", ").map(|part| {
            let mut nums = part[3..part.len() - 1]
              .split(',')
              .map(|n| n.parse::<f32>().unwrap());
            (
              nums.next().unwrap(),
              nums.next().unwrap(),
              nums.next().unwrap(),
            )
          });
          Particle {
            p: parts.next().unwrap(),
            v: parts.next().unwrap(),
            a: parts.next().unwrap(),
          }
        })
        .collect(),
    )
  }

  fn part1(&self, particles: &Parsed, _: Option<String>) -> Result<P1Out> {
    let mut distances: Vec<(usize, f32, f32, f32)> = particles
      .iter()
      .enumerate()
      .map(|(i, particle)| {
        (
          i,
          particle.p.0.abs() + particle.p.1.abs() + particle.p.2.abs(),
          particle.v.0.abs() + particle.v.1.abs() + particle.v.2.abs(),
          particle.a.0.abs() + particle.a.1.abs() + particle.a.2.abs(),
        )
      })
      .collect();
    distances.sort_by(|l, r| {
      let mut diff = l.3 - r.3;
      if diff == 0.0 {
        diff = l.2 - r.2;
      }
      if diff == 0.0 {
        diff = l.1 - r.1;
      }
      match diff {
        ..0.0 => std::cmp::Ordering::Less,
        0.0 => std::cmp::Ordering::Equal,
        1.0.. => std::cmp::Ordering::Greater,
        _ => std::cmp::Ordering::Equal, // NaN and Infinities
      }
    });
    Ok(distances[0].0)
  }

  fn part2(&self, in_particles: &Parsed, _: Option<String>) -> Result<P2Out> {
    // Since two particles colliding removes them at that timestamp, before any other
    // collisions with the same two particles can happen, we can't just remove particles
    // that would _ever_ collide with another.

    // Instead, we need to find the collision points and what times they would occur,
    // then sort them by earliest time first. Then for each collision found this way,
    // only remove the particles involved if more than one of them still exists by then.
    // The size of the remaining list is the answer to the question.

    let mut particles: Vec<Rc<Particle>> = in_particles.iter().map(|p| Rc::new(*p)).collect();

    // First: for each particle, store times where it would collide with each other.
    // Sort this earliest-collision-first.
    let mut collisions_by_time: FnvHashMap<usize, Vec<Rc<Particle>>> =
      FnvHashMap::with_hasher(FnvBuildHasher::default());
    for (i, a) in particles.iter().enumerate().take(in_particles.len() - 1) {
      for b in particles[i + 1..].iter() {
        for t in a.find_collision(b) {
          let collisions = collisions_by_time.entry(t).or_default();
          if !collisions.contains(a) {
            collisions.push(a.clone());
          }
          if !collisions.contains(b) {
            collisions.push(b.clone());
          }
        }
      }
    }

    // Remove particles as they collide, but ignore collisions where all but one
    // has already been removed by an earlier collision.
    for &t in collisions_by_time.keys().sorted() {
      let alive: Vec<Rc<Particle>> = collisions_by_time
        .get(&t)
        .unwrap()
        .iter()
        .filter(|&p| particles.contains(p))
        .cloned()
        .collect();
      if alive.len() > 1 {
        particles.retain(|p1| !alive.contains(p1));
      }
    }

    Ok(particles.len())
  }
}

fn main() -> Result<()> {
  Solver {}.run(2017, 20)
}
