use alloc::rc::Rc;
use std::io::{
  self,
  Read,
};

use stdcore::{
  cell::UnsafeCell,
  fmt::Debug,
  slice::from_raw_parts_mut,
};

use crate::{
  core::{
    Split,
    Streaming,
    Success,
  },
  stream::Position,
};

#[derive(Debug)]
struct Buf<Reader: Read, const N: usize> {
  buf: Vec<u8>,
  reader: Reader,
}

impl<Reader: Read, const N: usize> Buf<Reader, N> {
  fn new(reader: Reader) -> Self {
    Self {
      buf: Vec::new(),
      reader,
    }
  }

  fn read_all(&mut self) -> Result<usize, io::Error> {
    while self.read()? == 0 {}
    Ok(self.buf.len())
  }

  fn read(&mut self) -> Result<usize, io::Error> {
    unsafe {
      self.buf.reserve(N);
      let len = self.buf.len();
      let ptr = self.buf.as_mut_ptr();
      match self.reader.read(from_raw_parts_mut(ptr.add(len), N)) {
        Ok(n) => {
          self.buf.set_len(len + n);
          Ok(n)
        }
        Err(error) => Err(error),
      }
    }
  }

  fn get(&mut self, i: usize) -> Option<Result<u8, io::Error>> {
    loop {
      if let Some(o) = self.buf.get(i) {
        break Some(Ok(*o));
      }
      match self.read() {
        Ok(n) => {
          if n == 0 {
            break None;
          }
        }
        Err(error) => break Some(Err(error)),
      }
    }
  }
}

/// Stream that will read grow as needed by reading into a Reader.
#[derive(Debug)]
pub struct ReaderStream<Reader: Read, const N: usize> {
  buf: Rc<UnsafeCell<Buf<Reader, N>>>,
  position: Position,
}

impl<Reader: Read, const N: usize> Clone for ReaderStream<Reader, N> {
  fn clone(&self) -> Self {
    Self {
      buf: self.buf.clone(),
      position: self.position.clone(),
    }
  }
}

impl<Reader: Read, const N: usize> ReaderStream<Reader, N> {
  /// Return a new ReaderStream from a Reader
  pub fn new(reader: Reader) -> Self {
    Self {
      buf: Rc::new(UnsafeCell::new(Buf::new(reader))),
      position: Position::RangeFrom(0..),
    }
  }
}

impl<Reader: Read + Debug, const N: usize> Streaming for ReaderStream<Reader, N> {
  type Error = io::Error;
  type Item = u8;
  type Span = Self;

  fn split_first(self) -> Split<Self::Item, Self, Self::Error> {
    unsafe {
      let (i, position) = self.position.next();
      match i.and_then(|i| (*self.buf.get()).get(i)) {
        Some(Ok(o)) => Split::Success {
          item: o,
          stream: Self {
            buf: self.buf,
            position,
          },
        },
        None => Split::NotEnoughItem(self),
        Some(Err(error)) => Split::Error(error),
      }
    }
  }

  fn all(self) -> Result<Success<Self::Span, Self>, Self::Error> {
    unsafe {
      match self.position {
        Position::RangeFrom(range) => {
          let ret = (*self.buf.get()).read_all();
          match ret {
            Ok(end) => {
              let empty = Self {
                buf: self.buf.clone(),
                position: Position::Range(end..end),
              };
              Ok(Success {
                token: Self {
                  buf: self.buf,
                  position: Position::Range(range.start..end),
                },
                stream: empty,
              })
            }
            Err(error) => Err(error),
          }
        }
        Position::Range(range) => {
          let empty = Self {
            buf: self.buf.clone(),
            position: Position::Range(range.end..range.end),
          };
          Ok(Success {
            token: Self {
              buf: self.buf,
              position: Position::Range(range),
            },
            stream: empty,
          })
        }
      }
    }
  }

  fn split_at(self, _mid: usize) -> Split<Self::Span, Self, Self::Error> {
    todo!()
  }

  fn split_last(self) -> Split<Self::Item, Self, Self::Error> {
    todo!()
  }

  fn diff(self, other: &Self) -> Result<Self::Span, Self> {
    match self.position.range(&other.position) {
      Some(range) => Ok(Self {
        buf: self.buf,
        position: Position::Range(range),
      }),
      None => Err(self),
    }
  }
}

impl<Reader: Read, const N: usize> PartialEq for ReaderStream<Reader, N> {
  fn eq(&self, other: &Self) -> bool {
    Rc::ptr_eq(&self.buf, &other.buf) && self.position == other.position
  }
}

impl<Reader: Read, const N: usize> Eq for ReaderStream<Reader, N> {}
