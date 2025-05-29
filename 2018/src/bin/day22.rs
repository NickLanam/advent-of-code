use advent_lib::{
  grid::Infinite2dGrid,
  runner::{Day, PartId},
};
use anyhow::{Result, bail};
use fnv::{FnvBuildHasher, FnvHashMap, FnvHashSet};

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
enum TileType {
  Rocky,
  Wet,
  Narrow,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
enum Equipment {
  Neither,
  Torch,
  Climbing,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Tile {
  erosion: usize,
  tile_type: TileType,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Cursor {
  x: i32,
  y: i32,
  equipment: Equipment,
}

const STRETCH: usize = 50;

struct ClimbingMap {
  target_x: i32,
  target_y: i32,
  map: Infinite2dGrid<Tile>,
}
impl ClimbingMap {
  fn parse(lines: &[String]) -> Result<ClimbingMap> {
    let depth: usize = lines[0].split_once(": ").unwrap().1.parse()?;
    let (a, b) = lines[1]
      .split_once(": ")
      .unwrap()
      .1
      .split_once(',')
      .unwrap();
    let (target_x, target_y) = (a.parse()?, b.parse()?);

    // Pre-fill the map here, to speed up and simplify logic in both parts later
    let mut map: Infinite2dGrid<Tile> =
      Infinite2dGrid::new((target_x as usize + STRETCH) * (target_y as usize + STRETCH));
    for y in 0_i32..=target_y + (STRETCH as i32) {
      for x in 0_i32..=target_x + (STRETCH as i32) {
        let index = if (x == 0 && y == 0) || (x == target_x && y == target_y) {
          0
        } else if x == 0 {
          (y.unsigned_abs() as usize) * 48_271
        } else if y == 0 {
          (x.unsigned_abs() as usize) * 16_807
        } else {
          map.get(x - 1, y).expect("Bad iteration order").erosion
            * map.get(x, y - 1).expect("Bad iteration order").erosion
        };
        let erosion = (index + depth) % 20_183;
        let tile_type = match erosion % 3 {
          0 => TileType::Rocky,
          1 => TileType::Wet,
          2 => TileType::Narrow,
          _ => {
            unreachable!()
          }
        };
        map.insert(x, y, Tile { erosion, tile_type });
      }
    }

    Ok(ClimbingMap {
      target_x,
      target_y,
      map,
    })
  }

  fn tile_type_at(&self, x: i32, y: i32) -> TileType {
    self
      .map
      .get(x, y)
      .unwrap_or_else(|| panic!("Tried to get a tile that wasn't pre-populated: ({x}, {y})"))
      .tile_type
  }

  fn valid_moves(
    &self,
    cursor: &Cursor,
    seen: &FnvHashSet<Cursor>,
  ) -> Result<Vec<(Cursor, usize)>> {
    let mut neighbors: Vec<(Cursor, usize)> = vec![];
    let tile_type = self.tile_type_at(cursor.x, cursor.y);

    // Equipment change (exacly one should be possible)
    if let Some(next_equipment) = match (tile_type, cursor.equipment) {
      (TileType::Rocky, Equipment::Neither) => bail!("Cannot wear none on rocky"),
      (TileType::Rocky, Equipment::Torch) => Some(Equipment::Climbing),
      (TileType::Rocky, Equipment::Climbing) => Some(Equipment::Torch),
      (TileType::Wet, Equipment::Neither) => Some(Equipment::Climbing),
      (TileType::Wet, Equipment::Torch) => bail!("Cannot wear torch on wet"),
      (TileType::Wet, Equipment::Climbing) => Some(Equipment::Neither),
      (TileType::Narrow, Equipment::Neither) => Some(Equipment::Torch),
      (TileType::Narrow, Equipment::Torch) => Some(Equipment::Neither),
      (TileType::Narrow, Equipment::Climbing) => bail!("Cannot wear climbing on narrow"),
    } {
      neighbors.push((
        Cursor {
          equipment: next_equipment,
          ..*cursor
        },
        7,
      ));
    }

    // Movement without an equipment change (maybe 0, maybe 4)
    for (nx, ny) in [
      (cursor.x + 1, cursor.y),
      (cursor.x, cursor.y + 1),
      (cursor.x - 1, cursor.y),
      (cursor.x, cursor.y - 1),
    ] {
      if nx >= 0 && ny >= 0 {
        let neighbor_tile_type = self.tile_type_at(nx, ny);
        match (cursor.equipment, neighbor_tile_type) {
          (Equipment::Neither, TileType::Rocky)
          | (Equipment::Torch, TileType::Wet)
          | (Equipment::Climbing, TileType::Narrow) => {
            continue;
          }
          _ => {
            neighbors.push((
              Cursor {
                x: nx,
                y: ny,
                equipment: cursor.equipment,
              },
              1,
            ));
          }
        }
      }
    }

    // Don't explore a state that has already found its optimal solution.
    neighbors.retain(|(n, _c)| !seen.contains(n));

    Ok(neighbors)
  }
}

fn choose_cursor(
  seen: &FnvHashSet<Cursor>,
  dist: &FnvHashMap<Cursor, usize>,
  goal: &Cursor,
) -> Result<(Cursor, usize)> {
  let mut closest_manhattan = usize::MAX;
  let mut closest_score = usize::MAX;
  let mut cursor: (Cursor, usize) = (
    Cursor {
      x: -1,
      y: -1,
      equipment: Equipment::Torch,
    },
    usize::MAX,
  );
  for entry in dist.iter() {
    let (option, &score) = entry;
    // Bounds check helps a little here
    if !seen.contains(option)
      && option.x < goal.x + (STRETCH as i32)
      && option.y < goal.y + (STRETCH as i32)
    {
      // Cheapest first, but break dies on closest to goal
      let manhattan = ((option.x.abs_diff(goal.x)) + (option.y.abs_diff(goal.y))) as usize;
      if score < closest_score || (score == closest_score && manhattan < closest_manhattan) {
        closest_manhattan = manhattan;
        closest_score = score;
        cursor = (*option, score);
      }
    }
  }
  if cursor.0.x < 0 || cursor.0.y < 0 {
    bail!("Cursor would be out of bounds: {cursor:?}");
  }
  Ok(cursor)
}

type P1Out = usize;
type P2Out = usize;

struct Solver {}
impl Day<ClimbingMap, P1Out, P2Out> for Solver {
  fn parse(&self, lines: Vec<String>, _: Option<String>, _: PartId) -> Result<ClimbingMap> {
    ClimbingMap::parse(&lines)
  }

  fn part1(&self, map: &ClimbingMap, _: Option<String>) -> Result<P1Out> {
    let mut score = 0;
    for y in 0..=map.target_y {
      for x in 0..=map.target_x {
        score += match map.tile_type_at(x, y) {
          TileType::Rocky => 0,
          TileType::Wet => 1,
          TileType::Narrow => 2,
        };
      }
    }
    Ok(score)
  }

  // Dijkstra's, using (x, y, equipment) as the nodes.
  // Edges are equipment swaps and moves to tiles with allowed matching equipment.
  fn part2(&self, map: &ClimbingMap, _: Option<String>) -> Result<P2Out> {
    let goal = Cursor {
      x: map.target_x,
      y: map.target_y,
      equipment: Equipment::Torch,
    };

    let capacity = (((goal.x + (STRETCH as i32)).unsigned_abs()
      * (goal.y + (STRETCH as i32)).unsigned_abs())
      * 2) as usize;
    let mut seen: FnvHashSet<Cursor> =
      FnvHashSet::with_capacity_and_hasher(capacity, FnvBuildHasher::default());
    let mut dist: FnvHashMap<Cursor, usize> =
      FnvHashMap::with_capacity_and_hasher(capacity, FnvBuildHasher::default());
    dist.insert(
      Cursor {
        x: 0,
        y: 0,
        equipment: Equipment::Torch,
      },
      0,
    );

    for _ in 0..capacity {
      let (cursor, dist_cursor) = choose_cursor(&seen, &dist, &goal)?;
      seen.insert(cursor);

      let neighbors = map.valid_moves(&cursor, &seen)?;

      for (neighbor, move_cost) in neighbors {
        let alt = dist_cursor + move_cost;
        let entry = dist.entry(neighbor).or_insert(alt);
        *entry = (*entry).min(alt);
      }

      if cursor == goal {
        return Ok(dist_cursor);
      }
    }

    bail!("Never found the target");
  }
}

fn main() -> Result<()> {
  Solver {}.run(2018, 22)
}
