#[cfg(feature = "alloc")]
use alloc::collections::{
  BTreeMap,
  BTreeSet,
  BinaryHeap,
  LinkedList,
  VecDeque,
};
#[cfg(feature = "hashmap")]
use core::hash::{
  BuildHasher,
  Hash,
};
#[cfg(feature = "hashmap")]
use std::collections::{
  HashMap,
  HashSet,
};

#[cfg(feature = "smallvec")]
use smallvec::SmallVec;

/// Abstracts something which can extend an `&mut self`.
pub trait Extend {
  /// item of the collection
  type Item;

  /// Used to extend a collection with an IntoIterator object.
  fn extend<Items>(&mut self, items: Items)
  where
    Items: IntoIterator<Item = Self::Item>;
}

#[cfg(feature = "alloc")]
impl<Item> Extend for alloc::vec::Vec<Item> {
  type Item = Item;

  fn extend<Items>(&mut self, items: Items)
  where
    Items: IntoIterator<Item = Item>,
  {
    core::iter::Extend::extend(self, items);
  }
}

#[cfg(feature = "alloc")]
impl<Item> Extend for VecDeque<Item> {
  type Item = Item;

  fn extend<Items>(&mut self, items: Items)
  where
    Items: IntoIterator<Item = Self::Item>,
  {
    core::iter::Extend::extend(self, items);
  }
}

#[cfg(feature = "alloc")]
impl<Key: Ord, Value> Extend for BTreeMap<Key, Value> {
  type Item = (Key, Value);

  fn extend<Items>(&mut self, items: Items)
  where
    Items: IntoIterator<Item = Self::Item>,
  {
    core::iter::Extend::extend(self, items);
  }
}

#[cfg(feature = "alloc")]
impl<Item: Ord> Extend for BinaryHeap<Item> {
  type Item = Item;

  fn extend<Items>(&mut self, items: Items)
  where
    Items: IntoIterator<Item = Self::Item>,
  {
    core::iter::Extend::extend(self, items);
  }
}

#[cfg(feature = "hashmap")]
impl<Key: Eq + Hash, Value, Seed: BuildHasher> Extend for HashMap<Key, Value, Seed> {
  type Item = (Key, Value);

  fn extend<Items>(&mut self, items: Items)
  where
    Items: IntoIterator<Item = Self::Item>,
  {
    core::iter::Extend::extend(self, items);
  }
}

#[cfg(feature = "hashmap")]
impl<Item: Eq + Hash, Seed: BuildHasher> Extend for HashSet<Item, Seed> {
  type Item = Item;

  fn extend<Items>(&mut self, items: Items)
  where
    Items: IntoIterator<Item = Self::Item>,
  {
    core::iter::Extend::extend(self, items);
  }
}

#[cfg(feature = "alloc")]
impl<Item: Ord> Extend for BTreeSet<Item> {
  type Item = Item;

  fn extend<Items>(&mut self, items: Items)
  where
    Items: IntoIterator<Item = Self::Item>,
  {
    core::iter::Extend::extend(self, items);
  }
}

#[cfg(feature = "alloc")]
impl<Item> Extend for LinkedList<Item> {
  type Item = Item;

  fn extend<Items>(&mut self, items: Items)
  where
    Items: IntoIterator<Item = Self::Item>,
  {
    core::iter::Extend::extend(self, items);
  }
}

#[cfg(feature = "alloc")]
impl Extend for alloc::string::String {
  type Item = char;

  fn extend<Items>(&mut self, items: Items)
  where
    Items: IntoIterator<Item = Self::Item>,
  {
    core::iter::Extend::extend(self, items);
  }
}

#[cfg(feature = "smallvec")]
impl<Item, const N: usize> Extend for SmallVec<[Item; N]> {
  type Item = Item;

  fn extend<Items>(&mut self, items: Items)
  where
    Items: IntoIterator<Item = Self::Item>,
  {
    core::iter::Extend::extend(self, items)
  }
}

impl Extend for () {
  type Item = ();

  fn extend<Items>(&mut self, _items: Items)
  where
    Items: IntoIterator<Item = Self::Item>,
  {
  }
}
