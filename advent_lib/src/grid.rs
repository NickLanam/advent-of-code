use fnv::FnvBuildHasher;
use std::collections::{HashMap, HashSet};

pub fn to_key(x: i32, y: i32) -> u64 {
  let xu = x as u32;
  let yu = y as u32;
  return ((xu as u64) << 32) + (yu as u64);
}

pub fn from_key(k: u64) -> (i32, i32) {
  let x = (k >> 32) as u32;
  let y = k as u32;
  return (x as i32, y as i32);
}

pub struct Infinite2dSet {
  state: HashSet<u64, FnvBuildHasher>,
}

impl Infinite2dSet {
  pub fn new(capacity: usize) -> Infinite2dSet {
    Infinite2dSet {
      state: HashSet::with_capacity_and_hasher(capacity, FnvBuildHasher::default()),
    }
  }

  pub fn len(&self) -> usize {
    self.state.len()
  }

  pub fn has(&self, x: i32, y: i32) -> bool {
    self.state.contains(&to_key(x, y))
  }

  pub fn add(&mut self, x: i32, y: i32) -> bool {
    self.state.insert(to_key(x, y))
  }

  pub fn remove(&mut self, x: i32, y: i32) -> bool {
    self.state.remove(&to_key(x, y))
  }

  pub fn toggle(&mut self, x: i32, y: i32) -> bool {
    let k = to_key(x, y);
    let is_present = self.state.contains(&k);
    if is_present {
      self.state.remove(&k)
    } else {
      self.state.insert(k)
    }
  }

  pub fn keys(&mut self) -> impl Iterator<Item = (i32, i32)> + use<'_> {
    self.state.iter().map(|k| from_key(*k))
  }
}

pub struct Infinite2dGrid<V> {
  state: HashMap<u64, V, FnvBuildHasher>,
}

impl<V> Infinite2dGrid<V> {
  pub fn new(capacity: usize) -> Infinite2dGrid<V> {
    Infinite2dGrid {
      state: HashMap::with_capacity_and_hasher(capacity, FnvBuildHasher::default()),
    }
  }

  pub fn has(&self, x: i32, y: i32) -> bool {
    self.state.contains_key(&to_key(x, y))
  }

  pub fn get(&self, x: i32, y: i32) -> Option<&V> {
    self.state.get(&to_key(x, y))
  }

  pub fn get_or_default<'a>(&'a self, x: i32, y: i32, default: &'a V) -> &'a V {
    self.get(x, y).unwrap_or(default)
  }

  pub fn set(&mut self, x: i32, y: i32, v: V) -> Option<V> {
    self.state.insert(to_key(x, y), v)
  }

  pub fn set_action(&mut self, x: i32, y: i32, action: fn(Option<&V>) -> Option<V>) {
    let existing = self.get(x, y);
    let next = action(existing);
    if next.is_some() {
      self.state.insert(to_key(x, y), next.unwrap());
    } else {
      self.state.remove(&to_key(x, y));
    }
  }

  pub fn remove(&mut self, x: i32, y: i32) -> Option<V> {
    self.state.remove(&to_key(x, y))
  }

  pub fn keys(&self) -> impl Iterator<Item = (i32, i32)> + use<'_, V> {
    self.state.keys().map(|k| from_key(*k))
  }

  pub fn values(&self) -> impl Iterator<Item = &V> {
    self.state.values()
  }

  pub fn entries(&self) -> impl Iterator<Item = (i32, i32, &V)> {
    self.state.iter().map(|(k, v)| {
      let (x, y) = from_key(*k);
      (x, y, v)
    })
  }
}