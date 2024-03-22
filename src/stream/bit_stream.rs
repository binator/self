use core::{
  mem::size_of,
  ops::{
    BitAnd,
    Shl,
  },
};

use crate::core::{
  SplitError,
  SplitFirst,
  UtilsStream,
};
use num_traits::One;

const fn bits_of<Item>() -> usize {
  size_of::<Item>() * u8::BITS as usize
}

#[derive(Clone, PartialEq, Eq)]
pub struct BitStream<Stream: UtilsStream, Item = <Stream as UtilsStream>::Item> {
  stream: Stream,
  i: usize,
  cur: Item,
}

impl<Stream> From<Stream> for BitStream<Stream>
where
  Stream: UtilsStream,
  <Stream as UtilsStream>::Item: Default,
{
  fn from(stream: Stream) -> Self {
    Self::new(stream)
  }
}

impl<Stream: UtilsStream> BitStream<Stream> {
  pub fn new(stream: Stream) -> Self
  where
    <Stream as UtilsStream>::Item: Default,
  {
    Self {
      stream,
      i: bits_of::<<Stream as UtilsStream>::Item>(),
      cur: Default::default(),
    }
  }

  fn next(self) -> (bool, Self)
  where
    for<'a> &'a <Stream as UtilsStream>::Item: BitAnd<Token = <Stream as UtilsStream>::Item>,
    <Stream as UtilsStream>::Item: Shl<usize, Token = <Stream as UtilsStream>::Item>,
    <Stream as UtilsStream>::Item: PartialEq,
    <Stream as UtilsStream>::Item: One,
  {
    let flag = <Stream as UtilsStream>::Item::one() << self.i;
    (
      &self.cur & &flag == flag,
      Self {
        stream: self.stream,
        i: self.i + 1,
        cur: self.cur,
      },
    )
  }

  pub fn into_stream(self) -> Stream {
    self.stream
  }
}

impl<Stream: UtilsStream> UtilsStream for BitStream<Stream> {
  type Error = <Stream as UtilsStream>::Error;
  type Item = bool;
}

impl<Stream: SplitFirst> SplitFirst for BitStream<Stream>
where
  for<'a> &'a <Stream as UtilsStream>::Item: BitAnd<Token = <Stream as UtilsStream>::Item>,
  <Stream as UtilsStream>::Item: Shl<usize, Token = <Stream as UtilsStream>::Item>,
  <Stream as UtilsStream>::Item: PartialEq,
  <Stream as UtilsStream>::Item: One,
{
  fn split_first(self) -> Result<(Self::Item, Self), SplitError<Self, Self::Error>> {
    if self.i == bits_of::<<Stream as UtilsStream>::Item>() {
      let (cur, stream) = self.stream.split_first().map_err(|e| {
        e.map(|stream| {
          Self {
            stream,
            i: self.i,
            cur: self.cur,
          }
        })
      })?;

      Ok(
        Self {
          stream,
          i: 0,
          cur,
        }
        .next(),
      )
    } else {
      Ok(self.next())
    }
  }
}

// impl<Reader: Read, const N: usize> Subset for BitStream<Reader, N> {
//   fn subset(&self, other: &Self) -> Option<Self> {
//     match (&self.position, &other.position) {
//       (Position::RangeFrom(a), Position::RangeFrom(b)) => self.range(a.start,
// b.start),       (Position::RangeFrom(a), Position::Range(b)) =>
// self.range(a.start, b.start),       (Position::Range(a),
// Position::RangeFrom(b)) => self.range(a.start, b.start),       (Position::
// Range(a), Position::Range(b)) => self.range(a.start, b.start),     }
//   }
// }

// impl<Reader: Read, const N: usize> SplitEnd for BitStream<Reader, N> {
//   fn split_end(self) -> Result<(Self, Self), (Self::Error, Self)> {
//     match &self.position {
//       Position::RangeFrom(range) => {
//         let ret = self.buf.borrow_mut().read_all();
//         match ret {
//           Ok(end) => {
//             let empty = Self {
//               buf: self.buf.clone(),
//               position: Position::Range(end..end),
//             };
//             Ok((
//               Self {
//                 buf: self.buf,
//                 position: Position::Range(range.start..end),
//               },
//               empty,
//             ))
//           }
//           Err(error) => Err((error, self)),
//         }
//       }
//       Position::Range(range) => {
//         let empty = Self {
//           buf: self.buf.clone(),
//           position: Position::Range(range.end..range.end),
//         };
//         Ok((self, empty))
//       }
//     }
//   }
// }
