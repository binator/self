use crate::utils::TryPush;

/// This is very usefull to be use on combinator like try_fold.
/// For example, `.try_fold_bounds(.., || Ok(Vec::new), TryAcc::try_acc)`.
pub trait TryAcc: Sized {
  /// The error returned by the collection if push fail
  type Error;
  /// Item stocked in the collection
  type Item;

  /// Try to accumulate item into Self. For example, for a vector that simply a
  /// try_push.
  fn try_acc(self, item: Self::Item) -> Result<Self, Self::Error>;
}

impl<T> TryAcc for T
where
  Self: TryPush,
{
  type Error = <T as TryPush>::Error;
  type Item = <T as TryPush>::Item;

  fn try_acc(mut self, item: Self::Item) -> Result<Self, Self::Error> {
    match self.try_push(item).map(|_| ()) {
      Ok(_) => Ok(self),
      Err(e) => Err(e),
    }
  }
}
