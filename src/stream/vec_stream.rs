use alloc::rc::Rc;
use core::{
  convert::Infallible,
  fmt::Debug,
  ops::Range,
};

use crate::core::{
  Split,
  Streaming,
  Success,
};

/// A stream that return Data from a `Vec<u8>`
/// This can be used if you want give ownership of data to
/// the stream. This allow to return Context that reference
/// Span from Stream
#[derive(Debug, PartialEq, Eq)]
pub struct VecStream {
  vec: Rc<Vec<u8>>,
  range: Range<usize>,
}

impl VecStream {
  /// Return a new `VecStream` from a `Vec<u8>`
  pub fn new(vec: Vec<u8>) -> Self {
    Self {
      range: 0..vec.len(),
      vec: Rc::new(vec),
    }
  }
}

impl Clone for VecStream {
  fn clone(&self) -> Self {
    Self {
      vec: self.vec.clone(),
      range: self.range.clone(),
    }
  }
}

impl AsRef<[u8]> for VecStream {
  fn as_ref(&self) -> &[u8] {
    &self.vec[self.range.clone()]
  }
}

impl Streaming for VecStream {
  type Error = Infallible;
  type Item = u8;
  type Span = Self;

  fn split_first(self) -> Split<Self::Item, Self, Self::Error> {
    let stream = &self.vec[self.range.clone()];
    let mut iter = stream.iter();
    match iter.next() {
      Some(&o) => Split::Success {
        item: o,
        stream: Self {
          range: self.range.start + 1..self.range.end,
          vec: self.vec,
        },
      },
      None => Split::NotEnoughItem(self),
    }
  }

  fn all(self) -> Result<Success<Self::Span, Self>, Self::Error> {
    let empty = Self {
      vec: self.vec.clone(),
      range: self.range.end..self.range.end,
    };
    Ok(Success {
      token: self,
      stream: empty,
    })
  }

  fn split_at(self, _mid: usize) -> Split<Self, Self, Self::Error> {
    todo!()
  }

  fn split_last(self) -> Split<Self::Item, Self, Self::Error> {
    todo!()
  }

  fn diff(self, other: &Self) -> Result<Self::Span, Self> {
    if self.range.start >= other.range.start {
      Ok(Self {
        vec: self.vec,
        range: self.range.start..self.range.start - other.range.start,
      })
    } else {
      Err(self)
    }
  }
}
