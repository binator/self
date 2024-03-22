#[cfg(feature = "alloc")]
use alloc::collections::{
  BTreeMap,
  BTreeSet,
  BinaryHeap,
  LinkedList,
  TryReserveError,
  VecDeque,
};
use core::convert::Infallible;
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

/// Abstracts something which can try extend `&mut self`.
// FIXME https://github.com/rust-lang/rust/issues/48043
pub trait TryExtend {
  /// Item stocked in the collection
  type Item;
  /// Error produced by this collection
  type Error;

  /// Used to expect a collection with an IntoIterator object.
  fn try_extend<Items>(&mut self, items: Items) -> Result<(), Self::Error>
  where
    Items: IntoIterator<Item = Self::Item>;
}

#[cfg(feature = "alloc")]
impl<Item> TryExtend for alloc::vec::Vec<Item> {
  type Error = TryReserveError;
  type Item = Item;

  fn try_extend<Items>(&mut self, items: Items) -> Result<(), Self::Error>
  where
    Items: IntoIterator<Item = Item>,
  {
    core::iter::Extend::extend(self, items);
    Ok(())
  }
}

#[cfg(feature = "alloc")]
impl<Item> TryExtend for VecDeque<Item> {
  type Error = TryReserveError;
  type Item = Item;

  fn try_extend<Items>(&mut self, items: Items) -> Result<(), Self::Error>
  where
    Items: IntoIterator<Item = Self::Item>,
  {
    core::iter::Extend::extend(self, items);
    Ok(())
  }
}

#[cfg(feature = "alloc")]
impl<Key: Ord, Value> TryExtend for BTreeMap<Key, Value> {
  type Error = TryReserveError;
  type Item = (Key, Value);

  fn try_extend<Items>(&mut self, items: Items) -> Result<(), Self::Error>
  where
    Items: IntoIterator<Item = Self::Item>,
  {
    core::iter::Extend::extend(self, items);
    Ok(())
  }
}

#[cfg(feature = "alloc")]
impl<Item: Ord> TryExtend for BinaryHeap<Item> {
  type Error = TryReserveError;
  type Item = Item;

  fn try_extend<Items>(&mut self, items: Items) -> Result<(), Self::Error>
  where
    Items: IntoIterator<Item = Self::Item>,
  {
    core::iter::Extend::extend(self, items);
    Ok(())
  }
}

#[cfg(feature = "hashmap")]
impl<Key: Eq + Hash, Value, Seed: BuildHasher> TryExtend for HashMap<Key, Value, Seed> {
  type Error = TryReserveError;
  type Item = (Key, Value);

  fn try_extend<Items>(&mut self, items: Items) -> Result<(), Self::Error>
  where
    Items: IntoIterator<Item = Self::Item>,
  {
    core::iter::Extend::extend(self, items);
    Ok(())
  }
}

#[cfg(feature = "hashmap")]
impl<Item: Eq + Hash, Seed: BuildHasher> TryExtend for HashSet<Item, Seed> {
  type Error = TryReserveError;
  type Item = Item;

  fn try_extend<Items>(&mut self, items: Items) -> Result<(), Self::Error>
  where
    Items: IntoIterator<Item = Self::Item>,
  {
    core::iter::Extend::extend(self, items);
    Ok(())
  }
}

#[cfg(feature = "alloc")]
impl<Item: Ord> TryExtend for BTreeSet<Item> {
  type Error = TryReserveError;
  type Item = Item;

  fn try_extend<Items>(&mut self, items: Items) -> Result<(), Self::Error>
  where
    Items: IntoIterator<Item = Self::Item>,
  {
    core::iter::Extend::extend(self, items);
    Ok(())
  }
}

#[cfg(feature = "alloc")]
impl<Item> TryExtend for LinkedList<Item> {
  type Error = TryReserveError;
  type Item = Item;

  fn try_extend<Items>(&mut self, items: Items) -> Result<(), Self::Error>
  where
    Items: IntoIterator<Item = Self::Item>,
  {
    core::iter::Extend::extend(self, items);
    Ok(())
  }
}

#[cfg(feature = "alloc")]
impl TryExtend for alloc::string::String {
  type Error = TryReserveError;
  type Item = char;

  fn try_extend<Items>(&mut self, items: Items) -> Result<(), Self::Error>
  where
    Items: IntoIterator<Item = Self::Item>,
  {
    core::iter::Extend::extend(self, items);
    Ok(())
  }
}

#[cfg(feature = "smallvec")]
impl<Item, const N: usize> TryExtend for SmallVec<[Item; N]> {
  type Error = smallvec::CollectionAllocErr;
  type Item = Item;

  fn try_extend<Items>(&mut self, items: Items) -> Result<(), Self::Error>
  where
    Items: IntoIterator<Item = Self::Item>,
  {
    core::iter::Extend::extend(self, items);
    Ok(())
  }
}

impl TryExtend for () {
  type Error = Infallible;
  type Item = ();

  fn try_extend<Items>(&mut self, _items: Items) -> Result<(), Self::Error>
  where
    Items: IntoIterator<Item = Self::Item>,
  {
    Ok(())
  }
}
