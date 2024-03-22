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
use smallvec::{
  CollectionAllocErr,
  SmallVec,
};

/// Abstracts something you can try push item into `&mut self`
// FIXME https://github.com/rust-lang/rust/issues/48043
pub trait TryPush {
  /// Item stocked in the collection
  type Item;
  /// The error returned by the collection if push fail
  type Error;
  /// Represent a way to access Item in the collection directly after push
  type ItemView<'a>
  where
    Self: 'a;

  /// Try to push an item into a collection, no guarantee on ordering.
  fn try_push<'a>(&'a mut self, item: Self::Item) -> Result<Self::ItemView<'a>, Self::Error>;
}

#[cfg(feature = "alloc")]
impl<Item> TryPush for alloc::vec::Vec<Item> {
  type Error = (Self::Item, TryReserveError);
  type Item = Item;
  type ItemView<'a> = &'a mut Self::Item
  where
    Self: 'a;

  fn try_push<'a>(&'a mut self, item: Self::Item) -> Result<Self::ItemView<'a>, Self::Error> {
    match self.try_reserve(1) {
      Ok(_) => {
        self.push(item);
        Ok(self.last_mut().unwrap())
      }
      Err(e) => Err((item, e)),
    }
  }
}

#[cfg(feature = "alloc")]
impl TryPush for alloc::string::String {
  type Error = (Self::Item, TryReserveError);
  type Item = char;
  type ItemView<'a> = Self::Item;

  fn try_push<'a>(&'a mut self, c: Self::Item) -> Result<Self::ItemView<'a>, Self::Error> {
    match self.try_reserve(1) {
      Ok(_) => {
        self.push(c);
        Ok(self.chars().next_back().unwrap())
      }
      Err(e) => Err((c, e)),
    }
  }
}

#[cfg(feature = "alloc")]
impl<Item> TryPush for VecDeque<Item> {
  type Error = (Self::Item, TryReserveError);
  type Item = Item;
  type ItemView<'a> = &'a mut Self::Item
  where
    Self: 'a;

  fn try_push<'a>(&'a mut self, item: Self::Item) -> Result<Self::ItemView<'a>, Self::Error> {
    match self.try_reserve(1) {
      Ok(_) => {
        self.push_back(item);
        Ok(self.back_mut().unwrap())
      }
      Err(e) => Err((item, e)),
    }
  }
}

#[cfg(feature = "alloc")]
impl<Key: Ord, Value> TryPush for BTreeMap<Key, Value> {
  type Error = (Self::Item, TryReserveError);
  type Item = (Key, Value);
  // Not happy
  type ItemView<'a> = Option<Value>
  where
    Self: 'a;

  fn try_push<'a>(&'a mut self, item: Self::Item) -> Result<Self::ItemView<'a>, Self::Error> {
    Ok(self.insert(item.0, item.1))
  }
}

#[cfg(feature = "alloc")]
impl<Item: Ord> TryPush for BinaryHeap<Item> {
  type Error = (Self::Item, TryReserveError);
  type Item = Item;
  // not happy
  type ItemView<'a> = ()
  where
    Self: 'a;

  fn try_push<'a>(&'a mut self, item: Self::Item) -> Result<Self::ItemView<'a>, Self::Error> {
    match self.try_reserve(1) {
      Ok(_) => {
        self.push(item);
        Ok(())
      }
      Err(e) => Err((item, e)),
    }
  }
}

#[cfg(feature = "hashmap")]
impl<Key: Eq + Hash, Value, Seed: BuildHasher> TryPush for HashMap<Key, Value, Seed> {
  type Error = (Self::Item, TryReserveError);
  type Item = (Key, Value);
  // Not happy
  type ItemView<'a> = Option<Value>
  where
    Self: 'a;

  fn try_push<'a>(&'a mut self, item: Self::Item) -> Result<Self::ItemView<'a>, Self::Error> {
    Ok(self.insert(item.0, item.1))
  }
}

#[cfg(feature = "hashmap")]
impl<Item: Eq + Hash, Seed: BuildHasher> TryPush for HashSet<Item, Seed> {
  type Error = (Self::Item, TryReserveError);
  type Item = Item;
  // Not happy
  type ItemView<'a> = bool
  where
    Self: 'a;

  fn try_push<'a>(&'a mut self, item: Self::Item) -> Result<Self::ItemView<'a>, Self::Error> {
    Ok(self.insert(item))
  }
}

#[cfg(feature = "alloc")]
impl<Item: Ord> TryPush for BTreeSet<Item> {
  type Error = (Self::Item, TryReserveError);
  type Item = Item;
  // Not happy
  type ItemView<'a> = bool
  where
    Self: 'a;

  fn try_push(&mut self, item: Self::Item) -> Result<Self::ItemView<'_>, Self::Error> {
    Ok(self.insert(item))
  }
}

#[cfg(feature = "alloc")]
impl<Item> TryPush for LinkedList<Item> {
  type Error = (Self::Item, TryReserveError);
  type Item = Item;
  type ItemView<'a> = &'a mut Self::Item
  where
    Self: 'a;

  fn try_push<'a>(&'a mut self, item: Self::Item) -> Result<Self::ItemView<'a>, Self::Error> {
    self.push_back(item);
    Ok(self.back_mut().unwrap())
  }
}

#[cfg(feature = "smallvec")]
impl<Item, const N: usize> TryPush for SmallVec<[Item; N]> {
  type Error = (Self::Item, CollectionAllocErr);
  type Item = Item;
  type ItemView<'a> = &'a mut Self::Item
  where
    Self: 'a;

  fn try_push<'a>(&'a mut self, item: Self::Item) -> Result<Self::ItemView<'a>, Self::Error> {
    match self.try_reserve(1) {
      Ok(_) => {
        self.push(item);
        Ok(self.last_mut().unwrap())
      }
      Err(e) => Err((item, e)),
    }
  }
}

impl TryPush for () {
  type Error = Infallible;
  type Item = ();
  type ItemView<'a> = ();

  fn try_push(&mut self, _: Self::Item) -> Result<Self::ItemView<'_>, Self::Error> {
    Ok(())
  }
}
