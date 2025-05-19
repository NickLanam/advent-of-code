use fnv::FnvBuildHasher;
use std::collections::{HashMap, HashSet, hash_map::Entry};

pub fn to_key(x: i32, y: i32) -> u64 {
  let xu = x as u32;
  let yu = y as u32;
  ((xu as u64) << 32) + (yu as u64)
}

pub fn from_key(k: u64) -> (i32, i32) {
  let x = (k >> 32) as u32;
  let y = k as u32;
  (x as i32, y as i32)
}

#[derive(Clone, PartialEq)]
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

  pub fn is_empty(&self) -> bool {
    self.state.is_empty()
  }

  pub fn contains(&self, x: i32, y: i32) -> bool {
    self.state.contains(&to_key(x, y))
  }

  pub fn insert(&mut self, x: i32, y: i32) -> bool {
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

  pub fn keys(&self) -> impl Iterator<Item = (i32, i32)> + use<'_> {
    self.state.iter().map(|k| from_key(*k))
  }
}

#[derive(Clone, PartialEq)]
pub struct Infinite2dGrid<V>
where
  V: Clone + PartialEq,
{
  state: HashMap<u64, V, FnvBuildHasher>,
}

impl<V: Clone + PartialEq> Infinite2dGrid<V> {
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

  pub fn get_or_set_default<F>(&mut self, x: i32, y: i32, create_default: F) -> &V
  where
    F: FnOnce() -> V,
  {
    if !self.has(x, y) {
      self.set(x, y, create_default());
    }
    self.get(x, y).unwrap()
  }

  pub fn set(&mut self, x: i32, y: i32, v: V) -> Option<V> {
    self.state.insert(to_key(x, y), v)
  }

  pub fn set_action(&mut self, x: i32, y: i32, action: fn(Option<&V>) -> Option<V>) {
    let existing = self.get(x, y);
    let next = action(existing);
    if let Some(next) = next {
      self.state.insert(to_key(x, y), next);
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

  pub fn entry(&mut self, x: i32, y: i32) -> Entry<'_, u64, V> {
    self.state.entry(to_key(x, y))
  }

  pub fn entries(&self) -> impl Iterator<Item = (i32, i32, &V)> {
    self.state.iter().map(|(k, v)| {
      let (x, y) = from_key(*k);
      (x, y, v)
    })
  }
}
