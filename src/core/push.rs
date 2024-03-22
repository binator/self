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

/// Abstracts something which can push Item into self
pub trait Push {
  /// Item stocked in the collection
  type Item;
  /// Represent a way to access Item in the collection directly after push
  type ItemView<'a>
  where
    Self: 'a;

  /// push an item into a collection, no guarantee on ordering.
  fn push<'a>(&'a mut self, item: Self::Item) -> Self::ItemView<'a>;
}

#[cfg(feature = "alloc")]
impl<Item> Push for alloc::vec::Vec<Item> {
  type Item = Item;
  type ItemView<'a> = &'a mut Self::Item
  where
    Self: 'a;

  #[allow(clippy::only_used_in_recursion)]
  fn push<'a>(&'a mut self, item: Self::Item) -> Self::ItemView<'a> {
    self.push(item);
    self.last_mut().unwrap()
  }
}

#[cfg(feature = "alloc")]
impl Push for alloc::string::String {
  type Item = char;
  type ItemView<'a> = Self::Item;

  fn push<'a>(&'a mut self, c: Self::Item) -> Self::ItemView<'a> {
    self.push(c);
    self.chars().next_back().unwrap()
  }
}

#[cfg(feature = "alloc")]
impl<Item> Push for VecDeque<Item> {
  type Item = Item;
  type ItemView<'a> = &'a mut Self::Item
  where
    Self: 'a;

  fn push<'a>(&'a mut self, item: Self::Item) -> Self::ItemView<'a> {
    self.push_back(item);
    self.back_mut().unwrap()
  }
}

#[cfg(feature = "alloc")]
impl<Key: Ord, Value> Push for BTreeMap<Key, Value> {
  type Item = (Key, Value);
  // Not happy
  type ItemView<'a> = Option<Value>
  where
    Self: 'a;

  fn push<'a>(&'a mut self, item: Self::Item) -> Self::ItemView<'a> {
    self.insert(item.0, item.1)
  }
}

#[cfg(feature = "alloc")]
impl<Item: Ord> Push for BinaryHeap<Item> {
  type Item = Item;
  // not happy
  type ItemView<'a> = ()
  where
    Self: 'a;

  fn push<'a>(&'a mut self, item: Self::Item) -> Self::ItemView<'a> {
    self.push(item);
  }
}

#[cfg(feature = "hashmap")]
impl<Key: Eq + Hash, Value, Seed: BuildHasher> Push for HashMap<Key, Value, Seed> {
  type Item = (Key, Value);
  // Not happy
  type ItemView<'a> = Option<Value>
  where
    Self: 'a;

  fn push<'a>(&'a mut self, item: Self::Item) -> Self::ItemView<'a> {
    self.insert(item.0, item.1)
  }
}

#[cfg(feature = "hashmap")]
impl<Item: Eq + Hash, Seed: BuildHasher> Push for HashSet<Item, Seed> {
  type Item = Item;
  // Not happy
  type ItemView<'a> = bool
  where
    Self: 'a;

  fn push<'a>(&'a mut self, item: Self::Item) -> Self::ItemView<'a> {
    self.insert(item)
  }
}

#[cfg(feature = "alloc")]
impl<Item: Ord> Push for BTreeSet<Item> {
  type Item = Item;
  // Not happy
  type ItemView<'a> = bool
  where
    Self: 'a;

  fn push(&mut self, item: Self::Item) -> Self::ItemView<'_> {
    self.insert(item)
  }
}

#[cfg(feature = "alloc")]
impl<Item> Push for LinkedList<Item> {
  type Item = Item;
  type ItemView<'a> = &'a mut Self::Item
  where
    Self: 'a;

  fn push<'a>(&'a mut self, item: Self::Item) -> Self::ItemView<'a> {
    self.push_back(item);
    self.back_mut().unwrap()
  }
}

#[cfg(feature = "smallvec")]
impl<Item, const N: usize> Push for SmallVec<[Item; N]> {
  type Item = Item;
  type ItemView<'a> = &'a mut Self::Item
  where
    Self: 'a;

  fn push<'a>(&'a mut self, item: Self::Item) -> Self::ItemView<'a> {
    self.push(item);
    self.last_mut().unwrap()
  }
}

impl Push for () {
  type Item = ();
  type ItemView<'a> = ();

  fn push(&mut self, _: Self::Item) -> Self::ItemView<'_> {}
}
