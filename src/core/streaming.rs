use stdcore::{
  convert::Infallible,
  fmt::Debug,
  ops::{
    ControlFlow,
    FromResidual,
    Try,
  },
};

use crate::core::Success;

/// This trait must be implement by all struct that want to be a stream for
/// binator.
pub trait Streaming: Sized + Clone + Eq + Debug {
  /// Item produced by the stream
  type Item: Debug;
  /// Error that a stream can produce
  type Error: Debug;
  // considering remove it
  /// Span used to represent a delimited part of the stream
  type Span: Debug + Streaming;

  /// Will remove the first item from the stream
  fn split_first(self) -> Split<Self::Item, Self, Self::Error>;
  /// Will split the stream in half
  fn split_at(self, mid: usize) -> Split<Self::Span, Self, Self::Error>;
  /// Will remove the last item from the stream, not this is know as
  /// backtracking. It's generally not recommanded to do this, not all stream
  /// will be able to implement this
  fn split_last(self) -> Split<Self::Item, Self, Self::Error>;

  /// Will return all possible Item from the stream, note that again use this
  /// with caution.
  fn all(self) -> Result<Success<Self::Span, Self>, Self::Error>;
  /// Allow to get a span between two stream, self must be a stream earlier than
  /// other
  // Maybe better error ?
  fn diff(self, other: &Self) -> Result<Self::Span, Self>;

  /// This should be used with maximum **care**. This is used to tell to the
  /// stream, "drop already read data". It's mean all data before the cursor
  /// of this stream is gone. It's invalidate any Stream or Span with cursor
  /// that point to purged data. The purpose of this is to avoid keep all data
  /// in memory if not needed. This should be call only by end user parser or
  /// be documented properly.
  fn consume(self) -> Self {
    self
  }

  // fn as_octet(&self) -> &[u8];
}

/// Represent split Result
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Split<Item, Stream, Error> {
  /// When split is successfull
  Success {
    /// Item produced by the stream
    item: Item,
    /// Rest of the stream
    stream: Stream,
  },
  /// The stream don't have enough item
  NotEnoughItem(Stream),
  /// The stream encouter an error and can't proccess more item
  Error(Error),
}

impl<Item, Stream, Error> FromResidual for Split<Item, Stream, Error> {
  fn from_residual(residual: Split<Infallible, Stream, Error>) -> Self {
    match residual {
      Split::Success { .. } => unreachable!(),
      Split::NotEnoughItem(stream) => Split::NotEnoughItem(stream),
      Split::Error(error) => Split::Error(error),
    }
  }
}

impl<Item, Stream, Error> Try for Split<Item, Stream, Error> {
  type Output = Success<Item, Stream>;
  type Residual = Split<Infallible, Stream, Error>;

  fn from_output(Success { token, stream }: Self::Output) -> Self {
    Split::Success {
      item: token,
      stream,
    }
  }

  fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
    match self {
      Split::Success {
        item: token,
        stream,
      } => ControlFlow::Continue(Success { token, stream }),
      Split::NotEnoughItem(stream) => ControlFlow::Break(Split::NotEnoughItem(stream)),
      Split::Error(error) => ControlFlow::Break(Split::Error(error)),
    }
  }
}

// impl<'a> Streaming for &'a str {
//   type Error = Infallible;
//   type Item = char;
//   type Span = &'a str;

//   fn split_first(self) -> SplitResult<Self::Item, Self, Self::Error> {
//     let mut chars = self.chars();
//     if let Some(first) = chars.next() {
//       SplitResult::Ok {
//         token: first,
//         stream: chars.as_str(),
//       }
//     } else {
//       SplitResult::NotEnoughItem(self)
//     }
//   }

//   fn split_at(self, mid: usize) -> SplitResult<Self, Self, Self::Error> {
//     if let Some((mid, _)) = self.char_indices().nth(mid) {
//       SplitResult::Ok {
//         token: &self[..mid],
//         stream: &self[mid..],
//       }
//     } else {
//       SplitResult::NotEnoughItem(self)
//     }
//   }

//   fn split_last(self) -> SplitResult<Self::Item, Self, Self::Error> {
//     let mut chars = self.chars();
//     if let Some(c) = chars.next_back() {
//       SplitResult::Ok {
//         token: c,
//         stream: chars.as_str(),
//       }
//     } else {
//       SplitResult::NotEnoughItem(self)
//     }
//   }

//   fn all(self) -> Result<(Self, Self), Self::Error> {
//     Ok((self, &self[self.len()..]))
//   }

//   fn diff(self, other: &Self) -> Result<Self, Self> {
//     if let Some(ret) = (self.len() >= other.len())
//       .then(|| ..self.len() - other.len())
//       .and_then(|range| self.get(range))
//     {
//       Ok(ret)
//     } else {
//       Err(self)
//     }
//   }

//   fn take(self, n: usize) -> SplitResult<Self, Self, Self::Error> {
//     if self.is_char_boundary(n) {
//       SplitResult::Ok {
//         token: &self[..n],
//         stream: &self[n..],
//       }
//     } else {
//       SplitResult::NotEnoughItem(self)
//     }
//   }
// }

impl<'a> Streaming for &'a [u8] {
  type Error = Infallible;
  type Item = u8;
  type Span = &'a [u8];

  fn split_first(self) -> Split<Self::Item, Self, Self::Error> {
    if let [first, stream @ ..] = self {
      Split::Success {
        item: *first,
        stream,
      }
    } else {
      Split::NotEnoughItem(self)
    }
  }

  fn split_at(self, mid: usize) -> Split<Self, Self, Self::Error> {
    if mid <= self.len() {
      let (head, tail) = <[u8]>::split_at(self, mid);
      Split::Success {
        item: head,
        stream: tail,
      }
    } else {
      Split::NotEnoughItem(self)
    }
  }

  fn split_last(self) -> Split<Self::Item, Self, Self::Error> {
    if let [head @ .., last] = self {
      Split::Success {
        item: *last,
        stream: head,
      }
    } else {
      Split::NotEnoughItem(self)
    }
  }

  fn all(self) -> Result<Success<Self::Span, Self>, Self::Error> {
    Ok(Success {
      token: self,
      stream: &self[self.len()..],
    })
  }

  fn diff(self, other: &Self) -> Result<Self, Self> {
    if let Some(ret) = self
      .len()
      .checked_sub(other.len())
      .filter(|&offset| self[offset..].as_ptr() == other.as_ptr())
      .and_then(|offset| self.get(..offset))
    {
      Ok(ret)
    } else {
      Err(self)
    }
  }
}

// struct StreamIter<Stream> {
//   stream: Stream,
// }

// impl<Stream> Iterator for StreamIter<Stream>
// where
//   Stream: Streaming + Clone,
// {
//   type Item = Stream::Item;

//   fn next(&mut self) -> std::option::Option<<Self as Iterator>::Item> {
//     match self.stream.clone().split_first() {
//       SplitResult::Ok { token, stream } => {
//         self.stream = stream;
//         Some(token)
//       }
//       _ => None,
//     }
//   }
// }

#[cfg(test)]
mod tests {
  use super::{
    Split,
    Streaming,
  };

  #[test]
  fn split_first_slice() {
    let stream = &b"abcd"[..];
    let expected = Split::Success {
      item: b'a',
      stream: &stream[1..],
    };
    assert_eq!(Streaming::split_first(stream), expected);

    let stream = &b""[..];
    let expected = Split::NotEnoughItem(stream);
    assert_eq!(Streaming::split_first(stream), expected);
  }

  // #[test]
  // fn split_first_str() {
  //   let stream = &"abcd"[..];
  //   let expected = SplitResult::Ok {
  //     token: 'a',
  //     stream: &stream[1..],
  //   };
  //   assert_eq!(Streaming::split_first(stream), expected);

  //   let stream = &b""[..];
  //   let expected = SplitResult::NotEnoughItem(stream);
  //   assert_eq!(Streaming::split_first(stream), expected);
  // }

  #[test]
  fn split_at_slice() {
    let stream = &[0, 1, 2, 3, 4][..];
    for n in 0..stream.len() {
      let expected = Split::Success {
        item: &stream[..n],
        stream: &stream[n..],
      };
      assert_eq!(Streaming::split_at(stream, n), expected);
    }

    let n = stream.len() + 1;
    let expected = Split::NotEnoughItem(stream);
    assert_eq!(Streaming::split_at(stream, n), expected);
  }

  // #[test]
  // fn split_at_str() {
  //   let stream = &"abcd"[..];

  //   for (n, _) in stream.chars().enumerate() {
  //     let expected = SplitResult::Ok {
  //       token: &stream[..n],
  //       stream: &stream[n..],
  //     };
  //     assert_eq!(Streaming::split_at(stream, n), expected);
  //   }

  //   let n = stream.len() + 1;
  //   let expected = SplitResult::NotEnoughItem(stream);
  //   assert_eq!(Streaming::split_at(stream, n), expected);
  // }

  #[test]
  fn split_last_slice() {
    let stream = &b"abcd"[..];
    let expected = Split::Success {
      item: b'd',
      stream: &stream[..3],
    };
    assert_eq!(Streaming::split_last(stream), expected);

    let stream = &b""[..];
    let expected = Split::NotEnoughItem(stream);
    assert_eq!(Streaming::split_last(stream), expected);
  }

  // #[test]
  // fn split_last_str() {
  //   let stream = &"abcd"[..];
  //   let expected = SplitResult::Ok {
  //     token: 'd',
  //     stream: &stream[..3],
  //   };
  //   assert_eq!(Streaming::split_last(stream), expected);

  //   let stream = &b""[..];
  //   let expected = SplitResult::NotEnoughItem(stream);
  //   assert_eq!(Streaming::split_last(stream), expected);
  // }

  #[test]
  fn diff_slice_all() {
    let stream = &[0, 1, 2, 3, 4, 5, 6][..];
    assert_eq!(stream.diff(&&stream[stream.len()..]), Ok(&stream[..]));
  }

  #[test]
  fn diff_slice_mid() {
    let stream = &[0, 1, 2, 3, 4, 5, 6][..];
    assert_eq!(
      stream.diff(&&stream[stream.len() / 2..]),
      Ok(&stream[..stream.len() / 2])
    );
  }

  #[test]
  fn diff_slice_error() {
    let stream = &[0, 1, 2, 3, 4, 5, 6][..];
    assert_eq!(
      stream[..stream.len() / 2].as_ref().diff(&&stream[..]),
      Err(&stream[..stream.len() / 2])
    );
  }

  // #[test]
  // fn offset_str_same() {
  //   let stream = &"abcdefg"[..];
  //   assert_eq!(stream.diff(&&stream[stream.len()..]), Ok(&stream[..]));
  // }

  // #[test]
  // fn offset_str_after() {
  //   let stream = &"abcdefg"[..];
  //   assert_eq!(
  //     stream.diff(&&stream[stream.len() / 2..]),
  //     Ok(&stream[..stream.len() / 2])
  //   );
  // }

  // #[test]
  // fn offset_str_before() {
  //   let stream = &"abcdefg"[..];
  //   assert_eq!(
  //     (&stream[..stream.len() / 2]).diff(&&stream[..]),
  //     Err(&stream[..stream.len() / 2])
  //   );
  // }
}
