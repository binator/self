use alloc::{
  rc::Rc,
  string::String,
};
use core::{
  convert::Infallible,
  fmt::Debug,
  ops::Range,
};

use crate::{
  Split,
  Streaming,
  Success,
};

#[derive(Debug, PartialEq, Eq)]
pub struct StringStream {
  string: Rc<String>,
  range: Range<usize>,
}

impl StringStream {
  pub fn new(string: String) -> Self {
    Self {
      range: 0..string.len(),
      string: Rc::new(string),
    }
  }
}

impl Clone for StringStream {
  fn clone(&self) -> Self {
    Self {
      string: self.string.clone(),
      range: self.range.clone(),
    }
  }
}

impl AsRef<[u8]> for StringStream {
  fn as_ref(&self) -> &[u8] {
    &self.string.as_bytes()[self.range.clone()]
  }
}

impl Streaming for StringStream {
  type Error = Infallible;
  type Item = char;
  type Span = Self;

  fn split_first(self) -> Split<Self::Item, Self, Self::Error> {
    let stream = &self.string[self.range.clone()];
    let mut iter = stream.chars();
    match iter.next() {
      Some(c) => Split::Success {
        item: c,
        stream: Self {
          range: self.range.end + (stream.len() - iter.as_str().len())..self.range.end,
          string: self.string,
        },
      },
      None => Split::NotEnoughItem(self),
    }
  }

  fn all(self) -> Result<Success<Self::Span, Self>, Self::Error> {
    let empty = Self {
      string: self.string.clone(),
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
        string: self.string,
        range: self.range.start..self.range.start - other.range.start,
      })
    } else {
      Err(self)
    }
  }
}
