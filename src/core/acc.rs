use crate::core::Push;

/// This is very usefull to be use on combinator like fold.
/// For example, `.fold_bounds(.., Vec::new, Acc::acc)`.
pub trait Acc {
  /// Item stocked in the collection
  type Item;

  /// Accumulate item into Self. For example, for a vector that simply a push.
  fn acc(self, item: Self::Item) -> Self;
}

impl<T> Acc for T
where
  Self: Push,
{
  type Item = <T as Push>::Item;

  fn acc(mut self, item: Self::Item) -> Self {
    self.push(item);
    self
  }
}
